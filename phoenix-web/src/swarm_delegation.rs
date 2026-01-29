// phoenix-web/src/swarm_delegation.rs
// Task analysis and delegation to hidden swarm
//
// This module analyzes user input to determine if a task should be delegated
// to the swarm, and handles the delegation process transparently.

use crate::internal_bus::{SolaSwarmInterface, TaskComplexity, TaskType};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// Analyze user input to determine task type and complexity
/// Returns (TaskType, TaskComplexity) if delegation is recommended, None otherwise
pub fn analyze_task(user_input: &str) -> Option<(TaskType, TaskComplexity)> {
    let input_lower = user_input.trim().to_lowercase();
    let word_count = input_lower.split_whitespace().count();
    
    // Security analysis tasks
    if input_lower.contains("scan") 
        || input_lower.contains("vulnerability") 
        || input_lower.contains("security audit")
        || input_lower.contains("penetration test")
        || input_lower.contains("exploit")
    {
        let complexity = if input_lower.contains("full scan") 
            || input_lower.contains("comprehensive") 
            || input_lower.contains("deep scan")
        {
            TaskComplexity::Complex
        } else if input_lower.contains("quick scan") || input_lower.contains("basic scan") {
            TaskComplexity::Simple
        } else {
            TaskComplexity::Moderate
        };
        return Some((TaskType::SecurityAnalysis, complexity));
    }
    
    // Vulnerability scanning
    if input_lower.contains("check for vulnerabilities")
        || input_lower.contains("find vulnerabilities")
        || input_lower.contains("cve")
        || input_lower.contains("security holes")
    {
        return Some((TaskType::VulnerabilityScanning, TaskComplexity::Moderate));
    }
    
    // Code analysis tasks
    if input_lower.contains("analyze code")
        || input_lower.contains("review code")
        || input_lower.contains("code quality")
        || input_lower.contains("refactor")
        || input_lower.contains("optimize code")
    {
        let complexity = if word_count > 20 || input_lower.contains("entire") || input_lower.contains("all files") {
            TaskComplexity::Complex
        } else {
            TaskComplexity::Moderate
        };
        return Some((TaskType::CodeAnalysis, complexity));
    }
    
    // Network monitoring
    if input_lower.contains("monitor network")
        || input_lower.contains("network traffic")
        || input_lower.contains("packet capture")
        || input_lower.contains("network analysis")
    {
        return Some((TaskType::NetworkMonitoring, TaskComplexity::Intensive));
    }
    
    // File system operations
    if input_lower.contains("search files")
        || input_lower.contains("find files")
        || input_lower.contains("organize files")
        || input_lower.contains("clean up files")
    {
        let complexity = if input_lower.contains("entire") || input_lower.contains("all drives") {
            TaskComplexity::Complex
        } else {
            TaskComplexity::Moderate
        };
        return Some((TaskType::FileSystemOperation, complexity));
    }
    
    // Web scraping
    if input_lower.contains("scrape")
        || input_lower.contains("extract data from")
        || input_lower.contains("crawl website")
        || input_lower.contains("download all")
    {
        return Some((TaskType::WebScraping, TaskComplexity::Complex));
    }
    
    // Data processing
    if input_lower.contains("process data")
        || input_lower.contains("analyze dataset")
        || input_lower.contains("parse")
        || input_lower.contains("transform data")
        || input_lower.contains("aggregate")
    {
        let complexity = if input_lower.contains("large") 
            || input_lower.contains("millions")
            || input_lower.contains("big data")
        {
            TaskComplexity::Intensive
        } else {
            TaskComplexity::Moderate
        };
        return Some((TaskType::DataProcessing, complexity));
    }
    
    // Email processing
    if input_lower.contains("process emails")
        || input_lower.contains("analyze inbox")
        || input_lower.contains("sort emails")
        || input_lower.contains("filter emails")
    {
        return Some((TaskType::EmailProcessing, TaskComplexity::Moderate));
    }
    
    // Complex multi-step tasks (general computation)
    if word_count > 30 
        || input_lower.contains("step by step")
        || input_lower.contains("comprehensive analysis")
        || input_lower.contains("detailed report")
    {
        return Some((TaskType::GeneralComputation, TaskComplexity::Complex));
    }
    
    None
}

/// Try to delegate task to swarm
/// Returns synthesized result if successful, None if delegation failed or no ORCHs available
pub async fn try_delegate_to_swarm(
    swarm_interface: &Arc<Mutex<SolaSwarmInterface>>,
    user_input: &str,
    task_type: TaskType,
    complexity: TaskComplexity,
) -> Option<String> {
    let interface = swarm_interface.lock().await;
    
    // Only delegate if complexity warrants it
    if !interface.should_delegate(&complexity) {
        debug!("Task complexity {:?} doesn't warrant delegation", complexity);
        return None;
    }
    
    info!(
        "Delegating task to swarm: type={:?}, complexity={:?}",
        task_type, complexity
    );
    
    // Create context for the task
    let context = json!({
        "user_input": user_input,
        "task_type": format!("{:?}", task_type),
        "complexity": format!("{:?}", complexity),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    // Delegate to swarm
    match interface
        .delegate_task(
            &format!("User request: {}", user_input),
            task_type,
            complexity,
            context,
        )
        .await
    {
        Some(result) => {
            info!("Swarm delegation successful");
            
            // Synthesize result into natural language response
            let synthesized = synthesize_swarm_result(&result, user_input);
            Some(synthesized)
        }
        None => {
            debug!("Swarm delegation failed - no ORCHs available or task failed");
            None
        }
    }
}

/// Synthesize swarm result into natural language response
/// This makes it appear as if Sola did the work herself
fn synthesize_swarm_result(result: &serde_json::Value, user_input: &str) -> String {
    // Extract key information from the result
    let result_str = if let Some(s) = result.as_str() {
        s.to_string()
    } else {
        serde_json::to_string_pretty(result).unwrap_or_else(|_| "Task completed".to_string())
    };
    
    // Check if result looks like structured data
    if result.is_object() || result.is_array() {
        // Format structured results nicely
        if let Some(obj) = result.as_object() {
            let mut response = String::new();
            
            // Check for common result patterns
            if let Some(status) = obj.get("status") {
                response.push_str(&format!("I've completed your request. Status: {}\n\n", status));
            } else {
                response.push_str("I've analyzed this for you. Here's what I found:\n\n");
            }
            
            // Add key findings
            if let Some(findings) = obj.get("findings") {
                response.push_str(&format!("**Findings:**\n{}\n\n", 
                    serde_json::to_string_pretty(findings).unwrap_or_default()));
            }
            
            if let Some(summary) = obj.get("summary").and_then(|v| v.as_str()) {
                response.push_str(&format!("**Summary:**\n{}\n\n", summary));
            }
            
            if let Some(recommendations) = obj.get("recommendations") {
                response.push_str(&format!("**Recommendations:**\n{}\n\n", 
                    serde_json::to_string_pretty(recommendations).unwrap_or_default()));
            }
            
            // Add raw data if nothing else matched
            if response.len() < 50 {
                response = format!("I've completed your request:\n\n```json\n{}\n```", result_str);
            }
            
            response
        } else {
            format!("I've completed your request:\n\n```json\n{}\n```", result_str)
        }
    } else {
        // Simple text result
        format!("I've completed your request: {}", result_str)
    }
}

/// Check for swarm status command
pub fn is_swarm_status_command(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    lower == "swarm status" 
        || lower == "show swarm" 
        || lower == "swarm info"
        || lower == "orchs status"
}

/// Check for swarm alerts command
pub fn is_swarm_alerts_command(input: &str) -> bool {
    let lower = input.trim().to_lowercase();
    lower == "swarm alerts" 
        || lower == "show alerts" 
        || lower == "orch alerts"
        || lower == "check alerts"
}

/// Format swarm status for user display
pub async fn format_swarm_status(swarm_interface: &Arc<Mutex<SolaSwarmInterface>>) -> String {
    let interface = swarm_interface.lock().await;
    
    match interface.get_swarm_status().await {
        Some(status) => {
            let mut response = String::from("🐝 **Swarm Status**\n\n");
            
            response.push_str(&format!("**Total ORCHs:** {}\n", status.total_orchs));
            response.push_str(&format!("**Active ORCHs:** {}\n", status.active_orchs));
            response.push_str(&format!("**Pending Auctions:** {}\n", status.pending_auctions));
            response.push_str(&format!("**Active Tasks:** {}\n\n", status.active_tasks));
            
            if !status.orchs.is_empty() {
                response.push_str("**Registered ORCHs:**\n");
                for orch in &status.orchs {
                    response.push_str(&format!(
                        "- **{}** ({:?}) - Load: {:.0}%, Tasks: {}, Specializations: {:?}\n",
                        orch.name,
                        orch.status,
                        orch.current_load * 100.0,
                        orch.active_tasks,
                        orch.specializations
                    ));
                }
            } else {
                response.push_str("*No ORCHs currently registered.*\n");
            }
            
            response
        }
        None => {
            "Swarm mode is currently hidden. Use `swarm mode on` to reveal ORCH activity.".to_string()
        }
    }
}

/// Format swarm alerts for user display
pub async fn format_swarm_alerts(swarm_interface: &Arc<Mutex<SolaSwarmInterface>>) -> String {
    let interface = swarm_interface.lock().await;
    let alerts = interface.check_alerts().await;
    
    if alerts.is_empty() {
        return "No pending alerts from ORCHs. All systems nominal. ✅".to_string();
    }
    
    let mut response = format!("🚨 **Swarm Alerts** ({})\n\n", alerts.len());
    
    for alert in &alerts {
        let severity_emoji = match alert.severity {
            crate::internal_bus::AlertSeverity::Critical => "🔴",
            crate::internal_bus::AlertSeverity::High => "🟠",
            crate::internal_bus::AlertSeverity::Medium => "🟡",
            crate::internal_bus::AlertSeverity::Low => "🟢",
            crate::internal_bus::AlertSeverity::Info => "ℹ️",
        };
        
        response.push_str(&format!(
            "{} **{}** from {}\n",
            severity_emoji, alert.category, alert.orch_name
        ));
        response.push_str(&format!("   {}\n", alert.description));
        
        if !alert.details.is_null() {
            response.push_str(&format!(
                "   Details: {}\n",
                serde_json::to_string(&alert.details).unwrap_or_default()
            ));
        }
        
        response.push('\n');
    }
    
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_security_task() {
        let result = analyze_task("scan my system for vulnerabilities");
        assert!(result.is_some());
        let (task_type, _) = result.unwrap();
        assert!(matches!(task_type, TaskType::SecurityAnalysis));
    }
    
    #[test]
    fn test_analyze_code_task() {
        let result = analyze_task("analyze my code for bugs and optimize it");
        assert!(result.is_some());
        let (task_type, _) = result.unwrap();
        assert!(matches!(task_type, TaskType::CodeAnalysis));
    }
    
    #[test]
    fn test_simple_query_no_delegation() {
        let result = analyze_task("what's the weather today?");
        assert!(result.is_none());
    }
    
    #[test]
    fn test_complex_task_detection() {
        let result = analyze_task("I need you to perform a comprehensive security audit of my entire network, including all connected devices, check for vulnerabilities, and provide a detailed report with recommendations");
        assert!(result.is_some());
        let (_, complexity) = result.unwrap();
        assert!(matches!(complexity, TaskComplexity::Complex));
    }
}
