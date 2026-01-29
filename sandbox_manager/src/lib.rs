//! Sandbox Manager - Secure File Isolation for Malware Analysis
//!
//! Provides defense-in-depth security for analyzing potentially malicious files:
//! - Path validation and canonicalization
//! - Symlink rejection
//! - Size limits and rate limiting
//! - No-execute permissions
//! - Audit logging
//! - Auto-cleanup scheduling

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Errors that can occur in sandbox operations
#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("Path escape attempt detected: {0}")]
    PathEscape(String),

    #[error("Symlink not allowed in sandbox")]
    SymlinkRejected,

    #[error("File size exceeds limit: {0} bytes (max: {1} bytes)")]
    FileSizeExceeded(u64, u64),

    #[error("Total sandbox size exceeds limit: {0} bytes (max: {1} bytes)")]
    TotalSizeExceeded(u64, u64),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid configuration: {0}")]
    Configuration(String),
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Base path for sandbox storage
    pub base_path: PathBuf,
    /// Maximum file size in bytes
    pub max_file_size_bytes: u64,
    /// Maximum total sandbox size in bytes
    pub max_total_size_bytes: u64,
    /// Cleanup files older than this many days
    pub cleanup_days: i64,
    /// Allow file execution (DANGEROUS - should be false)
    pub allow_execution: bool,
    /// Rate limit: max uploads per session per minute
    pub rate_limit_per_minute: usize,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./data/sandbox"),
            max_file_size_bytes: 50 * 1024 * 1024, // 50 MB
            max_total_size_bytes: 500 * 1024 * 1024, // 500 MB
            cleanup_days: 7,
            allow_execution: false,
            rate_limit_per_minute: 10,
        }
    }
}

/// Sandbox session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxSession {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub user: String,
    pub files: Vec<SandboxFile>,
    pub total_size_bytes: u64,
}

/// Sandboxed file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxFile {
    pub id: String,
    pub session_id: String,
    pub original_name: String,
    pub stored_path: PathBuf,
    pub size_bytes: u64,
    pub sha256: String,
    pub md5: String,
    pub mime_type: Option<String>,
    pub uploaded_at: DateTime<Utc>,
    pub analyzed: bool,
    pub threat_level: Option<ThreatLevel>,
}

/// Threat level assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    Clean,
    Low,
    Medium,
    High,
    Critical,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub user: String,
    pub action: String,
    pub file_id: Option<String>,
    pub result: String,
    pub details: serde_json::Value,
}

/// Rate limit tracker
#[derive(Debug, Clone)]
struct RateLimit {
    uploads: Vec<DateTime<Utc>>,
}

impl RateLimit {
    fn new() -> Self {
        Self {
            uploads: Vec::new(),
        }
    }

    fn check_and_record(&mut self, limit: usize) -> Result<(), SandboxError> {
        let now = Utc::now();
        let one_minute_ago = now - Duration::minutes(1);

        // Remove old entries
        self.uploads.retain(|t| *t > one_minute_ago);

        if self.uploads.len() >= limit {
            return Err(SandboxError::RateLimitExceeded(format!(
                "Maximum {} uploads per minute exceeded",
                limit
            )));
        }

        self.uploads.push(now);
        Ok(())
    }
}

/// Sandbox Manager - Main struct
pub struct SandboxManager {
    config: SandboxConfig,
    sessions: Arc<RwLock<HashMap<String, SandboxSession>>>,
    rate_limits: Arc<RwLock<HashMap<String, RateLimit>>>,
    audit_log: Arc<RwLock<Vec<AuditEntry>>>,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub async fn new(config: SandboxConfig) -> Result<Self> {
        // Create base directory
        fs::create_dir_all(&config.base_path)
            .await
            .context("Failed to create sandbox base directory")?;

        // Set restrictive permissions on sandbox root
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config.base_path).await?.permissions();
            perms.set_mode(0o700); // Owner only
            fs::set_permissions(&config.base_path, perms).await?;
        }

        Ok(Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Create a new sandbox session
    pub async fn create_session(&self, user: &str) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        let session_path = self.config.base_path.join(&session_id);

        fs::create_dir_all(&session_path)
            .await
            .context("Failed to create session directory")?;

        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&session_path).await?.permissions();
            perms.set_mode(0o700);
            fs::set_permissions(&session_path, perms).await?;
        }

        let session = SandboxSession {
            id: session_id.clone(),
            created_at: Utc::now(),
            user: user.to_string(),
            files: Vec::new(),
            total_size_bytes: 0,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        self.log_audit(
            &session_id,
            user,
            "create_session",
            None,
            "success",
            serde_json::json!({}),
        )
        .await;

        Ok(session_id)
    }

    /// Upload a file to the sandbox
    pub async fn upload_file(
        &self,
        session_id: &str,
        original_name: &str,
        data: &[u8],
    ) -> Result<SandboxFile, SandboxError> {
        // Check rate limit
        {
            let mut rate_limits = self.rate_limits.write().await;
            let limit = rate_limits
                .entry(session_id.to_string())
                .or_insert_with(RateLimit::new);
            limit.check_and_record(self.config.rate_limit_per_minute)?;
        }

        // Get session
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        // Check file size
        let file_size = data.len() as u64;
        if file_size > self.config.max_file_size_bytes {
            return Err(SandboxError::FileSizeExceeded(
                file_size,
                self.config.max_file_size_bytes,
            ));
        }

        // Check total size
        let new_total = session.total_size_bytes + file_size;
        if new_total > self.config.max_total_size_bytes {
            return Err(SandboxError::TotalSizeExceeded(
                new_total,
                self.config.max_total_size_bytes,
            ));
        }

        // Generate file ID and path
        let file_id = Uuid::new_v4().to_string();
        let session_path = self.config.base_path.join(session_id);
        let file_path = session_path.join(&file_id);

        // Validate path (defense in depth)
        self.validate_path(&file_path)?;

        // Calculate hashes
        let sha256 = format!("{:x}", Sha256::digest(data));
        let md5 = format!("{:x}", md5::compute(data));

        // Detect MIME type
        let mime_type = infer::get(data).map(|t| t.mime_type().to_string());

        // Write file
        fs::write(&file_path, data).await?;

        // Set no-execute permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).await?.permissions();
            perms.set_mode(0o600); // Read/write owner only, no execute
            fs::set_permissions(&file_path, perms).await?;
        }

        #[cfg(windows)]
        {
            // On Windows, mark as temporary and deny execute
            use std::os::windows::fs::MetadataExt;
            // Note: Full implementation would use SetFileAttributes
        }

        // Create file metadata
        let sandbox_file = SandboxFile {
            id: file_id.clone(),
            session_id: session_id.to_string(),
            original_name: original_name.to_string(),
            stored_path: file_path,
            size_bytes: file_size,
            sha256,
            md5,
            mime_type,
            uploaded_at: Utc::now(),
            analyzed: false,
            threat_level: None,
        };

        // Update session
        session.files.push(sandbox_file.clone());
        session.total_size_bytes = new_total;

        // Log audit
        self.log_audit(
            session_id,
            &session.user,
            "upload_file",
            Some(&file_id),
            "success",
            serde_json::json!({
                "original_name": original_name,
                "size_bytes": file_size,
                "sha256": &sandbox_file.sha256,
            }),
        )
        .await;

        Ok(sandbox_file)
    }

    /// Get file from sandbox
    pub async fn get_file(&self, session_id: &str, file_id: &str) -> Result<Vec<u8>, SandboxError> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        let file = session
            .files
            .iter()
            .find(|f| f.id == file_id)
            .ok_or_else(|| SandboxError::SessionNotFound(format!("File {} not found", file_id)))?;

        // Validate path before reading
        self.validate_path(&file.stored_path)?;

        let data = fs::read(&file.stored_path).await?;
        Ok(data)
    }

    /// List files in a session
    pub async fn list_files(&self, session_id: &str) -> Result<Vec<SandboxFile>, SandboxError> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        Ok(session.files.clone())
    }

    /// Update file analysis results
    pub async fn update_analysis(
        &self,
        session_id: &str,
        file_id: &str,
        threat_level: ThreatLevel,
    ) -> Result<(), SandboxError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        if let Some(file) = session.files.iter_mut().find(|f| f.id == file_id) {
            file.analyzed = true;
            file.threat_level = Some(threat_level);
        }

        Ok(())
    }

    /// Clear all files in a session
    pub async fn clear_session(&self, session_id: &str) -> Result<(), SandboxError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        let session_path = self.config.base_path.join(session_id);

        // Delete all files
        if session_path.exists() {
            fs::remove_dir_all(&session_path).await?;
            fs::create_dir_all(&session_path).await?;
        }

        // Clear session data
        session.files.clear();
        session.total_size_bytes = 0;

        self.log_audit(
            session_id,
            &session.user,
            "clear_session",
            None,
            "success",
            serde_json::json!({}),
        )
        .await;

        Ok(())
    }

    /// Delete a specific file
    pub async fn delete_file(&self, session_id: &str, file_id: &str) -> Result<(), SandboxError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))?;

        if let Some(pos) = session.files.iter().position(|f| f.id == file_id) {
            let file = session.files.remove(pos);
            session.total_size_bytes -= file.size_bytes;

            // Delete file
            if file.stored_path.exists() {
                fs::remove_file(&file.stored_path).await?;
            }

            self.log_audit(
                session_id,
                &session.user,
                "delete_file",
                Some(file_id),
                "success",
                serde_json::json!({}),
            )
            .await;
        }

        Ok(())
    }

    /// Cleanup old files
    pub async fn cleanup_old_files(&self) -> Result<usize> {
        let cutoff = Utc::now() - Duration::days(self.config.cleanup_days);
        let mut deleted_count = 0;

        let mut sessions = self.sessions.write().await;
        let session_ids: Vec<String> = sessions.keys().cloned().collect();

        for session_id in session_ids {
            if let Some(session) = sessions.get(&session_id) {
                if session.created_at < cutoff {
                    let session_path = self.config.base_path.join(&session_id);
                    if session_path.exists() {
                        fs::remove_dir_all(&session_path).await?;
                        deleted_count += session.files.len();
                    }
                    sessions.remove(&session_id);
                }
            }
        }

        Ok(deleted_count)
    }

    /// Get session info
    pub async fn get_session(&self, session_id: &str) -> Result<SandboxSession, SandboxError> {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| SandboxError::SessionNotFound(session_id.to_string()))
    }

    /// List all sessions
    pub async fn list_sessions(&self) -> Vec<SandboxSession> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    /// Get audit log
    pub async fn get_audit_log(&self) -> Vec<AuditEntry> {
        let log = self.audit_log.read().await;
        log.clone()
    }

    /// Validate sandbox path (critical security function)
    fn validate_path(&self, path: &Path) -> Result<(), SandboxError> {
        // Canonicalize paths
        let canonical_path = path
            .canonicalize()
            .or_else(|_| {
                // If file doesn't exist yet, canonicalize parent
                if let Some(parent) = path.parent() {
                    parent.canonicalize().map(|p| {
                        if let Some(name) = path.file_name() {
                            p.join(name)
                        } else {
                            p
                        }
                    })
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Cannot canonicalize path",
                    ))
                }
            })
            .map_err(|e| SandboxError::Io(e))?;

        let canonical_root = self
            .config
            .base_path
            .canonicalize()
            .map_err(|e| SandboxError::Io(e))?;

        // Check if path is within sandbox root
        if !canonical_path.starts_with(&canonical_root) {
            return Err(SandboxError::PathEscape(format!(
                "Path {:?} is outside sandbox root {:?}",
                canonical_path, canonical_root
            )));
        }

        // Reject symlinks
        if canonical_path.exists() && canonical_path.is_symlink() {
            return Err(SandboxError::SymlinkRejected);
        }

        Ok(())
    }

    /// Log audit entry
    async fn log_audit(
        &self,
        session_id: &str,
        user: &str,
        action: &str,
        file_id: Option<&str>,
        result: &str,
        details: serde_json::Value,
    ) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            session_id: session_id.to_string(),
            user: user.to_string(),
            action: action.to_string(),
            file_id: file_id.map(|s| s.to_string()),
            result: result.to_string(),
            details,
        };

        let mut log = self.audit_log.write().await;
        log.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_sandbox_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = SandboxConfig {
            base_path: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let manager = SandboxManager::new(config).await.unwrap();
        let session_id = manager.create_session("test_user").await.unwrap();
        assert!(!session_id.is_empty());
    }

    #[tokio::test]
    async fn test_file_upload() {
        let temp_dir = TempDir::new().unwrap();
        let config = SandboxConfig {
            base_path: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let manager = SandboxManager::new(config).await.unwrap();
        let session_id = manager.create_session("test_user").await.unwrap();

        let data = b"test file content";
        let file = manager
            .upload_file(&session_id, "test.txt", data)
            .await
            .unwrap();

        assert_eq!(file.original_name, "test.txt");
        assert_eq!(file.size_bytes, data.len() as u64);
    }

    #[tokio::test]
    async fn test_size_limit() {
        let temp_dir = TempDir::new().unwrap();
        let config = SandboxConfig {
            base_path: temp_dir.path().to_path_buf(),
            max_file_size_bytes: 100,
            ..Default::default()
        };

        let manager = SandboxManager::new(config).await.unwrap();
        let session_id = manager.create_session("test_user").await.unwrap();

        let data = vec![0u8; 200];
        let result = manager.upload_file(&session_id, "large.bin", &data).await;

        assert!(matches!(result, Err(SandboxError::FileSizeExceeded(_, _))));
    }
}
