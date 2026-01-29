//! Browser Agent for Sola AGI System
//! 
//! This module provides headless browser capabilities for research scenarios,
//! with explicit content safety controls and memory layer isolation.

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use headless_chrome::{Browser, BrowserConfig, LaunchOptions};
use url::Url;
use std::time::Duration;
use crate::agents::memory_storage::{ProceduralMemoryStorage, ResearchClassification};

/// Memory layer classification for research data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemoryLayer {
    ProfessionalL1,
    CasualL2,
    PersonalL3,
    SensitiveL4,
    ProceduralL5Intimate,
}

/// Content classification for safety controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentClassification {
    Tier1Safe,     // General/SFW content
    Tier2NSFW,     // Adult/NSFW content
    Tier3Explicit, // Explicit adult content
}

/// Browser session state with safety controls
#[derive(Debug, Clone)]
pub struct BrowserSession {
    pub browser: Browser,
    pub session_id: String,
    pub consent_granted: bool,
    pub current_classification: ContentClassification,
    pub memory_layer: MemoryLayer,
}

/// Research request with safety parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRequest {
    pub target_url: String,
    pub keywords: Vec<String>,
    pub consent_required: bool,
    pub expected_classification: ContentClassification,
    pub session_cookies: Option<HashMap<String, String>>,
    pub timeout_seconds: Option<u64>,
}

/// Research result with safety metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub session_id: String,
    pub content: String,
    pub url: String,
    pub classification: ContentClassification,
    pub memory_layer: MemoryLayer,
    pub extracted_at: i64,
    pub content_length: usize,
    pub contains_explicit: bool,
}

#[derive(Debug)]
pub struct BrowserAgent {
    #[allow(dead_code)]
    sessions: Arc<RwLock<HashMap<String, BrowserSession>>>,
    professional_mode_enabled: Arc<RwLock<bool>>,
    consent_status: Arc<RwLock<bool>>,
    memory_storage: ProceduralMemoryStorage,
}

impl BrowserAgent {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            professional_mode_enabled: Arc::new(RwLock::new(false)),
            consent_status: Arc::new(RwLock::new(false)),
            memory_storage: ProceduralMemoryStorage::new(),
        }
    }

    /// Main research function - handles consent, classification, and memory isolation
    pub async fn research_scenario(
        &self,
        request: ResearchRequest,
    ) -> Result<ResearchResult, String> {
        // Safety checks before proceeding
        self.validate_research_request(&request).await?;

        // Determine memory layer based on content classification
        let memory_layer = self.classify_memory_layer(&request.expected_classification);

        // Check if we're in professional mode and request is explicit
        if self.is_nsfw_blocked(&memory_layer).await {
            return Err("Professional mode active - explicit content research blocked".to_string());
        }

        // Launch browser session
        let session = self.launch_session(&request, &memory_layer).await?;

        // Perform research
        let content = self.extract_content(&session, &request).await?;
        
        // Analyze content for explicit content
        let contains_explicit = self.analyze_explicit_content(&content);

        // Safety validation
        self.validate_integrity(&content, &request, &memory_layer).await?;

        let result = ResearchResult {
            session_id: session.session_id.clone(),
            content: content.trim().to_string(),
            url: request.target_url.clone(),
            classification: request.expected_classification.clone(),
            memory_layer,
            extracted_at: chrono::Utc::now().timestamp(),
            content_length: content.len(),
            contains_explicit,
        };

        // Store in procedural memory (Layer 5)
        if memory_layer == MemoryLayer::ProceduralL5Intimate {
            self.memory_storage.store_research(result.clone()).await
                .map_err(|e| format!("Failed to store research in memory: {}", e))?;
        }

        Ok(result)
    }

    /// Launch dedicated browser session with appropriate configuration
    async fn launch_session(
        &self,
        request: &ResearchRequest,
        memory_layer: &MemoryLayer,
    ) -> Result<BrowserSession, String> {
        let config = BrowserConfig::builder()
            .window_size(Some((1920, 1080)))
            .ignore_certificate_errors(true)
            .build()
            .map_err(|e| format!("Browser config failed: {}", e))?;

        let options = LaunchOptions::default_builder()
            .headless(true)
            .sandbox(false)
            .build()
            .map_err(|e| format!("Launch options failed: {}", e))?;

        let browser = Browser::new(config, options)
            .map_err(|e| format!("Browser launch failed: {}", e))?;

        let session_id = uuid::Uuid::new_v4().to_string();

        Ok(BrowserSession {
            browser,
            session_id,
            consent_granted: request.consent_required,
            current_classification: request.expected_classification.clone(),
            memory_layer: memory_layer.clone(),
        })
    }

    /// Extract and clean content from target URL
    async fn extract_content(
        &self,
        session: &BrowserSession,
        request: &ResearchRequest,
    ) -> Result<String, String> {
        let tab = session.browser.new_tab()
            .map_err(|e| format!("Tab creation failed: {}", e))?;

        // Navigate to target URL
        tab.navigate_to(&request.target_url)
            .map_err(|e| format!("Navigation failed: {}", e))?;

        // Wait for page load
        tab.wait_until_navigated()
            .map_err(|e| format!("Navigation wait failed: {}", e))?;

        // Extract main content using JavaScript
        let js_script = r#"
            // Remove navigation, ads, and other noise
            const noiseSelectors = [
                'nav', 'header', 'footer', 'aside',
                '.advertisement', '.ads', '.banner',
                '.navigation', '.menu', '.sidebar'
            ];
            
            noiseSelectors.forEach(selector => {
                document.querySelectorAll(selector).forEach(el => el.remove());
            });
            
            // Extract text content from main content areas
            const contentSelectors = [
                'main', 'article', '.content', '#content',
                '.main-content', '.post-content', '.entry-content'
            ];
            
            let content = '';
            contentSelectors.forEach(selector => {
                const elements = document.querySelectorAll(selector);
                elements.forEach(el => {
                    content += ' ' + el.textContent;
                });
            });
            
            // Fallback to body if no specific content found
            if (!content.trim()) {
                content = document.body.textContent;
            }
            
            // Clean and normalize content
            content = content.replace(/\\s+/g, ' ').trim();
            
            return content;
        "#;

        let content = tab.evaluate(js_script, false)
            .map_err(|e| format!("Content extraction failed: {}", e))?;

        if let Some(content_value) = content.value {
            if let Some(content_str) = content_value.as_str() {
                Ok(content_str.to_string())
            } else {
                Err("Extracted content is not a string".to_string())
            }
        } else {
            Err("No content extracted from page".to_string())
        }
    }

    /// Validate research request against safety rules
    async fn validate_research_request(&self, request: &ResearchRequest) -> Result<(), String> {
        // Check URL safety
        self.validate_url(&request.target_url).await?;
        
        // Check consent requirements
        if request.consent_required {
            let consent = *self.consent_status.read().await;
            if !consent {
                return Err("Consent required but not granted".to_string());
            }
        }

        Ok(())
    }

    /// Validate URL for safety and accessibility
    async fn validate_url(&self, url: &str) -> Result<(), String> {
        let parsed_url = Url::parse(url)
            .map_err(|_| format!("Invalid URL: {}", url))?;

        // Block certain protocols
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err("Unsupported URL scheme".to_string());
        }

        // Add additional URL validation as needed
        Ok(())
    }

    /// Classify memory layer based on content classification
    fn classify_memory_layer(&self, classification: &ContentClassification) -> MemoryLayer {
        match classification {
            ContentClassification::Tier1Safe => MemoryLayer::ProfessionalL1,
            ContentClassification::Tier2NSFW => MemoryLayer::ProceduralL5Intimate,
            ContentClassification::Tier3Explicit => MemoryLayer::ProceduralL5Intimate,
        }
    }

    /// Check if NSFW content research is blocked by professional mode
    async fn is_nsfw_blocked(&self, memory_layer: &MemoryLayer) -> bool {
        let professional_mode = *self.professional_mode_enabled.read().await;
        matches!(
            (professional_mode, memory_layer),
            (true, MemoryLayer::ProceduralL5Intimate)
        )
    }

    /// Analyze content for explicit material
    fn analyze_explicit_content(&self, content: &str) -> bool {
        let explicit_keywords = [
            "explicit", "adult", "xxx", "nsfw", "porn", "erotic", "intimate",
            "sensual", "nude", "sexual", "kink", "bdsm", "fetish"
        ];
        
        let content_lower = content.to_lowercase();
        explicit_keywords.iter().any(|keyword| content_lower.contains(keyword))
    }

    /// Final integrity check before returning research results
    async fn validate_integrity(
        &self,
        content: &str,
        request: &ResearchRequest,
        memory_layer: &MemoryLayer,
    ) -> Result<(), String> {
        // Ensure content isn't empty
        if content.trim().is_empty() {
            return Err("No content extracted from target URL".to_string());
        }

        // Check if content matches expected classification
        let actual_explicit = self.analyze_explicit_content(content);
        let expected_explicit = matches!(
            &request.expected_classification,
            ContentClassification::Tier2NSFW | ContentClassification::Tier3Explicit
        );

        if actual_explicit != expected_explicit {
            log::warn!(
                "Content classification mismatch: expected explicit={}, actual explicit={}",
                expected_explicit,
                actual_explicit
            );
        }

        Ok(())
    }

    /// Set professional mode status
    pub async fn set_professional_mode(&self, enabled: bool) {
        *self.professional_mode_enabled.write().await = enabled;
    }

    /// Set consent status
    pub async fn set_consent_status(&self, granted: bool) {
        *self.consent_status.write().await = granted;
    }

    /// Get current professional mode status
    pub async fn get_professional_mode(&self) -> bool {
        *self.professional_mode_enabled.read().await
    }

    /// Get current consent status
    pub async fn get_consent_status(&self) -> bool {
        *self.consent_status.read().await
    }
}

impl Default for BrowserAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_research_request_validation() {
        let agent = BrowserAgent::new();
        
        // Test valid request
        let request = ResearchRequest {
            target_url: "https://example.com".to_string(),
            keywords: vec!["test".to_string()],
            consent_required: false,
            expected_classification: ContentClassification::Tier1Safe,
            session_cookies: None,
            timeout_seconds: Some(30),
        };

        let result = agent.validate_research_request(&request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_memory_layer_classification() {
        let agent = BrowserAgent::new();
        
        let safe_layer = agent.classify_memory_layer(&ContentClassification::Tier1Safe);
        let nsfw_layer = agent.classify_memory_layer(&ContentClassification::Tier2NSFW);
        
        assert_eq!(safe_layer, MemoryLayer::ProfessionalL1);
        assert_eq!(nsfw_layer, MemoryLayer::ProceduralL5Intimate);
    }

    #[tokio::test]
    async fn test_explicit_content_detection() {
        let agent = BrowserAgent::new();
        
        let safe_content = "This is a normal article about technology.";
        let explicit_content = "This contains explicit adult material.";
        
        assert!(!agent.analyze_explicit_content(safe_content));
        assert!(agent.analyze_explicit_content(explicit_content));
    }
}