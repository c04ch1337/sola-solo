//! MITRE ATT&CK integration for reporting

use serde::{Deserialize, Serialize};

/// MITRE ATT&CK technique information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechnique {
    pub id: String,
    pub name: String,
    pub tactics: Vec<String>,
    pub description: String,
    pub mitigations: Vec<String>,
}

/// Map vulnerability types to MITRE ATT&CK techniques
pub fn map_vulnerability_to_mitre(vuln_type: &str) -> Vec<MitreTechnique> {
    match vuln_type.to_lowercase().as_str() {
        "xss" | "cross-site scripting" => vec![
            MitreTechnique {
                id: "T1189".to_string(),
                name: "Drive-by Compromise".to_string(),
                tactics: vec!["Initial Access".to_string()],
                description: "XSS can be leveraged for drive-by attacks".to_string(),
                mitigations: vec![
                    "Input validation".to_string(),
                    "Output encoding".to_string(),
                    "Content Security Policy".to_string(),
                ],
            },
        ],
        "sqli" | "sql injection" => vec![
            MitreTechnique {
                id: "T1190".to_string(),
                name: "Exploit Public-Facing Application".to_string(),
                tactics: vec!["Initial Access".to_string()],
                description: "SQL injection exploits web application vulnerabilities".to_string(),
                mitigations: vec![
                    "Parameterized queries".to_string(),
                    "Input validation".to_string(),
                    "Least privilege database access".to_string(),
                ],
            },
        ],
        "cmdinj" | "command injection" | "rce" => vec![
            MitreTechnique {
                id: "T1059".to_string(),
                name: "Command and Scripting Interpreter".to_string(),
                tactics: vec!["Execution".to_string()],
                description: "Command injection enables arbitrary command execution".to_string(),
                mitigations: vec![
                    "Avoid system calls".to_string(),
                    "Input validation".to_string(),
                    "Sandboxing".to_string(),
                ],
            },
        ],
        _ => vec![],
    }
}
