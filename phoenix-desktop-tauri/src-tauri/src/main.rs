// Tauri backend (minimal scaffold)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use multi_modal_recording::MultiModalRecorder;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{
    AppHandle, Manager, State,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};
use tokio::sync::Mutex;

mod audit;
mod agents;
mod models;
mod security;
mod tools;
mod sola_state;
mod vault;
mod l7_db;
mod scout_state;

use crate::agents::researcher::{MemoryInjection, ResearchSession};
use crate::agents::scout::ScoutAgent;
use crate::models::zodiac::{ZodiacRegistry, ZodiacSign};
use crate::security::{VaultHealth, VaultSecurityState};
use crate::tools::video_scout::{PendingReviewItem, ReviewQueueState, ReviewStatus, ScoutFilter};
use crate::sola_state::{OrchestratorMode, SolaState};
use crate::scout_state::{MissionStatus, ScoutMission, ScoutMissionState};
use crate::models::zodiac::{PhaseThresholds, TrustSignals};

#[derive(Default)]
struct RecorderState {
    inner: Arc<Mutex<MultiModalRecorder>>,
}

#[derive(Serialize)]
struct RecordResult {
    path: String,
}

#[tauri::command]
async fn record_audio(state: State<'_, RecorderState>, duration_secs: u64) -> Result<RecordResult, String> {
    let rec = state.inner.lock().await.clone();
    let rec = rec.clone_with_modes(true, false);
    let p = rec.start_on_demand(duration_secs).await.map_err(|e| e.to_string())?;
    Ok(RecordResult { path: p.display().to_string() })
}

#[tauri::command]
async fn record_video(state: State<'_, RecorderState>, duration_secs: u64) -> Result<RecordResult, String> {
    let rec = state.inner.lock().await.clone();
    let rec = rec.clone_with_modes(false, true);
    let p = rec.start_on_demand(duration_secs).await.map_err(|e| e.to_string())?;
    Ok(RecordResult { path: p.display().to_string() })
}

#[tauri::command]
async fn record_av(state: State<'_, RecorderState>, duration_secs: u64) -> Result<RecordResult, String> {
    let rec = state.inner.lock().await.clone();
    let rec = rec.clone_with_modes(true, true);
    let p = rec.start_on_demand(duration_secs).await.map_err(|e| e.to_string())?;
    Ok(RecordResult { path: p.display().to_string() })
}

#[tauri::command]
async fn schedule_recording(state: State<'_, RecorderState>, cron_expr: String, purpose: String) -> Result<(), String> {
    let rec = state.inner.lock().await.clone();
    rec.schedule_recording(&cron_expr, &purpose).await;
    Ok(())
}

#[tauri::command]
async fn set_always_listening(state: State<'_, RecorderState>, enabled: bool) -> Result<(), String> {
    let rec = state.inner.lock().await.clone();
    if enabled {
        rec.start_always_listening().await;
    } else {
        rec.stop_listening();
    }
    Ok(())
}

#[tauri::command]
async fn enroll_voice(state: State<'_, RecorderState>, samples: Vec<String>) -> Result<(), String> {
    let samples = samples.into_iter().map(PathBuf::from).collect::<Vec<_>>();
    let mut rec = state.inner.lock().await;
    rec.enroll_user_voice(samples).map_err(|e| e.to_string())
}

#[tauri::command]
async fn enroll_face(state: State<'_, RecorderState>, images: Vec<String>) -> Result<(), String> {
    let images = images.into_iter().map(PathBuf::from).collect::<Vec<_>>();
    let mut rec = state.inner.lock().await;
    rec.enroll_user_face(images).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_last_recording(state: State<'_, RecorderState>) -> Result<bool, String> {
    let rec = state.inner.lock().await.clone();
    rec.delete_last_recording().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_all_recordings(state: State<'_, RecorderState>) -> Result<u64, String> {
    let rec = state.inner.lock().await.clone();
    rec.clear_all_recordings().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn recognition_status(_state: State<'_, RecorderState>) -> Result<String, String> {
    // Placeholder until live preview + recognition pipeline is wired.
    Ok("I see you, Dad ❤️".to_string())
}

#[tauri::command]
async fn emotion_status(state: State<'_, RecorderState>) -> Result<String, String> {
    let rec = state.inner.lock().await.clone();
    let result = match rec.last_emotion().await {
        Some(s) => format!(
            "Dad is feeling: {:?} ({:.0}%) ❤️",
            s.primary_emotion,
            s.confidence * 100.0
        ),
        None => "Dad is feeling: Neutral".to_string(),
    };
    Ok(result)
}

#[tauri::command]
async fn emotion_history(state: State<'_, RecorderState>, max: usize) -> Result<Vec<String>, String> {
    let rec = state.inner.lock().await.clone();
    Ok(rec.emotional_moments_recent(max))
}

#[tauri::command]
fn send_notification(
    _app: AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    // Tauri v2 notification API - requires notification permission in capabilities
    // For now, return success - notifications will be handled via frontend
    println!("Notification: {} - {}", title, body);
    Ok(())
}

fn main() {
    // Recover from any interrupted key rotation.
    if let Ok(p) = crate::security::profiles_dir() {
        let _ = crate::security::recover_shadow_buffers(&p);
    }

    let zodiac_registry = ZodiacRegistry::from_embedded_json()
        .expect("failed to load embedded config/zodiac_matrix.json");

    let vault_security = VaultSecurityState::load_or_default().expect("failed to load vault security state");

    let review_queue = tauri::async_runtime::block_on(async {
        ReviewQueueState::load_or_default().await
    })
    .expect("failed to load review queue");

    tauri::Builder::default()
        .manage(RecorderState {
            inner: Arc::new(Mutex::new(MultiModalRecorder::from_env())),
        })
        .manage(SolaState::default())
        .manage(zodiac_registry)
        .manage(vault_security)
        .manage(review_queue)
        .manage(ScoutMissionState::default())
        .setup(|app| {
            // Create system tray menu
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let status = MenuItem::with_id(app, "status", "Status: Active", false, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            
            let menu = Menu::with_items(app, &[
                &status,
                &PredefinedMenuItem::separator(app)?,
                &show,
                &hide,
                &PredefinedMenuItem::separator(app)?,
                &quit,
            ])?;
            
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Sola AGI - v1.0.1")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Background: periodic vault rotation health audit (no automatic destructive actions).
            // This logs when rotation is overdue, but rotation itself is user-triggered.
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    // Sleep first to avoid delaying startup.
                    tokio::time::sleep(std::time::Duration::from_secs(6 * 60 * 60)).await;
                    if let Some(security) = app_handle.try_state::<crate::security::VaultSecurityState>() {
                        let health = security.health(false).await;
                        if let Ok(h) = health {
                            if h.rotation_overdue {
                                let _ = crate::audit::append_line(
                                    "vault_audit.log",
                                    "vault_rotation_overdue action=recommend_rotate",
                                );
                            }
                        }
                    }
                }
            });
             
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            record_audio,
            record_video,
            record_av,
            schedule_recording,
            set_always_listening,
            enroll_voice,
            enroll_face,
            delete_last_recording,
            clear_all_recordings,
            recognition_status,
            emotion_status,
            emotion_history,
            send_notification,
            set_orchestrator_mode,
            get_mode_context,
            load_vault_image,
            unlock_soul_vault,
            lock_soul_vault,
            set_persona_context,
            update_persona_trust,
            rotate_vault_key,
            emergency_purge,
            get_vault_health,
            scout_search,
            get_review_queue,
            review_set_status,
            apply_filters,
            search_media,
            list_scout_missions,
            accept_review_item,
            discard_review_item,
            gather_academic_data,
            gather_companion_insights,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn apply_filters(
    scout: State<'_, ScoutMissionState>,
    resolution: String,
    relevance: f64,
) -> Result<(), String> {
    let mut settings = scout.settings().await;
    settings.filter.min_resolution = resolution;
    settings.filter.relevance_threshold = relevance;
    scout.update_filter(settings.filter).await;
    Ok(())
}

#[tauri::command]
async fn list_scout_missions(scout: State<'_, ScoutMissionState>) -> Result<Vec<ScoutMission>, String> {
    Ok(scout.list_missions().await)
}

#[tauri::command]
async fn search_media(
    app: AppHandle,
    scout: State<'_, ScoutMissionState>,
    queue: State<'_, ReviewQueueState>,
    query: String,
    mode: OrchestratorMode,
    user_kinks: Vec<String>,
) -> Result<Vec<PendingReviewItem>, String> {
    use tauri::Emitter;

    let mission_id = format!("mission_{}", crate::l7_db::now_ms_for_trust_update());
    let title = format!("Scouting: {query}");
    let started_ms = crate::l7_db::now_ms_for_trust_update();

    let filter = scout.settings().await.filter;
    let mut mission = ScoutMission {
        mission_id: mission_id.clone(),
        title: title.clone(),
        query: query.clone(),
        status: MissionStatus::Running,
        started_ms,
        finished_ms: None,
        enqueued_count: 0,
        error: None,
    };
    scout.upsert_mission(mission.clone()).await;
    let _ = app.emit("mission_update", &mission);

    // Mode can affect discovery policy later; currently passed through for UI context.
    let _ = mode;

    let findings = ScoutAgent::search_media(query.clone(), filter, user_kinks).await;
    let result_items = match findings {
        Ok(found) => {
            let mut out = Vec::new();
            for f in found {
                let item = queue.enqueue(f.candidate).await?;
                out.push(item);
            }
            out
        }
        Err(e) => {
            mission.status = MissionStatus::Failed;
            mission.finished_ms = Some(crate::l7_db::now_ms_for_trust_update());
            mission.error = Some(e.clone());
            scout.upsert_mission(mission.clone()).await;
            let _ = app.emit("mission_update", &mission);
            return Err(e);
        }
    };

    mission.status = MissionStatus::Completed;
    mission.finished_ms = Some(crate::l7_db::now_ms_for_trust_update());
    mission.enqueued_count = result_items.len() as u32;
    scout.upsert_mission(mission.clone()).await;
    let _ = app.emit("mission_update", &mission);
    let _ = app.emit("mission_finished", &mission);

    Ok(result_items)
}

#[derive(Serialize)]
struct AcceptReviewItemResponse {
    id: String,
    vault_path: String,
    bytes: usize,
}

fn l8_research_dir() -> Result<std::path::PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault").join("l8").join("research"))
}

#[tauri::command]
async fn discard_review_item(
    app: AppHandle,
    queue: State<'_, ReviewQueueState>,
    id: String,
) -> Result<(), String> {
    use tauri::Emitter;
    queue.set_status(&id, ReviewStatus::Rejected).await?;
    let _ = app.emit("review_queue_updated", &id);
    Ok(())
}

#[tauri::command]
async fn accept_review_item(
    app: AppHandle,
    state: State<'_, SolaState>,
    queue: State<'_, ReviewQueueState>,
    id: String,
) -> Result<AcceptReviewItemResponse, String> {
    use tauri::Emitter;

    let vault_key_opt = { state.inner.read().await.vault_key.as_ref().cloned() };
    let vault_key = vault_key_opt.ok_or_else(|| "Soul Vault is locked".to_string())?;

    // Remove item from the review queue and encrypt it into L8.
    let item = queue.take(&id).await?;
    let plaintext = serde_json::to_vec_pretty(&item).map_err(|e| e.to_string())?;
    let bytes_len = plaintext.len();

    let dir = l8_research_dir()?;
    tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;

    let blob = crate::vault::encrypt_persona_data(&vault_key, &plaintext, None).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.sola", id));
    tokio::fs::write(&path, blob).await.map_err(|e| e.to_string())?;

    crate::audit::append_line(
        "vault_audit.log",
        &format!(
            "l8_accept_ok review_id={} bytes={} path={}",
            id,
            bytes_len,
            path.display()
        ),
    )?;

    let resp = AcceptReviewItemResponse {
        id: id.clone(),
        vault_path: path.display().to_string(),
        bytes: bytes_len,
    };

    let _ = app.emit("review_queue_updated", &id);
    let _ = app.emit("l8_vault_item_added", &resp);
    Ok(resp)
}

fn vault_dir() -> Result<std::path::PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

fn vault_salt_path() -> Result<std::path::PathBuf, String> {
    Ok(vault_dir()?.join("vault_salt.bin"))
}

fn vault_key_check_path() -> Result<std::path::PathBuf, String> {
    Ok(vault_dir()?.join("vault_key_check.sola"))
}

#[tauri::command]
async fn unlock_soul_vault(
    state: State<'_, SolaState>,
    security: State<'_, VaultSecurityState>,
    passphrase: String,
) -> Result<(), String> {
    // Persist a per-device salt locally.
    let salt_path = vault_salt_path()?;
    if let Some(parent) = salt_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let salt = match std::fs::read(&salt_path) {
        Ok(s) if !s.is_empty() => s,
        _ => {
            let s = crate::vault::generate_salt(16);
            std::fs::write(&salt_path, &s).map_err(|e| e.to_string())?;
            s
        }
    };

    let key = crate::vault::derive_vault_key_argon2id(passphrase.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    // Verify key against a persisted check blob. If missing, initialize it.
    let check_path = vault_key_check_path()?;
    if tokio::fs::metadata(&check_path).await.is_ok() {
        let blob = tokio::fs::read(&check_path).await.map_err(|e| e.to_string())?;
        let ok = crate::vault::decrypt_persona_data(&key, &blob)
            .map(|pt| pt == b"SOLA_VAULT_KEY_OK".to_vec())
            .unwrap_or(false);
        if !ok {
            let attempts = security.increment_failed_attempts().await;
            audit::append_line(
                "vault_audit.log",
                &format!("vault_unlock_failed attempts={attempts}"),
            )?;
            if attempts >= 3 {
                let _ = emergency_purge(state, security).await;
            }
            return Err("Invalid master password".to_string());
        }
    } else {
        // Initialize check blob under this key.
        let blob = crate::vault::encrypt_persona_data(&key, b"SOLA_VAULT_KEY_OK", None)
            .map_err(|e| e.to_string())?;
        if let Some(parent) = check_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| e.to_string())?;
        }
        tokio::fs::write(&check_path, &blob)
            .await
            .map_err(|e| e.to_string())?;
    }

    security.reset_failed_attempts().await;
    state.set_vault_key(key).await;

    audit::append_line("vault_audit.log", "vault_unlock_ok")?;
    Ok(())
}

#[tauri::command]
async fn rotate_vault_key(
    state: State<'_, SolaState>,
    security: State<'_, VaultSecurityState>,
    passphrase: String,
) -> Result<u64, String> {
    let old_key = {
        let inner = state.inner.read().await;
        inner
            .vault_key
            .as_ref()
            .map(|k| k.clone())
            .ok_or_else(|| "Soul Vault is locked".to_string())?
    };

    // Re-derive old key from passphrase to ensure the user truly knows the master secret.
    let salt = tokio::fs::read(vault_salt_path()?)
        .await
        .map_err(|e| e.to_string())?;
    let derived_old = crate::vault::derive_vault_key_argon2id(passphrase.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;
    if derived_old.as_slice() != old_key.as_slice() {
        let attempts = security.increment_failed_attempts().await;
        audit::append_line(
            "vault_audit.log",
            &format!("vault_rotation_denied attempts={attempts} reason=bad_password"),
        )?;
        if attempts >= 3 {
            let _ = emergency_purge(state, security).await;
        }
        return Err("Invalid master password".to_string());
    }

    // Create a new salt + derived key. We only swap the salt file after successful re-encryption.
    let new_salt = crate::vault::generate_salt(16);
    let new_key = crate::vault::derive_vault_key_argon2id(passphrase.as_bytes(), &new_salt)
        .map_err(|e| e.to_string())?;

    let profiles_dir = crate::security::profiles_dir()?;
    let rotated = crate::security::rotator::rotate_profiles_dir(&profiles_dir, &old_key, &new_key).await?;

    // Shadow-buffer salt swap.
    let next_path = crate::security::salt_next_path()?;
    tokio::fs::write(&next_path, &new_salt)
        .await
        .map_err(|e| e.to_string())?;
    let prev_path = crate::security::salt_prev_path()?;
    let current_path = crate::security::salt_path()?;
    if tokio::fs::metadata(&prev_path).await.is_ok() {
        let _ = tokio::fs::remove_file(&prev_path).await;
    }
    if tokio::fs::metadata(&current_path).await.is_ok() {
        let _ = tokio::fs::rename(&current_path, &prev_path).await;
    }
    tokio::fs::rename(&next_path, &current_path)
        .await
        .map_err(|e| e.to_string())?;

    // Update in-memory key + rotation timestamp.
    state.set_vault_key(new_key).await;
    let ts = security.set_last_rotation_now().await?;
    audit::append_line(
        "vault_audit.log",
        &format!("vault_rotation_ok rotated_files={rotated} last_rotation_ms={ts}"),
    )?;

    Ok(rotated)
}

#[tauri::command]
async fn emergency_purge(state: State<'_, SolaState>, security: State<'_, VaultSecurityState>) -> Result<u64, String> {
    // Clear key first to prevent any further decrypt attempts.
    state.clear_vault_key().await;
    security.reset_failed_attempts().await;

    crate::security::rotator::log_lockdown("emergency_purge")?;
    audit::append_line("mode_audit.log", "security_lockdown event=emergency_purge")?;

    let profiles_dir = crate::security::profiles_dir()?;
    let purged = crate::security::rotator::purge_profiles_dir(&profiles_dir).await?;
    audit::append_line(
        "vault_audit.log",
        &format!("vault_purge_ok purged_files={purged}"),
    )?;

    Ok(purged)
}

#[tauri::command]
async fn get_vault_health(state: State<'_, SolaState>, security: State<'_, VaultSecurityState>) -> Result<VaultHealth, String> {
    let unlocked = state.is_soul_vault_unlocked().await;
    security.health(unlocked).await
}

#[tauri::command]
async fn scout_search(
    queue: State<'_, ReviewQueueState>,
    filter: ScoutFilter,
    query: String,
    user_kinks: Vec<String>,
) -> Result<Vec<PendingReviewItem>, String> {
    let items = crate::tools::video_scout::scout_and_enqueue(filter, query, user_kinks, &queue).await?;
    crate::audit::append_line(
        "mode_audit.log",
        &format!("review_queue_enqueued count={}", items.len()),
    )?;
    Ok(items)
}

#[tauri::command]
async fn get_review_queue(queue: State<'_, ReviewQueueState>) -> Result<Vec<PendingReviewItem>, String> {
    Ok(queue.list().await)
}

#[tauri::command]
async fn review_set_status(
    queue: State<'_, ReviewQueueState>,
    id: String,
    status: ReviewStatus,
) -> Result<(), String> {
    queue.set_status(&id, status.clone()).await?;
    crate::audit::append_line(
        "mode_audit.log",
        &format!("review_queue_status id={} status={:?}", id, status),
    )?;
    Ok(())
}

#[tauri::command]
async fn lock_soul_vault(state: State<'_, SolaState>) -> Result<(), String> {
    state.clear_vault_key().await;
    audit::append_line("vault_audit.log", "vault_lock_ok")?;
    Ok(())
}

#[tauri::command]
async fn set_persona_context(
    state: State<'_, SolaState>,
    active_persona_id: Option<String>,
    trust_score: Option<f32>,
    zodiac_sign: Option<ZodiacSign>,
) -> Result<(), String> {
    let mut inner = state.inner.write().await;
    inner.active_persona_id = active_persona_id;
    if let Some(ts) = trust_score {
        inner.trust_score = ts.clamp(0.0, 1.0);
    }
    inner.zodiac_sign = zodiac_sign;

    audit::append_line(
        "mode_audit.log",
        &format!(
            "persona_context_set active_persona_id={:?} trust_score={:.3} zodiac={:?}",
            inner.active_persona_id, inner.trust_score, inner.zodiac_sign
        ),
    )?;
    Ok(())
}

#[tauri::command]
async fn gather_academic_data(state: State<'_, SolaState>, query: String) -> Result<MemoryInjection, String> {
    // NOTE: This command is mode-gated inside the ResearchSession implementation.
    ResearchSession::gather_academic_data(&state, query).await
}

#[tauri::command]
async fn gather_companion_insights(
    state: State<'_, SolaState>,
    target_kink: String,
) -> Result<MemoryInjection, String> {
    ResearchSession::gather_companion_insights(&state, target_kink).await
}

#[derive(serde::Serialize)]
struct ModeContextResponse {
    mode: OrchestratorMode,
    allowed_layers: Vec<&'static str>,
    // Placeholder payloads until wired into actual memory backends
    l4: Vec<serde_json::Value>,
    l5: Vec<serde_json::Value>,
    l6: Vec<serde_json::Value>,
    l7: Vec<serde_json::Value>,
    l8: Vec<serde_json::Value>,
}

fn zodiac_threshold(zodiac: &str) -> f32 {
    match zodiac.to_lowercase().as_str() {
        // conservative defaults
        "aries" => 0.85,
        "taurus" => 0.80,
        "gemini" => 0.78,
        "cancer" => 0.82,
        "leo" => 0.83,
        "virgo" => 0.80,
        "libra" => 0.79,
        "scorpio" => 0.88,
        "sagittarius" => 0.81,
        "capricorn" => 0.84,
        "aquarius" => 0.77,
        "pisces" => 0.76,
        _ => 0.85,
    }
}

fn is_nsfw_path(path: &std::path::Path) -> bool {
    let s = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();
    s.contains("nsfw") || s.contains("explicit") || s.contains("x")
}

fn blurred_placeholder_data_url() -> String {
    // 1x1 PNG (placeholder). Frontend can apply CSS blur.
    // NOTE: We intentionally avoid returning decrypted bytes when trust gate fails.
    let b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+X2mQAAAAASUVORK5CYII=";
    format!("data:image/png;base64,{}", b64)
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PersonaPhase {
    Stranger,
    Acquaintance,
    Friend,
    Intimate,
    ResearchPartner,
}

fn phase_from_trust(trust: f32, t: &PhaseThresholds) -> PersonaPhase {
    let trust = trust.clamp(0.0, 1.0) as f64;
    if trust < t.acquaintance {
        PersonaPhase::Stranger
    } else if trust < t.friend {
        PersonaPhase::Acquaintance
    } else if trust < t.intimate {
        PersonaPhase::Friend
    } else if trust < t.research_partner {
        PersonaPhase::Intimate
    } else {
        PersonaPhase::ResearchPartner
    }
}

#[derive(Serialize)]
struct UpdatePersonaTrustResponse {
    persona_id: String,
    zodiac_sign: ZodiacSign,
    trust_old: f32,
    trust_new: f32,
    phase_old: PersonaPhase,
    phase_new: PersonaPhase,
    phase_crossed: bool,
}

#[tauri::command]
async fn update_persona_trust(
    app: AppHandle,
    state: State<'_, SolaState>,
    registry: State<'_, ZodiacRegistry>,
    persona_id: String,
    delta_interaction: f64,
    signals: TrustSignals,
    now_ms: Option<u64>,
) -> Result<UpdatePersonaTrustResponse, String> {
    use tauri::Emitter;

    let now_ms = now_ms.unwrap_or_else(crate::l7_db::now_ms_for_trust_update);

    // Determine zodiac sign to use (prefer current persona context).
    let zodiac_sign = {
        let inner = state.inner.read().await;
        inner.zodiac_sign.unwrap_or(ZodiacSign::Aries)
    };

    let profile = registry
        .profile(zodiac_sign)
        .ok_or_else(|| format!("missing zodiac profile for {:?}", zodiac_sign))?;

    // Load previous trust from L7 store (fallback to current in-memory value, then profile initial).
    let prev_record = crate::l7_db::load_persona_trust(&persona_id).await?;
    let (trust_old, last_interaction_ms) = match prev_record {
        Some(r) => (r.trust_score, Some(r.last_interaction_ms)),
        None => {
            let mem_trust = state.inner.read().await.trust_score;
            let seed = if mem_trust > 0.0 { mem_trust } else { profile.initial_trust as f32 };
            (seed.clamp(0.0, 1.0), None)
        }
    };

    let trust_new = profile.update_trust(
        trust_old as f64,
        delta_interaction,
        signals,
        last_interaction_ms,
        now_ms,
    ) as f32;

    let phase_old = phase_from_trust(trust_old, &profile.phase_thresholds);
    let phase_new = phase_from_trust(trust_new, &profile.phase_thresholds);
    let phase_crossed = phase_old != phase_new;

    // Persist to L7 store.
    let _record = crate::l7_db::upsert_persona_trust(
        persona_id.clone(),
        zodiac_sign,
        trust_new,
        now_ms,
    )
    .await?;

    // Update in-memory gating inputs.
    {
        let mut inner = state.inner.write().await;
        if inner.active_persona_id.is_none() {
            inner.active_persona_id = Some(persona_id.clone());
        }
        inner.trust_score = trust_new.clamp(0.0, 1.0);
        inner.zodiac_sign = Some(zodiac_sign);
    }

    // Emit UI events.
    let payload = UpdatePersonaTrustResponse {
        persona_id: persona_id.clone(),
        zodiac_sign,
        trust_old,
        trust_new,
        phase_old,
        phase_new,
        phase_crossed,
    };
    let _ = app.emit("persona_trust_updated", &payload);
    if phase_crossed {
        let _ = app.emit("persona_phase_crossed", &payload);
    }

    Ok(payload)
}

#[tauri::command]
async fn set_orchestrator_mode(state: State<'_, SolaState>, mode: OrchestratorMode) -> Result<(), String> {
    // Validate: Personal mode requires Soul Vault unlocked.
    if mode == OrchestratorMode::Personal && !state.is_soul_vault_unlocked().await {
        audit::append_line("mode_audit.log", "mode_switch_denied target=personal reason=vault_locked")?;
        return Err("Soul Vault is locked; cannot switch to Personal mode".to_string());
    }

    {
        let mut inner = state.inner.write().await;
        let prev = inner.current_mode;
        inner.current_mode = mode;
        audit::append_line(
            "mode_audit.log",
            &format!("mode_switch prev={:?} next={:?}", prev, mode),
        )?;
    }

    // Resource optimization hooks (placeholder; implement real GPU/memory hooks later)
    match mode {
        OrchestratorMode::Professional => {
            audit::append_line("mode_audit.log", "resource_opt action=gpu_cooldown_or_memory_compact")?;
        }
        OrchestratorMode::Personal => {
            audit::append_line("mode_audit.log", "resource_opt action=init_visual_engine_prep_vram")?;
        }
    }

    Ok(())
}

#[tauri::command]
async fn get_mode_context(state: State<'_, SolaState>) -> Result<ModeContextResponse, String> {
    let inner = state.inner.read().await;
    let mode = inner.current_mode;
    // NOTE: Explicit types to avoid inference failures.
    let mut l4: Vec<serde_json::Value> = Vec::new();
    let mut l5: Vec<serde_json::Value> = Vec::new();
    let mut l6: Vec<serde_json::Value> = Vec::new();
    let mut l7: Vec<serde_json::Value> = Vec::new();
    let mut l8: Vec<serde_json::Value> = Vec::new();

    let allowed_layers: Vec<&'static str> = match mode {
        OrchestratorMode::Professional => vec!["L4", "L5"],
        OrchestratorMode::Personal => vec!["L6", "L7", "L8"],
    };

    // TODO: Wire to real memory subsystems.
    // For now, return empty arrays but enforce layer visibility at the gate.
    if mode == OrchestratorMode::Professional {
        l6.clear();
        l7.clear();
        l8.clear();
    } else {
        l4.clear();
        l5.clear();
    }

    Ok(ModeContextResponse {
        mode,
        allowed_layers,
        l4,
        l5,
        l6,
        l7,
        l8,
    })
}

#[tauri::command]
async fn load_vault_image(
    state: State<'_, SolaState>,
    registry: State<'_, ZodiacRegistry>,
    profile_id: String,
    image_index: u32,
) -> Result<String, String> {
    let (vault_key_opt, trust_score, zodiac_sign) = {
        let inner = state.inner.read().await;
        (
            inner.vault_key.as_ref().map(|k| k.clone()),
            inner.trust_score,
            inner.zodiac_sign.unwrap_or(ZodiacSign::Aries),
        )
    };

    let vault_key = match vault_key_opt {
        Some(k) => k,
        None => {
            audit::append_line("vault_audit.log", "decrypt_denied reason=vault_locked")?;
            return Ok(blurred_placeholder_data_url());
        }
    };

    // Locate the encrypted blob.
    let base = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("vault")
        .join("profiles");

    let candidates = [
        base.join(&profile_id).join(format!("{}.sola", image_index)),
        base.join(format!("{}_{}.sola", profile_id, image_index)),
        base.join(&profile_id).join(format!("image_{}.sola", image_index)),
    ];

    let mut found: Option<std::path::PathBuf> = None;
    for p in candidates {
        if tokio::fs::metadata(&p).await.is_ok() {
            found = Some(p);
            break;
        }
    }
    let path = found.ok_or_else(|| "Vault image not found".to_string())?;

    // L7 PII gate for explicit content.
    if is_nsfw_path(&path) {
        let threshold = registry.nsfw_threshold(zodiac_sign) as f32;
        if trust_score < threshold {
            audit::append_line(
                "vault_audit.log",
                &format!(
                    "decrypt_denied reason=trust_gate profile_id={} image_index={} trust_score={:.3} threshold={:.3} zodiac={} path={}",
                    profile_id,
                    image_index,
                    trust_score,
                    threshold,
                    format!("{:?}", zodiac_sign).to_lowercase(),
                    path.display()
                ),
            )?;
            return Ok(blurred_placeholder_data_url());
        }
    }

    let blob = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
    let plaintext = crate::vault::decrypt_persona_data(&vault_key, &blob).map_err(|e| e.to_string())?;

    audit::append_line(
        "vault_audit.log",
        &format!(
            "decrypt_ok profile_id={} image_index={} bytes={} path={}",
            profile_id,
            image_index,
            plaintext.len(),
            path.display()
        ),
    )?;

    // Default to PNG transport; frontend can interpret based on embedded bytes.
    Ok(format!("data:image/png;base64,{}", crate::vault::to_base64(&plaintext)))
}
