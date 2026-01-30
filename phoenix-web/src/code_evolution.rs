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
    /// Phase H: Performance delta from benchmarking (positive = improvement)
    #[serde(default)]
    pub performance_delta: Option<f64>,
    /// Phase H: JIT stability score (0.0 - 1.0, higher = more stable)
    #[serde(default)]
    pub jit_stability_score: Option<f64>,
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
            performance_delta: None,
            jit_stability_score: None,
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
    // TODO: Phase H integration - extract performance_delta from test_result benchmarks
    let performance_delta = test_result.as_ref().and_then(|t| {
        // If test has timing info, calculate a simple performance metric
        // In a full Phase H implementation, this would come from dedicated benchmarks
        if t.duration_ms > 0 {
            Some(0.0) // Placeholder - real implementation would compare against baseline
        } else {
            None
        }
    });
    
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
        performance_delta,
        jit_stability_score: None, // TODO: Phase H - measure JIT stability across runs
    };
    
    if let Err(e) = log_evolution(&entry, &permissions) {
        eprintln!("Warning: Failed to log evolution: {}", e);
    }
    
    // Index evolution in Vector KB for RAG (long-term memory)
    // This allows Sola to recall past evolutions and learn from them
    if !reverted {
        if let Err(e) = index_evolution_to_vector_kb(&entry).await {
            eprintln!("Warning: Failed to index evolution to Vector KB: {}", e);
        }
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

/// Index an evolution entry to the Vector KB for RAG (long-term memory)
///
/// This function stores evolution metadata in the vector database so Sola can:
/// - Recall past code changes and their outcomes
/// - Learn from successful and failed evolutions
/// - Make context-aware decisions during future evolutions
///
/// The text stored includes the file path, action, reason, and outcome,
/// which allows semantic search to find relevant past evolutions.
pub async fn index_evolution_to_vector_kb(entry: &EvolutionLogEntry) -> Result<(), String> {
    // Check if Vector KB is enabled
    let enabled = std::env::var("VECTOR_KB_ENABLED")
        .map(|s| s.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    
    if !enabled {
        return Ok(()); // Silently skip if not enabled
    }
    
    // Get the vector DB path
    let path = std::env::var("VECTOR_DB_PATH")
        .unwrap_or_else(|_| "./data/vector_db".to_string());
    
    // Initialize the vector KB
    let kb = match vector_kb::VectorKB::new(&path) {
        Ok(kb) => kb,
        Err(e) => return Err(format!("Failed to initialize Vector KB: {}", e)),
    };
    
    // Create a rich text representation for embedding
    let text = format!(
        "Evolution #{}: {} action on file '{}'. Reason: {}. Plan source: {}. Status: {:?}. {}",
        entry.id,
        match entry.action {
            EvolutionAction::Create => "CREATE",
            EvolutionAction::Modify => "MODIFY",
            EvolutionAction::Delete => "DELETE",
        },
        entry.file_path,
        entry.reason,
        entry.plan_source,
        entry.status,
        entry.test_result.as_ref()
            .map(|t| format!("Tests: {} ({}ms)", if t.passed { "PASSED" } else { "FAILED" }, t.duration_ms))
            .unwrap_or_else(|| "Tests: SKIPPED".to_string())
    );
    
    // Create metadata JSON with Phase H metrics for payload filtering
    let metadata = serde_json::json!({
        "type": "evolution",
        "evolution_id": entry.id,
        "file_path": entry.file_path,
        "action": format!("{:?}", entry.action),
        "status": format!("{:?}", entry.status),
        "timestamp": entry.timestamp.to_rfc3339(),
        "plan_source": entry.plan_source,
        "test_passed": entry.test_result.as_ref().map(|t| t.passed),
        "has_diff": entry.diff.is_some(),
        // Phase H: Performance metrics for "Fitness" filtering
        "performance_delta": entry.performance_delta,
        "jit_stability_score": entry.jit_stability_score,
        // Bare Metal metadata for hardware-specific filtering (Qdrant 1.13 payload filtering)
        "os_type": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
    });
    
    // Store in vector KB
    match kb.add_memory(&text, metadata).await {
        Ok(mem) => {
            eprintln!("✅ Evolution indexed to Vector KB: id={}", mem.id);
            Ok(())
        }
        Err(e) => Err(format!("Failed to add memory: {}", e)),
    }
}

/// Search the Vector KB for past evolutions related to a query
///
/// This allows Sola to recall relevant past evolutions when making decisions.
/// For example: "Have I tried to refactor this file before? What happened?"
///
/// # Arguments
/// * `query` - The search query
/// * `top_k` - Number of results to return
/// * `kb` - Optional shared VectorKB instance (from AppState). If None, will try to create a new one.
pub async fn search_evolution_history(
    query: &str,
    top_k: usize,
    kb: Option<&vector_kb::VectorKB>
) -> Result<Vec<vector_kb::MemoryResult>, String> {
    // Check if Vector KB is enabled
    let enabled = std::env::var("VECTOR_KB_ENABLED")
        .map(|s| s.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    
    if !enabled {
        return Err("Vector KB is not enabled".to_string());
    }
    
    // Use provided KB or create a new one (fallback for standalone usage)
    if let Some(kb) = kb {
        // Use the shared instance
        kb.semantic_search(query, top_k)
            .await
            .map_err(|e| format!("Search failed: {}", e))
    } else {
        // Fallback: try to create a new instance (may fail if already locked)
        let path = std::env::var("VECTOR_DB_PATH")
            .unwrap_or_else(|_| "./data/vector_db".to_string());
        
        let kb = match vector_kb::VectorKB::new(&path) {
            Ok(kb) => kb,
            Err(e) => return Err(format!("Failed to initialize Vector KB: {}", e)),
        };
        
        kb.semantic_search(query, top_k)
            .await
            .map_err(|e| format!("Search failed: {}", e))
    }
}

/// Query parameters for KB search endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbSearchQuery {
    pub q: String,
    #[serde(default = "default_top_k")]
    pub limit: usize,
}

fn default_top_k() -> usize {
    5
}

// ============================================================================
// PHASE I: SEMANTIC RECALL - "The Cognitive Bridge"
// ============================================================================
//
// This section implements the "Recall" phase of Sola's evolutionary cycle:
// | Phase   | Action                                      | Purpose                                    |
// |---------|---------------------------------------------|-------------------------------------------|
// | Recall  | Search `sola_history` for current file/task | Avoid repeating failed "ancestral" mutations |
// | Propose | Generate mutation using retrieved context   | Build upon previous successful optimizations |
// | Simulate| Sandbox test (Phase G)                      | Verify syntax and build integrity          |
// | Bench   | Measure Performance Delta (Phase H)         | Ensure "Fitness" before merging            |
// | Commit  | Index results back to Qdrant                | Log the "Lesson Learned" into long-term memory |

/// Represents a recalled evolution from the Vector KB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecalledEvolution {
    pub id: String,
    pub text: String,
    /// Base similarity score from vector search (0.0 - 1.0)
    pub score: f32,
    /// Temporally-adjusted score (includes recency boost)
    pub adjusted_score: f32,
    pub file_path: Option<String>,
    pub action: Option<String>,
    pub status: Option<String>,
    pub timestamp: Option<String>,
    pub test_passed: Option<bool>,
    pub reason: Option<String>,
    /// Phase H: Performance delta from benchmarking
    pub performance_delta: Option<f64>,
    /// Phase H: JIT stability score
    pub jit_stability_score: Option<f64>,
    /// Age of this evolution in days (for temporal weighting)
    pub age_days: Option<i64>,
}

/// Memory context block for LLM prompt injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryContext {
    /// Relevant past evolutions (top 3 by semantic similarity)
    pub lessons_learned: Vec<RecalledEvolution>,
    /// Known regressions to avoid (failed evolutions with high similarity)
    pub known_regressions: Vec<RecalledEvolution>,
    /// Current KB status
    pub kb_status: KbOperationStatus,
}

/// KB operation status for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KbOperationStatus {
    /// KB is idle, ready for queries
    Idle,
    /// KB is currently indexing new data
    Learning,
    /// KB is currently searching/recalling
    Recalling,
    /// KB is disabled or unavailable
    Offline,
}

impl Default for MemoryContext {
    fn default() -> Self {
        Self {
            lessons_learned: Vec::new(),
            known_regressions: Vec::new(),
            kb_status: KbOperationStatus::Idle,
        }
    }
}

/// Phase I+: Temporal Weighting Constants
/// Recent evolutions (within 7 days) get a +10% boost to similarity score
const TEMPORAL_BOOST_DAYS: i64 = 7;
const TEMPORAL_BOOST_FACTOR: f32 = 0.10;

/// Calculate temporal boost for a recalled evolution
///
/// Recent evolutions (within TEMPORAL_BOOST_DAYS) get a boost to prioritize
/// current architectural patterns over older, potentially outdated approaches.
fn calculate_temporal_boost(timestamp_str: Option<&str>) -> (f32, Option<i64>) {
    let Some(ts_str) = timestamp_str else {
        return (0.0, None);
    };
    
    // Parse the timestamp
    let Ok(timestamp) = DateTime::parse_from_rfc3339(ts_str) else {
        return (0.0, None);
    };
    
    let now = Utc::now();
    let age = now.signed_duration_since(timestamp.with_timezone(&Utc));
    let age_days = age.num_days();
    
    // Apply temporal boost for recent evolutions
    let boost = if age_days <= TEMPORAL_BOOST_DAYS {
        TEMPORAL_BOOST_FACTOR
    } else {
        0.0
    };
    
    (boost, Some(age_days))
}

/// Perform semantic recall for a given task/file context
///
/// This is the "Recall" phase of the evolutionary cycle. It searches the Vector KB
/// for relevant past evolutions and categorizes them into:
/// - `lessons_learned`: Successful evolutions to build upon
/// - `known_regressions`: Failed evolutions to avoid
///
/// ## Phase I+: Temporal Reflection
/// Recent evolutions (within 7 days) receive a +10% boost to their similarity score
/// to prioritize current architectural patterns over older approaches.
///
/// # Arguments
/// * `task_description` - The current task or file path being evolved
/// * `top_k` - Number of results to retrieve (default: 5)
///
/// # Returns
/// A `MemoryContext` containing categorized past evolutions
pub async fn semantic_recall(
    task_description: &str,
    top_k: usize,
    kb: Option<&vector_kb::VectorKB>
) -> MemoryContext {
    let mut context = MemoryContext {
        kb_status: KbOperationStatus::Recalling,
        ..Default::default()
    };
    
    // Check if Vector KB is enabled
    let enabled = std::env::var("VECTOR_KB_ENABLED")
        .map(|s| s.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    
    if !enabled {
        context.kb_status = KbOperationStatus::Offline;
        return context;
    }
    
    // Search for relevant past evolutions
    match search_evolution_history(task_description, top_k, kb).await {
        Ok(results) => {
            let mut recalled_evolutions: Vec<RecalledEvolution> = Vec::new();
            
            for result in results {
                // Extract timestamp for temporal weighting
                let timestamp_str = result.metadata.get("timestamp").and_then(|v| v.as_str());
                let (temporal_boost, age_days) = calculate_temporal_boost(timestamp_str);
                
                // Calculate adjusted score with temporal boost
                let base_score = result.score;
                let adjusted_score = (base_score * (1.0 + temporal_boost)).min(1.0);
                
                let recalled = RecalledEvolution {
                    id: result.id.clone(),
                    text: result.text.clone(),
                    score: base_score,
                    adjusted_score,
                    file_path: result.metadata.get("file_path").and_then(|v| v.as_str()).map(String::from),
                    action: result.metadata.get("action").and_then(|v| v.as_str()).map(String::from),
                    status: result.metadata.get("status").and_then(|v| v.as_str()).map(String::from),
                    timestamp: timestamp_str.map(String::from),
                    test_passed: result.metadata.get("test_passed").and_then(|v| v.as_bool()),
                    reason: None, // Extract from text if needed
                    // Phase H metrics
                    performance_delta: result.metadata.get("performance_delta").and_then(|v| v.as_f64()),
                    jit_stability_score: result.metadata.get("jit_stability_score").and_then(|v| v.as_f64()),
                    age_days,
                };
                
                recalled_evolutions.push(recalled);
            }
            
            // Sort by adjusted score (with temporal boost applied)
            recalled_evolutions.sort_by(|a, b| b.adjusted_score.partial_cmp(&a.adjusted_score).unwrap_or(std::cmp::Ordering::Equal));
            
            // Categorize based on status and test results
            for recalled in recalled_evolutions {
                let is_regression = recalled.status.as_deref() == Some("Reverted")
                    || recalled.status.as_deref() == Some("Failed")
                    || recalled.test_passed == Some(false);
                
                // Use adjusted_score for threshold comparisons
                if is_regression && recalled.adjusted_score > 0.6 {
                    // High similarity to a known failure - this is a regression warning
                    context.known_regressions.push(recalled);
                } else if recalled.adjusted_score > 0.5 {
                    // Relevant past evolution (successful or informative)
                    context.lessons_learned.push(recalled);
                }
            }
            
            // Limit to top 3 lessons and top 2 regressions
            context.lessons_learned.truncate(3);
            context.known_regressions.truncate(2);
            context.kb_status = KbOperationStatus::Idle;
        }
        Err(e) => {
            eprintln!("⚠️ Semantic recall failed: {}", e);
            context.kb_status = KbOperationStatus::Offline;
        }
    }
    
    context
}

/// Format memory context as a system prompt block for LLM injection
///
/// This creates a "Lessons Learned" section that can be prepended to the LLM's
/// system prompt, providing historical context for the current evolution task.
///
/// ## Phase I+: Enhanced Context
/// - Includes temporal weighting information (age of evolutions)
/// - Shows Phase H metrics (performance_delta, jit_stability_score)
/// - Highlights "High Fitness" evolutions for prioritization
///
/// # Arguments
/// * `context` - The memory context from semantic recall
///
/// # Returns
/// A formatted string block for prompt injection
pub fn format_memory_context_for_prompt(context: &MemoryContext) -> String {
    let mut prompt_block = String::new();
    
    if context.lessons_learned.is_empty() && context.known_regressions.is_empty() {
        return prompt_block; // No context to inject
    }
    
    prompt_block.push_str("\n\n## 🧠 MEMORY CONTEXT (Lessons from Past Evolutions)\n\n");
    
    // Add lessons learned with Phase H metrics
    if !context.lessons_learned.is_empty() {
        prompt_block.push_str("### ✅ Relevant Past Successes:\n");
        for (i, lesson) in context.lessons_learned.iter().enumerate() {
            // Show both base and adjusted scores for transparency
            let score_info = if (lesson.adjusted_score - lesson.score).abs() > 0.01 {
                format!("[Score: {:.2} → {:.2} (temporal boost)]", lesson.score, lesson.adjusted_score)
            } else {
                format!("[Score: {:.2}]", lesson.score)
            };
            
            prompt_block.push_str(&format!(
                "{}. {} {}\n",
                i + 1,
                score_info,
                lesson.text
            ));
            
            if let Some(ref file) = lesson.file_path {
                prompt_block.push_str(&format!("   📁 File: {}\n", file));
            }
            
            // Show Phase H metrics if available
            let mut metrics = Vec::new();
            if let Some(perf) = lesson.performance_delta {
                let emoji = if perf > 0.0 { "📈" } else { "📉" };
                metrics.push(format!("{} Perf: {:+.1}%", emoji, perf));
            }
            if let Some(jit) = lesson.jit_stability_score {
                let emoji = if jit > 0.8 { "🟢" } else if jit > 0.5 { "🟡" } else { "🔴" };
                metrics.push(format!("{} JIT: {:.0}%", emoji, jit * 100.0));
            }
            if let Some(age) = lesson.age_days {
                let freshness = if age <= 7 { "🔥 Recent" } else if age <= 30 { "📅 This month" } else { "📜 Older" };
                metrics.push(format!("{} ({} days ago)", freshness, age));
            }
            
            if !metrics.is_empty() {
                prompt_block.push_str(&format!("   {}\n", metrics.join(" | ")));
            }
        }
        prompt_block.push('\n');
    }
    
    // Add known regressions with explicit warnings and Phase H context
    if !context.known_regressions.is_empty() {
        prompt_block.push_str("### ⚠️ KNOWN REGRESSIONS (AVOID THESE APPROACHES):\n");
        for regression in &context.known_regressions {
            // Use adjusted score for display
            let similarity_pct = regression.adjusted_score * 100.0;
            let severity = if similarity_pct > 85.0 { "🛑 HARD CONFLICT" } else { "⚠️ WARNING" };
            
            prompt_block.push_str(&format!(
                "- {} **Evolution ID {}** [Similarity: {:.0}%]: {}\n",
                severity,
                regression.id,
                similarity_pct,
                regression.text
            ));
            
            if let Some(ref status) = regression.status {
                prompt_block.push_str(&format!("  Status: {} - DO NOT repeat this approach.\n", status));
            }
            
            // Show why it failed (Phase H metrics)
            if let Some(perf) = regression.performance_delta {
                prompt_block.push_str(&format!("  📉 Performance Impact: {:.1}% (this caused the failure)\n", perf));
            }
            if let Some(jit) = regression.jit_stability_score {
                if jit < 0.5 {
                    prompt_block.push_str(&format!("  🔴 JIT Stability: {:.0}% (unstable)\n", jit * 100.0));
                }
            }
        }
        
        // Check if any regressions require justification
        let has_hard_conflict = context.known_regressions.iter().any(|r| r.adjusted_score > 0.85);
        
        if has_hard_conflict {
            prompt_block.push_str("\n🛑 **HARD CONFLICT DETECTED**: One or more regressions have >85% similarity.\n");
            prompt_block.push_str("If you must proceed with a similar approach, you MUST include a section:\n");
            prompt_block.push_str("```\n**Justification for Re-attempt:**\n");
            prompt_block.push_str("1. Why the previous approach failed: [explanation]\n");
            prompt_block.push_str("2. What is different about this attempt: [explanation]\n");
            prompt_block.push_str("3. Why this approach will succeed: [explanation]\n");
            prompt_block.push_str("```\n\n");
        } else {
            prompt_block.push_str("\n**IMPORTANT**: The above approaches have been tried before and FAILED. ");
            prompt_block.push_str("Please propose an alternative strategy.\n\n");
        }
    }
    
    prompt_block
}

/// Phase I+: Conflict Hardening Constants
/// Mutations >85% similar to known regressions require justification
const HARD_CONFLICT_THRESHOLD: f32 = 0.85;

/// Result of regression conflict check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionConflictResult {
    /// Whether a conflict was detected
    pub has_conflict: bool,
    /// Warning message for the user
    pub warning: Option<String>,
    /// Whether justification is required to proceed
    pub requires_justification: bool,
    /// The regression ID that triggered the conflict
    pub regression_id: Option<String>,
    /// Similarity score to the regression
    pub similarity: Option<f32>,
    /// Performance delta from the failed evolution (if available)
    pub previous_performance_delta: Option<f64>,
}

impl Default for RegressionConflictResult {
    fn default() -> Self {
        Self {
            has_conflict: false,
            warning: None,
            requires_justification: false,
            regression_id: None,
            similarity: None,
            previous_performance_delta: None,
        }
    }
}

/// Check if a proposed evolution conflicts with known regressions
///
/// This implements "Conflict Resolution" - if Sola proposes a change that the KB
/// identifies as a "Known Regression", we can preemptively warn.
///
/// ## Phase I+: Conflict Hardening
/// If a mutation is >85% similar to a known regression, the agent MUST provide
/// a "Justification for Re-attempt" in the rationale before simulation can proceed.
///
/// # Arguments
/// * `proposed_file` - The file path being modified
/// * `proposed_action` - The action being taken (CREATE, MODIFY, DELETE)
/// * `context` - The memory context from semantic recall
///
/// # Returns
/// A `RegressionConflictResult` with conflict details and justification requirements
pub fn check_regression_conflict(
    proposed_file: &str,
    proposed_action: &str,
    context: &MemoryContext,
) -> RegressionConflictResult {
    let mut result = RegressionConflictResult::default();
    
    for regression in &context.known_regressions {
        // Use adjusted_score for threshold comparisons (includes temporal weighting)
        let similarity = regression.adjusted_score;
        
        // Check if the regression is for the same file
        if let Some(ref file) = regression.file_path {
            if file == proposed_file && similarity > 0.75 {
                result.has_conflict = true;
                result.regression_id = Some(regression.id.clone());
                result.similarity = Some(similarity);
                result.previous_performance_delta = regression.performance_delta;
                
                // Check if this requires justification (hard conflict)
                if similarity > HARD_CONFLICT_THRESHOLD {
                    result.requires_justification = true;
                    result.warning = Some(format!(
                        "🛑 HARD CONFLICT: A similar {} operation on '{}' was attempted in Evolution #{} \
                        and resulted in failure (similarity: {:.0}%). \
                        {} \
                        \n\n**JUSTIFICATION REQUIRED**: You must provide a 'Justification for Re-attempt' \
                        explaining why this approach will succeed where the previous one failed. \
                        Include this in your rationale before simulation can proceed.",
                        proposed_action,
                        proposed_file,
                        regression.id,
                        similarity * 100.0,
                        regression.performance_delta
                            .map(|d| format!("Previous performance impact: {:.1}%.", d))
                            .unwrap_or_default()
                    ));
                } else {
                    result.warning = Some(format!(
                        "⚠️ REGRESSION WARNING: A similar {} operation on '{}' was attempted in Evolution #{} \
                        and resulted in failure (similarity: {:.0}%). Consider an alternative approach.",
                        proposed_action,
                        proposed_file,
                        regression.id,
                        similarity * 100.0
                    ));
                }
                return result;
            }
        }
        
        // Check for high semantic similarity regardless of file (hard conflict threshold)
        if similarity > HARD_CONFLICT_THRESHOLD {
            result.has_conflict = true;
            result.requires_justification = true;
            result.regression_id = Some(regression.id.clone());
            result.similarity = Some(similarity);
            result.previous_performance_delta = regression.performance_delta;
            result.warning = Some(format!(
                "🛑 HARD CONFLICT: This evolution is highly similar ({:.0}%) to a previous failure \
                (Evolution #{}). The previous attempt failed with status: {}. \
                {} \
                \n\n**JUSTIFICATION REQUIRED**: You must provide a 'Justification for Re-attempt' \
                explaining why this approach will succeed where the previous one failed. \
                Include this in your rationale before simulation can proceed.",
                similarity * 100.0,
                regression.id,
                regression.status.as_deref().unwrap_or("Unknown"),
                regression.performance_delta
                    .map(|d| format!("Previous performance impact: {:.1}%.", d))
                    .unwrap_or_default()
            ));
            return result;
        }
    }
    
    result
}

/// Legacy wrapper for backward compatibility
/// Returns just the warning message string
pub fn check_regression_conflict_simple(
    proposed_file: &str,
    proposed_action: &str,
    context: &MemoryContext,
) -> Option<String> {
    let result = check_regression_conflict(proposed_file, proposed_action, context);
    result.warning
}

/// Validate that a justification is provided when required
///
/// # Arguments
/// * `conflict` - The regression conflict result
/// * `rationale` - The agent's rationale for the evolution
///
/// # Returns
/// `Ok(())` if no justification is required or if a valid justification is provided,
/// `Err(message)` if justification is required but not provided
pub fn validate_justification(
    conflict: &RegressionConflictResult,
    rationale: &str,
) -> Result<(), String> {
    if !conflict.requires_justification {
        return Ok(());
    }
    
    // Check for justification markers in the rationale
    let rationale_lower = rationale.to_lowercase();
    let has_justification = rationale_lower.contains("justification for re-attempt")
        || rationale_lower.contains("justification:")
        || rationale_lower.contains("re-attempt justification")
        || rationale_lower.contains("why this will succeed")
        || rationale_lower.contains("different approach because");
    
    if has_justification && rationale.len() > 100 {
        // Justification appears to be present and substantive
        Ok(())
    } else {
        Err(format!(
            "🛑 SIMULATION BLOCKED: This evolution is {:.0}% similar to a known regression (Evolution #{}). \
            You must provide a 'Justification for Re-attempt' in your rationale explaining: \
            1) Why the previous approach failed \
            2) What is different about this attempt \
            3) Why this approach will succeed \
            \nPlease update your rationale and try again.",
            conflict.similarity.unwrap_or(0.0) * 100.0,
            conflict.regression_id.as_deref().unwrap_or("Unknown")
        ))
    }
}

/// Response for KB search endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbSearchResponse {
    pub results: Vec<KbSearchResult>,
    pub count: usize,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbSearchResult {
    pub id: String,
    pub text: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

/// HTTP API endpoint for searching Sola's long-term memory (Vector KB)
///
/// GET /api/kb/search?q=<query>&limit=<n>
///
/// This endpoint allows the frontend to query Sola's "long-term memory"
/// for context-aware decisions during evolution and chat.
pub async fn api_kb_search(query: web::Query<KbSearchQuery>) -> impl Responder {
    let q = query.q.trim();
    if q.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Query parameter 'q' is required and cannot be empty"
        }));
    }
    
    match search_evolution_history(q, query.limit, None).await {
        Ok(results) => {
            let response = KbSearchResponse {
                count: results.len(),
                query: q.to_string(),
                results: results.into_iter().map(|r| KbSearchResult {
                    id: r.id,
                    text: r.text,
                    score: r.score,
                    metadata: r.metadata,
                }).collect(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e,
                "query": q
            }))
        }
    }
}

/// HTTP API endpoint for getting Vector KB status
///
/// GET /api/kb/status
///
/// Returns information about the Vector KB configuration and health.
/// This endpoint is designed to be displayed in the Recommendations panel
/// so users can see if Sola is currently "Learning" (indexing) or "Recalling" (searching).
pub async fn api_kb_status() -> impl Responder {
    let enabled = std::env::var("VECTOR_KB_ENABLED")
        .map(|s| s.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    
    let path = std::env::var("VECTOR_DB_PATH")
        .unwrap_or_else(|_| "./data/vector_db".to_string());
    
    let qdrant_url = std::env::var("VECTOR_DB_URL").ok();
    let collection = std::env::var("VECTOR_DB_COLLECTION")
        .unwrap_or_else(|_| "sola_history".to_string());
    
    let mut status = serde_json::json!({
        "enabled": enabled,
        "backend": if qdrant_url.is_some() { "qdrant" } else { "sled" },
        "path": path,
        "qdrant_url": qdrant_url,
        "collection": collection,
        "operation_status": if enabled { "idle" } else { "offline" },
        "operation_status_display": if enabled { "🧠 Ready" } else { "⚫ Offline" },
    });
    
    // Try to get memory count if enabled
    if enabled {
        if let Ok(kb) = vector_kb::VectorKB::new(&path) {
            if let Ok(all) = kb.all().await {
                let count = all.len();
                status["memory_count"] = serde_json::json!(count);
                status["memory_count_display"] = serde_json::json!(format!("{} memories indexed", count));
                
                // Categorize memories by type
                let evolution_count = all.iter()
                    .filter(|m| m.metadata.get("type").and_then(|v| v.as_str()) == Some("evolution"))
                    .count();
                status["evolution_count"] = serde_json::json!(evolution_count);
            }
        }
    }
    
    HttpResponse::Ok().json(status)
}

/// Request body for semantic recall endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRecallRequest {
    /// The task description or file path to search for
    pub query: String,
    /// Number of results to retrieve (default: 5)
    #[serde(default)]
    pub top_k: Option<usize>,
}

/// Response from semantic recall endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRecallResponse {
    pub success: bool,
    pub context: MemoryContext,
    pub prompt_block: String,
    pub regression_warning: Option<String>,
}

/// HTTP API endpoint for semantic recall
///
/// POST /api/kb/recall
///
/// This endpoint performs the "Recall" phase of the evolutionary cycle.
/// It searches the Vector KB for relevant past evolutions and returns:
/// - Lessons learned (successful past evolutions)
/// - Known regressions (failed past evolutions to avoid)
/// - A formatted prompt block for LLM injection
///
/// # Request Body
/// ```json
/// {
///   "query": "refactor authentication module",
///   "top_k": 5
/// }
/// ```
///
/// # Response
/// ```json
/// {
///   "success": true,
///   "context": { ... },
///   "prompt_block": "## 🧠 MEMORY CONTEXT...",
///   "regression_warning": null
/// }
/// ```
pub async fn api_kb_recall(body: web::Json<SemanticRecallRequest>) -> impl Responder {
    let query = body.query.trim();
    if query.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Query is required and cannot be empty"
        }));
    }
    
    let top_k = body.top_k.unwrap_or(5);
    
    // Perform semantic recall
    let context = semantic_recall(query, top_k, None).await;
    
    // Format the memory context for prompt injection
    let prompt_block = format_memory_context_for_prompt(&context);
    
    // Check for regression conflicts (using query as a proxy for file/action)
    let regression_warning = if !context.known_regressions.is_empty() {
        Some(format!(
            "⚠️ Found {} known regression(s) similar to this task. Review the context before proceeding.",
            context.known_regressions.len()
        ))
    } else {
        None
    };
    
    HttpResponse::Ok().json(SemanticRecallResponse {
        success: true,
        context,
        prompt_block,
        regression_warning,
    })
}

/// HTTP API endpoint for evolution with semantic recall
///
/// POST /api/agent/evolve-with-recall
///
/// This is an enhanced version of the evolve endpoint that automatically
/// performs semantic recall before applying the evolution. It:
/// 1. Searches for relevant past evolutions (Recall phase)
/// 2. Checks for known regressions (Conflict Resolution)
/// 3. Applies the evolution if no blocking conflicts
/// 4. Indexes the result back to the KB (Commit phase)
///
/// # Request Body
/// Same as `/api/agent/evolve` but with automatic context injection
pub async fn api_agent_evolve_with_recall(body: web::Json<EvolutionRequest>) -> HttpResponse {
    let plan = &body.plan;
    
    // Phase 1: RECALL - Search for relevant past evolutions
    let recall_query = format!(
        "{} {} {}",
        plan.file_path,
        match plan.action {
            EvolutionAction::Create => "create",
            EvolutionAction::Modify => "modify",
            EvolutionAction::Delete => "delete",
        },
        plan.reason
    );
    
    let context = semantic_recall(&recall_query, 5, None).await;
    
    // Phase 2: CONFLICT RESOLUTION - Check for known regressions (Phase I+ Hardened)
    let action_str = match plan.action {
        EvolutionAction::Create => "CREATE",
        EvolutionAction::Modify => "MODIFY",
        EvolutionAction::Delete => "DELETE",
    };
    
    let conflict = check_regression_conflict(&plan.file_path, action_str, &context);
    
    if conflict.has_conflict {
        // Phase I+: If justification is required, validate it
        if conflict.requires_justification {
            // Check if the rationale contains a valid justification
            if let Err(block_msg) = validate_justification(&conflict, &plan.reason) {
                // Block simulation until justification is provided
                return HttpResponse::Conflict().json(serde_json::json!({
                    "success": false,
                    "message": block_msg,
                    "conflict": {
                        "regression_id": conflict.regression_id,
                        "similarity": conflict.similarity,
                        "previous_performance_delta": conflict.previous_performance_delta,
                        "requires_justification": true
                    },
                    "context": context,
                    "hint": "Include a 'Justification for Re-attempt' section in your rationale explaining why this approach will succeed."
                }));
            }
            // Justification provided - log and continue
            eprintln!("✅ Justification accepted for re-attempt of regression-similar evolution");
        } else if let Some(ref warning) = conflict.warning {
            // Soft conflict - warn but allow force override
            if !body.force.unwrap_or(false) {
                return HttpResponse::Conflict().json(serde_json::json!({
                    "success": false,
                    "message": warning,
                    "conflict": {
                        "regression_id": conflict.regression_id,
                        "similarity": conflict.similarity,
                        "requires_justification": false
                    },
                    "context": context,
                    "requires_force": true,
                    "hint": "Set force=true to override this warning and proceed with the evolution."
                }));
            }
            // If force is set, log the warning but continue
            eprintln!("⚠️ Proceeding with evolution despite regression warning: {}", warning);
        }
    }
    
    // Phase 3: PROPOSE & SIMULATE - Inline the evolution logic
    // We can't call api_agent_evolve directly due to return type mismatch,
    // so we duplicate the core logic here with the memory context included
    let evolution_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now();
    
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
            performance_delta: None,
            jit_stability_score: None,
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
                if let Some(parent) = std::path::Path::new(&plan.file_path).parent() {
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
    
    // Log the evolution with memory context
    // TODO: Phase H integration - extract performance_delta from test_result benchmarks
    let performance_delta = test_result.as_ref().and_then(|t| {
        if t.duration_ms > 0 {
            Some(0.0) // Placeholder - real implementation would compare against baseline
        } else {
            None
        }
    });
    
    let entry = EvolutionLogEntry {
        id: evolution_id.clone(),
        timestamp,
        file_path: plan.file_path.clone(),
        action: plan.action.clone(),
        reason: format!("{} [Memory Context: {} lessons, {} regressions]",
            plan.reason,
            context.lessons_learned.len(),
            context.known_regressions.len()
        ),
        plan_source: plan.plan_source.clone(),
        test_result: test_result.clone(),
        status: status.clone(),
        diff: Some(diff),
        backup_path: backup_path.clone(),
        performance_delta,
        jit_stability_score: None, // TODO: Phase H - measure JIT stability across runs
    };
    
    if let Err(e) = log_evolution(&entry, &permissions) {
        eprintln!("Warning: Failed to log evolution: {}", e);
    }
    
    // Index evolution in Vector KB for RAG (long-term memory)
    if !reverted {
        if let Err(e) = index_evolution_to_vector_kb(&entry).await {
            eprintln!("Warning: Failed to index evolution to Vector KB: {}", e);
        }
    }
    
    let message = if reverted {
        format!("Evolution applied but reverted due to test failure. Backup at: {:?}", backup_path)
    } else {
        format!("Evolution successfully applied to {} (with {} memory context items)", 
            plan.file_path,
            context.lessons_learned.len() + context.known_regressions.len()
        )
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
