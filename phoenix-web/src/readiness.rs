use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessQuery {
    /// Optional stress log from the current session (frontend can send current input).
    #[serde(default)]
    pub stress_log: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub readiness_score: u8, // 0..100
    pub window_status: String, // Green | Yellow | Red
    pub reasons: Vec<String>,
    pub cooldown_seconds: u32,
    pub evaluated_at_ms: u128,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn clamp_score(v: i32) -> u8 {
    v.clamp(0, 100) as u8
}

fn score_to_window(score: u8) -> &'static str {
    if score >= 75 {
        "Green"
    } else if score >= 45 {
        "Yellow"
    } else {
        "Red"
    }
}

/// HALT-style readiness assessment.
///
/// Bare-metal (offline) heuristic:
/// - Looks for stress markers in the current stress log.
/// - Can be extended to incorporate grief intensity once events are persisted.
pub fn assess_readiness(stress_log: Option<&str>, recent_anger_intensity: Option<u8>, recent_tired_intensity: Option<u8>) -> ReadinessResponse {
    let mut score: i32 = 78;
    let mut reasons: Vec<String> = Vec::new();
    let mut cooldown_seconds: u32 = 0;

    // H: Hungry (proxy via words that often correlate)
    if let Some(s) = stress_log {
        let t = s.to_ascii_lowercase();
        if t.contains("hungry") || t.contains("haven't eaten") || t.contains("no time to eat") {
            score -= 18;
            reasons.push("HALT: Hungry signals detected".to_string());
        }

        // A: Angry (proxy)
        if t.contains("angry") || t.contains("furious") || t.contains("rage") || t.contains("pissed") {
            score -= 22;
            reasons.push("HALT: Angry signals detected".to_string());
            cooldown_seconds = cooldown_seconds.max(20 * 60);
        }

        // L: Lonely (proxy)
        if t.contains("lonely") || t.contains("isolated") || t.contains("alone") {
            score -= 12;
            reasons.push("HALT: Lonely signals detected".to_string());
        }

        // T: Tired (proxy)
        if t.contains("tired") || t.contains("exhaust") || t.contains("burnout") || t.contains("no sleep") {
            score -= 20;
            reasons.push("HALT: Tired signals detected".to_string());
            cooldown_seconds = cooldown_seconds.max(30 * 60);
        }
    }

    // Optional: integrate measured grief-map intensities if available.
    if let Some(a) = recent_anger_intensity {
        if a > 80 {
            score -= 24;
            reasons.push("Recent Anger intensity is high (>80%)".to_string());
            cooldown_seconds = cooldown_seconds.max(30 * 60);
        }
    }
    if let Some(t) = recent_tired_intensity {
        if t > 80 {
            score -= 22;
            reasons.push("Recent Tiredness intensity is high (>80%)".to_string());
            cooldown_seconds = cooldown_seconds.max(45 * 60);
        }
    }

    // Ensure we always return something actionable
    if reasons.is_empty() {
        reasons.push("No flooding indicators detected. Proceed gently.".to_string());
    }

    let readiness_score = clamp_score(score);
    let window_status = score_to_window(readiness_score).to_string();
    let ready = readiness_score >= 55;

    ReadinessResponse {
        ready,
        readiness_score,
        window_status,
        reasons,
        cooldown_seconds,
        evaluated_at_ms: now_ms(),
    }
}

