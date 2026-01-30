//! Windows workstation locking functionality.
//!
//! This module provides the ability to lock the Windows workstation
//! as a security response to detected threats.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use security::system_lock::lock_workstation;
//!
//! if let Err(e) = lock_workstation() {
//!     eprintln!("Failed to lock workstation: {}", e);
//! }
//! ```

/// Lock the Windows workstation.
///
/// This function calls the Windows API `LockWorkStation()` to immediately
/// lock the computer, requiring the user to re-authenticate.
///
/// # Returns
///
/// Returns Ok(()) on success, or an error message on failure.
///
/// # Platform Support
///
/// This function only works on Windows. On other platforms, it returns
/// an error indicating the platform is not supported.
#[cfg(all(windows, feature = "system-lock"))]
pub fn lock_workstation() -> Result<(), String> {
    use windows::Win32::System::Shutdown::LockWorkStation;

    unsafe {
        if LockWorkStation().is_ok() {
            eprintln!("[system_lock] Workstation locked successfully");
            Ok(())
        } else {
            let error = std::io::Error::last_os_error();
            Err(format!("LockWorkStation failed: {}", error))
        }
    }
}

/// Lock the workstation (stub for non-Windows platforms).
#[cfg(not(all(windows, feature = "system-lock")))]
pub fn lock_workstation() -> Result<(), String> {
    #[cfg(not(windows))]
    {
        Err("Workstation locking is only supported on Windows".to_string())
    }
    
    #[cfg(windows)]
    {
        Err("system-lock feature not enabled".to_string())
    }
}

/// Check if workstation locking is available on this platform.
pub fn is_lock_available() -> bool {
    #[cfg(all(windows, feature = "system-lock"))]
    {
        true
    }
    
    #[cfg(not(all(windows, feature = "system-lock")))]
    {
        false
    }
}

/// Lock workstation with a delay.
///
/// This function waits for the specified duration before locking,
/// allowing time for any cleanup or notification.
#[cfg(all(windows, feature = "system-lock"))]
pub async fn lock_workstation_delayed(delay_secs: u64) -> Result<(), String> {
    tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
    lock_workstation()
}

/// Lock workstation with delay (stub for non-Windows platforms).
#[cfg(not(all(windows, feature = "system-lock")))]
pub async fn lock_workstation_delayed(_delay_secs: u64) -> Result<(), String> {
    lock_workstation()
}

/// Attempt to lock the workstation and send a Telegram notification.
///
/// This is a convenience function that combines locking with notification.
///
/// # Arguments
///
/// * `telegram_config` - Optional Telegram configuration for sending notification
/// * `reason` - The reason for the lock (e.g., "auto_lock", "manual_request")
pub async fn lock_and_notify(
    telegram_config: Option<&super::remote_alerts::TelegramConfig>,
    reason: &str,
) -> Result<(), String> {
    // Send notification first (before lock)
    if let Some(config) = telegram_config {
        if let Err(e) = super::remote_alerts::send_lock_notification(config, reason).await {
            eprintln!("[system_lock] Failed to send lock notification: {}", e);
            // Continue with lock even if notification fails
        }
    }

    // Lock the workstation
    lock_workstation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lock_available() {
        // This test just verifies the function compiles and runs
        let available = is_lock_available();
        println!("Lock available: {}", available);
    }
}
