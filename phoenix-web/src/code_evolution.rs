// phoenix-web/src/code_evolution.rs
// Code Self-Modification System for Sola Solo
//
// This module enables Sola to safely modify her own codebase with:
// - Permission-based access control (Safe Zones / No-Go Zones)
// - Automatic backup before modifications
// - Test validation with auto-revert on failure
// - Comprehensive audit logging

use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Permissions configuration loaded from permissions.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsConfig {
    pub safe_zones: SafeZones,
    pub no_go_zones: NoGoZones,
    pub evolution_rules: EvolutionRules,
    pub audit_settings: AuditSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeZones {
    pub directories: Vec<String>,
    pub file_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoGoZones {
    pub directories: Vec<String>,
    pub files: Vec<String>,
    pub file_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRules {
    pub require_backup: bool,
    pub backup_directory: String,
    pub max_file_size_bytes: usize,
    pub max_changes_per_session: usize,
    pub require_test_pass: bool,
    pub test_commands: TestCommands,
    pub auto_revert_on_failure: bool,
    pub require_human_approval_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCommands {
    pub rust: String,
    pub typescript: String,
    pub all: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSettings {
    pub log_file: String,
    pub log_format: String,
    pub include_diff: bool,
    pub include_timestamp: bool,
    pub include_reason: bool,
    pub include_result: bool,
}

/// Evolution plan from OpenRouter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPlan {
    pub file_path: String,
    pub action: EvolutionAction,
    pub content: Option<String>,
    pub reason: String,
    pub plan_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EvolutionAction {
    Create,
    Modify,
    Delete,
}

/// Request body for /api/agent/evolve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRequest {
    pub plan: EvolutionPlan,
    pub skip_tests: Option<bool>,
    pub force: Option<bool>,
}

/// Response from /api/agent/evolve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResponse {
    pub success: bool,
    pub message: String,
    pub evolution_id: String,
    pub backup_path: Option<String>,
    pub test_result: Option<TestResult>,
    pub reverted: bool,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub command: String,
    pub output: String,
    pub duration_ms: u64,
}

/// Evolution log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub file_path: String,
    pub action: EvolutionAction,
    pub reason: String,
    pub plan_source: String,
    pub test_result: Option<TestResult>,
    pub status: EvolutionStatus,
    pub diff: Option<String>,
    pub backup_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvolutionStatus {
    Applied,
    Reverted,
    PendingApproval,
    Failed,
}

/// Global evolution counter for session limits
static EVOLUTION_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Load permissions from permissions.json
pub fn load_permissions() -> Result<PermissionsConfig, String> {
    let path = Path::new("permissions.json");
    if !path.exists() {
        return Err("permissions.json not found".to_string());
    }

    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read permissions.json: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse permissions.json: {}", e))
}

/// Check if a path is in a safe zone
pub fn is_safe_zone(path: &str, permissions: &PermissionsConfig) -> bool {
    let path = Path::new(path);
    
    // Check if in no-go directories
    for no_go_dir in &permissions.no_go_zones.directories {
        if path.starts_with(no_go_dir) {
            return false;
        }
    }
    
    // Check if it's a no-go file
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        if permissions.no_go_zones.files.contains(&file_name_str.to_string()) {
            return false;
        }
        
        // Check no-go file patterns
        for pattern in &permissions.no_go_zones.file_patterns {
            if matches_pattern(&file_name_str, pattern) {
                return false;
            }
        }
    }
    
    // Check if in safe directories
    for safe_dir in &permissions.safe_zones.directories {
        if path.starts_with(safe_dir) {
            // Also check file pattern
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                for pattern in &permissions.safe_zones.file_patterns {
                    if matches_pattern(&file_name_str, pattern) {
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

/// Simple glob pattern matching (supports * wildcard)
fn matches_pattern(name: &str, pattern: &str) -> bool {
    if pattern.starts_with("*.") {
        let ext = &pattern[1..];
        name.ends_with(ext)
    } else if pattern.ends_with("*") {
        let prefix = &pattern[..pattern.len() - 1];
        name.starts_with(prefix)
    } else {
        name == pattern
    }
}

/// Check if file requires human approval
pub fn requires_approval(path: &str, permissions: &PermissionsConfig) -> bool {
    let path = Path::new(path);
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        permissions.evolution_rules.require_human_approval_for.contains(&file_name_str.to_string())
    } else {
        false
    }
}

/// Create backup of a file
pub fn create_backup(file_path: &str, backup_dir: &str) -> Result<String, String> {
    let source = Path::new(file_path);
    if !source.exists() {
        return Ok(String::new()); // No backup needed for new files
    }
    
    // Create backup directory if it doesn't exist
    fs::create_dir_all(backup_dir).map_err(|e| format!("Failed to create backup directory: {}", e))?;
    
    // Generate backup filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let file_name = source.file_name().unwrap_or_default().to_string_lossy();
    let backup_name = format!("{}_{}.bak", file_name, timestamp);
    let backup_path = Path::new(backup_dir).join(&backup_name);
    
    fs::copy(source, &backup_path).map_err(|e| format!("Failed to create backup: {}", e))?;
    
    Ok(backup_path.to_string_lossy().to_string())
}

/// Restore from backup
pub fn restore_from_backup(backup_path: &str, original_path: &str) -> Result<(), String> {
    if backup_path.is_empty() {
        // If no backup, delete the file (it was newly created)
        if Path::new(original_path).exists() {
            fs::remove_file(original_path).map_err(|e| format!("Failed to remove file: {}", e))?;
        }
        return Ok(());
    }
    
    fs::copy(backup_path, original_path).map_err(|e| format!("Failed to restore from backup: {}", e))?;
    Ok(())
}

/// Run tests to validate changes
pub fn run_tests(file_path: &str, permissions: &PermissionsConfig) -> TestResult {
    let start = std::time::Instant::now();
    
    // Determine which test command to run based on file extension
    let command = if file_path.ends_with(".rs") {
        &permissions.evolution_rules.test_commands.rust
    } else if file_path.ends_with(".ts") || file_path.ends_with(".tsx") {
        &permissions.evolution_rules.test_commands.typescript
    } else {
        &permissions.evolution_rules.test_commands.all
    };
    
    // Run the test command
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
    } else {
        Command::new("sh")
            .args(["-c", command])
            .output()
    };
    
    let duration_ms = start.elapsed().as_millis() as u64;
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined_output = format!("{}\n{}", stdout, stderr);
            
            TestResult {
                passed: output.status.success(),
                command: command.clone(),
                output: combined_output.chars().take(2000).collect(), // Limit output size
                duration_ms,
            }
        }
        Err(e) => TestResult {
            passed: false,
            command: command.clone(),
            output: format!("Failed to run test command: {}", e),
            duration_ms,
        },
    }
}

/// Generate diff between old and new content
pub fn generate_diff(old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    
    let mut diff = String::new();
    
    // Simple line-by-line diff (not a full diff algorithm)
    let max_lines = old_lines.len().max(new_lines.len());
    for i in 0..max_lines {
        let old_line = old_lines.get(i).unwrap_or(&"");
        let new_line = new_lines.get(i).unwrap_or(&"");
        
        if old_line != new_line {
            if !old_line.is_empty() {
                diff.push_str(&format!("- {}\n", old_line));
            }
            if !new_line.is_empty() {
                diff.push_str(&format!("+ {}\n", new_line));
            }
        }
    }
    
    if diff.is_empty() {
        diff = "(no changes)".to_string();
    }
    
    diff
}

/// Append entry to EVOLUTION_LOG.md
pub fn log_evolution(entry: &EvolutionLogEntry, permissions: &PermissionsConfig) -> Result<(), String> {
    let log_path = &permissions.audit_settings.log_file;
    
    let mut log_content = fs::read_to_string(log_path).unwrap_or_default();
    
    // Find the "## Evolution History" section and insert after it
    let marker = "## Evolution History";
    if let Some(pos) = log_content.find(marker) {
        let insert_pos = pos + marker.len();
        
        let entry_text = format!(
            r#"

### [{}] - Evolution #{}

**File:** `{}`
**Action:** {:?}
**Reason:** {}
**Plan Source:** {}
**Test Result:** {}
**Status:** {:?}
{}
#### Outcome
Evolution {} at {}.

---"#,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            entry.id,
            entry.file_path,
            entry.action,
            entry.reason,
            entry.plan_source,
            entry.test_result.as_ref().map(|t| if t.passed { "PASS" } else { "FAIL" }).unwrap_or("SKIPPED"),
            entry.status,
            if permissions.audit_settings.include_diff {
                format!("\n#### Changes\n```diff\n{}\n```\n", entry.diff.as_deref().unwrap_or("(no diff available)"))
            } else {
                String::new()
            },
            match entry.status {
                EvolutionStatus::Applied => "successfully applied",
                EvolutionStatus::Reverted => "was reverted due to test failure",
                EvolutionStatus::PendingApproval => "is pending human approval",
                EvolutionStatus::Failed => "failed to apply",
            },
            entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        );
        
        log_content.insert_str(insert_pos, &entry_text);
    }
    
    // Update statistics
    let stats_marker = "| Total Evolutions |";
    if let Some(pos) = log_content.find(stats_marker) {
        // Simple increment - in production, parse and update properly
        // For now, just note that stats should be updated
    }
    
    // Update last updated timestamp
    let last_updated_marker = "*Last updated:";
    if let Some(pos) = log_content.find(last_updated_marker) {
        let end_pos = log_content[pos..].find('*').map(|p| pos + p + 1).unwrap_or(log_content.len());
        let new_timestamp = format!("*Last updated: {}*", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        log_content.replace_range(pos..end_pos, &new_timestamp);
    }
    
    fs::write(log_path, log_content).map_err(|e| format!("Failed to write evolution log: {}", e))?;
    
    Ok(())
}

/// Main evolution endpoint handler
pub async fn api_agent_evolve(body: web::Json<EvolutionRequest>) -> impl Responder {
    let evolution_id = uuid::Uuid::new_v4().to_string();
    let timestamp = Utc::now();
    
    // Load permissions
    let permissions = match load_permissions() {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError().json(EvolutionResponse {
                success: false,
                message: format!("Failed to load permissions: {}", e),
                evolution_id,
                backup_path: None,
                test_result: None,
                reverted: false,
                requires_approval: false,
            });
        }
    };
    
    let plan = &body.plan;
    let skip_tests = body.skip_tests.unwrap_or(false);
    let force = body.force.unwrap_or(false);
    
    // Check session limit
    let current_count = EVOLUTION_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if current_count >= permissions.evolution_rules.max_changes_per_session && !force {
        return HttpResponse::TooManyRequests().json(EvolutionResponse {
            success: false,
            message: format!("Session limit reached ({} changes). Use force=true to override.", 
                           permissions.evolution_rules.max_changes_per_session),
            evolution_id,
            backup_path: None,
            test_result: None,
            reverted: false,
            requires_approval: false,
        });
    }
    
    // Check if path is in safe zone
    if !is_safe_zone(&plan.file_path, &permissions) {
        return HttpResponse::Forbidden().json(EvolutionResponse {
            success: false,
            message: format!("Path '{}' is not in a safe zone or is in a no-go zone", plan.file_path),
            evolution_id,
            backup_path: None,
            test_result: None,
            reverted: false,
            requires_approval: false,
        });
    }
    
    // Check if requires human approval
    if requires_approval(&plan.file_path, &permissions) && !force {
        let entry = EvolutionLogEntry {
            id: evolution_id.clone(),
            timestamp,
            file_path: plan.file_path.clone(),
            action: plan.action.clone(),
            reason: plan.reason.clone(),
            plan_source: plan.plan_source.clone(),
            test_result: None,
            status: EvolutionStatus::PendingApproval,
            diff: None,
            backup_path: None,
        };
        
        let _ = log_evolution(&entry, &permissions);
        
        return HttpResponse::Ok().json(EvolutionResponse {
            success: false,
            message: format!("File '{}' requires human approval. Use force=true to override.", plan.file_path),
            evolution_id,
            backup_path: None,
            test_result: None,
            reverted: false,
            requires_approval: true,
        });
    }
    
    // Create backup if required
    let backup_path = if permissions.evolution_rules.require_backup {
        match create_backup(&plan.file_path, &permissions.evolution_rules.backup_directory) {
            Ok(path) => Some(path),
            Err(e) => {
                return HttpResponse::InternalServerError().json(EvolutionResponse {
                    success: false,
                    message: format!("Failed to create backup: {}", e),
                    evolution_id,
                    backup_path: None,
                    test_result: None,
                    reverted: false,
                    requires_approval: false,
                });
            }
        }
    } else {
        None
    };
    
    // Read old content for diff
    let old_content = fs::read_to_string(&plan.file_path).unwrap_or_default();
    
    // Apply the change
    let apply_result = match &plan.action {
        EvolutionAction::Create | EvolutionAction::Modify => {
            if let Some(content) = &plan.content {
                // Create parent directories if needed
                if let Some(parent) = Path::new(&plan.file_path).parent() {
                    let _ = fs::create_dir_all(parent);
                }
                fs::write(&plan.file_path, content)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No content provided"))
            }
        }
        EvolutionAction::Delete => {
            fs::remove_file(&plan.file_path)
        }
    };
    
    if let Err(e) = apply_result {
        return HttpResponse::InternalServerError().json(EvolutionResponse {
            success: false,
            message: format!("Failed to apply change: {}", e),
            evolution_id,
            backup_path,
            test_result: None,
            reverted: false,
            requires_approval: false,
        });
    }
    
    // Generate diff
    let new_content = plan.content.as_deref().unwrap_or("");
    let diff = generate_diff(&old_content, new_content);
    
    // Run tests if required
    let test_result = if permissions.evolution_rules.require_test_pass && !skip_tests {
        Some(run_tests(&plan.file_path, &permissions))
    } else {
        None
    };
    
    // Check if we need to revert
    let mut reverted = false;
    let mut status = EvolutionStatus::Applied;
    
    if let Some(ref result) = test_result {
        if !result.passed && permissions.evolution_rules.auto_revert_on_failure {
            // Revert from backup
            if let Some(ref backup) = backup_path {
                if let Err(e) = restore_from_backup(backup, &plan.file_path) {
                    return HttpResponse::InternalServerError().json(EvolutionResponse {
                        success: false,
                        message: format!("Tests failed and revert also failed: {}", e),
                        evolution_id,
                        backup_path,
                        test_result,
                        reverted: false,
                        requires_approval: false,
                    });
                }
                reverted = true;
                status = EvolutionStatus::Reverted;
            }
        }
    }
    
    // Log the evolution
    let entry = EvolutionLogEntry {
        id: evolution_id.clone(),
        timestamp,
        file_path: plan.file_path.clone(),
        action: plan.action.clone(),
        reason: plan.reason.clone(),
        plan_source: plan.plan_source.clone(),
        test_result: test_result.clone(),
        status: status.clone(),
        diff: Some(diff),
        backup_path: backup_path.clone(),
    };
    
    if let Err(e) = log_evolution(&entry, &permissions) {
        eprintln!("Warning: Failed to log evolution: {}", e);
    }
    
    let message = if reverted {
        format!("Evolution applied but reverted due to test failure. Backup at: {:?}", backup_path)
    } else {
        format!("Evolution successfully applied to {}", plan.file_path)
    };
    
    HttpResponse::Ok().json(EvolutionResponse {
        success: !reverted,
        message,
        evolution_id,
        backup_path,
        test_result,
        reverted,
        requires_approval: false,
    })
}

/// Get current permissions
pub async fn api_agent_permissions() -> impl Responder {
    match load_permissions() {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

/// Get evolution statistics
pub async fn api_agent_evolution_stats() -> impl Responder {
    let current_count = EVOLUTION_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
    
    let permissions = load_permissions().ok();
    let max_changes = permissions.as_ref()
        .map(|p| p.evolution_rules.max_changes_per_session)
        .unwrap_or(10);
    
    HttpResponse::Ok().json(serde_json::json!({
        "session_evolutions": current_count,
        "max_per_session": max_changes,
        "remaining": max_changes.saturating_sub(current_count),
    }))
}

/// Reset session counter (for testing/admin)
pub async fn api_agent_reset_counter() -> impl Responder {
    EVOLUTION_COUNTER.store(0, std::sync::atomic::Ordering::SeqCst);
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Evolution counter reset",
        "session_evolutions": 0
    }))
}
