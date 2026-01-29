//! Mobile device access module.
//!
//! Design goals:
//! - Modular device controllers (Android/iOS)
//! - Safe-by-default with explicit consent checks
//! - Cross-platform via subprocess tool orchestration (adb/scrcpy/libimobiledevice)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod android;
pub mod connection;
pub mod control;
pub mod ios;
pub mod orchestrator;
pub mod security;
pub mod setup;
pub mod universal;

pub use android::AndroidController;
pub use ios::IosController;
pub use orchestrator::Orchestrator;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Android,
    Ios,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionMode {
    Usb,
    Wifi,
}

#[derive(Debug, thiserror::Error)]
pub enum MobileError {
    #[error("Tool deployment failed: {0}")]
    Deployment(String),
    #[error("Connection unauthorized")]
    Unauthorized,
    #[error("Device not found")]
    NotFound,
    #[error("Consent required for device: {0}")]
    ConsentRequired(String),
    #[error("Invalid configuration: {0}")]
    Config(String),
    #[error("Subprocess failed: {0}")]
    Subprocess(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub model: String,
    #[serde(rename = "type")]
    pub type_: DeviceType,
    pub status: String,
}

/// Persisted config for mobile tooling.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Absolute path to adb (adb.exe on Windows).
    pub adb_path: Option<PathBuf>,
    /// Absolute path to scrcpy.
    pub scrcpy_path: Option<PathBuf>,
    /// Directory containing libimobiledevice binaries (e.g., idevice_id).
    pub libimobiledevice_bin_dir: Option<PathBuf>,

    /// Optional explicit path to a Python interpreter for uiautomator2 subprocess flows.
    pub python_path: Option<PathBuf>,

    /// Enable uiautomator2-based automation (requires Python environment).
    pub uiautomator2_enabled: bool,

    /// Enable action audit log file under `~/.mobile_access/logs/actions.jsonl`.
    pub audit_log_enabled: bool,

    /// Allowlisted devices. No operations run against a device unless its id is present here.
    pub authorized_devices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceEventKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceEvent {
    pub kind: DeviceEventKind,
    pub device: DeviceInfo,
}

pub trait DeviceController {
    fn detect(&self) -> Result<Vec<DeviceInfo>, MobileError>;
    fn connect(&mut self, device_id: &str, mode: ConnectionMode) -> Result<(), MobileError>;
    fn execute_command(&self, cmd: &str) -> Result<String, MobileError>;
    fn pull_file(&self, remote_path: &str, local_path: &str) -> Result<(), MobileError>;
    fn capture_screen(&self) -> Result<PathBuf, MobileError>;
}
