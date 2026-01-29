// sub_agent_evolution/src/mitre.rs
// MITRE ATT&CK integration for security-focused sub-agents.
//
// Queries the MITRE ATT&CK API for techniques, maps file behaviors to tactics,
// and enables proactive re-analysis on new ATT&CK updates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MITRE_API_BASE: &str = "https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechnique {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tactics: Vec<String>,
    pub detection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorMapping {
    pub behavior: String,
    pub technique_id: String,
    pub confidence: f64,
}

/// Check for new MITRE ATT&CK patterns relevant to the agent.
///
/// This is a simplified implementation — in production, you'd cache the ATT&CK
/// matrix locally and query it efficiently.
pub async fn check_new_patterns(agent_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("[{}] Checking MITRE ATT&CK for new patterns...", agent_name);

    // In a real implementation, you would:
    // 1. Fetch the latest ATT&CK matrix from GitHub or the official API
    // 2. Compare with cached version to find new techniques
    // 3. Filter by relevance to the agent's domain
    // 4. Return new patterns

    // For now, return a placeholder
    let patterns = vec![
        "T1027: Obfuscated Files or Information".to_string(),
        "T1055: Process Injection".to_string(),
        "T1059: Command and Scripting Interpreter".to_string(),
    ];

    Ok(patterns)
}

/// Map a file behavior to MITRE ATT&CK techniques.
///
/// This uses heuristics to match observed behaviors to known tactics/techniques.
pub fn map_behavior_to_technique(behavior: &str) -> Vec<BehaviorMapping> {
    let mut mappings = Vec::new();

    // Simple keyword-based mapping (in production, use ML or rule engine)
    let behavior_lower = behavior.to_lowercase();

    if behavior_lower.contains("obfuscate") || behavior_lower.contains("encode") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1027".to_string(),
            confidence: 0.85,
        });
    }

    if behavior_lower.contains("inject") || behavior_lower.contains("process") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1055".to_string(),
            confidence: 0.80,
        });
    }

    if behavior_lower.contains("script") || behavior_lower.contains("powershell") || behavior_lower.contains("cmd") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1059".to_string(),
            confidence: 0.90,
        });
    }

    if behavior_lower.contains("registry") || behavior_lower.contains("reg add") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1112".to_string(),
            confidence: 0.88,
        });
    }

    if behavior_lower.contains("credential") || behavior_lower.contains("password") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1003".to_string(),
            confidence: 0.92,
        });
    }

    if behavior_lower.contains("network") || behavior_lower.contains("socket") || behavior_lower.contains("http") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1071".to_string(),
            confidence: 0.75,
        });
    }

    if behavior_lower.contains("file") && (behavior_lower.contains("delete") || behavior_lower.contains("modify")) {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1485".to_string(),
            confidence: 0.70,
        });
    }

    if behavior_lower.contains("persistence") || behavior_lower.contains("startup") {
        mappings.push(BehaviorMapping {
            behavior: behavior.to_string(),
            technique_id: "T1547".to_string(),
            confidence: 0.82,
        });
    }

    mappings
}

/// Fetch MITRE ATT&CK technique details (simplified).
///
/// In production, this would query the official MITRE ATT&CK API or local cache.
pub async fn fetch_technique_details(technique_id: &str) -> Result<MitreTechnique, Box<dyn std::error::Error>> {
    // Placeholder implementation
    // In production, fetch from:
    // https://attack.mitre.org/techniques/{technique_id}/
    // or the STIX/TAXII API

    let technique = match technique_id {
        "T1027" => MitreTechnique {
            id: "T1027".to_string(),
            name: "Obfuscated Files or Information".to_string(),
            description: "Adversaries may attempt to make an executable or file difficult to discover or analyze by encrypting, encoding, or otherwise obfuscating its contents.".to_string(),
            tactics: vec!["Defense Evasion".to_string()],
            detection: Some("Monitor for suspicious file modifications or encoding activities.".to_string()),
        },
        "T1055" => MitreTechnique {
            id: "T1055".to_string(),
            name: "Process Injection".to_string(),
            description: "Adversaries may inject code into processes in order to evade process-based defenses or elevate privileges.".to_string(),
            tactics: vec!["Defense Evasion".to_string(), "Privilege Escalation".to_string()],
            detection: Some("Monitor for unusual process behavior and memory modifications.".to_string()),
        },
        "T1059" => MitreTechnique {
            id: "T1059".to_string(),
            name: "Command and Scripting Interpreter".to_string(),
            description: "Adversaries may abuse command and script interpreters to execute commands, scripts, or binaries.".to_string(),
            tactics: vec!["Execution".to_string()],
            detection: Some("Monitor for suspicious command-line activity and script execution.".to_string()),
        },
        _ => {
            return Err(format!("Unknown technique: {}", technique_id).into());
        }
    };

    Ok(technique)
}

/// Generate a detection rule based on MITRE ATT&CK technique.
pub fn generate_detection_rule(technique: &MitreTechnique) -> String {
    format!(
        "# Detection Rule for {}\n\
        # Technique: {} ({})\n\
        # Tactics: {}\n\
        # Description: {}\n\
        # Detection: {}\n\
        \n\
        rule detect_{} {{\n\
        \tmeta:\n\
        \t\ttechnique = \"{}\"\n\
        \t\ttactics = \"{}\"\n\
        \tstrings:\n\
        \t\t// Add detection patterns here\n\
        \tcondition:\n\
        \t\t// Add detection logic here\n\
        }}",
        technique.name,
        technique.id,
        technique.name,
        technique.tactics.join(", "),
        technique.description,
        technique.detection.as_deref().unwrap_or("N/A"),
        technique.id.to_lowercase().replace("-", "_"),
        technique.id,
        technique.tactics.join(", ")
    )
}

/// Proactive re-analysis: check if files should be re-scanned based on new ATT&CK updates.
pub async fn should_rescan(
    file_path: &str,
    last_scan_ts: i64,
    mitre_last_update_ts: i64,
) -> bool {
    // If MITRE ATT&CK was updated after the last scan, re-scan
    mitre_last_update_ts > last_scan_ts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavior_mapping() {
        let mappings = map_behavior_to_technique("File uses obfuscated strings");
        assert!(!mappings.is_empty());
        assert_eq!(mappings[0].technique_id, "T1027");
        assert!(mappings[0].confidence > 0.8);
    }

    #[test]
    fn test_multiple_behaviors() {
        let mappings = map_behavior_to_technique("Process injection via PowerShell script");
        assert!(mappings.len() >= 2);
        let ids: Vec<&str> = mappings.iter().map(|m| m.technique_id.as_str()).collect();
        assert!(ids.contains(&"T1055"));
        assert!(ids.contains(&"T1059"));
    }

    #[tokio::test]
    async fn test_fetch_technique() {
        let result = fetch_technique_details("T1027").await;
        assert!(result.is_ok());
        let technique = result.unwrap();
        assert_eq!(technique.id, "T1027");
        assert!(technique.name.contains("Obfuscated"));
    }
}
