use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

use uuid::Uuid;

use crate::counselor_api::GriefEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCorrelation {
    pub tag: String,
    pub frequency: usize,
    pub avg_intensity: f32, // 0..100
    pub avg_energy: f32,    // 0..100
    /// Pearson correlation coefficient between energy and intensity for events with this tag.
    pub corr_energy_intensity: f32, // -1..1
    /// Heuristic label for impact pattern.
    pub impact: String,
    /// Risk score 0..100 emphasizing "high intensity / low energy" windows.
    pub risk_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationsResponse {
    pub success: bool,
    pub window_days: u32,
    pub total_events: usize,
    pub correlations: Vec<TagCorrelation>,
    /// Highest-risk tag if any.
    pub top_trigger: Option<TagCorrelation>,

    /// Semantic ↔ episodic bridge: context tags that appear in the global context note
    /// and are associated with high-intensity events.
    #[serde(default)]
    pub hotspots: Vec<Hotspot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub tag: String,
    pub frequency: u32,
    /// 0..=100
    pub avg_intensity: u8,
}

fn normalize_label(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
}

fn default_stopwords() -> HashSet<&'static str> {
    // Small, conservative list. Expand as needed.
    [
        "a", "an", "and", "are", "as", "at", "be", "because", "but", "by", "for", "from",
        "have", "i", "if", "in", "into", "is", "it", "its", "me", "my", "of", "on", "or",
        "our", "so", "that", "the", "their", "then", "this", "to", "was", "we", "with", "you",
        "your",
    ]
    .into_iter()
    .collect()
}

/// Semantic memory → episodic tags intersection.
///
/// - Tokenizes the context note (alnum-only, lowercased, stopwords removed)
/// - Matches tokens against `context_tags` using a normalized comparator
/// - Flags tags where intensity is meaningfully high
pub fn find_contextual_hotspots(context: &str, events: &[GriefEvent]) -> Vec<Hotspot> {
    let stop = default_stopwords();

    // Build token presence from context note.
    let mut context_tokens: HashSet<String> = HashSet::new();
    for raw in context
        .split(|c: char| !c.is_ascii_alphanumeric())
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
    {
        let t_lc = raw.to_ascii_lowercase();
        if t_lc.len() < 3 {
            continue;
        }
        if stop.contains(t_lc.as_str()) {
            continue;
        }
        context_tokens.insert(normalize_label(&t_lc));
    }

    if context_tokens.is_empty() {
        return vec![];
    }

    // Aggregate high-intensity occurrences for tags that appear in context tokens.
    let mut tag_acc: HashMap<String, (u32, u32)> = HashMap::new(); // tag -> (count, sum_intensity)
    let hi_threshold: u8 = 70;
    for e in events {
        if e.intensity < hi_threshold {
            continue;
        }
        for tag in &e.context_tags {
            let ntag = normalize_label(tag);
            if ntag.is_empty() {
                continue;
            }
            if !context_tokens.contains(&ntag) {
                continue;
            }
            let entry = tag_acc.entry(tag.trim().to_string()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += e.intensity as u32;
        }
    }

    let mut out: Vec<Hotspot> = tag_acc
        .into_iter()
        .filter_map(|(tag, (count, sum_i))| {
            if count == 0 {
                return None;
            }
            let avg = (sum_i as f32 / count as f32).round().clamp(0.0, 100.0) as u8;
            Some(Hotspot {
                tag,
                frequency: count,
                avg_intensity: avg,
            })
        })
        .collect();

    // Highest frequency first; then highest intensity.
    out.sort_by(|a, b| {
        b.frequency
            .cmp(&a.frequency)
            .then_with(|| b.avg_intensity.cmp(&a.avg_intensity))
            .then_with(|| a.tag.cmp(&b.tag))
    });

    out
}

fn clamp01(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

fn pearson_corr(xs: &[f32], ys: &[f32]) -> f32 {
    if xs.len() != ys.len() || xs.len() < 2 {
        return 0.0;
    }
    let n = xs.len() as f32;
    let mean_x = xs.iter().sum::<f32>() / n;
    let mean_y = ys.iter().sum::<f32>() / n;

    let mut num = 0.0;
    let mut den_x = 0.0;
    let mut den_y = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        let dx = x - mean_x;
        let dy = y - mean_y;
        num += dx * dy;
        den_x += dx * dx;
        den_y += dy * dy;
    }
    if den_x <= 0.0 || den_y <= 0.0 {
        return 0.0;
    }
    (num / (den_x.sqrt() * den_y.sqrt())).clamp(-1.0, 1.0)
}

fn impact_label(avg_i: f32, avg_e: f32, corr: f32) -> String {
    // simple qualitative mapping
    if avg_i >= 70.0 && avg_e <= 35.0 {
        return "High Intensity / Low Energy".to_string();
    }
    if avg_i >= 70.0 && avg_e >= 70.0 {
        return "High Intensity / High Energy".to_string();
    }
    if avg_i <= 35.0 && avg_e <= 35.0 {
        return "Low Intensity / Low Energy".to_string();
    }
    if avg_i <= 35.0 && avg_e >= 70.0 {
        return "Low Intensity / High Energy".to_string();
    }
    if corr < -0.35 {
        return "Energy drop correlates with intensity spike".to_string();
    }
    "Mixed".to_string()
}

fn risk_score(avg_i: f32, avg_e: f32, freq: usize) -> u8 {
    // Emphasize burnout windows: intensity high + energy low.
    // Add a mild frequency boost.
    let intensity = clamp01(avg_i / 100.0);
    let energy_low = clamp01(1.0 - (avg_e / 100.0));
    let base = intensity * 0.65 + energy_low * 0.35;
    let freq_boost = (freq as f32).ln_1p() / 3.0; // ~0..1
    ((base * 90.0 + freq_boost * 10.0).round().clamp(0.0, 100.0)) as u8
}

/// Group events by tags and compute per-tag correlations.
pub fn calculate_trigger_correlations(events: &[GriefEvent]) -> Vec<TagCorrelation> {
    let mut tag_to_events: HashMap<String, Vec<&GriefEvent>> = HashMap::new();
    for e in events {
        for tag in &e.context_tags {
            let t = tag.trim();
            if t.is_empty() {
                continue;
            }
            tag_to_events.entry(t.to_string()).or_default().push(e);
        }
    }

    let mut out: Vec<TagCorrelation> = Vec::new();
    for (tag, evs) in tag_to_events {
        let freq = evs.len();
        if freq == 0 {
            continue;
        }
        let intensities: Vec<f32> = evs.iter().map(|e| e.intensity as f32).collect();
        let energies: Vec<f32> = evs.iter().map(|e| e.energy_level as f32).collect();
        let avg_i = intensities.iter().sum::<f32>() / freq as f32;
        let avg_e = energies.iter().sum::<f32>() / freq as f32;
        let corr = pearson_corr(&energies, &intensities);
        let impact = impact_label(avg_i, avg_e, corr);
        let rs = risk_score(avg_i, avg_e, freq);

        out.push(TagCorrelation {
            tag,
            frequency: freq,
            avg_intensity: avg_i,
            avg_energy: avg_e,
            corr_energy_intensity: corr,
            impact,
            risk_score: rs,
        });
    }

    // Sort highest-risk first
    out.sort_by(|a, b| {
        b.risk_score
            .cmp(&a.risk_score)
            .then_with(|| b.frequency.cmp(&a.frequency))
            .then_with(|| a.tag.cmp(&b.tag))
    });

    out
}

// ---
// Phase 16b: Drift Analysis (Ghost session enmeshment)
// ---

static GHOST_SESSION_STARTS: OnceLock<Mutex<HashMap<Uuid, u8>>> = OnceLock::new();

fn ghost_session_map() -> &'static Mutex<HashMap<Uuid, u8>> {
    GHOST_SESSION_STARTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostDrift {
    pub session_id: String,
    /// 0..=100
    pub system_load_start: u8,
    /// 0..=100
    pub system_load_end: u8,
    /// Signed delta: end - start
    pub drift_delta: i16,
    /// True when the stress delta is significant.
    pub drift_alert: bool,
}

/// Records the start of a ghost session and returns a session id.
pub fn record_ghost_session_start(system_load_start: u8) -> Uuid {
    let id = Uuid::new_v4();
    if let Ok(mut m) = ghost_session_map().lock() {
        m.insert(id, system_load_start.min(100));
        // Best-effort GC: bound the map.
        if m.len() > 2_000 {
            // Drain arbitrary oldest-ish entries (HashMap has no order; this is best-effort).
            let to_remove: Vec<Uuid> = m.keys().take(500).cloned().collect();
            for k in to_remove {
                m.remove(&k);
            }
        }
    }
    id
}

/// Calculates drift based on previously recorded start load and the current end load.
///
/// If the session id is unknown, assumes `start == end`.
pub fn calculate_drift(session_id: Uuid, system_load_end: u8) -> GhostDrift {
    let end = system_load_end.min(100);
    let start = if let Ok(mut m) = ghost_session_map().lock() {
        m.remove(&session_id).unwrap_or(end)
    } else {
        end
    };

    let delta: i16 = (end as i16) - (start as i16);

    // Alert heuristic: large spike and/or high ending load.
    // - delta >= +18 is a meaningful jump
    // - end >= 85 is “machine heartbeat” at high strain
    let drift_alert = delta >= 18 || end >= 85;

    GhostDrift {
        session_id: session_id.to_string(),
        system_load_start: start,
        system_load_end: end,
        drift_delta: delta,
        drift_alert,
    }
}

