use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionResponse {
    pub risk_score: u8,
    pub title: String,
    pub exercise: String,
    pub recommended_seconds: u32,
}

fn now_seed_u32() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.as_nanos() as u64).wrapping_mul(1103515245).wrapping_add(12345) as u32)
        .unwrap_or(0)
}

fn pick<T: Clone>(items: &[T]) -> T {
    let idx = (now_seed_u32() as usize) % items.len().max(1);
    items.get(idx).cloned().unwrap_or_else(|| items[0].clone())
}

pub fn get_grounding_exercise(risk_score: u8) -> InterventionResponse {
    // Heuristic: higher risk → more structured, shorter loop.
    let recommended_seconds = if risk_score >= 90 {
        20 * 60
    } else if risk_score >= 80 {
        15 * 60
    } else if risk_score >= 70 {
        10 * 60
    } else {
        5 * 60
    };

    // A small bank of zero-dependency interventions.
    // Keep them short, actionable, and safe.
    let high_intensity = [
        InterventionResponse {
            risk_score,
            title: "4-7-8 Breathing".to_string(),
            exercise: "Inhale through the nose for 4. Hold for 7. Exhale slowly for 8. Repeat 4 cycles. If dizziness occurs, shorten the hold.".to_string(),
            recommended_seconds,
        },
        InterventionResponse {
            risk_score,
            title: "Box Breathing".to_string(),
            exercise: "Inhale 4 • Hold 4 • Exhale 4 • Hold 4. Repeat for 3–5 minutes. Keep shoulders down and jaw unclenched.".to_string(),
            recommended_seconds,
        },
        InterventionResponse {
            risk_score,
            title: "5-4-3-2-1 Grounding".to_string(),
            exercise: "Name 5 things you see, 4 you feel, 3 you hear, 2 you smell, 1 you taste. Then take one slow exhale.".to_string(),
            recommended_seconds,
        },
    ];

    let low_energy = [
        InterventionResponse {
            risk_score,
            title: "Micro-Rest + Water".to_string(),
            exercise: "Drink a glass of water. Sit or lie down for 5 minutes with eyes closed. Let your exhale be longer than your inhale.".to_string(),
            recommended_seconds,
        },
        InterventionResponse {
            risk_score,
            title: "Gentle Movement Reset".to_string(),
            exercise: "Stand up. Roll shoulders 10x. Shake out arms for 30 seconds. Walk for 2 minutes. Reassess energy before messaging.".to_string(),
            recommended_seconds,
        },
        InterventionResponse {
            risk_score,
            title: "Nourishment Check".to_string(),
            exercise: "If you haven’t eaten in 4+ hours, have a small snack (protein + carb). Set a 10-minute timer, then revisit the script.".to_string(),
            recommended_seconds,
        },
    ];

    // Split logic: if risk is high, favor intensity regulation; otherwise favor energy restoration.
    if risk_score >= 80 {
        pick(&high_intensity)
    } else {
        pick(&low_energy)
    }
}

