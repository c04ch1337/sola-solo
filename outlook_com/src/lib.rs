//! Outlook COM Automation for Phoenix AGI
//!
//! Windows-only module that provides direct access to Outlook via COM automation.
//! This allows Phoenix to:
//! - Read all Outlook folders (including subfolders)
//! - Send/receive emails
//! - Access contacts, calendar, tasks
//! - Parse email bodies, attachments
//! - Set rules, categories, flags
//! - Works with cached Exchange/365 data
//!
//! **Platform**: Windows only
//! **Requirements**: Outlook 2010-2021/O365 installed and configured

#[cfg(windows)]
mod windows_impl;

#[cfg(not(windows))]
mod stub_impl;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Email message from Outlook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookEmail {
    pub entry_id: String,
    pub subject: String,
    pub from: String,
    pub to: String,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub body: String,
    pub body_html: Option<String>,
    pub received_time: Option<String>,
    pub sent_time: Option<String>,
    pub importance: String, // "Low", "Normal", "High"
    pub is_read: bool,
    pub has_attachments: bool,
    pub attachments: Vec<OutlookAttachment>,
    pub categories: Vec<String>,
}

/// Email attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookAttachment {
    pub name: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub file_path: Option<String>, // If saved to temp file
}

/// Outlook contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookContact {
    pub entry_id: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email_addresses: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
}

/// Outlook calendar appointment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookAppointment {
    pub entry_id: String,
    pub subject: String,
    pub start_time: String,
    pub end_time: String,
    pub location: Option<String>,
    pub body: Option<String>,
    pub organizer: Option<String>,
    pub required_attendees: Vec<String>,
    pub optional_attendees: Vec<String>,
    pub is_all_day: bool,
    pub reminder_minutes: Option<u32>,
}

/// Outlook folder information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookFolder {
    pub name: String,
    pub entry_id: String,
    pub item_count: u32,
    pub unread_count: u32,
    pub subfolders: Vec<OutlookFolder>,
}

/// Error types for Outlook COM operations
#[derive(Debug, Error)]
pub enum OutlookError {
    #[error("Outlook COM initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Outlook not installed or not accessible: {0}")]
    OutlookNotAvailable(String),

    #[error("Folder not found: {0}")]
    FolderNotFound(String),

    #[error("Email not found: {0}")]
    EmailNotFound(String),

    #[error("COM operation failed: {0}")]
    ComOperationFailed(String),

    #[error("Platform not supported: Outlook COM is Windows-only")]
    PlatformNotSupported,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

/// Main Outlook COM manager
pub struct OutlookComManager {
    #[cfg(windows)]
    inner: windows_impl::OutlookComManagerImpl,

    #[cfg(not(windows))]
    _phantom: std::marker::PhantomData<()>,
}

impl OutlookComManager {
    /// Create a new Outlook COM manager
    ///
    /// This will attempt to connect to a running Outlook instance.
    /// If Outlook is not running, it will attempt to start it.
    pub fn new() -> Result<Self, OutlookError> {
        #[cfg(windows)]
        {
            Ok(Self {
                inner: windows_impl::OutlookComManagerImpl::new()?,
            })
        }

        #[cfg(not(windows))]
        {
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Check if Outlook is available and accessible
    pub fn is_available(&self) -> bool {
        #[cfg(windows)]
        {
            self.inner.is_available()
        }

        #[cfg(not(windows))]
        {
            false
        }
    }

    /// Get list of all folders (including subfolders)
    pub async fn list_folders(&self) -> Result<Vec<OutlookFolder>, OutlookError> {
        #[cfg(windows)]
        {
            self.inner.list_folders().await
        }

        #[cfg(not(windows))]
        {
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Get emails from a specific folder
    ///
    /// `folder_name` can be:
    /// - "Inbox" (default)
    /// - "Sent Items"
    /// - "Drafts"
    /// - "Deleted Items"
    /// - Or any custom folder name
    pub async fn get_emails(
        &self,
        folder_name: &str,
        max_count: Option<usize>,
    ) -> Result<Vec<OutlookEmail>, OutlookError> {
        #[cfg(windows)]
        {
            self.inner.get_emails(folder_name, max_count).await
        }

        #[cfg(not(windows))]
        {
            let _ = (folder_name, max_count);
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Send an email via Outlook
    #[allow(clippy::too_many_arguments)]
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        html_body: Option<&str>,
        cc: Option<&str>,
        bcc: Option<&str>,
        attachments: Option<Vec<&str>>, // File paths
    ) -> Result<(), OutlookError> {
        #[cfg(windows)]
        {
            self.inner
                .send_email(to, subject, body, html_body, cc, bcc, attachments)
                .await
        }

        #[cfg(not(windows))]
        {
            let _ = (to, subject, body, html_body, cc, bcc, attachments);
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Get all contacts
    pub async fn get_contacts(&self) -> Result<Vec<OutlookContact>, OutlookError> {
        #[cfg(windows)]
        {
            self.inner.get_contacts().await
        }

        #[cfg(not(windows))]
        {
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Get calendar appointments in a date range
    pub async fn get_appointments(
        &self,
        start_date: Option<&str>, // ISO 8601 format
        end_date: Option<&str>,   // ISO 8601 format
    ) -> Result<Vec<OutlookAppointment>, OutlookError> {
        #[cfg(windows)]
        {
            self.inner.get_appointments(start_date, end_date).await
        }

        #[cfg(not(windows))]
        {
            let _ = (start_date, end_date);
            Err(OutlookError::PlatformNotSupported)
        }
    }

    /// Create a new calendar appointment
    #[allow(clippy::too_many_arguments)]
    pub async fn create_appointment(
        &self,
        subject: &str,
        start_time: &str, // ISO 8601 format
        end_time: &str,   // ISO 8601 format
        location: Option<&str>,
        body: Option<&str>,
        required_attendees: Option<Vec<&str>>,
        optional_attendees: Option<Vec<&str>>,
        reminder_minutes: Option<u32>,
    ) -> Result<String, OutlookError> {
        #[cfg(windows)]
        {
            self.inner
                .create_appointment(
                    subject,
                    start_time,
                    end_time,
                    location,
                    body,
                    required_attendees,
                    optional_attendees,
                    reminder_minutes,
                )
                .await
        }

        #[cfg(not(windows))]
        {
            let _ = (
                subject,
                start_time,
                end_time,
                location,
                body,
                required_attendees,
                optional_attendees,
                reminder_minutes,
            );
            Err(OutlookError::PlatformNotSupported)
        }
    }
}

// Re-export for convenience
pub use OutlookComManager as Outlook;
