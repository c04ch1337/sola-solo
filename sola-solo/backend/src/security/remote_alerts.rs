//! Remote alert system via Telegram.
//!
//! This module provides functionality to send security alerts to a Telegram
//! chat, including photo attachments of detected intruders.
//!
//! ## Configuration
//!
//! Set the following environment variables:
//! - `TELEGRAM_BOT_TOKEN`: Your Telegram bot token from @BotFather
//! - `TELEGRAM_CHAT_ID`: The chat ID to send alerts to
//!
//! ## Retry Mechanism
//!
//! All Telegram sends use exponential backoff retry:
//! - 3 retry attempts
//! - Initial delay: 1 second
//! - Backoff multiplier: 2x
//! - Falls back to Windows notification on failure
//!
//! ## Usage
//!
//! ```rust,ignore
//! use security::remote_alerts::{send_intruder_alert, TelegramConfig};
//!
//! let config = TelegramConfig {
//!     bot_token: "your_bot_token".to_string(),
//!     chat_id: 123456789,
//! };
//!
//! send_intruder_alert(&config, Some(jpeg_data)).await?;
//! ```

use chrono::Local;
use std::time::Duration;

/// Configuration for Telegram alerts.
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    /// Telegram bot token from @BotFather
    pub bot_token: String,
    /// Chat ID to send alerts to
    pub chat_id: i64,
}

impl TelegramConfig {
    /// Create a new TelegramConfig from environment variables.
    ///
    /// Returns None if the required environment variables are not set.
    pub fn from_env() -> Option<Self> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").ok()?;
        let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID").ok()?.parse().ok()?;
        
        Some(Self { bot_token, chat_id })
    }
}

/// Retry configuration for Telegram sends.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay between retries (in milliseconds)
    pub initial_delay_ms: u64,
    /// Backoff multiplier (delay is multiplied by this after each retry)
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Send a Windows notification as fallback when Telegram fails.
#[cfg(all(windows, feature = "notifications"))]
fn send_fallback_notification(title: &str, message: &str) {
    use winrt_notification::{Duration, Sound, Toast};
    
    let _ = Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(message)
        .text2("(Telegram alert failed - this is a local fallback)")
        .sound(Some(Sound::Default))
        .duration(Duration::Long)
        .show();
    
    eprintln!("[remote_alerts] Fallback Windows notification sent");
}

/// Send a Windows notification as fallback (stub for non-Windows).
#[cfg(not(all(windows, feature = "notifications")))]
fn send_fallback_notification(title: &str, message: &str) {
    eprintln!("[remote_alerts] Fallback notification (no Windows support): {} - {}", title, message);
}

/// Send an intruder alert to Telegram with exponential backoff retry.
///
/// This function sends a message with an optional photo attachment to the
/// configured Telegram chat. If sending fails, it will retry up to 3 times
/// with exponential backoff, then fall back to a local Windows notification.
///
/// # Arguments
///
/// * `config` - Telegram configuration
/// * `snapshot` - Optional JPEG image data of the intruder
///
/// # Returns
///
/// Returns Ok(()) on success, or an error message on failure.
#[cfg(feature = "telegram")]
pub async fn send_intruder_alert(
    config: &TelegramConfig,
    snapshot: Option<Vec<u8>>,
) -> Result<(), String> {
    let retry_config = RetryConfig::default();
    send_intruder_alert_with_retry(config, snapshot, &retry_config).await
}

/// Send an intruder alert with custom retry configuration.
#[cfg(feature = "telegram")]
pub async fn send_intruder_alert_with_retry(
    config: &TelegramConfig,
    snapshot: Option<Vec<u8>>,
    retry_config: &RetryConfig,
) -> Result<(), String> {
    use teloxide::prelude::*;
    use teloxide::types::InputFile;

    let bot = Bot::new(&config.bot_token);
    let chat_id = ChatId(config.chat_id);
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let message = format!(
        "🚨 SECURITY ALERT: Unknown face detected at {}\n\n\
        Sola has detected an unrecognized person in front of your PC.\n\
        Security level has been elevated to ALERT.\n\
        Critical tools are now locked.",
        timestamp
    );

    // Retry loop for text message
    let mut last_error = String::new();
    let mut delay_ms = retry_config.initial_delay_ms;
    
    for attempt in 0..=retry_config.max_retries {
        if attempt > 0 {
            eprintln!(
                "[remote_alerts] Telegram send failed, retrying in {}ms (attempt {}/{})",
                delay_ms, attempt, retry_config.max_retries
            );
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            delay_ms = (delay_ms as f64 * retry_config.backoff_multiplier) as u64;
        }
        
        match bot.send_message(chat_id, &message).await {
            Ok(_) => {
                eprintln!("[remote_alerts] Telegram message sent successfully");
                
                // Send the photo if available (with retry)
                if let Some(ref jpeg_data) = snapshot {
                    let photo = InputFile::memory(jpeg_data.clone()).file_name("intruder.jpg");
                    let caption = format!("📸 Snapshot captured at {}", timestamp);
                    
                    // Reset delay for photo retry
                    let mut photo_delay_ms = retry_config.initial_delay_ms;
                    
                    for photo_attempt in 0..=retry_config.max_retries {
                        if photo_attempt > 0 {
                            eprintln!(
                                "[remote_alerts] Photo send failed, retrying in {}ms (attempt {}/{})",
                                photo_delay_ms, photo_attempt, retry_config.max_retries
                            );
                            tokio::time::sleep(Duration::from_millis(photo_delay_ms)).await;
                            photo_delay_ms = (photo_delay_ms as f64 * retry_config.backoff_multiplier) as u64;
                        }
                        
                        let photo = InputFile::memory(jpeg_data.clone()).file_name("intruder.jpg");
                        match bot.send_photo(chat_id, photo).caption(&caption).await {
                            Ok(_) => {
                                eprintln!("[remote_alerts] Telegram photo sent successfully");
                                return Ok(());
                            }
                            Err(e) => {
                                last_error = format!("Failed to send photo: {}", e);
                            }
                        }
                    }
                    
                    // Photo failed but message succeeded - partial success
                    eprintln!("[remote_alerts] Photo send failed after retries, but message was sent");
                    return Ok(());
                }
                
                return Ok(());
            }
            Err(e) => {
                last_error = format!("Failed to send message: {}", e);
            }
        }
    }
    
    // All retries failed - fall back to Windows notification
    eprintln!(
        "[remote_alerts] Telegram send failed after {} retries: {}",
        retry_config.max_retries, last_error
    );
    
    send_fallback_notification(
        "🚨 SECURITY ALERT",
        "Unknown face detected! Telegram alert failed - check your connection.",
    );
    
    Err(format!(
        "Telegram send failed after {} retries (fallback notification sent): {}",
        retry_config.max_retries, last_error
    ))
}

/// Send an intruder alert (stub when telegram feature is disabled).
#[cfg(not(feature = "telegram"))]
pub async fn send_intruder_alert(
    _config: &TelegramConfig,
    _snapshot: Option<Vec<u8>>,
) -> Result<(), String> {
    eprintln!("[remote_alerts] Telegram feature not enabled, alert not sent");
    Ok(())
}

/// Send a security status update to Telegram.
///
/// This can be used to send periodic status updates or when security
/// level changes.
#[cfg(feature = "telegram")]
pub async fn send_security_status(
    config: &TelegramConfig,
    level: u8,
    message: &str,
) -> Result<(), String> {
    use teloxide::prelude::*;

    let bot = Bot::new(&config.bot_token);
    let chat_id = ChatId(config.chat_id);
    
    let emoji = match level {
        0 => "✅",
        1 => "⚠️",
        2 => "🚨",
        _ => "❓",
    };
    
    let level_name = match level {
        0 => "SECURE",
        1 => "WARNING",
        2 => "ALERT",
        _ => "UNKNOWN",
    };
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let full_message = format!(
        "{} Security Level: {} ({})\n\n{}\n\nTimestamp: {}",
        emoji, level_name, level, message, timestamp
    );

    if let Err(e) = bot.send_message(chat_id, &full_message).await {
        return Err(format!("Failed to send Telegram status: {}", e));
    }

    Ok(())
}

/// Send a security status update (stub when telegram feature is disabled).
#[cfg(not(feature = "telegram"))]
pub async fn send_security_status(
    _config: &TelegramConfig,
    _level: u8,
    _message: &str,
) -> Result<(), String> {
    eprintln!("[remote_alerts] Telegram feature not enabled, status not sent");
    Ok(())
}

/// Send a workstation lock notification to Telegram.
///
/// # Arguments
///
/// * `config` - Telegram configuration
/// * `reason` - The reason for the lock (e.g., "auto_lock", "manual_request")
#[cfg(feature = "telegram")]
pub async fn send_lock_notification(config: &TelegramConfig, reason: &str) -> Result<(), String> {
    use teloxide::prelude::*;

    let bot = Bot::new(&config.bot_token);
    let chat_id = ChatId(config.chat_id);
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let reason_text = match reason {
        "auto_lock" => "Sola has automatically locked your workstation due to sustained security alert.\n\
            An unknown person was detected for more than 60 seconds without intervention.",
        "manual_request" => "Workstation was locked via manual API request.",
        _ => reason,
    };
    
    let message = format!(
        "🔒 WORKSTATION LOCKED\n\n\
        {}\n\n\
        Timestamp: {}",
        reason_text, timestamp
    );

    if let Err(e) = bot.send_message(chat_id, &message).await {
        return Err(format!("Failed to send lock notification: {}", e));
    }

    Ok(())
}

/// Send a workstation lock notification (stub when telegram feature is disabled).
#[cfg(not(feature = "telegram"))]
pub async fn send_lock_notification(_config: &TelegramConfig, _reason: &str) -> Result<(), String> {
    eprintln!("[remote_alerts] Telegram feature not enabled, lock notification not sent");
    Ok(())
}

/// Send a test message to verify Telegram configuration.
#[cfg(feature = "telegram")]
pub async fn send_test_message(config: &TelegramConfig) -> Result<(), String> {
    use teloxide::prelude::*;

    let bot = Bot::new(&config.bot_token);
    let chat_id = ChatId(config.chat_id);
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let message = format!(
        "🤖 Sola Security System Test\n\n\
        This is a test message to verify your Telegram alert configuration.\n\
        If you receive this message, your security alerts are working correctly.\n\n\
        Timestamp: {}",
        timestamp
    );

    if let Err(e) = bot.send_message(chat_id, &message).await {
        return Err(format!("Failed to send test message: {}", e));
    }

    Ok(())
}

/// Send a test message (stub when telegram feature is disabled).
#[cfg(not(feature = "telegram"))]
pub async fn send_test_message(_config: &TelegramConfig) -> Result<(), String> {
    Err("Telegram feature not enabled".to_string())
}
