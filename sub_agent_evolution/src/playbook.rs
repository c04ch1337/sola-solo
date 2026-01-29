// sub_agent_evolution/src/playbook.rs
// Playbook management for sub-agents — load, update, evolve.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub version: u32,
    pub updates: Vec<PlaybookUpdate>,
    pub telemetry: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookUpdate {
    pub ts_unix: i64,
    pub update: String,
}

impl Playbook {
    /// Load playbook from YAML file.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let playbook: Playbook = serde_yaml::from_str(&content)?;
        Ok(playbook)
    }

    /// Save playbook to YAML file.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add an update to the playbook.
    pub fn add_update(&mut self, update: String) {
        let ts_unix = chrono::Utc::now().timestamp();
        self.updates.push(PlaybookUpdate { ts_unix, update });
    }

    /// Record telemetry metric.
    pub fn record_metric(&mut self, key: String, value: f64) {
        self.telemetry.insert(key, value);
    }
}
