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
mod sola_state;
mod vault;

use crate::agents::researcher::{MemoryInjection, ResearchSession};
use crate::models::zodiac::{ZodiacRegistry, ZodiacSign};
use crate::sola_state::{OrchestratorMode, SolaState};

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
    let zodiac_registry = ZodiacRegistry::from_embedded_json()
        .expect("failed to load embedded config/zodiac_matrix.json");

    tauri::Builder::default()
        .manage(RecorderState {
            inner: Arc::new(Mutex::new(MultiModalRecorder::from_env())),
        })
        .manage(SolaState::default())
        .manage(zodiac_registry)
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
            gather_academic_data,
            gather_companion_insights,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn vault_dir() -> Result<std::path::PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

fn vault_salt_path() -> Result<std::path::PathBuf, String> {
    Ok(vault_dir()?.join("vault_salt.bin"))
}

#[tauri::command]
async fn unlock_soul_vault(state: State<'_, SolaState>, passphrase: String) -> Result<(), String> {
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
    state.set_vault_key(key).await;

    audit::append_line("vault_audit.log", "vault_unlock_ok")?;
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
