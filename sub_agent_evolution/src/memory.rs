// sub_agent_evolution/src/memory.rs
// Memory access layer for sub-agents — read-only or append-only access to Phoenix's LTM.

use neural_cortex_strata::{MemoryLayer, NeuralCortexStrata};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAccessLevel {
    ReadOnly,
    AppendOnly,
    None,
}

/// Memory accessor for sub-agents.
///
/// Provides bounded access to Phoenix's shared memory layers.
pub struct SubAgentMemory {
    access_level: MemoryAccessLevel,
    cortex: Option<Arc<NeuralCortexStrata>>,
    agent_prefix: String,
}

impl SubAgentMemory {
    pub fn new(
        access_level: MemoryAccessLevel,
        cortex: Option<Arc<NeuralCortexStrata>>,
        agent_name: &str,
    ) -> Self {
        Self {
            access_level,
            cortex,
            agent_prefix: format!("agent:{}:", agent_name),
        }
    }

    /// Read from shared LTM (if access granted).
    pub fn read_ltm(&self, key: &str) -> Option<MemoryLayer> {
        match self.access_level {
            MemoryAccessLevel::None => None,
            _ => self.cortex.as_ref()?.recall(key),
        }
    }

    /// Append to shared LTM (if append-only access granted).
    pub fn append_ltm(&self, key: &str, layer: MemoryLayer) -> Result<(), String> {
        match self.access_level {
            MemoryAccessLevel::AppendOnly => {
                let prefixed_key = format!("{}{}", self.agent_prefix, key);
                self.cortex
                    .as_ref()
                    .ok_or_else(|| "No cortex available".to_string())?
                    .etch(layer, &prefixed_key)
                    .map_err(|e| format!("Failed to etch memory: {}", e))
            }
            _ => Err("Append access not granted".to_string()),
        }
    }

    /// Recall memories with agent-specific prefix.
    pub fn recall_agent_memories(&self, limit: usize) -> Vec<(String, MemoryLayer)> {
        match self.access_level {
            MemoryAccessLevel::None => Vec::new(),
            _ => self
                .cortex
                .as_ref()
                .map(|c| c.recall_prefix(&self.agent_prefix, limit))
                .unwrap_or_default(),
        }
    }
}
