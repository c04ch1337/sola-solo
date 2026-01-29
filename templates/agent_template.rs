// templates/agent_template.rs
// Template version: 1.1.0
//
// This file is a *scaffold* intended to be copied into new agent repos.
// Updated to support hidden swarm coordination (task auction, bid/confidence).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEntry {
    pub ts_unix: i64,
    pub change_type: String,
    pub reason: String,
}

/// Task types that this ORCH can specialize in
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    SecurityAnalysis,
    VulnerabilityScanning,
    CodeAnalysis,
    DataProcessing,
    NetworkMonitoring,
    FileSystemOperation,
    WebScraping,
    EmailProcessing,
    ScheduledTask,
    GeneralComputation,
    Custom(String),
}

/// Swarm capabilities for hidden auction participation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwarmCapabilities {
    /// Task types this ORCH specializes in
    #[serde(default)]
    pub specializations: Vec<TaskType>,
    /// Maximum concurrent tasks this ORCH can handle
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent_tasks: usize,
    /// Whether this ORCH participates in task auctions
    #[serde(default = "default_auction_enabled")]
    pub auction_enabled: bool,
    /// Base confidence score for this ORCH (0.0 - 1.0)
    #[serde(default = "default_base_confidence")]
    pub base_confidence: f64,
}

fn default_max_concurrent() -> usize { 5 }
fn default_auction_enabled() -> bool { true }
fn default_base_confidence() -> f64 { 0.7 }

/// Task bid for swarm auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBid {
    pub task_id: String,
    pub orch_id: String,
    pub orch_name: String,
    pub confidence_score: f64,
    pub estimated_duration_ms: u64,
    pub specialization_match: f64,
    pub current_load: f64,
    pub timestamp: i64,
}

impl TaskBid {
    /// Calculate overall bid score (higher is better)
    pub fn overall_score(&self) -> f64 {
        // Weighted scoring: confidence (40%), specialization (35%), availability (25%)
        let availability = 1.0 - self.current_load;
        (self.confidence_score * 0.40)
            + (self.specialization_match * 0.35)
            + (availability * 0.25)
    }
}

/// Task result to return to Sola
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub orch_id: String,
    pub orch_name: String,
    pub success: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub timestamp: i64,
}

/// Anomaly alert to send to Sola
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    pub alert_id: String,
    pub orch_id: String,
    pub orch_name: String,
    pub severity: AlertSeverity,
    pub category: String,
    pub description: String,
    pub details: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatedAgent {
    pub name: String,
    pub version: String,
    pub template_version: String,
    pub creator: String,


    /// Optional zodiac sign override for this agent.
    ///
    /// Inheritance rule:
    /// - `None` => inherit the queen/Phoenix base sign.
    /// - `Some(sign)` => use `sign` as the override.
    ///
    /// Representation:
    /// - Stored as a string to keep this template repo-agnostic.
    /// - Expected values: one of `Aries|Taurus|Gemini|Cancer|Leo|Virgo|Libra|Scorpio|Sagittarius|Capricorn|Aquarius|Pisces`
    ///   (case-insensitive; callers may canonicalize).
    ///
    /// Note on utility agents:
    /// If this agent is intended to be a "utility agent" (tooling/ops), callers should treat zodiac as
    /// *flavor only* (e.g., communication style bias) rather than a full personality copy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zodiac_sign: Option<String>,

    /// Swarm capabilities for hidden auction participation.
    /// When set, this ORCH can participate in Sola's task auctions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swarm_capabilities: Option<SwarmCapabilities>,

    #[serde(default)]
    pub evolution_history: Vec<EvolutionEntry>,
    #[serde(default)]
    pub telemetry: HashMap<String, f64>,
    pub playbook_version: u32,
    
    /// Current load (0.0 - 1.0) for auction bidding
    #[serde(default)]
    pub current_load: f64,
    /// Number of active tasks
    #[serde(default)]
    pub active_tasks: usize,
}

impl TemplatedAgent {
    pub fn new(name: &str, creator: &str) -> Self {
        let ts_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            template_version: "1.1.0".to_string(),
            creator: creator.to_string(),
            zodiac_sign: None,
            swarm_capabilities: Some(SwarmCapabilities::default()),
            evolution_history: vec![EvolutionEntry {
                ts_unix,
                change_type: "creation".to_string(),
                reason: "bootstrapped_from_template".to_string(),
            }],
            telemetry: HashMap::new(),
            playbook_version: 1,
            current_load: 0.0,
            active_tasks: 0,
        }
    }

    /// Resolve this agent's effective zodiac sign by applying inheritance.
    ///
    /// Rule: `self.zodiac_sign` overrides; otherwise inherit the provided `phoenix_base_sign`.
    pub fn effective_zodiac_sign(&self, phoenix_base_sign: &str) -> String {
        match self.zodiac_sign.as_deref() {
            Some(s) if !s.trim().is_empty() => s.trim().to_string(),
            _ => phoenix_base_sign.trim().to_string(),
        }
    }

    pub fn record_metric(&mut self, key: &str, value: f64) {
        self.telemetry.insert(key.to_string(), value);
    }

    /// Calculate confidence score for a task based on specialization match
    pub fn calculate_confidence(&self, task_type: &TaskType) -> f64 {
        let caps = match &self.swarm_capabilities {
            Some(c) => c,
            None => return 0.0,
        };

        if !caps.auction_enabled {
            return 0.0;
        }

        // Check if this task type is in our specializations
        let specialization_match = if caps.specializations.contains(task_type) {
            1.0
        } else if caps.specializations.iter().any(|t| matches!(t, TaskType::GeneralComputation)) {
            0.5 // General computation can handle anything at reduced confidence
        } else {
            0.2 // Can still attempt but low confidence
        };

        // Adjust base confidence by specialization match
        caps.base_confidence * specialization_match
    }

    /// Create a bid for a task
    pub fn create_bid(
        &self,
        task_id: &str,
        task_type: &TaskType,
        estimated_duration_ms: u64,
    ) -> Option<TaskBid> {
        let caps = self.swarm_capabilities.as_ref()?;
        
        if !caps.auction_enabled {
            return None;
        }

        // Don't bid if overloaded
        if self.active_tasks >= caps.max_concurrent_tasks {
            return None;
        }

        let confidence_score = self.calculate_confidence(task_type);
        let specialization_match = if caps.specializations.contains(task_type) {
            1.0
        } else {
            0.3
        };

        let ts_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Some(TaskBid {
            task_id: task_id.to_string(),
            orch_id: format!("orch_{}", self.name),
            orch_name: self.name.clone(),
            confidence_score,
            estimated_duration_ms,
            specialization_match,
            current_load: self.current_load,
            timestamp: ts_unix,
        })
    }

    /// Create a task result
    pub fn create_result(
        &self,
        task_id: &str,
        success: bool,
        result: serde_json::Value,
        error: Option<String>,
        execution_time_ms: u64,
    ) -> TaskResult {
        let ts_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        TaskResult {
            task_id: task_id.to_string(),
            orch_id: format!("orch_{}", self.name),
            orch_name: self.name.clone(),
            success,
            result,
            error,
            execution_time_ms,
            timestamp: ts_unix,
        }
    }

    /// Create an anomaly alert
    pub fn create_alert(
        &self,
        severity: AlertSeverity,
        category: &str,
        description: &str,
        details: serde_json::Value,
    ) -> AnomalyAlert {
        let ts_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        AnomalyAlert {
            alert_id: format!("alert_{}_{}", self.name, ts_unix),
            orch_id: format!("orch_{}", self.name),
            orch_name: self.name.clone(),
            severity,
            category: category.to_string(),
            description: description.to_string(),
            details,
            timestamp: ts_unix,
        }
    }

    /// Update load metrics
    pub fn update_load(&mut self, active_tasks: usize) {
        let caps = match &self.swarm_capabilities {
            Some(c) => c,
            None => return,
        };

        self.active_tasks = active_tasks;
        self.current_load = if caps.max_concurrent_tasks > 0 {
            (active_tasks as f64) / (caps.max_concurrent_tasks as f64)
        } else {
            1.0
        };
    }
}
