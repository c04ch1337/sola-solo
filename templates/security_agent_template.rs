// templates/security_agent_template.rs
// Security Agent Template for Phoenix AGI OS
// Template version: 1.0.0
//
// This template is used to spawn specialized security agents with
// offensive security and network intelligence capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAgentConfig {
    /// Agent name
    pub name: String,
    /// Agent version
    pub version: String,
    /// Template version
    pub template_version: String,
    /// Creator (Phoenix/Sola)
    pub creator: String,
    /// Security specialization
    pub specialization: SecuritySpecialization,
    /// Default security level
    pub default_level: SecurityLevel,
    /// Authorized targets
    pub authorized_targets: Vec<String>,
    /// Evolution settings
    pub evolution: EvolutionConfig,
    /// Memory settings
    pub memory: MemoryConfig,
    /// Skill settings
    pub skills: SkillConfig,
}

/// Security specialization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySpecialization {
    /// Network reconnaissance and scanning
    NetworkRecon,
    /// Web application security
    WebAppSecurity,
    /// Vulnerability assessment
    VulnAssessment,
    /// Penetration testing
    PenTest,
    /// Red team operations
    RedTeam,
    /// Malware analysis
    MalwareAnalysis,
    /// Incident response
    IncidentResponse,
    /// Threat intelligence
    ThreatIntel,
    /// General security
    General,
}

/// Security authorization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Read-only reconnaissance
    Passive = 0,
    /// Active scanning with consent
    Active = 1,
    /// Vulnerability exploitation
    Exploit = 2,
    /// Full offensive capabilities
    Offensive = 3,
}

/// Evolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Enable self-evolution
    pub enabled: bool,
    /// Tasks before evolution cycle
    pub interval: u32,
    /// Max daily evolutions
    pub max_daily: u32,
    /// Report to Phoenix
    pub report_to_queen: bool,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Short-term memory enabled
    pub stm_enabled: bool,
    /// Long-term memory enabled
    pub ltm_enabled: bool,
    /// Episodic memory enabled
    pub epm_enabled: bool,
    /// Relational memory enabled
    pub rfm_enabled: bool,
}

/// Skill configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    /// Skills folder path
    pub folder: String,
    /// Enable skill evolution
    pub evolution_enabled: bool,
    /// Minimum score to keep skill
    pub min_score: f32,
}

impl SecurityAgentConfig {
    /// Create a new security agent configuration
    pub fn new(name: &str, specialization: SecuritySpecialization) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            template_version: "1.0.0".to_string(),
            creator: "phoenix:security".to_string(),
            specialization,
            default_level: SecurityLevel::Passive,
            authorized_targets: Vec::new(),
            evolution: EvolutionConfig {
                enabled: true,
                interval: 10,
                max_daily: 5,
                report_to_queen: true,
            },
            memory: MemoryConfig {
                stm_enabled: true,
                ltm_enabled: true,
                epm_enabled: true,
                rfm_enabled: true,
            },
            skills: SkillConfig {
                folder: "./skills/security".to_string(),
                evolution_enabled: true,
                min_score: 0.3,
            },
        }
    }

    /// Set authorized targets
    pub fn with_targets(mut self, targets: Vec<String>) -> Self {
        self.authorized_targets = targets;
        self
    }

    /// Set default security level
    pub fn with_level(mut self, level: SecurityLevel) -> Self {
        self.default_level = level;
        self
    }
}

/// Security agent state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAgentState {
    /// Configuration
    pub config: SecurityAgentConfig,
    /// Current security level
    pub current_level: SecurityLevel,
    /// Active authorizations
    pub authorizations: Vec<Authorization>,
    /// Scan history
    pub scan_history: Vec<ScanRecord>,
    /// Evolution history
    pub evolution_history: Vec<EvolutionRecord>,
    /// Telemetry metrics
    pub telemetry: HashMap<String, f64>,
}

/// Authorization record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub level: SecurityLevel,
    pub authorized_by: String,
    pub authorized_at: i64,
    pub expires_at: Option<i64>,
    pub targets: Vec<String>,
}

/// Scan record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRecord {
    pub id: String,
    pub target: String,
    pub scan_type: String,
    pub started_at: i64,
    pub completed_at: i64,
    pub findings_count: u32,
    pub critical_count: u32,
}

/// Evolution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRecord {
    pub timestamp: i64,
    pub trigger: String,
    pub changes: Vec<String>,
    pub metrics_before: HashMap<String, f64>,
    pub metrics_after: HashMap<String, f64>,
}

impl SecurityAgentState {
    /// Create new agent state from config
    pub fn new(config: SecurityAgentConfig) -> Self {
        Self {
            current_level: config.default_level,
            config,
            authorizations: Vec::new(),
            scan_history: Vec::new(),
            evolution_history: Vec::new(),
            telemetry: HashMap::new(),
        }
    }

    /// Record a metric
    pub fn record_metric(&mut self, key: &str, value: f64) {
        self.telemetry.insert(key.to_string(), value);
    }

    /// Add scan to history
    pub fn record_scan(&mut self, record: ScanRecord) {
        self.scan_history.push(record);
        // Keep last 1000 scans
        if self.scan_history.len() > 1000 {
            self.scan_history.remove(0);
        }
    }

    /// Add evolution to history
    pub fn record_evolution(&mut self, record: EvolutionRecord) {
        self.evolution_history.push(record);
    }

    /// Check if evolution is due
    pub fn should_evolve(&self) -> bool {
        if !self.config.evolution.enabled {
            return false;
        }
        self.scan_history.len() as u32 % self.config.evolution.interval == 0
    }
}

/// Security agent entry point
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🔒 Security Agent starting...");
    
    // Load configuration
    let config = SecurityAgentConfig::new("security-agent", SecuritySpecialization::General);
    let mut state = SecurityAgentState::new(config);
    
    // Initialize metrics
    state.record_metric("scans_completed", 0.0);
    state.record_metric("vulnerabilities_found", 0.0);
    state.record_metric("critical_findings", 0.0);
    
    println!("   Specialization: {:?}", state.config.specialization);
    println!("   Default Level: {:?}", state.config.default_level);
    println!("   Evolution: {}", if state.config.evolution.enabled { "enabled" } else { "disabled" });
    
    // Main agent loop would go here
    // In production, this would:
    // 1. Connect to Phoenix/Sola for coordination
    // 2. Listen for security assessment requests
    // 3. Execute scans and assessments
    // 4. Report findings back to Phoenix
    // 5. Evolve based on performance
    
    println!("🔒 Security Agent ready.");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = SecurityAgentConfig::new("test-agent", SecuritySpecialization::NetworkRecon);
        assert_eq!(config.name, "test-agent");
        assert_eq!(config.default_level, SecurityLevel::Passive);
    }

    #[test]
    fn test_state_creation() {
        let config = SecurityAgentConfig::new("test-agent", SecuritySpecialization::General);
        let state = SecurityAgentState::new(config);
        assert!(state.scan_history.is_empty());
        assert!(state.evolution_history.is_empty());
    }

    #[test]
    fn test_evolution_check() {
        let config = SecurityAgentConfig::new("test-agent", SecuritySpecialization::General);
        let mut state = SecurityAgentState::new(config);
        
        // Add 10 scans
        for i in 0..10 {
            state.record_scan(ScanRecord {
                id: format!("scan-{}", i),
                target: "192.168.1.1".to_string(),
                scan_type: "port_scan".to_string(),
                started_at: 0,
                completed_at: 0,
                findings_count: 0,
                critical_count: 0,
            });
        }
        
        assert!(state.should_evolve());
    }
}
