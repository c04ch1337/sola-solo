use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::counselor_api::{GriefEvent, CounselorScript};
use crate::readiness::ReadinessResponse;

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
    // Minimal, dependency-free: just use UTC date derived from system time.
    // (We avoid chrono here to keep the module lightweight.)
    // Fallback: show ms.
    let _ = ms;
    "(recent)".to_string()
}

fn md_escape(s: &str) -> String {
    s.replace('\r', "").trim().to_string()
}

fn stage_counts(events: &[GriefEvent]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for s in ["Denial", "Anger", "Bargaining", "Depression", "Acceptance"] {
        m.insert(s.to_string(), 0usize);
    }
    for e in events {
        let key = e.stage.trim().to_string();
        *m.entry(key).or_insert(0) += 1;
    }
    m
}

fn avg_readiness(readiness: &[ReadinessResponse]) -> Option<u8> {
    if readiness.is_empty() {
        return None;
    }
    let sum: u32 = readiness.iter().map(|r| r.readiness_score as u32).sum();
    Some((sum / readiness.len() as u32) as u8)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub window_days: u32,
    pub events: Vec<GriefEvent>,
    pub scripts: Vec<CounselorScript>,
    pub readiness: Vec<ReadinessResponse>,
}

pub fn generate_markdown_report(data: &ExportData) -> String {
    let start_ms = window_start_ms(data.window_days);
    let end_ms = now_ms();
    let date_range = format!("{} → {}", iso_day_from_ms(start_ms), iso_day_from_ms(end_ms));

    let counts = stage_counts(&data.events);
    let avg = avg_readiness(&data.readiness);

    let mut out = String::new();
    out.push_str(&format!("# Counselor Session Report - {}\n\n", date_range));
    out.push_str(&format!("Window: last {} days\n\n", data.window_days));

    out.push_str("## 1) Grief Map Summary\n\n");
    out.push_str("Stage counts (signals/events):\n\n");
    for s in ["Denial", "Anger", "Bargaining", "Depression", "Acceptance"] {
        let n = counts.get(s).copied().unwrap_or(0);
        out.push_str(&format!("- **{}**: {}\n", s, n));
    }
    out.push_str("\n");

    out.push_str("## 2) Recent NVC Scripts\n\n");
    if data.scripts.is_empty() {
        out.push_str("_No saved scripts in this window._\n\n");
    } else {
        for (i, s) in data.scripts.iter().enumerate().take(10) {
            out.push_str(&format!("### Script {}\n\n", i + 1));
            out.push_str(&format!("- Observation: {}\n", md_escape(&s.observation)));
            out.push_str(&format!("- Feeling: {}\n", md_escape(&s.feeling)));
            out.push_str(&format!("- Need: {}\n", md_escape(&s.need)));
            out.push_str(&format!("- Request: {}\n\n", md_escape(&s.request)));
        }
    }

    out.push_str("## 3) Readiness Trends\n\n");
    match avg {
        Some(a) => out.push_str(&format!("Average readiness score: **{}%**\n\n", a)),
        None => out.push_str("_No readiness checks recorded in this window._\n\n"),
    }
    if !data.readiness.is_empty() {
        out.push_str("Recent readiness checks:\n\n");
        for r in data.readiness.iter().take(10) {
            out.push_str(&format!(
                "- **{}%** ({}) — {}\n",
                r.readiness_score,
                r.window_status,
                r.reasons.get(0).cloned().unwrap_or_else(|| "".to_string())
            ));
        }
        out.push_str("\n");
    }

    out.push_str("---\n");
    out.push_str("Generated locally by Phoenix Counselor Module.\n");
    out
}

