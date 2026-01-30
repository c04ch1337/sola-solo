//! Security module for Sola's threat detection and response.
//!
//! This module provides:
//! - Remote alerts via Telegram
//! - Windows workstation locking
//! - Auto-lock policy enforcement
//! - Intruder detection and response

pub mod remote_alerts;
pub mod system_lock;

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

pub use remote_alerts::{send_intruder_alert, TelegramConfig};
pub use system_lock::lock_workstation;

/// Configuration for the security module.
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Telegram bot token (from TELEGRAM_BOT_TOKEN env var)
    pub telegram_token: Option<String>,
    /// Telegram chat ID to send alerts to (from TELEGRAM_CHAT_ID env var)
    pub telegram_chat_id: Option<i64>,
    /// Auto-lock timeout in seconds (default: 60)
    pub auto_lock_timeout_secs: u64,
    /// Whether to enable auto-lock on sustained alert
    pub auto_lock_enabled: bool,
    /// Grace period before lock (audio warning) in seconds (default: 10)
    pub grace_period_secs: u64,
    /// Whether to play audio warning during grace period
    pub audio_warning_enabled: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            telegram_token: std::env::var("TELEGRAM_BOT_TOKEN").ok(),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            auto_lock_timeout_secs: 60,
            auto_lock_enabled: true,
            grace_period_secs: 10,
            audio_warning_enabled: true,
        }
    }
}

impl SecurityConfig {
    /// Check if Telegram alerts are configured.
    pub fn telegram_enabled(&self) -> bool {
        self.telegram_token.is_some() && self.telegram_chat_id.is_some()
    }
}

/// Security response coordinator.
///
/// This struct coordinates security responses including:
/// - Sending Telegram alerts
/// - Triggering workstation lock
/// - Managing auto-lock timers
/// - Audio warning during grace period
pub struct SecurityCoordinator {
    config: SecurityConfig,
    /// Timestamp when alert level 2 was first entered
    alert_started_at: Arc<RwLock<Option<u64>>>,
    /// Whether auto-lock has been triggered for current alert
    auto_lock_triggered: Arc<RwLock<bool>>,
    /// Whether the audio warning has been issued for current alert
    warning_issued: Arc<RwLock<bool>>,
}

/// Play a system beep as audio warning.
#[cfg(windows)]
fn play_warning_beep() {
    use std::process::Command;
    
    // Use PowerShell to play a beep sound
    let _ = Command::new("powershell")
        .args(["-Command", "[console]::beep(1000, 500)"])
        .spawn();
    
    eprintln!("[security] Audio warning beep played");
}

/// Play a system beep as audio warning (non-Windows stub).
#[cfg(not(windows))]
fn play_warning_beep() {
    // On non-Windows, just print a message
    eprintln!("[security] Audio warning (beep not available on this platform)");
}

impl SecurityCoordinator {
    /// Create a new security coordinator with default configuration.
    pub fn new() -> Self {
        Self {
            config: SecurityConfig::default(),
            alert_started_at: Arc::new(RwLock::new(None)),
            auto_lock_triggered: Arc::new(RwLock::new(false)),
            warning_issued: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new security coordinator with custom configuration.
    pub fn with_config(config: SecurityConfig) -> Self {
        Self {
            config,
            alert_started_at: Arc::new(RwLock::new(None)),
            warning_issued: Arc::new(RwLock::new(false)),
            auto_lock_triggered: Arc::new(RwLock::new(false)),
        }
    }

    /// Handle an unknown face detection event.
    ///
    /// This will:
    /// 1. Send a Telegram alert with the snapshot (if configured)
    /// 2. Start the auto-lock timer
    pub async fn handle_unknown_face(&self, snapshot: Option<Vec<u8>>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Record when alert started
        {
            let mut started = self.alert_started_at.write().await;
            if started.is_none() {
                *started = Some(now);
                eprintln!("[security] Alert started at timestamp: {}", now);
            }
        }

        // Reset auto-lock trigger and warning flags
        {
            let mut triggered = self.auto_lock_triggered.write().await;
            let mut warning = self.warning_issued.write().await;
            *triggered = false;
            *warning = false;
        }

        // Send Telegram alert if configured
        if self.config.telegram_enabled() {
            if let (Some(token), Some(chat_id)) = (&self.config.telegram_token, self.config.telegram_chat_id) {
                let config = TelegramConfig {
                    bot_token: token.clone(),
                    chat_id,
                };
                
                if let Err(e) = send_intruder_alert(&config, snapshot).await {
                    eprintln!("[security] Failed to send Telegram alert: {}", e);
                } else {
                    eprintln!("[security] Telegram alert sent successfully");
                }
            }
        }
    }

    /// Check if auto-lock should be triggered.
    ///
    /// Returns true if:
    /// - Auto-lock is enabled
    /// - Alert level has been at 2 for longer than the timeout
    /// - Auto-lock hasn't already been triggered for this alert
    pub async fn should_auto_lock(&self) -> bool {
        if !self.config.auto_lock_enabled {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let started = self.alert_started_at.read().await;
        let triggered = self.auto_lock_triggered.read().await;

        if *triggered {
            return false;
        }

        if let Some(start_time) = *started {
            let elapsed = now - start_time;
            return elapsed >= self.config.auto_lock_timeout_secs;
        }

        false
    }

    /// Check if we're in the grace period (warning issued but lock not yet triggered).
    pub async fn is_in_grace_period(&self) -> bool {
        let warning = self.warning_issued.read().await;
        let triggered = self.auto_lock_triggered.read().await;
        *warning && !*triggered
    }

    /// Check if the grace period has elapsed and lock should be triggered.
    async fn should_lock_after_grace(&self) -> bool {
        if !self.config.auto_lock_enabled {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let started = self.alert_started_at.read().await;
        let triggered = self.auto_lock_triggered.read().await;
        let warning = self.warning_issued.read().await;

        if *triggered {
            return false;
        }

        // Must have warning issued first
        if !*warning {
            return false;
        }

        if let Some(start_time) = *started {
            let elapsed = now - start_time;
            // Lock after timeout + grace period
            let total_timeout = self.config.auto_lock_timeout_secs + self.config.grace_period_secs;
            return elapsed >= total_timeout;
        }

        false
    }

    /// Trigger auto-lock if conditions are met.
    ///
    /// This function implements a two-phase lock:
    /// 1. After `auto_lock_timeout_secs`: Issue audio warning, enter grace period
    /// 2. After `grace_period_secs` more: Actually lock the workstation
    ///
    /// Returns true if lock was triggered.
    pub async fn trigger_auto_lock_if_needed(&self) -> bool {
        // Phase 1: Check if we should issue warning
        if self.should_auto_lock().await {
            let warning = self.warning_issued.read().await;
            if !*warning {
                drop(warning);
                
                // Issue warning
                {
                    let mut warning = self.warning_issued.write().await;
                    *warning = true;
                }
                
                eprintln!(
                    "[security] Auto-lock warning! Workstation will lock in {} seconds unless identity is verified.",
                    self.config.grace_period_secs
                );
                
                // Play audio warning if enabled
                if self.config.audio_warning_enabled {
                    play_warning_beep();
                    
                    // Play multiple beeps for urgency
                    tokio::spawn(async {
                        for _ in 0..3 {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            play_warning_beep();
                        }
                    });
                }
                
                // Send Telegram warning
                self.notify_lock_warning().await;
                
                return false; // Don't lock yet, just warned
            }
        }

        // Phase 2: Check if grace period has elapsed
        if !self.should_lock_after_grace().await {
            return false;
        }

        // Mark as triggered
        {
            let mut triggered = self.auto_lock_triggered.write().await;
            *triggered = true;
        }

        let total_time = self.config.auto_lock_timeout_secs + self.config.grace_period_secs;
        eprintln!(
            "[security] Auto-lock triggered after {} seconds ({} timeout + {} grace period)",
            total_time, self.config.auto_lock_timeout_secs, self.config.grace_period_secs
        );

        // Lock the workstation
        match lock_workstation() {
            Ok(_) => {
                eprintln!("[security] Workstation locked successfully");
                
                // Send Telegram notification
                self.notify_lock_triggered("auto_lock").await;
                
                true
            }
            Err(e) => {
                eprintln!("[security] Failed to lock workstation: {}", e);
                false
            }
        }
    }

    /// Send a warning notification that lock is imminent.
    async fn notify_lock_warning(&self) {
        if !self.config.telegram_enabled() {
            return;
        }

        let config = TelegramConfig {
            bot_token: self.config.telegram_token.clone().unwrap(),
            chat_id: self.config.telegram_chat_id.unwrap(),
        };

        let message = format!(
            "⚠️ LOCK WARNING\n\n\
            Workstation will be locked in {} seconds unless you verify your identity.\n\
            An unknown person has been detected for {} seconds.",
            self.config.grace_period_secs,
            self.config.auto_lock_timeout_secs
        );

        if let Err(e) = remote_alerts::send_security_status(&config, 2, &message).await {
            eprintln!("[security] Failed to send lock warning: {}", e);
        }
    }

    /// Reset the alert state (called when security level drops below 2).
    pub async fn reset_alert_state(&self) {
        let mut started = self.alert_started_at.write().await;
        let mut triggered = self.auto_lock_triggered.write().await;
        let mut warning = self.warning_issued.write().await;
        
        *started = None;
        *triggered = false;
        *warning = false;
        
        eprintln!("[security] Alert state reset");
    }

    /// Get the current alert duration in seconds (if in alert state).
    pub async fn get_alert_duration(&self) -> Option<u64> {
        let started = self.alert_started_at.read().await;
        
        if let Some(start_time) = *started {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            Some(now - start_time)
        } else {
            None
        }
    }

    /// Check if Telegram is enabled.
    pub fn telegram_enabled(&self) -> bool {
        self.config.telegram_enabled()
    }

    /// Check if Telegram chat ID is configured.
    pub fn telegram_chat_configured(&self) -> bool {
        self.config.telegram_chat_id.is_some()
    }

    /// Send a test message to Telegram.
    pub async fn send_test_message(&self) -> Result<(), String> {
        if !self.config.telegram_enabled() {
            return Err("Telegram not configured".to_string());
        }

        let config = TelegramConfig {
            bot_token: self.config.telegram_token.clone().unwrap(),
            chat_id: self.config.telegram_chat_id.unwrap(),
        };

        remote_alerts::send_test_message(&config).await
    }

    /// Notify that workstation lock was triggered.
    pub async fn notify_lock_triggered(&self, reason: &str) {
        if !self.config.telegram_enabled() {
            return;
        }

        let config = TelegramConfig {
            bot_token: self.config.telegram_token.clone().unwrap(),
            chat_id: self.config.telegram_chat_id.unwrap(),
        };

        if let Err(e) = remote_alerts::send_lock_notification(&config, reason).await {
            eprintln!("[security] Failed to send lock notification: {}", e);
        }
    }
}

impl Default for SecurityCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
