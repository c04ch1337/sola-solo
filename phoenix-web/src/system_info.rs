// phoenix-web/src/system_info.rs
//
// System Information Module for Proactive Environmental Agency
//
// This module provides Sola with "eyes" into the local environment,
// allowing her to know the current time, timezone, and OS without
// asking the user. This transforms her from a "brain in a jar" to
// a system-aware agent.

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

/// System context information that can be injected into LLM prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemContext {
    /// Current local time in RFC3339 format
    pub local_time: String,
    /// Current UTC time in RFC3339 format
    pub utc_time: String,
    /// IANA timezone identifier (e.g., "America/Chicago")
    pub timezone_iana: String,
    /// UTC offset string (e.g., "-06:00")
    pub utc_offset: String,
    /// Operating system name
    pub os: String,
    /// Operating system architecture
    pub arch: String,
}

impl SystemContext {
    /// Get current system context
    pub fn now() -> Self {
        let local_now: DateTime<Local> = Local::now();
        let utc_now: DateTime<Utc> = Utc::now();
        
        // Get IANA timezone using iana-time-zone crate
        let timezone_iana = iana_time_zone::get_timezone()
            .unwrap_or_else(|_| "Unknown".to_string());
        
        Self {
            local_time: local_now.to_rfc3339(),
            utc_time: utc_now.to_rfc3339(),
            timezone_iana,
            utc_offset: local_now.offset().to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }
    
    /// Format as a concise string for prompt injection
    pub fn to_prompt_block(&self) -> String {
        format!(
            "SYSTEM CONTEXT (auto-injected):\n\
            - Local Time: {}\n\
            - Timezone: {} (UTC{})\n\
            - OS: {} ({})",
            self.local_time,
            self.timezone_iana,
            self.utc_offset,
            self.os,
            self.arch
        )
    }
    
    /// Convert to JSON value for API responses
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "local_time": self.local_time,
            "utc_time": self.utc_time,
            "timezone_iana": self.timezone_iana,
            "utc_offset": self.utc_offset,
            "os": self.os,
            "arch": self.arch
        })
    }
}

/// Get local system information as a JSON value
/// 
/// This is the primary function for retrieving system context.
/// It can be called before any LLM interaction to provide
/// environmental awareness.
pub fn get_local_info() -> serde_json::Value {
    SystemContext::now().to_json()
}

/// Get system context as a prompt-ready string
/// 
/// This returns a formatted block that can be prepended to
/// the system prompt, giving the LLM immediate awareness of
/// the user's local environment.
pub fn get_system_context_prompt() -> String {
    SystemContext::now().to_prompt_block()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_context_creation() {
        let ctx = SystemContext::now();
        
        // Verify all fields are populated
        assert!(!ctx.local_time.is_empty());
        assert!(!ctx.utc_time.is_empty());
        assert!(!ctx.timezone_iana.is_empty());
        assert!(!ctx.utc_offset.is_empty());
        assert!(!ctx.os.is_empty());
        assert!(!ctx.arch.is_empty());
    }
    
    #[test]
    fn test_prompt_block_format() {
        let ctx = SystemContext::now();
        let prompt = ctx.to_prompt_block();
        
        // Verify prompt contains expected sections
        assert!(prompt.contains("SYSTEM CONTEXT"));
        assert!(prompt.contains("Local Time:"));
        assert!(prompt.contains("Timezone:"));
        assert!(prompt.contains("OS:"));
    }
    
    #[test]
    fn test_json_output() {
        let json = get_local_info();
        
        // Verify JSON structure
        assert!(json.get("local_time").is_some());
        assert!(json.get("utc_time").is_some());
        assert!(json.get("timezone_iana").is_some());
        assert!(json.get("utc_offset").is_some());
        assert!(json.get("os").is_some());
        assert!(json.get("arch").is_some());
    }
}
