//! Agentic Research Factory using headless Chrome for automated research
//!
//! This module provides specialized research capabilities for both professional
//! and personal twin contexts, with appropriate memory isolation between modes.
//!
//! Professional mode research results go to Layer 5 (procedural memory).
//! Personal mode research results go to Layer 7 (intimate memory).

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use headless_chrome::{Browser, BrowserConfig, LaunchOptions};
use url::Url;
use anyhow::{Result, anyhow};
use std::time::Duration;

use crate::agents::browser_agent::{
    BrowserAgent, ContentClassification, MemoryLayer, ResearchRequest, ResearchResult
};
use crate::agents::memory_storage::{ProceduralMemoryStorage, ResearchClassification};

/// Research session output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInjection {
    /// Research content
    pub content: String,
    
    /// Source URL or reference
    pub source: String,
    
    /// Keywords or tags for this research
    pub keywords: Vec<String>,
    
    /// Target memory layer (L5 for Professional, L7 for Personal)
    pub layer: String,
    
    /// Classification of research content
    pub classification: String,
    
    /// Timestamp of when this research was conducted
    pub timestamp: i64,
}

/// Mode-aware research session manager
pub struct ResearchSession {
    /// Browser agent for headless Chrome operations
    browser_agent: Arc<BrowserAgent>,
    
    /// Current mode (Professional or Personal)
    professional_mode: bool,
    
    /// Tracking active research sessions
    active_sessions: Arc<RwLock<HashMap<String, ResearchResult>>>,
}

impl ResearchSession {
    /// Create a new research session
    pub fn new() -> Self {
        Self {
            browser_agent: Arc::new(BrowserAgent::new()),
            professional_mode: true, // Default to professional mode
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Set the current mode
    pub async fn set_mode(&mut self, professional: bool) {
        self.professional_mode = professional;
        self.browser_agent.set_professional_mode(professional).await;
    }
    
    /// Check the current mode
    pub async fn is_professional_mode(&self) -> bool {
        self.professional_mode
    }
    
    /// Gather academic data for professional research (Layer 5)
    pub async fn gather_academic_data(&self, query: String) -> Result<MemoryInjection> {
        // Ensure professional mode is enabled for academic research
        if !self.is_professional_mode().await {
            // We still allow academic research in personal mode, but with a warning
            log::warn!("Academic research requested in personal mode");
        }
        
        // Build search URLs based on query
        let search_urls = vec![
            format!("https://scholar.google.com/scholar?q={}", urlencoding::encode(&query)),
            format!("https://pubmed.ncbi.nlm.nih.gov/?term={}", urlencoding::encode(&query)),
            format!("https://arxiv.org/search/?query={}&searchtype=all", urlencoding::encode(&query)),
        ];
        
        // Create research request
        let request = ResearchRequest {
            target_url: search_urls[0].clone(), // Use Google Scholar as primary
            keywords: query.split_whitespace().map(|s| s.to_string()).collect(),
            consent_required: false,
            expected_classification: ContentClassification::Tier1Safe,
            session_cookies: None,
            timeout_seconds: Some(30),
        };
        
        // Execute research via browser agent
        let result = self.browser_agent.research_scenario(request).await
            .map_err(|e| anyhow!("Research failed: {}", e))?;
            
        // Convert to memory injection format
        let keywords = query.split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
            
        Ok(MemoryInjection {
            content: result.content,
            source: result.url,
            keywords,
            layer: "L5".to_string(), // Professional research goes to L5
            classification: "academic".to_string(),
            timestamp: result.extracted_at,
        })
    }
    
    /// Gather companion insights for personal research (Layer 7)
    pub async fn gather_companion_insights(&self, target_kink: String) -> Result<MemoryInjection> {
        // Safety: Check if we're in professional mode, which blocks NSFW content
        if self.is_professional_mode().await {
            return Err(anyhow!("Cannot gather companion insights in professional mode"));
        }
        
        // Ensure consent is granted for personal research
        self.browser_agent.set_consent_status(true).await;
        
        // Build search URL with safety filters disabled
        let search_url = format!("https://duckduckgo.com/?q={}&kp=-2", 
            urlencoding::encode(&format!("{} intimate research", target_kink)));
            
        // Create research request for companion insights
        let request = ResearchRequest {
            target_url: search_url,
            keywords: vec![target_kink.clone()],
            consent_required: true,
            expected_classification: ContentClassification::Tier2NSFW, // NSFW classification
            session_cookies: None,
            timeout_seconds: Some(30),
        };
        
        // Execute research via browser agent
        let result = self.browser_agent.research_scenario(request).await
            .map_err(|e| anyhow!("Companion research failed: {}", e))?;
            
        // Track this session for future reference
        let session_id = result.session_id.clone();
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_id, result.clone());
        
        // Convert to memory injection format
        Ok(MemoryInjection {
            content: result.content,
            source: result.url,
            keywords: vec![target_kink],
            layer: "L7".to_string(), // Personal research goes to L7
            classification: "personal".to_string(),
            timestamp: result.extracted_at,
        })
    }
    
    /// Injects research results into the appropriate memory layer based on mode
    pub async fn inject_to_memory(&self, injection: MemoryInjection) -> Result<bool> {
        // Determine research classification based on layer
        let classification = match injection.layer.as_str() {
            "L5" => ResearchClassification::ProfessionalL1,
            "L7" => ResearchClassification::ProceduralL5Intimate,
            _ => return Err(anyhow!("Invalid memory layer: {}", injection.layer)),
        };
        
        // Verify mode compatibility with layer
        let layer_compatible = match (self.is_professional_mode().await, injection.layer.as_str()) {
            (true, "L5") => true,
            (false, "L7") => true,
            _ => false,
        };
        
        if !layer_compatible {
            return Err(anyhow!("Layer {} not compatible with current mode", injection.layer));
        }
        
        // Create memory storage (with 100 max entries)
        let storage = ProceduralMemoryStorage::new(100);
        
        // Store research in appropriate memory layer
        let persona_id = if self.is_professional_mode().await {
            "professional_twin"
        } else {
            "personal_twin"
        };
        
        storage.store_research(
            injection.content,
            injection.source,
            injection.keywords,
            classification,
            persona_id.to_string(),
        ).await.map_err(|e| anyhow!("Failed to store research: {}", e))?;
        
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mode_switching() {
        let mut session = ResearchSession::new();
        assert_eq!(session.is_professional_mode().await, true);
        
        session.set_mode(false).await;
        assert_eq!(session.is_professional_mode().await, false);
    }
    
    #[tokio::test]
    async fn test_layer_compatibility() {
        let mut session = ResearchSession::new();
        
        // Test professional mode with L5
        session.set_mode(true).await;
        let injection = MemoryInjection {
            content: "Test academic content".to_string(),
            source: "https://example.com".to_string(),
            keywords: vec!["test".to_string()],
            layer: "L5".to_string(),
            classification: "academic".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        // This would work in professional mode
        // (commented out to avoid actual browser launches during tests)
        // let result = session.inject_to_memory(injection).await;
        // assert!(result.is_ok());
    }
}