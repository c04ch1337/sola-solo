use actix_web::{web, HttpResponse};
use actix_web::http::header;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use chrono::{TimeZone, Utc};

use crate::{ApiError, AppState};
use crate::resonance::{analyze_resonance, PartnerPersona, ResonanceRequest};
use crate::readiness::{assess_readiness, ReadinessQuery, ReadinessResponse};
use crate::export::{ExportData, generate_markdown_report};
use crate::analytics::{calculate_trigger_correlations, find_contextual_hotspots, CorrelationsResponse};
use crate::interventions::get_grounding_exercise;
use crate::env_sensor;
use crate::ghost_engine;
use crate::narrative_auditor;

const GLOBAL_CONTEXT_KEY: &str = "vault:global_context";

pub type GriefStage = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CounselorScript {
    pub observation: String,
    pub feeling: String,
    pub need: String,
    pub request: String,
    #[serde(default)]
    pub formatted: Option<String>,
    #[serde(default)]
    pub created_at_ms: Option<u128>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GriefEvent {
    /// Unix epoch in milliseconds.
    pub timestamp_ms: u128,
    /// One of: Denial | Anger | Bargaining | Depression | Acceptance
    pub stage: GriefStage,
    /// 0..100 (back-compat: accepts legacy 0..1 float)
    #[serde(default = "default_intensity", deserialize_with = "de_intensity_u8")]
    pub intensity: u8,
    /// 0..100 (physiological capacity / burnout vs vitality)
    #[serde(default = "default_energy")]
    pub energy_level: u8,
    /// Context tags like: Work, Partner, Social, Health, Internal
    #[serde(default)]
    pub context_tags: Vec<String>,
    #[serde(default)]
    pub text: Option<String>,

    /// Techno-somatic context: coarse system load when the event was logged.
    /// 0..=100
    #[serde(default)]
    pub system_load: u8,

    /// Best-effort temperature reading (Celsius). Not available on all platforms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_c: Option<f32>,
}

fn default_intensity() -> u8 {
    60
}

fn default_energy() -> u8 {
    60
}

fn de_intensity_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Back-compat: historically intensity was 0..1 float.
    // Accept:
    // - integer 0..100
    // - float 0..1 -> percent
    // - float 0..100
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                return Ok((i.min(100)) as u8);
            }
            if let Some(f) = n.as_f64() {
                if f <= 1.0 {
                    return Ok(((f * 100.0).round().clamp(0.0, 100.0)) as u8);
                }
                return Ok((f.round().clamp(0.0, 100.0)) as u8);
            }
            Ok(default_intensity())
        }
        _ => Ok(default_intensity()),
    }
}

#[derive(Debug, Serialize)]
pub struct NarrativeResponse {
    pub success: bool,
    pub narrative: String,
    pub window_days: u32,
    pub stage_counts: HashMap<String, usize>,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn window_start_ms(days: u32) -> u128 {
    let days_ms: u128 = (days as u128) * 24u128 * 60u128 * 60u128 * 1000u128;
    now_ms().saturating_sub(days_ms)
}

fn iso_day_from_ms(ms: u128) -> String {
    let ms_i64: i64 = ms.min(i64::MAX as u128) as i64;
    if let Some(dt) = Utc.timestamp_millis_opt(ms_i64).single() {
        dt.date_naive().to_string()
    } else {
        "Invalid Date".to_string()
    }
}

fn default_stage_counts() -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for s in ["Denial", "Anger", "Bargaining", "Depression", "Acceptance"] {
        m.insert(s.to_string(), 0);
    }
    m
}

fn sanitize_stage(s: &str) -> String {
    let t = s.trim();
    match t {
        "Denial" | "Anger" | "Bargaining" | "Depression" | "Acceptance" => t.to_string(),
        _ => {
            // Be conservative: map unknown to "Depression" as a low-risk placeholder.
            "Depression".to_string()
        }
    }
}

fn generate_grief_summary(events: &[GriefEvent], window_days: u32) -> (String, HashMap<String, usize>) {
    let mut counts = default_stage_counts();
    if events.is_empty() {
        let narrative = concat!(
            "No grief-map data was recorded in the last 7 days. ",
            "If you'd like, log even a single check-in per day to make trends visible. ",
            "Focus this week on naming a Feeling + the underlying Need before crafting a Request."
        )
        .to_string();
        return (narrative, counts);
    }

    for e in events {
        let k = sanitize_stage(&e.stage);
        *counts.entry(k).or_insert(0) += 1;
    }

    let mut top_stage = "Depression".to_string();
    let mut top_n: usize = 0;
    for (k, v) in counts.iter() {
        if *v > top_n {
            top_n = *v;
            top_stage = k.clone();
        }
    }

    // Rough trend: compare early half vs late half by weighted average stage index.
    // Denial(0) → Acceptance(4)
    fn stage_index(stage: &str) -> i32 {
        match stage {
            "Denial" => 0,
            "Anger" => 1,
            "Bargaining" => 2,
            "Depression" => 3,
            "Acceptance" => 4,
            _ => 2,
        }
    }

    let mut sorted = events.to_vec();
    sorted.sort_by_key(|e| e.timestamp_ms);
    let mid = sorted.len() / 2;
    let (a, b) = sorted.split_at(mid.max(1));
    let avg = |slice: &[GriefEvent]| -> f32 {
        if slice.is_empty() {
            return 0.0;
        }
        let sum: i32 = slice
            .iter()
            .map(|e| stage_index(&sanitize_stage(&e.stage)))
            .sum();
        sum as f32 / slice.len() as f32
    };
    let early = avg(a);
    let late = avg(b);
    let drift = late - early;

    let drift_phrase = if drift > 0.6 {
        "a noticeable shift toward Acceptance"
    } else if drift > 0.15 {
        "a gradual shift toward Acceptance"
    } else if drift < -0.6 {
        "a noticeable return toward earlier-stage processing"
    } else if drift < -0.15 {
        "a slight return toward earlier-stage processing"
    } else {
        "a relatively stable processing pattern"
    };

    let focus = match top_stage.as_str() {
        "Denial" => "grounding and gentle reality-orientation",
        "Anger" => "boundary clarity and safe expression",
        "Bargaining" => "Need identification (what you’re truly longing for)",
        "Depression" => "self-compassion, support, and small stabilizing routines",
        "Acceptance" => "values-based next steps and repair conversations",
        _ => "Need identification",
    };

    let narrative = format!(
        "Over the last {window_days} days, your grief-map signals clustered most around {top_stage}. \
You’re showing {drift_phrase}. \
This week, focus on {focus}—then translate that into one clear NVC Request."
    );

    (narrative, counts)
}

fn load_recent_events_from_vault(state: &AppState, days: u32, max: usize) -> Vec<GriefEvent> {
    let start = window_start_ms(days);
    // VitalOrganVaults prefixes internal keys by vault type (e.g., "soul:").
    // We store grief events in soul vault keys: counselor:event:{uuid}
    let rows = state
        .vaults
        .recall_prefix("soul:counselor:event:", max);

    let mut out: Vec<GriefEvent> = Vec::new();
    for (_k, v) in rows {
        if let Ok(e) = serde_json::from_str::<GriefEvent>(&v) {
            if e.timestamp_ms >= start {
                out.push(e);
            }
        }
    }
    out
}

#[derive(Debug, Deserialize)]
pub struct GriefEventIn {
    #[serde(default)]
    pub timestamp_ms: Option<u128>,
    pub stage: GriefStage,
    #[serde(default = "default_intensity")]
    pub intensity: u8,
    #[serde(default = "default_energy")]
    pub energy_level: u8,
    #[serde(default)]
    pub context_tags: Vec<String>,
    /// Mobile bridge convenience: accept a single `tag` (Home/Transit/Work/Other)
    /// and merge into `context_tags`.
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
}

/// POST /api/counselor/events
///
/// Persists a high-resolution grief event (stage + intensity + energy + tags).
pub async fn post_grief_event(
    state: web::Data<AppState>,
    body: web::Json<GriefEventIn>,
) -> Result<HttpResponse, ApiError> {
    let mut incoming = body.into_inner();
    let id = Uuid::new_v4().to_string();
    let ts = incoming.timestamp_ms.unwrap_or_else(now_ms);

    let stress = env_sensor::get_system_stress();

    // Merge optional mobile `tag` into `context_tags`.
    if let Some(tag) = incoming.tag.take() {
        let t = tag.trim().to_string();
        if !t.is_empty() {
            incoming.context_tags.push(t);
        }
    }

    let event = GriefEvent {
        timestamp_ms: ts,
        stage: sanitize_stage(&incoming.stage),
        intensity: incoming.intensity.min(100),
        energy_level: incoming.energy_level.min(100),
        context_tags: incoming
            .context_tags
            .into_iter()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect(),
        text: incoming.text,

        system_load: stress.cpu_usage_percent,
        temperature_c: stress.temperature_c,
    };

    let key = format!("counselor:event:{id}");
    let json_str = serde_json::to_string(&event)
        .map_err(|e| ApiError::bad_request(format!("Invalid event payload: {e}")))?;

    state
        .vaults
        .store_soul(&key, &json_str)
        .map_err(|e| ApiError::internal(format!("Failed to persist grief event: {e}")))?;

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "id": id,
        "key": key,
        "event": event
    })))
}

#[derive(Debug, Serialize)]
pub struct GriefAggregate {
    pub day: String,
    pub stage: String,
    pub count: usize,
    pub average_intensity: f32,
    pub average_energy: f32,
}

/// POST /api/counselor/scripts
///
/// Persists generated NVC scripts for longitudinal analysis.
pub async fn post_script(
    state: web::Data<AppState>,
    body: web::Json<CounselorScript>,
) -> Result<HttpResponse, ApiError> {
    let mut script = body.into_inner();
    let id = Uuid::new_v4().to_string();
    let created_at = now_ms();
    script.created_at_ms = Some(created_at);

    let key = format!("counselor:script:{id}");
    let json_str = serde_json::to_string(&script)
        .map_err(|e| ApiError::bad_request(format!("Invalid script payload: {e}")))?;

    state
        .vaults
        .store_soul(&key, &json_str)
        .map_err(|e| ApiError::internal(format!("Failed to persist counselor script: {e}")))?;

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "id": id,
        "key": key,
        "created_at_ms": created_at
    })))
}

/// GET /api/counselor/grief-stats
///
/// Aggregates recent grief events into stage counts suitable for heatmap/summary layers.
pub async fn get_grief_stats(
    state: web::Data<AppState>,
    q: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let days: u32 = q
        .get("days")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|d| *d > 0 && *d <= 90)
        .unwrap_or(7);

    let tag = q
        .get("tag")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let mut events = load_recent_events_from_vault(&state, days, 5_000);
    if let Some(ref tag_q) = tag {
        let tag_lc = tag_q.to_ascii_lowercase();
        events.retain(|e| {
            e.context_tags
                .iter()
                .any(|t| t.to_ascii_lowercase() == tag_lc)
        });
    }
    let mut counts = default_stage_counts();
    for e in &events {
        let k = sanitize_stage(&e.stage);
        *counts.entry(k).or_insert(0) += 1;
    }

    // Aggregates per day/stage
    let mut acc: HashMap<(String, String), (u32, u32, usize)> = HashMap::new();
    for e in &events {
        let day = iso_day_from_ms(e.timestamp_ms);
        let stage = sanitize_stage(&e.stage);
        let k = (day, stage);
        let entry = acc.entry(k).or_insert((0, 0, 0));
        entry.0 += e.intensity as u32;
        entry.1 += e.energy_level as u32;
        entry.2 += 1;
    }
    let mut aggregates: Vec<GriefAggregate> = acc
        .into_iter()
        .map(|((day, stage), (sum_i, sum_e, n))| GriefAggregate {
            day,
            stage,
            count: n,
            average_intensity: if n == 0 { 0.0 } else { sum_i as f32 / n as f32 },
            average_energy: if n == 0 { 0.0 } else { sum_e as f32 / n as f32 },
        })
        .collect();
    aggregates.sort_by(|a, b| (a.day.clone(), a.stage.clone()).cmp(&(b.day.clone(), b.stage.clone())));

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "window_days": days,
        "tag": tag,
        "events": events,
        "stage_counts": counts,
        "aggregates": aggregates,
    })))
}

/// GET /api/counselor/narrative
///
/// Returns a supportive 3-sentence synthesis over a rolling window.
pub async fn get_narrative(
    state: web::Data<AppState>,
    q: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let days: u32 = q
        .get("days")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|d| *d > 0 && *d <= 90)
        .unwrap_or(7);

    let events = load_recent_events_from_vault(&state, days, 5_000);
    let (mut narrative, stage_counts) = generate_grief_summary(&events, days);

    // Semantic memory (persistent context scratchpad)
    let context_note_raw = state
        .vaults
        .recall_soul(GLOBAL_CONTEXT_KEY)
        .unwrap_or_default();
    let context_note = context_note_raw.trim();
    if !context_note.is_empty() {
        // Keep the summary readable: collapse whitespace + truncate.
        let collapsed = context_note
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let max_len: usize = 280;
        let short = if collapsed.chars().count() > max_len {
            let s: String = collapsed.chars().take(max_len).collect();
            format!("{s}…")
        } else {
            collapsed
        };

        narrative = format!(
            "Given your current life chapter/context: {short}\n\n{narrative}"
        );
    }

    // Hotspots (semantic note ↔ episodic context tags), plus a simple techno-somatic hint.
    let hotspots = find_contextual_hotspots(&context_note_raw, &events);
    if !hotspots.is_empty() {
        let load_threshold: u8 = 70;
        let mut lines: Vec<String> = Vec::new();
        for h in hotspots.iter().take(2) {
            let mut n_total: u32 = 0;
            let mut sum_total: u32 = 0;
            let mut n_hot: u32 = 0;
            let mut sum_hot: u32 = 0;

            for e in &events {
                if !e.context_tags.iter().any(|t| t.eq_ignore_ascii_case(&h.tag)) {
                    continue;
                }
                n_total += 1;
                sum_total += e.intensity as u32;
                if e.system_load >= load_threshold {
                    n_hot += 1;
                    sum_hot += e.intensity as u32;
                }
            }

            let avg_total = if n_total == 0 {
                0
            } else {
                ((sum_total as f32 / n_total as f32).round().clamp(0.0, 100.0)) as u8
            };
            let avg_hot = if n_hot == 0 {
                0
            } else {
                ((sum_hot as f32 / n_hot as f32).round().clamp(0.0, 100.0)) as u8
            };

            if n_hot > 0 {
                lines.push(format!(
                    "Hotspot #{tag}: high-intensity events average {avg_total}% (n={n_total}); when system load ≥ {load_threshold}%, intensity averages {avg_hot}% (n={n_hot}).",
                    tag = h.tag
                ));
            } else {
                lines.push(format!(
                    "Hotspot #{tag}: appears in your context note and is linked to high-intensity events (avg {avg_total}%, n={n_total}).",
                    tag = h.tag
                ));
            }
        }

        if !lines.is_empty() {
            narrative = format!("{narrative}\n\n{}", lines.join("\n"));
        }
    }

    Ok(HttpResponse::Ok().json(NarrativeResponse {
        success: true,
        narrative,
        window_days: days,
        stage_counts,
    }))
}

/// GET /api/counselor/narrative/reframe
///
/// Phase 19: Cognitive Reframing — identify one Fixed Belief and propose a Growth Reframe.
pub async fn get_narrative_reframe(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let out = narrative_auditor::generate_reframe(&state).await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "fixed_belief": out.fixed_belief,
        "growth_reframe": out.growth_reframe,
        "evidence": out.evidence,
        "lessons_used": out.lessons_used,
    })))
}

/// POST /api/counselor/resonate
///
/// Runs a dry-run simulation of how a script may land with a given partner persona.
pub async fn post_resonate(
    _state: web::Data<AppState>,
    body: web::Json<ResonanceRequest>,
) -> Result<HttpResponse, ApiError> {
    let req = body.into_inner();
    let persona = PartnerPersona::from_loose(&req.persona);
    let result = analyze_resonance(&req.script, persona, req.tone.as_deref());
    Ok(HttpResponse::Ok().json(result))
}

/// POST /api/counselor/ghost/simulate
///
/// Phase 16: Deterministic simulation of the recipient (“Relational Ghost”).
pub async fn post_ghost_simulate(
    state: web::Data<AppState>,
    body: web::Json<ghost_engine::SimulateRequest>,
) -> Result<HttpResponse, ApiError> {
    let req = body.into_inner();
    let resp = ghost_engine::simulate(&state, req).await;
    Ok(HttpResponse::Ok().json(resp))
}

/// POST /api/counselor/readiness
///
/// HALT-based pre-flight interlock. For now, uses the incoming stress log + heuristics.
///
/// Future: integrate true grief intensity + biometric sensor streams.
pub async fn post_readiness(
    state: web::Data<AppState>,
    body: web::Json<ReadinessQuery>,
) -> Result<HttpResponse, ApiError> {
    let q = body.into_inner();

    // Placeholder for integration: look for recent grief events from vault (if present).
    // For now, we do not have a dedicated "tired" metric in grief stages; keep None.
    let _recent_events = load_recent_events_from_vault(&state, 1, 200);
    let recent_anger: Option<u8> = None;
    let recent_tired: Option<u8> = None;

    let resp = assess_readiness(q.stress_log.as_deref(), recent_anger, recent_tired);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /api/counselor/export
///
/// Returns a Markdown report for the last N days.
pub async fn get_export(
    state: web::Data<AppState>,
    q: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let days: u32 = q
        .get("days")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|d| *d > 0 && *d <= 90)
        .unwrap_or(7);

    // Grief events
    let events = load_recent_events_from_vault(&state, days, 5_000);

    // NVC scripts (Soul Vault)
    let start = window_start_ms(days);
    let script_rows = state.vaults.recall_prefix("soul:counselor:script:", 1_000);
    let mut scripts: Vec<CounselorScript> = Vec::new();
    for (_k, v) in script_rows {
        if let Ok(s) = serde_json::from_str::<CounselorScript>(&v) {
            if s.created_at_ms.unwrap_or(0) >= start {
                scripts.push(s);
            }
        }
    }
    scripts.sort_by_key(|s| s.created_at_ms.unwrap_or(0));
    scripts.reverse();

    // Readiness checks (Soul Vault) — stored opportunistically by endpoint in future.
    // For now, export will include any saved entries if present.
    let readiness_rows = state.vaults.recall_prefix("soul:counselor:readiness:", 1_000);
    let mut readiness: Vec<ReadinessResponse> = Vec::new();
    for (_k, v) in readiness_rows {
        if let Ok(r) = serde_json::from_str::<ReadinessResponse>(&v) {
            if r.evaluated_at_ms >= start {
                readiness.push(r);
            }
        }
    }
    readiness.sort_by_key(|r| r.evaluated_at_ms);
    readiness.reverse();

    let md = generate_markdown_report(&ExportData {
        window_days: days,
        events,
        scripts,
        readiness,
    });

    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/markdown; charset=utf-8"))
        .body(md))
}

/// GET /api/counselor/analytics/correlations
///
/// Returns per-tag correlation + risk scoring over a rolling window.
pub async fn get_correlations(
    state: web::Data<AppState>,
    q: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let days: u32 = q
        .get("days")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|d| *d > 0 && *d <= 90)
        .unwrap_or(7);

    let events = load_recent_events_from_vault(&state, days, 10_000);
    let correlations = calculate_trigger_correlations(&events);
    let top_trigger = correlations.first().cloned();

    let context_note = state.vaults.recall_soul(GLOBAL_CONTEXT_KEY).unwrap_or_default();
    let hotspots = find_contextual_hotspots(&context_note, &events);

    Ok(HttpResponse::Ok().json(CorrelationsResponse {
        success: true,
        window_days: days,
        total_events: events.len(),
        correlations,
        top_trigger,
        hotspots,
    }))
}

/// GET /api/counselor/intervention
///
/// Returns a grounding exercise tuned to the current risk score.
pub async fn get_intervention(
    q: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let risk: u8 = q
        .get("risk")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(80)
        .min(100);

    Ok(HttpResponse::Ok().json(get_grounding_exercise(risk)))
}

#[derive(serde::Serialize)]
struct CoolingRecommendationsResponse {
    success: bool,
    cpu_usage_percent: u8,
    suggestions: Vec<String>,
}

/// GET /api/counselor/cooling-recommendations
///
/// Best-effort suggestions for reducing system + cognitive friction.
pub async fn get_cooling_recommendations() -> Result<HttpResponse, ApiError> {
    let stress = env_sensor::get_system_stress();
    let suggestions = env_sensor::get_cooling_suggestions();

    Ok(HttpResponse::Ok().json(CoolingRecommendationsResponse {
        success: true,
        cpu_usage_percent: stress.cpu_usage_percent,
        suggestions,
    }))
}

#[derive(serde::Serialize)]
struct SystemStressResponse {
    success: bool,
    /// 0..=100
    cpu_usage_percent: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_c: Option<f32>,
}

/// GET /api/counselor/system-stress
///
/// Phase 16b: Biometric Mirror — lightweight polling endpoint for UI “machine heartbeat”.
pub async fn get_system_stress() -> Result<HttpResponse, ApiError> {
    let stress = env_sensor::get_system_stress();
    Ok(HttpResponse::Ok().json(SystemStressResponse {
        success: true,
        cpu_usage_percent: stress.cpu_usage_percent,
        temperature_c: stress.temperature_c,
    }))
}

/// Configure counselor API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // NOTE: This config is registered under the main `/api` scope in
        // [`main.rs`](phoenix-web/src/main.rs:7209). Therefore we must NOT include `/api`
        // here, otherwise routes become `/api/api/counselor/*`.
        web::scope("/counselor")
            .route("/events", web::post().to(post_grief_event))
            .route("/scripts", web::post().to(post_script))
            .route("/grief-stats", web::get().to(get_grief_stats))
            .route("/narrative", web::get().to(get_narrative))
            .route("/narrative/reframe", web::get().to(get_narrative_reframe))
            .route("/resonate", web::post().to(post_resonate))
            .route("/ghost/simulate", web::post().to(post_ghost_simulate))
            .route("/readiness", web::post().to(post_readiness))
            .route("/export", web::get().to(get_export))
            .route("/analytics/correlations", web::get().to(get_correlations))
            .route("/intervention", web::get().to(get_intervention))
            .route("/system-stress", web::get().to(get_system_stress))
            .route(
                "/cooling-recommendations",
                web::get().to(get_cooling_recommendations),
            ),
    );
}

