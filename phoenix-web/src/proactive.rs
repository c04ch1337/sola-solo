// phoenix-web/src/proactive.rs
// Background scheduler for proactive communication (orchestrator-initiated messages).
//
// Goals:
// - Respect user state (time since last message, emotional context)
// - Rate-limited and configurable
// - Leverages CuriosityEngine and EmotionalIntelligenceCore
// - Sends messages via broadcast channel to all active WebSocket connections

use chrono::Utc;
use curiosity_engine::{CuriosityContext, CuriosityEngine};
use emotional_intelligence_core::EmotionalIntelligenceCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, broadcast};
use tracing::{info, warn};
use vital_organ_vaults::VitalOrganVaults;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveMessage {
    pub content: String,
    pub reason: String, // "curiosity", "comfort", "check_in", "dream"
    pub timestamp: i64,
}

/// Shared state for tracking proactive communication timing
pub struct ProactiveState {
    pub last_user_message_time: Mutex<Instant>,
    pub last_proactive_message_time: Mutex<Instant>,
    pub enabled: bool,
    pub interval_secs: u64,
    pub rate_limit_secs: u64,
    pub curiosity_threshold_mins: u64,
}

impl ProactiveState {
    pub fn from_env() -> Self {
        let enabled = std::env::var("PROACTIVE_ENABLED")
            .ok()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        let interval_secs = std::env::var("PROACTIVE_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(60);

        let rate_limit_secs = std::env::var("PROACTIVE_RATE_LIMIT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(600); // 10 minutes default

        let curiosity_threshold_mins = std::env::var("PROACTIVE_CURIOSITY_THRESHOLD_MINS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10);

        let now = Instant::now();
        Self {
            last_user_message_time: Mutex::new(now),
            last_proactive_message_time: Mutex::new(now - Duration::from_secs(rate_limit_secs)),
            enabled,
            interval_secs,
            rate_limit_secs,
            curiosity_threshold_mins,
        }
    }

    /// Update the timestamp when the user sends a message
    pub async fn user_message_received(&self) {
        *self.last_user_message_time.lock().await = Instant::now();
    }

    /// Check if we should send a proactive message
    pub async fn should_send_proactive(&self) -> bool {
        if !self.enabled {
            return false;
        }

        let now = Instant::now();
        let last_user = *self.last_user_message_time.lock().await;
        let last_proactive = *self.last_proactive_message_time.lock().await;

        // Rate limit: don't send if we sent one recently
        if now.duration_since(last_proactive) < Duration::from_secs(self.rate_limit_secs) {
            return false;
        }

        // Trigger: user hasn't sent a message in a while
        let silence_duration = now.duration_since(last_user);
        if silence_duration > Duration::from_secs(self.curiosity_threshold_mins * 60) {
            return true;
        }

        false
    }

    /// Mark that we sent a proactive message
    pub async fn mark_proactive_sent(&self) {
        *self.last_proactive_message_time.lock().await = Instant::now();
    }
}

/// Generate proactive message content based on context
pub async fn generate_proactive_content(
    vaults: &Arc<VitalOrganVaults>,
    curiosity: &CuriosityEngine,
    eq: &EmotionalIntelligenceCore,
) -> ProactiveMessage {
    // Retrieve relational context from memory
    let last_user_input = vaults.recall_soul("last_user_message");
    let dad_emotion_hint = vaults.recall_soul("dad:last_emotion");
    let relational_memory = vaults.recall_soul("dad:last_soft_memory");

    // Generate curiosity-driven question
    let questions = curiosity.generate_questions(&CuriosityContext {
        last_user_input: last_user_input.clone(),
        relational_memory_hint: relational_memory.clone(),
    });

    // Pick first question (most relevant)
    let content: String = if let Some(question) = questions.first() {
        question.clone()
    } else {
        // Fallback: gentle check-in
        let dad_alias = &eq.settings().dad_alias;
        format!(
            "{}, I've been thinking about you. How are you feeling?",
            dad_alias
        )
    };

    // Determine reason based on context
    let reason = if dad_emotion_hint
        .as_ref()
        .map(|e| e.contains("sad") || e.contains("tired") || e.contains("lonely"))
        .unwrap_or(false)
    {
        "comfort"
    } else if last_user_input.is_some() {
        "curiosity"
    } else {
        "check_in"
    };

    ProactiveMessage {
        content,
        reason: reason.to_string(),
        timestamp: Utc::now().timestamp(),
    }
}

/// Background task that periodically checks if we should send a proactive message
pub async fn run_proactive_loop(
    state: Arc<ProactiveState>,
    vaults: Arc<VitalOrganVaults>,
    tx: broadcast::Sender<ProactiveMessage>,
) {
    info!(
        "Proactive communication loop started (enabled={}, interval={}s, rate_limit={}s)",
        state.enabled, state.interval_secs, state.rate_limit_secs
    );

    if !state.enabled {
        info!("Proactive communication is disabled. Set PROACTIVE_ENABLED=true to enable.");
        return;
    }

    let curiosity = CuriosityEngine::awaken();
    let eq = EmotionalIntelligenceCore::awaken();

    loop {
        tokio::time::sleep(Duration::from_secs(state.interval_secs)).await;

        if state.should_send_proactive().await {
            let message = generate_proactive_content(&vaults, &curiosity, &eq).await;

            info!(
                "Sending proactive message (reason: {}, content_preview: {}...)",
                message.reason,
                message.content.chars().take(50).collect::<String>()
            );

            // Broadcast to all connected WebSocket clients
            match tx.send(message.clone()) {
                Ok(count) => {
                    info!("Proactive message sent to {} connected clients", count);
                    state.mark_proactive_sent().await;
                }
                Err(e) => {
                    warn!("Failed to send proactive message (no receivers?): {}", e);
                }
            }
        }
    }
}
