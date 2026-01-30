// phoenix-web/src/mission_control.rs
//
// Mission Control: Observability & Steering for Level 5 Autonomy
//
// This module provides:
// - Server-Sent Events (SSE) for real-time Chain of Thought streaming
// - Autonomy state management (pause/resume)
// - Post-mortem logging after failed attempts
// - Tool usage tracking and visualization data

use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn};

/// Global autonomy state
static AUTONOMY_PAUSED: AtomicBool = AtomicBool::new(false);
static CURRENT_ATTEMPT: AtomicUsize = AtomicUsize::new(0);
static MAX_ATTEMPTS: usize = 3;

/// Self-correction record for tracking failed attempts and lessons learned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCorrection {
    pub method: String,
    pub tool_used: String,
    pub failure_reason: String,
    pub lesson_learned: String,
    pub severity: SelfCorrectionSeverity,
    pub timestamp: DateTime<Utc>,
}

/// Severity level for self-corrections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelfCorrectionSeverity {
    /// Minor issue, easily recoverable
    Yellow,
    /// Major issue, required method switch
    Red,
}

/// Chain of Thought event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CoTEvent {
    /// Sola is starting a new task
    #[serde(rename = "task_start")]
    TaskStart {
        task_id: String,
        description: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Sola is using a tool
    #[serde(rename = "tool_call")]
    ToolCall {
        task_id: String,
        tool: String,
        input: serde_json::Value,
        attempt: usize,
        max_attempts: usize,
        timestamp: DateTime<Utc>,
    },
    
    /// Tool returned a result
    #[serde(rename = "tool_result")]
    ToolResult {
        task_id: String,
        tool: String,
        success: bool,
        output: Option<String>,
        error: Option<String>,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },
    
    /// Sola is thinking/reasoning
    #[serde(rename = "thought")]
    Thought {
        task_id: String,
        content: String,
        confidence: f64,
        timestamp: DateTime<Utc>,
    },
    
    /// Sola is switching methods in the Persistence Protocol
    #[serde(rename = "method_switch")]
    MethodSwitch {
        task_id: String,
        from_method: String,
        to_method: String,
        reason_for_switch: String,
        lesson_learned: String,
        attempt_number: usize,
        timestamp: DateTime<Utc>,
    },
    
    /// Task completed successfully
    #[serde(rename = "task_complete")]
    TaskComplete {
        task_id: String,
        result: String,
        total_attempts: usize,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },
    
    /// Task failed after max attempts
    #[serde(rename = "task_failed")]
    TaskFailed {
        task_id: String,
        reason: String,
        reason_for_retry: Option<String>,
        attempts_made: usize,
        self_corrections: Vec<SelfCorrection>,
        post_mortem: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Autonomy was paused by user
    #[serde(rename = "autonomy_paused")]
    AutonomyPaused {
        task_id: Option<String>,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Autonomy was resumed
    #[serde(rename = "autonomy_resumed")]
    AutonomyResumed {
        task_id: Option<String>,
        timestamp: DateTime<Utc>,
    },
    
    /// Hard stop - all child processes killed
    #[serde(rename = "hard_stop")]
    HardStop {
        task_id: Option<String>,
        processes_killed: usize,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Heartbeat to keep connection alive
    #[serde(rename = "heartbeat")]
    Heartbeat {
        timestamp: DateTime<Utc>,
        autonomy_paused: bool,
        current_attempt: usize,
    },
}

/// Mission Control state shared across the application
pub struct MissionControlState {
    /// Broadcast channel for CoT events
    pub event_tx: broadcast::Sender<CoTEvent>,
    /// Current task ID (if any)
    pub current_task_id: Arc<tokio::sync::RwLock<Option<String>>>,
}

impl MissionControlState {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            event_tx,
            current_task_id: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }
    
    /// Emit a CoT event
    pub fn emit(&self, event: CoTEvent) {
        let _ = self.event_tx.send(event);
    }
    
    /// Start a new task
    pub async fn start_task(&self, task_id: &str, description: &str) {
        *self.current_task_id.write().await = Some(task_id.to_string());
        CURRENT_ATTEMPT.store(0, Ordering::SeqCst);
        
        self.emit(CoTEvent::TaskStart {
            task_id: task_id.to_string(),
            description: description.to_string(),
            timestamp: Utc::now(),
        });
    }
    
    /// Record a tool call
    pub fn record_tool_call(&self, task_id: &str, tool: &str, input: serde_json::Value) {
        let attempt = CURRENT_ATTEMPT.fetch_add(1, Ordering::SeqCst) + 1;
        
        self.emit(CoTEvent::ToolCall {
            task_id: task_id.to_string(),
            tool: tool.to_string(),
            input,
            attempt,
            max_attempts: MAX_ATTEMPTS,
            timestamp: Utc::now(),
        });
    }
    
    /// Record a tool result
    pub fn record_tool_result(
        &self,
        task_id: &str,
        tool: &str,
        success: bool,
        output: Option<String>,
        error: Option<String>,
        duration_ms: u64,
    ) {
        self.emit(CoTEvent::ToolResult {
            task_id: task_id.to_string(),
            tool: tool.to_string(),
            success,
            output,
            error,
            duration_ms,
            timestamp: Utc::now(),
        });
    }
    
    /// Record a thought
    pub fn record_thought(&self, task_id: &str, content: &str, confidence: f64) {
        self.emit(CoTEvent::Thought {
            task_id: task_id.to_string(),
            content: content.to_string(),
            confidence,
            timestamp: Utc::now(),
        });
    }
    
    /// Complete a task
    pub async fn complete_task(&self, task_id: &str, result: &str, duration_ms: u64) {
        let attempts = CURRENT_ATTEMPT.load(Ordering::SeqCst);
        *self.current_task_id.write().await = None;
        
        self.emit(CoTEvent::TaskComplete {
            task_id: task_id.to_string(),
            result: result.to_string(),
            total_attempts: attempts,
            duration_ms,
            timestamp: Utc::now(),
        });
    }
    
    /// Fail a task with post-mortem
    pub async fn fail_task(&self, task_id: &str, reason: &str, post_mortem: &str) {
        let attempts = CURRENT_ATTEMPT.load(Ordering::SeqCst);
        *self.current_task_id.write().await = None;
        
        self.emit(CoTEvent::TaskFailed {
            task_id: task_id.to_string(),
            reason: reason.to_string(),
            reason_for_retry: None,
            attempts_made: attempts,
            self_corrections: vec![],
            post_mortem: post_mortem.to_string(),
            timestamp: Utc::now(),
        });
    }
    
    /// Fail a task with detailed self-corrections
    pub async fn fail_task_with_corrections(
        &self,
        task_id: &str,
        reason: &str,
        reason_for_retry: Option<&str>,
        self_corrections: Vec<SelfCorrection>,
        post_mortem: &str,
    ) {
        let attempts = CURRENT_ATTEMPT.load(Ordering::SeqCst);
        *self.current_task_id.write().await = None;
        
        self.emit(CoTEvent::TaskFailed {
            task_id: task_id.to_string(),
            reason: reason.to_string(),
            reason_for_retry: reason_for_retry.map(|s| s.to_string()),
            attempts_made: attempts,
            self_corrections,
            post_mortem: post_mortem.to_string(),
            timestamp: Utc::now(),
        });
    }
    
    /// Record a method switch in the Persistence Protocol
    pub fn record_method_switch(
        &self,
        task_id: &str,
        from_method: &str,
        to_method: &str,
        reason_for_switch: &str,
        lesson_learned: &str,
    ) {
        let attempt = CURRENT_ATTEMPT.load(Ordering::SeqCst);
        
        // Also emit a thought explaining the switch
        self.emit(CoTEvent::Thought {
            task_id: task_id.to_string(),
            content: format!(
                "Method {} was insufficient: {}. Switching to {} because {}",
                from_method, reason_for_switch, to_method, lesson_learned
            ),
            confidence: 0.7,
            timestamp: Utc::now(),
        });
        
        self.emit(CoTEvent::MethodSwitch {
            task_id: task_id.to_string(),
            from_method: from_method.to_string(),
            to_method: to_method.to_string(),
            reason_for_switch: reason_for_switch.to_string(),
            lesson_learned: lesson_learned.to_string(),
            attempt_number: attempt,
            timestamp: Utc::now(),
        });
    }
}

impl Default for MissionControlState {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if autonomy is paused
pub fn is_autonomy_paused() -> bool {
    AUTONOMY_PAUSED.load(Ordering::SeqCst)
}

/// Pause autonomy
pub fn pause_autonomy() {
    AUTONOMY_PAUSED.store(true, Ordering::SeqCst);
    info!("Mission Control: Autonomy PAUSED");
}

/// Resume autonomy
pub fn resume_autonomy() {
    AUTONOMY_PAUSED.store(false, Ordering::SeqCst);
    info!("Mission Control: Autonomy RESUMED");
}

/// Get current attempt number
pub fn get_current_attempt() -> usize {
    CURRENT_ATTEMPT.load(Ordering::SeqCst)
}

/// Check if max attempts reached
pub fn max_attempts_reached() -> bool {
    CURRENT_ATTEMPT.load(Ordering::SeqCst) >= MAX_ATTEMPTS
}

/// Generate a post-mortem report
pub fn generate_post_mortem(
    task_description: &str,
    attempts: &[(String, String, bool)], // (tool, input_summary, success)
) -> String {
    let mut report = format!(
        "## Post-Mortem Report\n\n\
        **Task:** {}\n\
        **Attempts:** {}\n\
        **Status:** Failed after {} attempts\n\n\
        ### Attempt History\n\n",
        task_description,
        attempts.len(),
        attempts.len()
    );
    
    for (i, (tool, input, success)) in attempts.iter().enumerate() {
        let status = if *success { "✅" } else { "❌" };
        report.push_str(&format!(
            "{}. **{}** - {} {}\n   Input: {}\n\n",
            i + 1,
            tool,
            status,
            if *success { "Success" } else { "Failed" },
            input
        ));
    }
    
    report.push_str("\n### Recommendations\n\n");
    report.push_str("- Consider providing more specific input\n");
    report.push_str("- Check if required API keys are configured\n");
    report.push_str("- Verify network connectivity for web searches\n");
    
    report
}

// ============================================================================
// API Handlers
// ============================================================================

/// Tracked child processes for hard stop
static CHILD_PIDS: std::sync::LazyLock<std::sync::Mutex<Vec<u32>>> = 
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

/// Register a child process PID for potential hard stop
pub fn register_child_process(pid: u32) {
    if let Ok(mut pids) = CHILD_PIDS.lock() {
        pids.push(pid);
        info!("Mission Control: Registered child process PID {}", pid);
    }
}

/// Unregister a child process PID (when it completes normally)
pub fn unregister_child_process(pid: u32) {
    if let Ok(mut pids) = CHILD_PIDS.lock() {
        pids.retain(|&p| p != pid);
    }
}

/// Hard stop - kill all tracked child processes
pub fn hard_stop_all_processes() -> usize {
    let mut killed = 0;
    
    if let Ok(mut pids) = CHILD_PIDS.lock() {
        for pid in pids.drain(..) {
            #[cfg(windows)]
            {
                // On Windows, use taskkill
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/PID", &pid.to_string()])
                    .output();
                killed += 1;
                info!("Mission Control: Hard stopped process PID {}", pid);
            }
            
            #[cfg(unix)]
            {
                // On Unix, send SIGKILL
                unsafe {
                    libc::kill(pid as i32, libc::SIGKILL);
                }
                killed += 1;
                info!("Mission Control: Hard stopped process PID {}", pid);
            }
        }
    }
    
    // Also pause autonomy
    pause_autonomy();
    
    killed
}

/// Request to pause/resume autonomy
#[derive(Debug, Deserialize)]
pub struct AutonomyControlRequest {
    pub action: String, // "pause", "resume", or "hard_stop"
    pub reason: Option<String>,
}

/// GET /api/agent/events - SSE endpoint for Chain of Thought streaming
pub async fn api_agent_events(
    state: web::Data<crate::AppState>,
) -> impl Responder {
    use actix_web::http::header;
    use futures_util::StreamExt;
    
    // Get or create mission control state
    let mission_control = state.mission_control.clone();
    let mut rx = mission_control.event_tx.subscribe();
    
    // Create SSE stream
    let stream = async_stream::stream! {
        // Send initial heartbeat
        let heartbeat = CoTEvent::Heartbeat {
            timestamp: Utc::now(),
            autonomy_paused: is_autonomy_paused(),
            current_attempt: get_current_attempt(),
        };
        yield Ok::<_, actix_web::Error>(
            web::Bytes::from(format!("data: {}\n\n", serde_json::to_string(&heartbeat).unwrap()))
        );
        
        // Stream events
        loop {
            tokio::select! {
                // Receive events from broadcast channel
                result = rx.recv() => {
                    match result {
                        Ok(event) => {
                            let json = serde_json::to_string(&event).unwrap();
                            yield Ok(web::Bytes::from(format!("data: {}\n\n", json)));
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            // Missed some events, continue
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            break;
                        }
                    }
                }
                // Send heartbeat every 30 seconds
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(30)) => {
                    let heartbeat = CoTEvent::Heartbeat {
                        timestamp: Utc::now(),
                        autonomy_paused: is_autonomy_paused(),
                        current_attempt: get_current_attempt(),
                    };
                    yield Ok(web::Bytes::from(format!("data: {}\n\n", serde_json::to_string(&heartbeat).unwrap())));
                }
            }
        }
    };
    
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/event-stream"))
        .insert_header((header::CACHE_CONTROL, "no-cache"))
        .insert_header((header::CONNECTION, "keep-alive"))
        .streaming(stream)
}

/// POST /api/agent/autonomy - Control autonomy (pause/resume/hard_stop)
pub async fn api_agent_autonomy_control(
    body: web::Json<AutonomyControlRequest>,
    state: web::Data<crate::AppState>,
) -> impl Responder {
    let mission_control = state.mission_control.clone();
    let current_task = mission_control.current_task_id.read().await.clone();
    
    match body.action.as_str() {
        "pause" => {
            pause_autonomy();
            mission_control.emit(CoTEvent::AutonomyPaused {
                task_id: current_task,
                reason: body.reason.clone().unwrap_or_else(|| "User requested pause".to_string()),
                timestamp: Utc::now(),
            });
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "action": "paused",
                "message": "Autonomy paused. Sola will wait for manual input."
            }))
        }
        "resume" => {
            resume_autonomy();
            mission_control.emit(CoTEvent::AutonomyResumed {
                task_id: current_task,
                timestamp: Utc::now(),
            });
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "action": "resumed",
                "message": "Autonomy resumed. Sola will continue autonomous operation."
            }))
        }
        "hard_stop" => {
            let processes_killed = hard_stop_all_processes();
            let reason = body.reason.clone().unwrap_or_else(|| "Emergency hard stop requested".to_string());
            
            mission_control.emit(CoTEvent::HardStop {
                task_id: current_task.clone(),
                processes_killed,
                reason: reason.clone(),
                timestamp: Utc::now(),
            });
            
            // Also emit pause event
            mission_control.emit(CoTEvent::AutonomyPaused {
                task_id: current_task,
                reason: format!("Hard stop: {}", reason),
                timestamp: Utc::now(),
            });
            
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "action": "hard_stop",
                "processes_killed": processes_killed,
                "message": format!("Hard stop executed. {} child processes terminated. Autonomy paused.", processes_killed)
            }))
        }
        _ => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid action. Use 'pause', 'resume', or 'hard_stop'."
            }))
        }
    }
}

/// GET /api/agent/autonomy/status - Get current autonomy status
pub async fn api_agent_autonomy_status(
    state: web::Data<crate::AppState>,
) -> impl Responder {
    let mission_control = state.mission_control.clone();
    let current_task = mission_control.current_task_id.read().await.clone();
    
    HttpResponse::Ok().json(serde_json::json!({
        "autonomy_paused": is_autonomy_paused(),
        "current_attempt": get_current_attempt(),
        "max_attempts": MAX_ATTEMPTS,
        "current_task_id": current_task,
        "progress": format!("{}/{}", get_current_attempt(), MAX_ATTEMPTS),
    }))
}

/// POST /api/agent/emit - Manually emit a CoT event (for testing/debugging)
pub async fn api_agent_emit_event(
    body: web::Json<CoTEvent>,
    state: web::Data<crate::AppState>,
) -> impl Responder {
    let mission_control = state.mission_control.clone();
    mission_control.emit(body.into_inner());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Event emitted"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_autonomy_pause_resume() {
        // Start unpaused
        resume_autonomy();
        assert!(!is_autonomy_paused());
        
        // Pause
        pause_autonomy();
        assert!(is_autonomy_paused());
        
        // Resume
        resume_autonomy();
        assert!(!is_autonomy_paused());
    }
    
    #[test]
    fn test_attempt_tracking() {
        CURRENT_ATTEMPT.store(0, Ordering::SeqCst);
        assert_eq!(get_current_attempt(), 0);
        assert!(!max_attempts_reached());
        
        CURRENT_ATTEMPT.store(3, Ordering::SeqCst);
        assert!(max_attempts_reached());
    }
    
    #[test]
    fn test_post_mortem_generation() {
        let attempts = vec![
            ("vector_kb".to_string(), "search: test".to_string(), false),
            ("filesystem".to_string(), "grep: test".to_string(), false),
            ("web_search".to_string(), "query: test".to_string(), false),
        ];
        
        let report = generate_post_mortem("Find test information", &attempts);
        assert!(report.contains("Post-Mortem Report"));
        assert!(report.contains("3 attempts"));
        assert!(report.contains("vector_kb"));
    }
}
