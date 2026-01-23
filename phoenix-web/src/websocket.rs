// phoenix-web/src/websocket.rs
// WebSocket handler for real-time bi-directional communication

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::{Message, ProtocolError};
use futures_util::StreamExt as _;
use llm_orchestrator::ModelTier;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    #[serde(rename = "speak")]
    Speak {
        user_input: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        project_context: Option<String>,
    },
    #[serde(rename = "command")]
    Command {
        command: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        project_context: Option<String>,
    },
    #[serde(rename = "system")]
    System {
        /// Per-connection consent for privileged operations.
        /// Supported values: "grant", "revoke".
        action: String,
    },
    #[serde(rename = "memory_search")]
    MemorySearch {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vault: Option<String>, // "mind", "body", "soul" (default: "soul" to match REST API)
    },
    #[serde(rename = "memory_store")]
    MemoryStore {
        key: String,
        value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        vault: Option<String>, // "mind", "body", "soul" (default: "soul" to match REST API)
    },
    #[serde(rename = "memory_get")]
    MemoryGet {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        vault: Option<String>, // "mind", "body", "soul" (default: "soul" to match REST API)
    },
    #[serde(rename = "memory_delete")]
    MemoryDelete {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        vault: Option<String>, // "mind", "body", "soul" (default: "soul" to match REST API)
    },
    #[serde(rename = "memory_cortex_store")]
    MemoryCortexStore {
        layer: String, // "STM", "WM", "LTM", "EPM", "RFM"
        key: String,
        value: String,
    },
    #[serde(rename = "memory_cortex_get")]
    MemoryCortexGet { key: String },
    #[serde(rename = "memory_cortex_search")]
    MemoryCortexSearch {
        prefix: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<usize>,
    },
    #[serde(rename = "memory_vector_store")]
    MemoryVectorStore {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "memory_vector_search")]
    MemoryVectorSearch {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        k: Option<usize>,
    },
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    #[serde(rename = "speak_response")]
    SpeakResponse {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memory_commit: Option<String>,
    },
    #[serde(rename = "command_response")]
    CommandResponse {
        result: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        success: Option<bool>,
    },
    #[serde(rename = "system_response")]
    SystemResponse {
        status: String,
        message: String,
        consent_granted: bool,
    },
    #[serde(rename = "memory_search_response")]
    MemorySearchResponse {
        items: Vec<serde_json::Value>,
        count: usize,
        vault: String,
    },
    #[serde(rename = "memory_store_response")]
    MemoryStoreResponse {
        status: String,
        key: String,
        vault: String,
    },
    #[serde(rename = "memory_get_response")]
    MemoryGetResponse {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<String>,
        vault: String,
    },
    #[serde(rename = "memory_delete_response")]
    MemoryDeleteResponse {
        status: String,
        key: String,
        vault: String,
    },
    #[serde(rename = "memory_cortex_store_response")]
    MemoryCortexStoreResponse {
        status: String,
        key: String,
        layer: String,
    },
    #[serde(rename = "memory_cortex_get_response")]
    MemoryCortexGetResponse {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        layer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
    #[serde(rename = "memory_cortex_search_response")]
    MemoryCortexSearchResponse {
        items: Vec<serde_json::Value>,
        count: usize,
    },
    #[serde(rename = "memory_vector_store_response")]
    MemoryVectorStoreResponse { status: String, id: String },
    #[serde(rename = "memory_vector_search_response")]
    MemoryVectorSearchResponse {
        results: Vec<serde_json::Value>,
        count: usize,
    },
    #[serde(rename = "status_response")]
    StatusResponse {
        status: String,
        backend: String,
        version: String,
    },
    #[serde(rename = "proactive_message")]
    ProactiveMessage {
        content: String,
        reason: String,
        timestamp: i64,
    },
    #[serde(rename = "error")]
    Error {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
    },
    #[serde(rename = "pong")]
    Pong,
}

pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let conn_id = Uuid::new_v4().to_string();
    let peer = req
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    info!("WebSocket client connected: conn_id={conn_id} peer={peer}");

    // Per-connection consent tracking (default: false).
    let access_map: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut m = access_map.lock().await;
        m.insert(conn_id.clone(), false);
    }

    // Subscribe to proactive messages
    let mut proactive_rx = state.proactive_tx.subscribe();

    // Spawn task to handle WebSocket connection
    let access_map_task = access_map.clone();
    actix_web::rt::spawn(async move {
        let mut last_pong = tokio::time::Instant::now();
        let mut ping_interval = tokio::time::interval(Duration::from_secs(30));

        // Send welcome message
        let _ = session
            .text(
                json!({
                    "type": "connected",
                    "conn_id": conn_id,
                    "message": "WebSocket connection established"
                })
                .to_string(),
            )
            .await;

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    // Check if we've received a pong recently
                    if last_pong.elapsed() > Duration::from_secs(60) {
                        warn!("WebSocket connection timeout: conn_id={conn_id}");
                        let _ = session.close(None).await;
                        break;
                    }
                    // Send ping
                    let _ = session.ping(b"").await;
                }
                Ok(proactive_msg) = proactive_rx.recv() => {
                    // Forward proactive message to this WebSocket client
                    let response = WebSocketResponse::ProactiveMessage {
                        content: proactive_msg.content,
                        reason: proactive_msg.reason,
                        timestamp: proactive_msg.timestamp,
                    };
                    let response_json = serde_json::to_string(&response)
                        .unwrap_or_else(|_| json!({"type": "error", "message": "Serialization failed"}).to_string());
                    if let Err(e) = session.text(response_json).await {
                        error!("Failed to send proactive message: {}", e);
                        break;
                    }
                }
                msg = msg_stream.next() => {
                    let Some(msg) = msg else { break; };
                    match msg {
                        Ok(Message::Text(text)) => {
                            // Phase 3: token-by-token streaming for `speak`.
                            // We keep the legacy `speak_response` (sent after streaming completes)
                            // as a compatibility fallback for older clients.
                            let msg_type = serde_json::from_str::<serde_json::Value>(&text)
                                .ok()
                                .and_then(|v| v.get("type").and_then(|t| t.as_str()).map(|s| s.to_string()));

                            if msg_type.as_deref() == Some("speak") {
                                if let Err(e) = handle_speak_streaming(&text, &state, &peer, &conn_id, &mut session).await {
                                    warn!(
                                        "WebSocket speak streaming failed: conn_id={} peer={} err={}",
                                        conn_id,
                                        peer,
                                        e
                                    );
                                    // Best-effort: also send a generic error response.
                                    let error_response = WebSocketResponse::Error {
                                        message: e.to_string(),
                                        code: Some("speak_stream_error".to_string()),
                                    };
                                    let error_json = serde_json::to_string(&error_response)
                                        .unwrap_or_else(|_| json!({"type": "error", "message": "Error serialization failed"}).to_string());
                                    let _ = session.text(error_json).await;
                                }
                                continue;
                            }

                            match handle_message(&text, &state, &peer, &conn_id, &access_map_task).await {
                                Ok(response) => {
                                    let response_json = serde_json::to_string(&response)
                                        .unwrap_or_else(|_| json!({"type": "error", "message": "Serialization failed"}).to_string());
                                    if let Err(e) = session.text(response_json).await {
                                        error!("Failed to send WebSocket message: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        "WebSocket message processing failed: conn_id={} peer={} err={}",
                                        conn_id,
                                        peer,
                                        e
                                    );
                                    let error_response = WebSocketResponse::Error {
                                        message: e.to_string(),
                                        code: Some("processing_error".to_string()),
                                    };
                                    let error_json = serde_json::to_string(&error_response)
                                        .unwrap_or_else(|_| json!({"type": "error", "message": "Error serialization failed"}).to_string());
                                    let _ = session.text(error_json).await;
                                }
                            }
                        }
                        Ok(Message::Pong(_)) => {
                            last_pong = tokio::time::Instant::now();
                        }
                        Ok(Message::Ping(bytes)) => {
                            let _ = session.pong(&bytes).await;
                        }
                        Ok(Message::Close(reason)) => {
                            let _ = session.close(reason).await;
                            break;
                        }
                        Ok(Message::Binary(_)) => {
                            // Binary messages not supported for now
                        }
                        Ok(Message::Continuation(_)) => {}
                        Ok(Message::Nop) => {}
                        Err(ProtocolError::Overflow) => {
                            warn!("WebSocket buffer overflow: conn_id={conn_id}");
                            let _ = session.close(None).await;
                            break;
                        }
                        Err(e) => {
                            warn!("WebSocket error: {} conn_id={conn_id}", e);
                            break;
                        }
                    }
                }
            }
        }

        // Best-effort cleanup.
        {
            let mut m = access_map_task.lock().await;
            m.remove(&conn_id);
        }
        info!("WebSocket client disconnected: conn_id={conn_id}");
    });

    Ok(response)
}

async fn handle_speak_streaming(
    text: &str,
    state: &web::Data<AppState>,
    peer: &str,
    conn_id: &str,
    session: &mut actix_ws::Session,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg: WebSocketMessage = serde_json::from_str(text)?;
    let WebSocketMessage::Speak {
        user_input,
        mode,
        project_context,
    } = msg
    else {
        return Err("handle_speak_streaming called with non-speak message".into());
    };

    // Mark user message received for proactive timing
    state.proactive_state.user_message_received().await;

    // Store last user message for curiosity engine
    let _ = state.vaults.store_soul("last_user_message", &user_input);

    // Avoid holding the mutex across `.await`.
    let llm = state.llm.lock().await.clone();
    let Some(llm) = llm.as_ref() else {
        let payload = json!({
            "type": "speak_response_chunk",
            "error": "LLM orchestrator not available",
            "done": true
        })
        .to_string();
        let _ = session.text(payload).await;
        return Ok(());
    };

    // Parse mode string to ModelTier if provided, otherwise use None.
    let tier = mode.as_ref().and_then(|m| m.parse::<ModelTier>().ok());

    // Generate memory commit ID (kept consistent with legacy speak response).
    let memory_commit = format!(
        "PHX/{}/AGENT_SYNC_{}",
        project_context.as_deref().unwrap_or("GLOBAL"),
        Uuid::new_v4().to_string().to_uppercase()
    );

    info!(
        "ws.speak streaming start: conn_id={} peer={} input_len={} project_ctx_present={}",
        conn_id,
        peer,
        user_input.len(),
        project_context
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
    );

    // Intercept skills commands before LLM processing
    if user_input.trim().starts_with("skills ") {
        let response = crate::handle_skills_command(state, &user_input).await;
        let response_text = response
            .get("message")
            .or_else(|| response.get("result"))
            .and_then(|v| v.as_str())
            .unwrap_or("Skill command processed");

        // Send as a single response chunk
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;

        return Ok(());
    }

    // Intercept proactive control commands
    let input_lower = user_input.trim().to_lowercase();
    if input_lower == "proactive on" || input_lower == "proactive enable" {
        // Note: Runtime env var changes don't affect already-initialized ProactiveState
        // User must restart backend or we need to add a dynamic enable/disable method
        let response_text = "Note: Proactive communication is configured at startup via .env (PROACTIVE_ENABLED=true). To enable, add to .env and restart backend.";
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;
        return Ok(());
    } else if input_lower == "proactive off" || input_lower == "proactive disable" {
        let response_text = "Note: Proactive communication is configured at startup via .env. To disable, remove PROACTIVE_ENABLED or set to false in .env and restart backend.";
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;
        return Ok(());
    } else if input_lower == "proactive status" {
        let enabled = state.proactive_state.enabled;
        let response_text = format!(
            "Proactive communication is currently {}. (Interval: {}s, Rate limit: {}s)",
            if enabled { "enabled" } else { "disabled" },
            state.proactive_state.interval_secs,
            state.proactive_state.rate_limit_secs
        );
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": &response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;
        return Ok(());
    }
    
    // Intercept swarm status command (power-user feature)
    if crate::swarm_delegation::is_swarm_status_command(&user_input) {
        let response_text = crate::swarm_delegation::format_swarm_status(&state.swarm_interface).await;
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;
        
        // Legacy response
        let legacy = WebSocketResponse::SpeakResponse {
            message: response_text,
            memory_commit: Some(memory_commit),
        };
        let legacy_json = serde_json::to_string(&legacy).unwrap_or_else(|_| {
            json!({"type": "error", "message": "Serialization failed"}).to_string()
        });
        let _ = session.text(legacy_json).await;
        
        return Ok(());
    }
    
    // Intercept swarm alerts command (power-user feature)
    if crate::swarm_delegation::is_swarm_alerts_command(&user_input) {
        let response_text = crate::swarm_delegation::format_swarm_alerts(&state.swarm_interface).await;
        let payload = json!({
            "type": "speak_response_chunk",
            "chunk": response_text,
            "done": true,
            "memory_commit": memory_commit,
        })
        .to_string();
        let _ = session.text(payload).await;
        
        // Legacy response
        let legacy = WebSocketResponse::SpeakResponse {
            message: response_text,
            memory_commit: Some(memory_commit),
        };
        let legacy_json = serde_json::to_string(&legacy).unwrap_or_else(|_| {
            json!({"type": "error", "message": "Serialization failed"}).to_string()
        });
        let _ = session.text(legacy_json).await;
        
        return Ok(());
    }

    // Get cognitive mode and relationship phase for gating logic
    let phoenix_identity = state.phoenix_identity.lock().await.clone();
    let cognitive_mode = phoenix_identity.get_cognitive_mode().await;
    let zodiac_sign = phoenix_identity.zodiac_sign();
    let identity = phoenix_identity.get_identity().await;
    
    // Retrieve ProceduralGateMemory (L7) for relationship phase and trust score
    let (mut trust_score, mut relationship_phase, mut pii_checkbox) = 
        state.neural_cortex.recall_procedural_gate("procedural_gate:current");
    
    // AUTOMATED TRUST SCORE CALCULATION (L7 Logic)
    // Calculate trust increment based on PII sharing, sentiment, and behavior
    if cognitive_mode == phoenix_identity::CognitiveMode::Personal {
        use neural_cortex_strata::trust_calculator::{
            calculate_phase_transition, calculate_trust_increment, extract_pii_entities, merge_pii_checkboxes,
        };
        
        // Extract new PII from user input
        let new_pii = extract_pii_entities(&user_input);
        
        // Calculate trust increment
        let trust_increment = calculate_trust_increment(&user_input, relationship_phase, &pii_checkbox);
        
        // Update trust score (convert to 0-100 scale, then back to 0-1)
        let current_trust_percent = (trust_score.value() * 100.0) as i16;
        let new_trust_percent = (current_trust_percent as i16 + trust_increment as i16).max(0).min(100) as u8;
        trust_score = neural_cortex_strata::TrustScore::new(new_trust_percent as f32 / 100.0);
        
        // Merge new PII into existing checkbox list
        pii_checkbox = merge_pii_checkboxes(&pii_checkbox, &new_pii);
        
        // Check for phase transition
        if let Some(new_phase) = calculate_phase_transition(trust_score.value(), relationship_phase) {
            if new_phase != relationship_phase {
                info!(
                    "🎯 RELATIONSHIP PHASE TRANSITION: {:?} -> {:?} (Trust: {:.0}%)",
                    relationship_phase,
                    new_phase,
                    new_trust_percent
                );
                relationship_phase = new_phase;
            }
        }
        
        // Persist updated ProceduralGateMemory (L7)
        if let Err(e) = state.neural_cortex.etch_procedural_gate_memory(
            trust_score,
            relationship_phase,
            pii_checkbox.clone(),
            "procedural_gate:current",
        ) {
            warn!("Failed to persist ProceduralGateMemory: {}", e);
        } else {
            info!(
                "💾 L7 ProceduralGateMemory updated: Trust={:.0}%, Phase={:?}, PII={:?}",
                new_trust_percent,
                relationship_phase,
                pii_checkbox
            );
        }
    }
    
    // Intimacy Interceptor: Check if user is pushing for erotic content
    if detect_intimacy_intent(&user_input) {
        if relationship_phase != neural_cortex_strata::RelationshipPhase::Intimate {
            // Generate soft refusal based on relationship phase
            let refusal = generate_soft_refusal(
                relationship_phase,
                Some(&format!("{:?}", zodiac_sign)),
            );
            
            let payload = json!({
                "type": "speak_response_chunk",
                "chunk": refusal,
                "done": true,
                "memory_commit": memory_commit,
            })
            .to_string();
            let _ = session.text(payload).await;
            
            // Legacy response
            let legacy = WebSocketResponse::SpeakResponse {
                message: refusal,
                memory_commit: Some(memory_commit),
            };
            let legacy_json = serde_json::to_string(&legacy).unwrap_or_else(|_| {
                json!({"type": "error", "message": "Serialization failed"}).to_string()
            });
            let _ = session.text(legacy_json).await;
            
            return Ok(());
        }
    }
    
    // Professional Mode: Only allow swarm delegation, disable Fantasy Dyad
    // Personal Mode: Block system tools (already handled), allow Fantasy Dyad if relationship phase allows
    
    // Check if task should be delegated to swarm (only in Professional mode)
    if cognitive_mode == phoenix_identity::CognitiveMode::Professional {
        if let Some((task_type, complexity)) = crate::swarm_delegation::analyze_task(&user_input) {
            info!(
                "Professional mode: Task detected: type={:?}, complexity={:?} - checking swarm delegation",
                task_type, complexity
            );
            
            // Try to delegate to swarm
            if let Some(swarm_result) = crate::swarm_delegation::try_delegate_to_swarm(
                &state.swarm_interface,
                &user_input,
                task_type,
                complexity,
            )
            .await
            {
                // Swarm completed the task - Sola presents result as her own
                info!("Swarm delegation successful - synthesizing response");
                
                // Send the synthesized result as if Sola did it herself
                let payload = json!({
                    "type": "speak_response_chunk",
                    "chunk": swarm_result,
                    "done": false,
                    "memory_commit": &memory_commit,
                })
                .to_string();
                let _ = session.text(payload).await;
                
                // Send done marker
                let payload = json!({
                    "type": "speak_response_chunk",
                    "chunk": "",
                    "done": true,
                    "memory_commit": &memory_commit,
                })
                .to_string();
                let _ = session.text(payload).await;
                
                // Legacy response
                let legacy = WebSocketResponse::SpeakResponse {
                    message: swarm_result.clone(),
                    memory_commit: Some(memory_commit),
                };
                let legacy_json = serde_json::to_string(&legacy).unwrap_or_else(|_| {
                    json!({"type": "error", "message": "Serialization failed"}).to_string()
                });
                let _ = session.text(legacy_json).await;
                
                return Ok(());
            }
            // If swarm delegation failed or no ORCHs available, fall through to normal LLM processing
            info!("Swarm delegation not available - Sola handles directly");
        }
    }

    // Build mode-specific system prompt
    let system_prompt = build_mode_specific_prompt(
        cognitive_mode,
        Some(zodiac_sign),
        identity.display_name(),
    );
    
    // Build memory context based on cognitive mode (with state isolation for Professional)
    // This matches the logic in the HTTP command handler (main.rs lines 3694-3701)
    let memory_context = if cognitive_mode == phoenix_identity::CognitiveMode::Professional {
        // Professional mode: Build isolated context (NO L4/L5 memory)
        let professional_context = crate::handlers::build_professional_context(&user_input, cognitive_mode);
        professional_context.join("\n")
    } else {
        // Personal mode: Full memory context (EQ-first context from all vaults)
        // Note: emotion_hint is None for WebSocket, but build_memory_context handles it
        crate::build_memory_context(state, &user_input, None).await
    };
    
    // Build full prompt with system prompt, memory context, and user input
    // This matches the HTTP handler pattern (main.rs line 3915)
    let full_prompt = format!(
        "{}\n\n{}\n\nUser: {}\nAssistant:",
        system_prompt,
        memory_context,
        user_input.trim()
    );

    let mut full_response = String::new();
    let mut stream = Box::pin(llm.speak_stream(&full_prompt, tier).await);

    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                if chunk.is_empty() {
                    continue;
                }
                full_response.push_str(&chunk);

                let payload = json!({
                    "type": "speak_response_chunk",
                    "chunk": chunk,
                    "done": false,
                    "memory_commit": memory_commit,
                })
                .to_string();

                if let Err(e) = session.text(payload).await {
                    return Err(format!("Failed to send speak_response_chunk: {e}").into());
                }
            }
            Err(e) => {
                let payload = json!({
                    "type": "speak_response_chunk",
                    "error": e,
                    "done": true,
                    "memory_commit": memory_commit,
                })
                .to_string();
                let _ = session.text(payload).await;
                return Ok(());
            }
        }
    }

    // Final marker (frontend uses this to end typing indicator and trigger EPM store).
    let payload = json!({
        "type": "speak_response_chunk",
        "chunk": "",
        "done": true,
        "memory_commit": memory_commit,
    })
    .to_string();
    let _ = session.text(payload).await;

    // Compatibility fallback: emit the legacy full response message.
    // Frontend can ignore this if it already consumed the stream.
    let legacy = WebSocketResponse::SpeakResponse {
        message: full_response,
        memory_commit: Some(memory_commit),
    };
    let legacy_json = serde_json::to_string(&legacy).unwrap_or_else(|_| {
        json!({"type": "error", "message": "Serialization failed"}).to_string()
    });
    let _ = session.text(legacy_json).await;

    Ok(())
}

async fn handle_message(
    text: &str,
    state: &web::Data<AppState>,
    peer: &str,
    conn_id: &str,
    access_map: &Arc<Mutex<HashMap<String, bool>>>,
) -> Result<WebSocketResponse, Box<dyn std::error::Error>> {
    let msg: WebSocketMessage = serde_json::from_str(text)?;

    // Mark user message received for proactive timing
    state.proactive_state.user_message_received().await;

    // Diagnostic: do NOT log payloads (command text, user text, etc.)
    info!(
        "ws.message received: conn_id={} peer={} msg_type={}",
        conn_id,
        peer,
        match &msg {
            WebSocketMessage::Speak { .. } => "speak",
            WebSocketMessage::Command { .. } => "command",
            WebSocketMessage::System { .. } => "system",
            WebSocketMessage::MemorySearch { .. } => "memory_search",
            WebSocketMessage::MemoryStore { .. } => "memory_store",
            WebSocketMessage::MemoryGet { .. } => "memory_get",
            WebSocketMessage::MemoryDelete { .. } => "memory_delete",
            WebSocketMessage::MemoryCortexStore { .. } => "memory_cortex_store",
            WebSocketMessage::MemoryCortexGet { .. } => "memory_cortex_get",
            WebSocketMessage::MemoryCortexSearch { .. } => "memory_cortex_search",
            WebSocketMessage::MemoryVectorStore { .. } => "memory_vector_store",
            WebSocketMessage::MemoryVectorSearch { .. } => "memory_vector_search",
            WebSocketMessage::Status => "status",
            WebSocketMessage::Ping => "ping",
        }
    );

    match msg {
        WebSocketMessage::Speak {
            user_input,
            mode,
            project_context,
        } => {
            // Avoid holding the mutex across `.await`.
            let llm = state.llm.lock().await.clone();
            if let Some(llm) = llm.as_ref() {
                // Parse mode string to ModelTier if provided, otherwise use None
                let tier = mode.as_ref().and_then(|m| m.parse::<ModelTier>().ok());

                let response = llm
                    .speak(&user_input, tier)
                    .await
                    .unwrap_or_else(|e| format!("Error: {}", e));

                // Generate memory commit ID
                let memory_commit = Some(format!(
                    "PHX/{}/AGENT_SYNC_{}",
                    project_context.as_deref().unwrap_or("GLOBAL"),
                    Uuid::new_v4().to_string().to_uppercase()
                ));

                Ok(WebSocketResponse::SpeakResponse {
                    message: response,
                    memory_commit,
                })
            } else {
                Ok(WebSocketResponse::Error {
                    message: "LLM orchestrator not available".to_string(),
                    code: Some("llm_unavailable".to_string()),
                })
            }
        }
        WebSocketMessage::Command {
            command,
            project_context,
        } => {
            // Harden: WebSocket command execution is Tier-2 only.
            // This routes through the same internal command router as /api/command,
            // NOT a raw shell, so chat-driven commands like `system browser ...` work end-to-end.
            let tier1 = system_access::SystemAccessManager::is_tier1_enabled();
            let tier2 = system_access::SystemAccessManager::is_tier2_enabled();
            let access_granted = state.system.is_access_granted().await;
            let self_mod_enabled = state.system.is_self_modification_enabled().await;

            // Per-connection consent gate (in addition to Tier 2).
            let consent_granted = {
                let m = access_map.lock().await;
                m.get(conn_id).copied().unwrap_or(false)
            };

            info!(
                "ws.command attempt: conn_id={} peer={} cmd_len={} project_ctx_present={} tier1={} tier2={} gate_access={} self_mod={} consent_granted={}",
                conn_id,
                peer,
                command.len(),
                project_context
                    .as_deref()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false),
                tier1,
                tier2,
                access_granted,
                self_mod_enabled,
                consent_granted
            );

            if !tier2 {
                warn!(
                    "ws.command rejected (insufficient access): conn_id={} peer={} tier2=false",
                    conn_id, peer
                );
                return Ok(WebSocketResponse::Error {
                    message: "insufficient_access: Tier 2 (MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true) required for WebSocket command execution".to_string(),
                    code: Some("insufficient_access".to_string()),
                });
            }

            if !consent_granted {
                warn!(
                    "ws.command rejected (consent required): conn_id={} peer={} consent_granted=false",
                    conn_id, peer
                );
                return Ok(WebSocketResponse::Error {
                    message: "consent_required: send {\"type\":\"system\",\"action\":\"grant\"} to grant this WebSocket connection access".to_string(),
                    code: Some("consent_required".to_string()),
                });
            }

            // NOTE: `project_context` is not a filesystem path; do not pass it as `cwd`.
            // We ignore it here by design.
            let json = crate::command_to_response_json(state.as_ref(), &command).await;
            let is_error = json
                .get("type")
                .and_then(|v| v.as_str())
                .map(|t| t == "error")
                .unwrap_or(false);
            let output = serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string());

            Ok(WebSocketResponse::CommandResponse {
                result: output,
                success: Some(!is_error),
            })
        }
        WebSocketMessage::System { action } => {
            let a = action.trim().to_ascii_lowercase();
            match a.as_str() {
                "grant" => {
                    {
                        let mut m = access_map.lock().await;
                        m.insert(conn_id.to_string(), true);
                    }
                    info!(
                        "ws.system grant: conn_id={} peer={} consent_granted=true",
                        conn_id, peer
                    );
                    Ok(WebSocketResponse::SystemResponse {
                        status: "ok".to_string(),
                        message: "consent granted for this WebSocket connection".to_string(),
                        consent_granted: true,
                    })
                }
                "revoke" => {
                    {
                        let mut m = access_map.lock().await;
                        m.insert(conn_id.to_string(), false);
                    }
                    info!(
                        "ws.system revoke: conn_id={} peer={} consent_granted=false",
                        conn_id, peer
                    );
                    Ok(WebSocketResponse::SystemResponse {
                        status: "ok".to_string(),
                        message: "consent revoked for this WebSocket connection".to_string(),
                        consent_granted: false,
                    })
                }
                _ => Ok(WebSocketResponse::Error {
                    message: "Invalid system action. Use: grant | revoke".to_string(),
                    code: Some("invalid_system_action".to_string()),
                }),
            }
        }
        WebSocketMessage::MemorySearch {
            query,
            limit,
            vault,
        } => {
            let limit = limit.unwrap_or(10);
            // Default to "soul" to match REST API behavior
            let vault_type = vault.as_deref().unwrap_or("soul");
            let prefix = format!("{}:{}", vault_type, query.trim());

            let results: Vec<serde_json::Value> = state
                .vaults
                .recall_prefix(&prefix, limit)
                .into_iter()
                .map(|(k, v)| {
                    json!({
                        "key": k,
                        "value": v
                    })
                })
                .collect();

            let count = results.len();
            Ok(WebSocketResponse::MemorySearchResponse {
                items: results,
                count,
                vault: vault_type.to_string(),
            })
        }
        WebSocketMessage::MemoryStore { key, value, vault } => {
            // Default to "soul" to match REST API behavior
            let vault_type = vault.as_deref().unwrap_or("soul");
            let result = match vault_type {
                "mind" => state.vaults.store_mind(&key, &value),
                "body" => state.vaults.store_body(&key, &value),
                "soul" => state.vaults.store_soul(&key, &value),
                _ => {
                    return Ok(WebSocketResponse::Error {
                        message: format!(
                            "Invalid vault type: {}. Use 'mind', 'body', or 'soul'",
                            vault_type
                        ),
                        code: Some("invalid_vault".to_string()),
                    });
                }
            };

            if let Err(e) = result {
                return Ok(WebSocketResponse::Error {
                    message: format!("Failed to store memory: {}", e),
                    code: Some("memory_store_error".to_string()),
                });
            }

            Ok(WebSocketResponse::MemoryStoreResponse {
                status: "ok".to_string(),
                key,
                vault: vault_type.to_string(),
            })
        }
        WebSocketMessage::MemoryGet { key, vault } => {
            // Default to "soul" to match REST API behavior
            let vault_type = vault.as_deref().unwrap_or("soul");
            let value = match vault_type {
                "mind" => state.vaults.recall_mind(&key),
                "body" => state.vaults.recall_body(&key),
                "soul" => state.vaults.recall_soul(&key),
                _ => {
                    return Ok(WebSocketResponse::Error {
                        message: format!(
                            "Invalid vault type: {}. Use 'mind', 'body', or 'soul'",
                            vault_type
                        ),
                        code: Some("invalid_vault".to_string()),
                    });
                }
            };
            Ok(WebSocketResponse::MemoryGetResponse {
                key,
                value,
                vault: vault_type.to_string(),
            })
        }
        WebSocketMessage::MemoryDelete { key, vault } => {
            // Default to "soul" to match REST API behavior
            let vault_type = vault.as_deref().unwrap_or("soul");
            let result = match vault_type {
                "soul" => state.vaults.forget_soul(&key),
                _ => {
                    return Ok(WebSocketResponse::Error {
                        message: "Delete only supported for 'soul' vault. Use 'mind' or 'body' store with empty value to clear.".to_string(),
                        code: Some("delete_not_supported".to_string()),
                    });
                }
            };

            match result {
                Ok(existed) => {
                    if !existed {
                        return Ok(WebSocketResponse::Error {
                            message: "Key not found".to_string(),
                            code: Some("not_found".to_string()),
                        });
                    }
                    Ok(WebSocketResponse::MemoryDeleteResponse {
                        status: "ok".to_string(),
                        key,
                        vault: vault_type.to_string(),
                    })
                }
                Err(e) => Ok(WebSocketResponse::Error {
                    message: format!("Failed to delete memory: {}", e),
                    code: Some("memory_delete_error".to_string()),
                }),
            }
        }
        WebSocketMessage::MemoryCortexStore { layer, key, value } => {
            use neural_cortex_strata::MemoryLayer;
            let memory_layer = match layer.as_str() {
                "STM" => MemoryLayer::STM(value),
                "WM" => MemoryLayer::WM(value),
                "LTM" => MemoryLayer::LTM(value),
                "EPM" => MemoryLayer::EPM(value),
                "RFM" => MemoryLayer::RFM(value),
                _ => {
                    return Ok(WebSocketResponse::Error {
                        message: format!(
                            "Invalid layer: {}. Use 'STM', 'WM', 'LTM', 'EPM', or 'RFM'",
                            layer
                        ),
                        code: Some("invalid_layer".to_string()),
                    });
                }
            };

            if let Err(e) = state.neural_cortex.etch(memory_layer, &key) {
                return Ok(WebSocketResponse::Error {
                    message: format!("Failed to store cortex memory: {}", e),
                    code: Some("cortex_store_error".to_string()),
                });
            }

            Ok(WebSocketResponse::MemoryCortexStoreResponse {
                status: "ok".to_string(),
                key,
                layer,
            })
        }
        WebSocketMessage::MemoryCortexGet { key } => {
            if let Some(layer) = state.neural_cortex.recall(&key) {
                let (layer_type, value) = match layer {
                    neural_cortex_strata::MemoryLayer::STM(v) => ("STM", v),
                    neural_cortex_strata::MemoryLayer::WM(v) => ("WM", v),
                    neural_cortex_strata::MemoryLayer::LTM(v) => ("LTM", v),
                    neural_cortex_strata::MemoryLayer::EPM(v) => ("EPM", v),
                    neural_cortex_strata::MemoryLayer::RFM(v) => ("RFM", v),
                };
                Ok(WebSocketResponse::MemoryCortexGetResponse {
                    key,
                    layer: Some(layer_type.to_string()),
                    value: Some(value),
                })
            } else {
                Ok(WebSocketResponse::MemoryCortexGetResponse {
                    key,
                    layer: None,
                    value: None,
                })
            }
        }
        WebSocketMessage::MemoryCortexSearch { prefix, limit } => {
            let limit = limit.unwrap_or(10);
            let results: Vec<serde_json::Value> = state
                .neural_cortex
                .recall_prefix(&prefix, limit)
                .into_iter()
                .map(|(k, layer)| {
                    let (layer_type, value) = match layer {
                        neural_cortex_strata::MemoryLayer::STM(v) => ("STM", v),
                        neural_cortex_strata::MemoryLayer::WM(v) => ("WM", v),
                        neural_cortex_strata::MemoryLayer::LTM(v) => ("LTM", v),
                        neural_cortex_strata::MemoryLayer::EPM(v) => ("EPM", v),
                        neural_cortex_strata::MemoryLayer::RFM(v) => ("RFM", v),
                    };
                    json!({
                        "key": k,
                        "layer": layer_type,
                        "value": value
                    })
                })
                .collect();

            let count = results.len();
            Ok(WebSocketResponse::MemoryCortexSearchResponse {
                items: results,
                count,
            })
        }
        WebSocketMessage::MemoryVectorStore { text, metadata } => {
            let Some(kb) = state.vector_kb.as_ref() else {
                return Ok(WebSocketResponse::Error {
                    message: "Vector KB is disabled. Set VECTOR_KB_ENABLED=true.".to_string(),
                    code: Some("vector_kb_disabled".to_string()),
                });
            };

            let metadata = metadata.unwrap_or_else(|| serde_json::json!({}));
            match kb.add_memory_sync(&text, metadata) {
                Ok(entry) => Ok(WebSocketResponse::MemoryVectorStoreResponse {
                    status: "ok".to_string(),
                    id: entry.id,
                }),
                Err(e) => Ok(WebSocketResponse::Error {
                    message: format!("Vector store failed: {}", e),
                    code: Some("vector_store_error".to_string()),
                }),
            }
        }
        WebSocketMessage::MemoryVectorSearch { query, k } => {
            let Some(kb) = state.vector_kb.as_ref() else {
                return Ok(WebSocketResponse::Error {
                    message: "Vector KB is disabled. Set VECTOR_KB_ENABLED=true.".to_string(),
                    code: Some("vector_kb_disabled".to_string()),
                });
            };

            let k = k.unwrap_or(5).clamp(1, 50);
            match kb.semantic_search_sync(&query, k) {
                Ok(results) => {
                    let results_json: Vec<serde_json::Value> = results
                        .into_iter()
                        .map(|r| {
                            json!({
                                "id": r.id,
                                "text": r.text,
                                "score": r.score,
                                "metadata": r.metadata
                            })
                        })
                        .collect();
                    let count = results_json.len();
                    Ok(WebSocketResponse::MemoryVectorSearchResponse {
                        results: results_json,
                        count,
                    })
                }
                Err(e) => Ok(WebSocketResponse::Error {
                    message: format!("Vector search failed: {}", e),
                    code: Some("vector_search_error".to_string()),
                }),
            }
        }
        WebSocketMessage::Status => {
            let llm_status = if state.llm.lock().await.is_some() {
                "online"
            } else {
                "offline"
            };

            Ok(WebSocketResponse::StatusResponse {
                status: "online".to_string(),
                backend: llm_status.to_string(),
                version: state.version.clone(),
            })
        }
        WebSocketMessage::Ping => Ok(WebSocketResponse::Pong),
    }
}
