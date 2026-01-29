//! Procedural Memory Storage for Layer 5 Intimate Research Data
//!
//! This module provides isolated storage for NSFW research data with explicit safety controls
//! ensuring data never leaks into professional memory layers.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Research data container with safety classifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralResearch {
    pub content: String,
    pub source_url: String,
    pub keywords: Vec<String>,
    pub classification: ResearchClassification,
    pub timestamp: DateTime<Utc>,
    pub persona_id: String,
    pub content_hash: String,
}

/// Safety classifications for research data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResearchClassification {
    ProfessionalL1,
    ProceduralL5Intimate,
}

/// Procedural memory storage for Layer 5 data
#[derive(Debug)]
pub struct ProceduralMemoryStorage {
    storage: Arc<RwLock<HashMap<String, VecDeque<ProceduralResearch>>>>,
    max_entries_per_persona: usize,
}

impl ProceduralMemoryStorage {
    pub fn new(max_entries: usize) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            max_entries_per_persona: max_entries,
        }
    }

    /// Store research data with safety validation
    pub async fn store_research(
        &self,
        content: String,
        source_url: String,
        keywords: Vec<String>,
        classification: ResearchClassification,
        persona_id: String,
    ) -> Result<(), String> {
        // Safety validation - cannot store intimate data in professional mode
        #[cfg(feature = "professional_mode")]
        if classification == ResearchClassification::ProceduralL5Intimate {
            return Err("Cannot store intimate research in professional mode".to_string());
        }

        let timestamp = Utc::now();
        let content_hash = Self::generate_content_hash(&content);

        let research = ProceduralResearch {
            content,
            source_url,
            keywords,
            classification,
            timestamp,
            persona_id: persona_id.clone(),
            content_hash,
        };

        let mut storage = self.storage.write().await;
        storage
            .entry(persona_id)
            .or_insert_with(|| VecDeque::with_capacity(self.max_entries_per_persona))
            .push_back(research);

        // Enforce maximum entries per persona
        if let Some(entries) = storage.get_mut(&persona_id) {
            while entries.len() > self.max_entries_per_persona {
                entries.pop_front();
            }
        }

        Ok(())
    }

    /// Retrieves research data for a persona
    pub async fn get_research_for_persona(
        &self,
        persona_id: &str,
    ) -> Result<Vec<ProceduralResearch>, String> {
        let storage = self.storage.read().await;
        if let Some(research) = storage.get(persona_id) {
            Ok(research.iter().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }

    /// Filters research by classification
    pub async fn get_research_by_classification(
        &self,
        persona_id: &str,
        classification: ResearchClassification,
    ) -> Result<Vec<ProceduralResearch>, String> {
        let storage = self.storage.read().await;
        if let Some(research) = storage.get(persona_id) {
            let filtered: Vec<ProceduralResearch> = research
                .iter()
                .filter(|r| r.classification == classification)
                .cloned()
                .collect();
            Ok(filtered)
        } else {
            Ok(Vec::new())
        }
    }

    /// Clears all research for a persona
    pub async fn clear_persona_research(&self, persona_id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.remove(persona_id);
        Ok(())
    }

    /// Generates cryptographic hash for content
    fn generate_content_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Safety check for professional mode compatibility
    pub async fn is_professional_mode_safe(&self, persona_id: &str) -> Result<bool, String> {
        let storage = self.storage.read().await;
        if let Some(research) = storage.get(persona_id) {
            Ok(research
                .iter()
                .all(|r| r.classification == ResearchClassification::ProfessionalL1))
        } else {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_procedural_memory_storage() {
        let storage = ProceduralMemoryStorage::new(10);
        let persona_id = "test_persona".to_string();

        // Test storing research
        storage
            .store_research(
                "Test content".to_string(),
                "https://test.com".to_string(),
                vec!["test".to_string()],
                ResearchClassification::ProfessionalL1,
                persona_id.clone(),
            )
            .await
            .unwrap();

        // Test retrieval
        let research = storage.get_research_for_persona(&persona_id).await.unwrap();
        assert_eq!(research.len(), 1);
        assert_eq!(research[0].content, "Test content");
    }
}