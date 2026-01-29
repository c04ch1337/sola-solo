// sub_agent_evolution/src/skills.rs
// Skills management for sub-agents — load, evolve, track usage.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLibrary {
    pub schema: String,
    pub notes: String,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub learned_at: String,
    pub usage_count: u64,
    pub love_score: Option<f64>,
    pub utility_score: Option<f64>,
}

impl SkillLibrary {
    /// Load skills from JSON file.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let library: SkillLibrary = serde_json::from_str(&content)?;
        Ok(library)
    }

    /// Save skills to JSON file.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add a new skill.
    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    /// Increment usage count for a skill.
    pub fn use_skill(&mut self, skill_id: &str) {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.id == skill_id) {
            skill.usage_count += 1;
        }
    }

    /// Update love/utility scores based on feedback.
    pub fn update_scores(&mut self, skill_id: &str, love_delta: f64, utility_delta: f64) {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.id == skill_id) {
            skill.love_score = Some(skill.love_score.unwrap_or(0.5) + love_delta);
            skill.utility_score = Some(skill.utility_score.unwrap_or(0.5) + utility_delta);

            // Clamp to [0, 1]
            if let Some(ref mut love) = skill.love_score {
                *love = love.clamp(0.0, 1.0);
            }
            if let Some(ref mut utility) = skill.utility_score {
                *utility = utility.clamp(0.0, 1.0);
            }
        }
    }

    /// Get top skills by usage.
    pub fn top_skills(&self, limit: usize) -> Vec<&Skill> {
        let mut sorted = self.skills.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        sorted.into_iter().take(limit).collect()
    }
}
