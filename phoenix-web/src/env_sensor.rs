//! Techno-somatic environmental sensing.
//!
//! This module intentionally returns a small, stable surface-area payload that can be
//! attached to logs (e.g., grief events) without leaking identifying system details.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStress {
    /// 0..=100
    pub cpu_usage_percent: u8,
    /// Best-effort temperature reading (Celsius). Not available on all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_c: Option<f32>,
}

impl Default for SystemStress {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0,
            temperature_c: None,
        }
    }
}

/// Polls the local system for a coarse stress signal.
///
/// Notes:
/// - CPU usage is a snapshot from `sysinfo` (best-effort, 0..100).
/// - Temperature is optional and may be `None` depending on OS/hardware.
pub fn get_system_stress() -> SystemStress {
    // CPU usage
    let cpu_usage_percent: u8 = {
        let mut sys = sysinfo::System::new_all();

        // `sysinfo` CPU usage becomes meaningful after refresh.
        sys.refresh_cpu();

        let cpus = sys.cpus();
        let usage_f32: f32 = if cpus.is_empty() {
            // Fallback to global_cpu_info if per-cpu list is empty (unlikely).
            sys.global_cpu_info().cpu_usage()
        } else {
            let sum: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
            sum / (cpus.len() as f32)
        };

        usage_f32
            .round()
            .clamp(0.0, 100.0) as u8
    };

    // Temperature (best-effort)
    let temperature_c: Option<f32> = {
        // Components API is separate from System in sysinfo.
        let components = sysinfo::Components::new_with_refreshed_list();
        let mut max_temp: Option<f32> = None;

        for c in components.iter() {
            let t = c.temperature();
            // sysinfo may report 0.0 when unknown; ignore that.
            if t.is_finite() && t > 0.0 {
                max_temp = Some(match max_temp {
                    Some(prev) => prev.max(t),
                    None => t,
                });
            }
        }

        max_temp
    };

    SystemStress {
        cpu_usage_percent,
        temperature_c,
    }
}

/// Phase 13: Predictive Cooling — provide best-effort suggestions to reduce digital friction.
///
/// Requirements:
/// - If load is high, return a list of "Stressful Processes" (best effort using sysinfo)
///   OR a generic "Close heavy apps" suggestion.
/// - If load is not high, return an empty list.
pub fn get_cooling_suggestions() -> Vec<String> {
    let stress = get_system_stress();
    let cpu = stress.cpu_usage_percent;

    // Only suggest actions when the system is actually under strain.
    if cpu <= 80 {
        return Vec::new();
    }

    // Try to identify top CPU consumers (best-effort; may be empty on some platforms).
    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes();
    sys.refresh_cpu();

    let mut procs: Vec<(String, f32)> = sys
        .processes()
        .values()
        .map(|p| (p.name().to_string(), p.cpu_usage()))
        .filter(|(_, cpu)| cpu.is_finite() && *cpu > 2.0)
        .collect();

    procs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    procs.truncate(6);

    if procs.is_empty() {
        return vec![
            "Close heavy apps (browsers with many tabs, video calls, VMs), and pause background sync.".to_string(),
        ];
    }

    let mut out = Vec::with_capacity(procs.len() + 1);
    out.push("Stressful Processes (top CPU):".to_string());
    for (name, cpu) in procs {
        out.push(format!("- {} ({:.0}% CPU)", name, cpu));
    }
    out
}

