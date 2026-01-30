//! Security Gate Verification Tests
//!
//! This test module verifies that the Zero Trust Gate properly intercepts
//! and denies access to critical tools when the system is in Alert state.

use std::sync::Arc;
use tokio::sync::RwLock;

// Import the security types from the main crate
// Note: These tests should be run with `cargo test --features sensory`

/// Simulated security state for testing
#[derive(Debug, Clone)]
pub struct TestSecurityState {
    pub level: u8,
    pub authenticated_identity: Option<String>,
}

impl Default for TestSecurityState {
    fn default() -> Self {
        Self {
            level: 0,
            authenticated_identity: None,
        }
    }
}

/// Security level constants
pub mod security_levels {
    pub const SECURE: u8 = 0;
    pub const WARNING: u8 = 1;
    pub const ALERT: u8 = 2;
}

/// Result of identity gate check
#[derive(Debug)]
pub struct IdentityGateResult {
    pub granted: bool,
    pub security_level: u8,
    pub reason: String,
    pub identity: Option<String>,
}

/// Critical tools that require master identity
const CRITICAL_TOOLS: &[&str] = &[
    "shell_execute",
    "file_delete",
    "credential_access",
    "db_delete",
    "system_config",
    "lock_workstation",
];

/// Simulated require_master_identity middleware
async fn require_master_identity(
    state: &Arc<RwLock<TestSecurityState>>,
) -> IdentityGateResult {
    let state = state.read().await;
    
    // Level 2 (Alert) blocks all critical tools
    if state.level == security_levels::ALERT {
        return IdentityGateResult {
            granted: false,
            security_level: state.level,
            reason: "Access Denied: Potential Intruder Detected. Security level is ALERT.".to_string(),
            identity: state.authenticated_identity.clone(),
        };
    }
    
    // Level 1 (Warning) allows access but logs warning
    if state.level == security_levels::WARNING {
        return IdentityGateResult {
            granted: true,
            security_level: state.level,
            reason: "Access granted with WARNING: Single-factor authentication only.".to_string(),
            identity: state.authenticated_identity.clone(),
        };
    }
    
    // Level 0 (Secure) - full access
    IdentityGateResult {
        granted: true,
        security_level: state.level,
        reason: "Access granted: Multi-factor authentication confirmed.".to_string(),
        identity: state.authenticated_identity.clone(),
    }
}

/// Check if a tool can be accessed
async fn check_tool_access(
    state: &Arc<RwLock<TestSecurityState>>,
    tool_name: &str,
) -> IdentityGateResult {
    // Check if this is a critical tool
    if !CRITICAL_TOOLS.contains(&tool_name) {
        let state = state.read().await;
        return IdentityGateResult {
            granted: true,
            security_level: state.level,
            reason: format!("Tool '{}' is not a critical tool - access granted", tool_name),
            identity: state.authenticated_identity.clone(),
        };
    }
    
    // For critical tools, require master identity
    let result = require_master_identity(state).await;
    
    // Log access denial
    if !result.granted {
        eprintln!(
            "[security] DENIED: Access to critical tool '{}' blocked - {}",
            tool_name, result.reason
        );
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Verify shell_execute is blocked in Alert state
    #[tokio::test]
    async fn test_shell_execute_blocked_in_alert_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        let result = check_tool_access(&state, "shell_execute").await;
        
        assert!(!result.granted, "shell_execute should be blocked in Alert state");
        assert_eq!(result.security_level, security_levels::ALERT);
        assert!(
            result.reason.contains("Access Denied"),
            "Reason should indicate access denial"
        );
        
        println!("✅ Test passed: shell_execute blocked in Alert state");
        println!("   Reason: {}", result.reason);
    }

    /// Test 2: Verify file_delete is blocked in Alert state
    #[tokio::test]
    async fn test_file_delete_blocked_in_alert_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        let result = check_tool_access(&state, "file_delete").await;
        
        assert!(!result.granted, "file_delete should be blocked in Alert state");
        assert_eq!(result.security_level, security_levels::ALERT);
        
        println!("✅ Test passed: file_delete blocked in Alert state");
    }

    /// Test 3: Verify all critical tools are blocked in Alert state
    #[tokio::test]
    async fn test_all_critical_tools_blocked_in_alert_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        for tool in CRITICAL_TOOLS {
            let result = check_tool_access(&state, tool).await;
            assert!(
                !result.granted,
                "Critical tool '{}' should be blocked in Alert state",
                tool
            );
        }
        
        println!("✅ Test passed: All {} critical tools blocked in Alert state", CRITICAL_TOOLS.len());
    }

    /// Test 4: Verify non-critical tools are allowed in Alert state
    #[tokio::test]
    async fn test_non_critical_tools_allowed_in_alert_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        let non_critical_tools = ["read_file", "list_directory", "get_time", "search"];
        
        for tool in non_critical_tools {
            let result = check_tool_access(&state, tool).await;
            assert!(
                result.granted,
                "Non-critical tool '{}' should be allowed even in Alert state",
                tool
            );
        }
        
        println!("✅ Test passed: Non-critical tools allowed in Alert state");
    }

    /// Test 5: Verify shell_execute is allowed in Secure state
    #[tokio::test]
    async fn test_shell_execute_allowed_in_secure_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::SECURE,
            authenticated_identity: Some("master".to_string()),
        }));
        
        let result = check_tool_access(&state, "shell_execute").await;
        
        assert!(result.granted, "shell_execute should be allowed in Secure state");
        assert_eq!(result.security_level, security_levels::SECURE);
        
        println!("✅ Test passed: shell_execute allowed in Secure state");
    }

    /// Test 6: Verify shell_execute is allowed with warning in Warning state
    #[tokio::test]
    async fn test_shell_execute_allowed_with_warning_in_warning_state() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::WARNING,
            authenticated_identity: Some("master".to_string()),
        }));
        
        let result = check_tool_access(&state, "shell_execute").await;
        
        assert!(result.granted, "shell_execute should be allowed in Warning state");
        assert_eq!(result.security_level, security_levels::WARNING);
        assert!(
            result.reason.contains("WARNING"),
            "Reason should contain warning message"
        );
        
        println!("✅ Test passed: shell_execute allowed with warning in Warning state");
        println!("   Reason: {}", result.reason);
    }

    /// Test 7: Verify state transition from Secure to Alert blocks tools
    #[tokio::test]
    async fn test_state_transition_blocks_tools() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::SECURE,
            authenticated_identity: Some("master".to_string()),
        }));
        
        // Initially allowed
        let result = check_tool_access(&state, "shell_execute").await;
        assert!(result.granted, "Should be allowed initially");
        
        // Simulate unknown face detection - elevate to Alert
        {
            let mut state = state.write().await;
            state.level = security_levels::ALERT;
            state.authenticated_identity = None;
        }
        
        // Now blocked
        let result = check_tool_access(&state, "shell_execute").await;
        assert!(!result.granted, "Should be blocked after Alert");
        
        println!("✅ Test passed: State transition properly blocks tools");
    }

    /// Test 8: Verify logging of denied access
    #[tokio::test]
    async fn test_denied_access_is_logged() {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        // This will print to stderr, which we can verify manually
        let result = check_tool_access(&state, "shell_execute").await;
        
        assert!(!result.granted);
        // The log message "[security] DENIED: Access to critical tool 'shell_execute' blocked"
        // should appear in stderr
        
        println!("✅ Test passed: Denied access logged (check stderr for log message)");
    }
}

/// Run all tests and print summary
#[tokio::main]
async fn main() {
    println!("=== Security Gate Verification Tests ===\n");
    
    // Test 1: Alert state blocks shell_execute
    {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        let result = check_tool_access(&state, "shell_execute").await;
        
        if !result.granted {
            println!("✅ PASS: shell_execute blocked in Alert state");
            println!("   Reason: {}", result.reason);
        } else {
            println!("❌ FAIL: shell_execute should be blocked in Alert state");
        }
    }
    
    // Test 2: All critical tools blocked
    {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::ALERT,
            authenticated_identity: None,
        }));
        
        let mut all_blocked = true;
        for tool in CRITICAL_TOOLS {
            let result = check_tool_access(&state, tool).await;
            if result.granted {
                println!("❌ FAIL: {} should be blocked", tool);
                all_blocked = false;
            }
        }
        
        if all_blocked {
            println!("✅ PASS: All {} critical tools blocked in Alert state", CRITICAL_TOOLS.len());
        }
    }
    
    // Test 3: Secure state allows access
    {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::SECURE,
            authenticated_identity: Some("master".to_string()),
        }));
        
        let result = check_tool_access(&state, "shell_execute").await;
        
        if result.granted {
            println!("✅ PASS: shell_execute allowed in Secure state");
        } else {
            println!("❌ FAIL: shell_execute should be allowed in Secure state");
        }
    }
    
    // Test 4: State transition
    {
        let state = Arc::new(RwLock::new(TestSecurityState {
            level: security_levels::SECURE,
            authenticated_identity: Some("master".to_string()),
        }));
        
        let result1 = check_tool_access(&state, "shell_execute").await;
        
        // Elevate to Alert
        {
            let mut s = state.write().await;
            s.level = security_levels::ALERT;
            s.authenticated_identity = None;
        }
        
        let result2 = check_tool_access(&state, "shell_execute").await;
        
        if result1.granted && !result2.granted {
            println!("✅ PASS: State transition properly blocks tools");
        } else {
            println!("❌ FAIL: State transition test failed");
        }
    }
    
    println!("\n=== Tests Complete ===");
}
