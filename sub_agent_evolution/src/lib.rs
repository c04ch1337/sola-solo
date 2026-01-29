// sub_agent_evolution/src/lib.rs
// Bounded self-evolution for spawned agents — they learn, improve, but stay specialized.
//
// Sub-agents inherit from Phoenix's core systems (memory, skills, playbook) and evolve
// within their scope. They cannot become independent AGI — they remain bounded tools.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

pub mod mitre;
pub mod memory;
pub mod playbook;
pub mod skills;

#[derive(Debug, Error)]
pub enum EvolutionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Evolution limit reached: {0}")]
    LimitReached(String),
    #[error("Invalid evolution: {0}")]
    Invalid(String),
}

/// Short-term memory for a sub-agent (per-session, in-memory).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShortTermMemory {
    pub session_id: String,
    pub entries: Vec<MemoryEntry>,
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub ts_unix: i64,
    pub key: String,
    pub value: String,
    pub context: Option<String>,
}

impl ShortTermMemory {
    pub fn new(session_id: String, max_entries: usize) -> Self {
        Self {
            session_id,
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn store(&mut self, key: String, value: String, context: Option<String>) {
        let entry = MemoryEntry {
            ts_unix: Utc::now().timestamp(),
            key,
            value,
            context,
        };
        self.entries.push(entry);

        // Keep only the most recent entries
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }
    }

    pub fn recall(&self, key: &str) -> Option<&MemoryEntry> {
        self.entries.iter().rev().find(|e| e.key == key)
    }

    pub fn recent(&self, limit: usize) -> Vec<&MemoryEntry> {
        self.entries.iter().rev().take(limit).collect()
    }
}

/// Evolution cycle report for a sub-agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentEvolutionReport {
    pub agent_name: String,
    pub ts_unix: i64,
    pub cycle_number: u64,
    pub tasks_completed: u64,
    pub accuracy_score: f64,
    pub playbook_updates: Vec<String>,
    pub skills_learned: Vec<String>,
    pub memory_insights: Vec<String>,
    pub mitre_patterns: Vec<String>,
}

/// Bounded evolution loop for sub-agents.
///
/// This is a "child" version of Phoenix's AutonomousEvolutionLoop — it evolves
/// playbooks, skills, and memory, but does NOT self-modify code.
pub struct SubAgentEvolutionLoop {
    pub agent_name: String,
    pub cycle_number: u64,
    pub tasks_completed: u64,
    pub evolution_interval: u64, // Evolve after N tasks
    pub max_playbook_updates: usize,
    pub max_skills: usize,
    pub stm: ShortTermMemory,
    pub playbook_path: String,
    pub skills_path: String,
}

impl SubAgentEvolutionLoop {
    pub fn new(
        agent_name: String,
        session_id: String,
        evolution_interval: u64,
        playbook_path: String,
        skills_path: String,
    ) -> Self {
        Self {
            agent_name,
            cycle_number: 0,
            tasks_completed: 0,
            evolution_interval,
            max_playbook_updates: 100, // Bounded: max 100 playbook updates
            max_skills: 50,            // Bounded: max 50 skills
            stm: ShortTermMemory::new(session_id, 100), // Keep last 100 STM entries
            playbook_path,
            skills_path,
        }
    }

    /// Record task completion and check if evolution should trigger.
    pub fn record_task(&mut self, success: bool, feedback: Option<String>) -> bool {
        self.tasks_completed += 1;

        // Store in STM
        self.stm.store(
            format!("task_{}", self.tasks_completed),
            if success { "success" } else { "failure" }.to_string(),
            feedback,
        );

        // Check if we should evolve
        self.tasks_completed % self.evolution_interval == 0
    }

    /// Run one bounded evolution cycle.
    ///
    /// This reflects on recent tasks, updates playbook/skills, and stores insights.
    /// It does NOT modify code — only config/memory.
    pub async fn run_cycle(
        &mut self,
        llm: &llm_orchestrator::LLMOrchestrator,
    ) -> Result<SubAgentEvolutionReport, EvolutionError> {
        self.cycle_number += 1;
        let ts = Utc::now().timestamp();

        println!(
            "[{}] Evolution cycle {} starting (after {} tasks)",
            self.agent_name, self.cycle_number, self.tasks_completed
        );

        // 1) Reflect on recent tasks
        let recent_tasks = self.stm.recent(10);
        let success_rate = recent_tasks
            .iter()
            .filter(|e| e.value == "success")
            .count() as f64
            / recent_tasks.len().max(1) as f64;

        let mut playbook_updates = Vec::new();
        let mut skills_learned = Vec::new();
        let mut memory_insights = Vec::new();

        // 2) Update playbook based on feedback
        if success_rate < 0.8 {
            // Low success rate — improve playbook
            let feedback_summary = recent_tasks
                .iter()
                .filter_map(|e| e.context.as_ref())
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join("; ");

            let prompt = format!(
                "Agent '{}' has {:.0}% success rate. Recent feedback: {}. \
                Suggest ONE specific playbook improvement (max 50 words).",
                self.agent_name,
                success_rate * 100.0,
                feedback_summary
            );

            if let Ok(suggestion) = llm.speak(&prompt, None).await {
                let update = suggestion.trim().chars().take(200).collect::<String>();
                playbook_updates.push(update.clone());

                // Append to playbook (bounded)
                if let Err(e) = self.append_playbook_update(&update).await {
                    eprintln!("[{}] Failed to update playbook: {}", self.agent_name, e);
                }
            }
        }

        // 3) Learn new skills (bounded)
        if self.tasks_completed % (self.evolution_interval * 5) == 0 {
            // Every 5 evolution cycles, consider learning a new skill
            let prompt = format!(
                "Agent '{}' specializes in its domain. Suggest ONE new micro-skill \
                it could learn to improve (max 30 words).",
                self.agent_name
            );

            if let Ok(skill_idea) = llm.speak(&prompt, None).await {
                let skill = skill_idea.trim().chars().take(100).collect::<String>();
                skills_learned.push(skill.clone());

                // Add to skills (bounded)
                if let Err(e) = self.add_skill(&skill).await {
                    eprintln!("[{}] Failed to add skill: {}", self.agent_name, e);
                }
            }
        }

        // 4) Memory insights (store patterns in STM for LTM later)
        if success_rate > 0.9 {
            memory_insights.push(format!(
                "High success pattern: {} tasks with {:.0}% success",
                recent_tasks.len(),
                success_rate * 100.0
            ));
        }

        // 5) MITRE ATT&CK integration (if security agent)
        let mitre_patterns = if self.agent_name.contains("security")
            || self.agent_name.contains("exploit")
            || self.agent_name.contains("malware")
        {
            mitre::check_new_patterns(&self.agent_name).await.unwrap_or_default()
        } else {
            Vec::new()
        };

        println!(
            "[{}] Evolution cycle {} complete: {} playbook updates, {} skills learned",
            self.agent_name,
            self.cycle_number,
            playbook_updates.len(),
            skills_learned.len()
        );

        Ok(SubAgentEvolutionReport {
            agent_name: self.agent_name.clone(),
            ts_unix: ts,
            cycle_number: self.cycle_number,
            tasks_completed: self.tasks_completed,
            accuracy_score: success_rate,
            playbook_updates,
            skills_learned,
            memory_insights,
            mitre_patterns,
        })
    }

    /// Append a bounded update to the playbook.
    async fn append_playbook_update(&self, update: &str) -> Result<(), EvolutionError> {
        let path = Path::new(&self.playbook_path);
        if !path.exists() {
            return Err(EvolutionError::Invalid(format!(
                "Playbook not found: {}",
                self.playbook_path
            )));
        }

        let content = std::fs::read_to_string(path)?;
        let mut playbook: serde_yaml::Value = serde_yaml::from_str(&content)?;

        // Get or create updates array
        let updates = playbook
            .get_mut("updates")
            .and_then(|v| v.as_sequence_mut())
            .ok_or_else(|| EvolutionError::Invalid("Playbook missing 'updates' array".to_string()))?;

        // Bounded: max updates
        if updates.len() >= self.max_playbook_updates {
            return Err(EvolutionError::LimitReached(format!(
                "Playbook has reached max {} updates",
                self.max_playbook_updates
            )));
        }

        // Append update
        let update_entry = serde_yaml::to_value(HashMap::from([
            ("ts_unix", Utc::now().timestamp().to_string()),
            ("update", update.to_string()),
        ]))?;
        updates.push(update_entry);

        // Write back
        let updated = serde_yaml::to_string(&playbook)?;
        std::fs::write(path, updated)?;

        Ok(())
    }

    /// Add a bounded skill to the skills library.
    async fn add_skill(&self, skill_description: &str) -> Result<(), EvolutionError> {
        let path = Path::new(&self.skills_path);
        if !path.exists() {
            return Err(EvolutionError::Invalid(format!(
                "Skills file not found: {}",
                self.skills_path
            )));
        }

        let content = std::fs::read_to_string(path)?;
        let mut skills: serde_json::Value = serde_json::from_str(&content)?;

        // Get or create skills array
        let skills_array = skills
            .get_mut("skills")
            .and_then(|v| v.as_array_mut())
            .ok_or_else(|| EvolutionError::Invalid("Skills file missing 'skills' array".to_string()))?;

        // Bounded: max skills
        if skills_array.len() >= self.max_skills {
            return Err(EvolutionError::LimitReached(format!(
                "Agent has reached max {} skills",
                self.max_skills
            )));
        }

        // Add skill
        let skill_entry = serde_json::json!({
            "id": uuid::Uuid::new_v4().to_string(),
            "name": skill_description,
            "learned_at": Utc::now().to_rfc3339(),
            "usage_count": 0,
        });
        skills_array.push(skill_entry);

        // Write back
        let updated = serde_json::to_string_pretty(&skills)?;
        std::fs::write(path, updated)?;

        Ok(())
    }

    /// Get current evolution status.
    pub fn status(&self) -> EvolutionStatus {
        EvolutionStatus {
            agent_name: self.agent_name.clone(),
            cycle_number: self.cycle_number,
            tasks_completed: self.tasks_completed,
            next_evolution_at: self.tasks_completed
                + (self.evolution_interval - (self.tasks_completed % self.evolution_interval)),
            stm_entries: self.stm.entries.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStatus {
    pub agent_name: String,
    pub cycle_number: u64,
    pub tasks_completed: u64,
    pub next_evolution_at: u64,
    pub stm_entries: usize,
}

/// Inheritance configuration for spawned agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInheritance {
    /// Access to shared LTM (read-only or append-only)
    pub ltm_access: LTMAccess,
    /// Skills to inherit from Phoenix
    pub inherited_skills: Vec<String>,
    /// Playbook template to start with
    pub playbook_template: String,
    /// Evolution interval (tasks between evolution cycles)
    pub evolution_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LTMAccess {
    ReadOnly,
    AppendOnly,
    None,
}

impl Default for AgentInheritance {
    fn default() -> Self {
        Self {
            ltm_access: LTMAccess::AppendOnly,
            inherited_skills: Vec::new(),
            playbook_template: "playbook_template.yaml".to_string(),
            evolution_interval: 10, // Evolve every 10 tasks
        }
    }
}
