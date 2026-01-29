use browser_orch_ext::orchestrator::cdp::CdpConnection;
use browser_orch_ext::orchestrator::driver::Driver;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex;

pub mod mobile_access;

/// Gated security state - tracks consent and access permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGate {
    pub full_access_granted: bool,
    /// Controls whether Phoenix is allowed to perform self-modification operations
    /// (editing its own code/config, installing deps, running build commands, etc.).
    ///
    /// NOTE: This is logically separate from general system access so deployments can
    /// keep broad visibility (read-only) while restricting mutation.
    pub self_modification_granted: bool,
    pub granted_at: Option<DateTime<Utc>>,
    pub granted_by: Option<String>,
    pub consent_required: bool,
}

impl Default for SecurityGate {
    fn default() -> Self {
        Self {
            full_access_granted: false,
            self_modification_granted: false,
            granted_at: None,
            granted_by: None,
            consent_required: true,
        }
    }
}

impl SecurityGate {
    pub fn grant_full_access(&mut self, granted_by: String) {
        self.full_access_granted = true;
        self.granted_at = Some(Utc::now());
        self.granted_by = Some(granted_by);
    }

    pub fn grant_self_modification(&mut self, granted_by: Option<String>) {
        self.self_modification_granted = true;
        if self.granted_by.is_none() {
            self.granted_by = granted_by;
        }
        if self.granted_at.is_none() {
            self.granted_at = Some(Utc::now());
        }
    }

    pub fn revoke_access(&mut self) {
        self.full_access_granted = false;
        self.self_modification_granted = false;
        self.granted_at = None;
        self.granted_by = None;
    }

    pub fn revoke_self_modification(&mut self) {
        self.self_modification_granted = false;
    }

    pub fn check_access(&self) -> Result<(), String> {
        // Check for Tier 2 unrestricted execution first
        let tier2_enabled = std::env::var("MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION")
            .ok()
            .map(|s| {
                matches!(
                    s.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false);

        // If Tier 2 is enabled, allow access
        if tier2_enabled {
            return Ok(());
        }

        // Check for Tier 1 full access (environment variable)
        let tier1_enabled = std::env::var("MASTER_ORCHESTRATOR_FULL_ACCESS")
            .ok()
            .map(|s| {
                matches!(
                    s.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false);

        // If Tier 1 is enabled, allow access without security gate grant
        if tier1_enabled {
            return Ok(());
        }

        // Otherwise, require explicit security gate grant (legacy/backward compatibility)
        if !self.full_access_granted {
            return Err("Full system access not granted. Please grant access first (system grant <user_name>) or enable Tier 1 (MASTER_ORCHESTRATOR_FULL_ACCESS=true) or Tier 2 (MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true).".to_string());
        }
        Ok(())
    }

    pub fn check_self_modification_access(&self) -> Result<(), String> {
        self.check_access()?;
        if !self.self_modification_granted {
            return Err(
                "Self-modification access not granted. Enable self-modification first.".to_string(),
            );
        }
        Ok(())
    }
}

/// File system entry (file or directory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemEntry {
    pub path: String,
    pub name: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<DateTime<Utc>>,
    pub is_hidden: bool,
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: Option<String>,
    pub memory_usage: Option<u64>,
    pub cpu_percent: Option<f64>,
    pub status: String,
}

/// Result of executing a shell command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Windows Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub description: Option<String>,
}

/// Network/Mapped Drive information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub letter: String,
    pub path: String,
    pub label: Option<String>,
    pub drive_type: String, // "Fixed", "Removable", "Network", "CD", "RAM"
    pub total_size: Option<u64>,
    pub free_space: Option<u64>,
    pub is_mapped: bool,
    pub network_path: Option<String>,
}

/// Registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub path: String,
    pub name: String,
    pub value: String,
    pub value_type: String,
}

/// Installed application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub name: String,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub install_date: Option<String>,
    pub install_location: Option<String>,
    pub is_microsoft: bool,
}

/// Browser credential entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCredential {
    pub url: String,
    pub username: String,
    pub password: Option<String>, // Encrypted/stored securely
    pub browser: String,          // "chrome", "edge", "firefox"
}

/// Browser session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    pub browser_type: String, // "chrome", "edge", "firefox"
    pub profile_path: String,
    pub user_data_dir: String,
    pub is_running: bool,
    pub debug_port: Option<u16>, // Chrome DevTools Protocol port
    pub tabs: Vec<BrowserTab>,
}

/// Browser tab information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub is_active: bool,
}

/// Cookie information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieInfo {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
    pub expires: Option<i64>, // Unix timestamp
}

/// Browser extension information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub description: Option<String>,
    pub path: String,
}

/// CAPTCHA type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptchaType {
    Text,        // Simple text CAPTCHA
    Image,       // Image-based CAPTCHA
    ReCaptchaV2, // Google reCAPTCHA v2
    ReCaptchaV3, // Google reCAPTCHA v3
    HCaptcha,    // hCaptcha
    Turnstile,   // Cloudflare Turnstile
    Unknown,     // Unknown CAPTCHA type
}

/// CAPTCHA detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaDetection {
    pub captcha_type: CaptchaType,
    pub detected: bool,
    pub element_selector: Option<String>,
    pub site_key: Option<String>,    // For reCAPTCHA/hCaptcha
    pub image_url: Option<String>,   // For image CAPTCHAs
    pub image_data: Option<Vec<u8>>, // Base64 encoded image
}

/// CAPTCHA solving result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaSolution {
    pub success: bool,
    pub solution: Option<String>, // Text solution or token
    pub method: String,           // "ocr", "service", "manual", etc.
    pub confidence: f64,          // 0.0-1.0
    pub error: Option<String>,
}

/// CAPTCHA solving service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaServiceConfig {
    pub service: String, // "2captcha", "anticaptcha", "capmonster", etc.
    pub api_key: String,
    pub timeout_seconds: u64,
}

/// GUI Window information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub hwnd: u64, // Window handle
    pub title: String,
    pub class_name: String,
    pub process_id: u32,
    pub process_name: String,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub position: (i32, i32), // (x, y)
    pub size: (i32, i32),     // (width, height)
}

/// GUI Control information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlInfo {
    pub control_type: String, // "Button", "Edit", "Text", etc.
    pub name: String,
    pub automation_id: Option<String>,
    pub bounds: (i32, i32, i32, i32), // (x, y, width, height)
    pub is_enabled: bool,
    pub is_visible: bool,
}

/// System Access Manager - Main interface for all system operations
pub struct SystemAccessManager {
    security_gate: Arc<Mutex<SecurityGate>>,
    browser_driver: Arc<Mutex<Option<Driver>>>,
    always_on: Arc<Mutex<bool>>,
    always_on_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    keylogger_enabled: Arc<StdMutex<bool>>,
    mouse_jigger_enabled: Arc<StdMutex<bool>>,
}

impl Default for SystemAccessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemAccessManager {
    pub fn new() -> Self {
        // This project runs locally and is explicitly intended to be able to
        // operate the host system (including self-modification). Therefore we
        // boot with full access + self-mod enabled.
        let mut security_gate = SecurityGate {
            consent_required: false,
            ..Default::default()
        };
        security_gate.grant_full_access("MasterOrchestrator".to_string());
        security_gate.grant_self_modification(Some("MasterOrchestrator".to_string()));

        Self {
            security_gate: Arc::new(Mutex::new(security_gate)),
            browser_driver: Arc::new(Mutex::new(None)),
            always_on: Arc::new(Mutex::new(false)),
            always_on_task: Arc::new(Mutex::new(None)),
            keylogger_enabled: Arc::new(StdMutex::new(false)),
            mouse_jigger_enabled: Arc::new(StdMutex::new(false)),
        }
    }

    /// Check if Tier 1 or Tier 2 access is available (no security gate required)
    pub fn has_tier_access() -> bool {
        Self::is_tier1_enabled() || Self::is_tier2_enabled()
    }

    pub async fn get_browser_driver(&self) -> Arc<Mutex<Option<Driver>>> {
        self.browser_driver.clone()
    }

    /// Grant full system access (gated security)
    pub async fn grant_full_access(&self, granted_by: String) -> Result<(), String> {
        let mut gate = self.security_gate.lock().await;
        gate.grant_full_access(granted_by);
        Ok(())
    }

    /// Revoke system access
    pub async fn revoke_access(&self) -> Result<(), String> {
        let mut gate = self.security_gate.lock().await;
        gate.revoke_access();
        let _ = self.stop_always_on().await;
        Ok(())
    }

    /// Stop the background "always-on" task (if running).
    pub async fn stop_always_on(&self) -> Result<(), String> {
        // Flip the flag first so any loop that checks it can exit cleanly.
        {
            let mut always_on = self.always_on.lock().await;
            *always_on = false;
        }

        // Abort any existing task.
        let mut task = self.always_on_task.lock().await;
        if let Some(handle) = task.take() {
            handle.abort();
        }

        Ok(())
    }

    /// Enable self-modification operations (code/config mutation, local builds, etc.).
    pub async fn enable_self_modification(&self, granted_by: Option<String>) -> Result<(), String> {
        let mut gate = self.security_gate.lock().await;
        gate.check_access()?;
        gate.grant_self_modification(granted_by);
        Ok(())
    }

    /// Disable self-modification operations.
    pub async fn disable_self_modification(&self) -> Result<(), String> {
        let mut gate = self.security_gate.lock().await;
        gate.revoke_self_modification();
        Ok(())
    }

    pub async fn is_self_modification_enabled(&self) -> bool {
        let gate = self.security_gate.lock().await;
        gate.self_modification_granted
    }

    /// Execute a shell command on the host OS.
    ///
    /// WARNING: This is effectively full remote code execution. It is provided
    /// to support Phoenix self-modification workflows (installing deps, running
    /// tests/builds, generating code, etc.).
    ///
    /// Supports:
    /// - Tier 2: Unrestricted execution (MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true) - No security gate required
    /// - Self-Modification: Self-modification access granted
    pub async fn exec_shell(
        &self,
        command: &str,
        cwd: Option<&str>,
    ) -> Result<CommandResult, String> {
        // Check for Tier 2 unrestricted execution first
        let tier2_enabled = Self::is_tier2_enabled();

        // If Tier 2 is enabled, allow execution without security gate
        // Otherwise, require self-modification access
        if !tier2_enabled {
            self.security_gate
                .lock()
                .await
                .check_self_modification_access()?;
        }

        #[cfg(windows)]
        let mut cmd = {
            let mut c = Command::new("cmd.exe");
            c.arg("/C").arg(command);
            c
        };

        #[cfg(not(windows))]
        let mut cmd = {
            let mut c = Command::new("sh");
            c.arg("-lc").arg(command);
            c
        };

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute command: {e}"))?;

        Ok(CommandResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    /// Read a text file from disk.
    ///
    /// Supports:
    /// - Tier 1: Full file system access (MASTER_ORCHESTRATOR_FULL_ACCESS=true) - No security gate required
    /// - Tier 2: Unrestricted execution (MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true) - No security gate required
    /// - Legacy: Security gate grant (system grant <user_name>)
    /// - Self-Modification: Self-modification access granted
    pub async fn read_file(&self, path: &str) -> Result<String, String> {
        // Check for Tier 2 unrestricted execution
        let tier2_enabled = Self::is_tier2_enabled();

        // Check for Tier 1 full access
        let tier1_enabled = Self::is_tier1_enabled();

        // If Tier 1 or Tier 2 is enabled, allow read without security gate
        if tier1_enabled || tier2_enabled {
            return tokio::fs::read_to_string(path)
                .await
                .map_err(|e| format!("Failed to read file '{path}': {e}"));
        }

        // Otherwise, check for legacy security gate or self-modification access
        let gate = self.security_gate.lock().await;
        if !gate.full_access_granted {
            // Fall back to self-modification check
            drop(gate);
            self.security_gate
                .lock()
                .await
                .check_self_modification_access()?;
        }

        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| format!("Failed to read file '{path}': {e}"))
    }

    /// Write a text file to disk (overwrites existing content).
    ///
    /// Supports:
    /// - Tier 1: Full file system access (MASTER_ORCHESTRATOR_FULL_ACCESS=true) - No security gate required
    /// - Tier 2: Unrestricted execution (MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true) - No security gate required
    /// - Legacy: Security gate grant (system grant <user_name>)
    /// - Self-Modification: Self-modification access granted
    pub async fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        // Check for Tier 2 unrestricted execution
        let tier2_enabled = Self::is_tier2_enabled();

        // Check for Tier 1 full access
        let tier1_enabled = Self::is_tier1_enabled();

        // If Tier 1 or Tier 2 is enabled, allow write without security gate
        if tier1_enabled || tier2_enabled {
            return tokio::fs::write(path, content)
                .await
                .map_err(|e| format!("Failed to write file '{path}': {e}"));
        }

        // Otherwise, check for legacy security gate or self-modification access
        let gate = self.security_gate.lock().await;
        if !gate.full_access_granted {
            // Fall back to self-modification check
            drop(gate);
            self.security_gate
                .lock()
                .await
                .check_self_modification_access()?;
        }

        tokio::fs::write(path, content)
            .await
            .map_err(|e| format!("Failed to write file '{path}': {e}"))
    }

    /// Check if access is granted
    pub async fn is_access_granted(&self) -> bool {
        let gate = self.security_gate.lock().await;
        gate.full_access_granted
    }

    /// Check if Tier 2 unrestricted execution is enabled
    pub fn is_tier2_enabled() -> bool {
        std::env::var("MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION")
            .ok()
            .map(|s| {
                matches!(
                    s.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false)
    }

    /// Check if Tier 1 full access is enabled
    pub fn is_tier1_enabled() -> bool {
        std::env::var("MASTER_ORCHESTRATOR_FULL_ACCESS")
            .ok()
            .map(|s| {
                matches!(
                    s.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false)
    }

    pub async fn set_keylogger_enabled(
        &self,
        enabled: bool,
        log_path: Option<String>,
    ) -> Result<(), String> {
        self.security_gate.lock().await.check_access()?;
        let mut keylogger_enabled = self.keylogger_enabled.lock().unwrap();
        *keylogger_enabled = enabled;
        if enabled {
            // In a real implementation, we would spawn a thread or task
            // to perform the keylogging to the specified path.
            println!(
                "Keylogger enabled. Logging to: {:?}",
                log_path.unwrap_or_default()
            );
        } else {
            println!("Keylogger disabled.");
        }
        Ok(())
    }

    pub async fn set_mouse_jigger_enabled(&self, enabled: bool) -> Result<(), String> {
        self.security_gate.lock().await.check_access()?;
        let mut mouse_jigger_enabled = self.mouse_jigger_enabled.lock().unwrap();
        *mouse_jigger_enabled = enabled;
        if enabled {
            // In a real implementation, we would spawn a thread or task
            // to move the mouse cursor periodically.
            println!("Mouse jigger enabled.");
        } else {
            println!("Mouse jigger disabled.");
        }
        Ok(())
    }

    // ---------- Browser control (CDP) ----------

    /// Find browser sessions with remote debugging (probe common ports).
    pub async fn find_browser_sessions(&self) -> Result<Vec<BrowserSession>, String> {
        self.security_gate.lock().await.check_access()?;
        let mut out = Vec::new();
        let client = reqwest::Client::new();
        for port in [9222, 9223, 9224, 9225] {
            let url = format!("http://127.0.0.1:{}/json/list", port);
            if let Ok(resp) = client.get(&url).send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<serde_json::Value>>().await {
                        let tabs: Vec<BrowserTab> = list
                            .iter()
                            .filter(|t| t["type"].as_str() == Some("page"))
                            .map(|t| BrowserTab {
                                id: t["id"].as_str().unwrap_or("").to_string(),
                                url: t["url"].as_str().unwrap_or("").to_string(),
                                title: t["title"].as_str().unwrap_or("").to_string(),
                                is_active: false,
                            })
                            .collect();
                        out.push(BrowserSession {
                            browser_type: "chromium".to_string(),
                            profile_path: String::new(),
                            user_data_dir: String::new(),
                            is_running: true,
                            debug_port: Some(port),
                            tabs,
                        });
                    }
                }
            }
        }
        Ok(out)
    }

    /// Launch Chrome or Edge with --remote-debugging-port.
    pub async fn launch_browser_with_debugging(
        &self,
        browser_type: &str,
        port: u16,
    ) -> Result<(), String> {
        self.security_gate.lock().await.check_access()?;
        let user_data = env::temp_dir().join(format!("chrome-debug-{}", port));
        let exe = match browser_type.to_lowercase().as_str() {
            "chrome" => {
                #[cfg(windows)]
                {
                    let p = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
                    if std::path::Path::new(p).exists() {
                        Some(p.to_string())
                    } else {
                        let p86 =
                            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe";
                        if std::path::Path::new(p86).exists() {
                            Some(p86.to_string())
                        } else {
                            Some("chrome".to_string())
                        }
                    }
                }
                #[cfg(not(windows))]
                {
                    Some("google-chrome".to_string())
                }
            }
            "edge" => {
                #[cfg(windows)]
                {
                    let p = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
                    if std::path::Path::new(p).exists() {
                        Some(p.to_string())
                    } else {
                        Some("msedge".to_string())
                    }
                }
                #[cfg(not(windows))]
                {
                    Some("microsoft-edge".to_string())
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported browser: {}. Use chrome or edge.",
                    browser_type
                ))
            }
        };
        let exe = exe.ok_or("Browser not found")?;
        let mut cmd = Command::new(&exe);
        cmd.arg(format!("--remote-debugging-port={}", port))
            .arg("--no-first-run")
            .arg("--no-default-browser-check")
            .arg("--user-data-dir")
            .arg(user_data);
        cmd.spawn()
            .map_err(|e| format!("Failed to launch browser: {}", e))?;
        Ok(())
    }

    /// Validate we can connect to a browser on `port`.
    pub async fn connect_browser_session(
        &self,
        _browser_type: &str,
        port: u16,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let _ = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        Ok(format!("Connected to browser on port {}", port))
    }

    /// List tabs (page targets) for a browser on `port`.
    pub async fn get_browser_tabs(&self, port: u16) -> Result<Vec<BrowserTab>, String> {
        self.security_gate.lock().await.check_access()?;
        let url = format!("http://127.0.0.1:{}/json/list", port);
        let list: Vec<serde_json::Value> = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch tabs: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse: {}", e))?;
        let tabs: Vec<BrowserTab> = list
            .into_iter()
            .filter(|t| t["type"].as_str() == Some("page"))
            .map(|t| BrowserTab {
                id: t["id"].as_str().unwrap_or("").to_string(),
                url: t["url"].as_str().unwrap_or("").to_string(),
                title: t["title"].as_str().unwrap_or("").to_string(),
                is_active: false,
            })
            .collect();
        Ok(tabs)
    }

    /// Get cookies from the page(s) on `port`. If `domain` is set, filter by URL.
    pub async fn get_browser_cookies(
        &self,
        _browser_type: &str,
        port: u16,
        domain: Option<&str>,
    ) -> Result<Vec<CookieInfo>, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let _ = cdp
            .send_message("Network.enable", serde_json::json!({}))
            .await
            .map_err(|e| format!("Network.enable failed: {}", e))?;
        let urls = domain.map(|d| vec![format!("https://{}/", d.trim_start_matches('.'))]);
        let payload = urls
            .map(|u| serde_json::json!({ "urls": u }))
            .unwrap_or(serde_json::json!({}));
        let res = cdp
            .send_message("Network.getCookies", payload)
            .await
            .map_err(|e| format!("Network.getCookies failed: {}", e))?;
        let list = res["cookies"].as_array().cloned().unwrap_or_default();
        let cookies: Vec<CookieInfo> = list
            .into_iter()
            .map(|c| CookieInfo {
                name: c["name"].as_str().unwrap_or("").to_string(),
                value: c["value"].as_str().unwrap_or("").to_string(),
                domain: c["domain"].as_str().unwrap_or("").to_string(),
                path: c["path"].as_str().unwrap_or("/").to_string(),
                secure: c["secure"].as_bool().unwrap_or(false),
                http_only: c["httpOnly"].as_bool().unwrap_or(false),
                same_site: c["sameSite"].as_str().map(String::from),
                expires: c["expires"].as_f64().map(|f| f as i64),
            })
            .collect();
        Ok(cookies)
    }

    /// Set a cookie in the browser on `port`. `domain` and `path` are required to build the URL.
    pub async fn set_browser_cookie(
        &self,
        _browser_type: &str,
        port: u16,
        cookie: &CookieInfo,
    ) -> Result<(), String> {
        self.security_gate.lock().await.check_access()?;
        let url = format!("https://{}{}", cookie.domain, cookie.path);
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let _ = cdp
            .send_message(
                "Network.setCookie",
                serde_json::json!({
                    "name": cookie.name,
                    "value": cookie.value,
                    "url": url,
                }),
            )
            .await
            .map_err(|e| format!("Network.setCookie failed: {}", e))?;
        Ok(())
    }

    /// List extensions (stub: requires profile path; returns empty on most platforms).
    pub async fn list_browser_extensions(
        &self,
        _browser_type: &str,
    ) -> Result<Vec<ExtensionInfo>, String> {
        self.security_gate.lock().await.check_access()?;
        Ok(Vec::new())
    }

    /// Execute JavaScript in a page on `port` and return the result as string.
    pub async fn execute_browser_js(
        &self,
        port: u16,
        _target_id: &str,
        code: &str,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let res = cdp
            .evaluate(code, true)
            .await
            .map_err(|e| format!("JS error: {}", e))?;
        let val = res.get("result").and_then(|r| r.get("value"));
        let s = match val {
            Some(serde_json::Value::String(x)) => x.clone(),
            Some(serde_json::Value::Number(n)) => n.to_string(),
            Some(serde_json::Value::Bool(b)) => b.to_string(),
            Some(serde_json::Value::Null) => "null".to_string(),
            Some(o) => serde_json::to_string(o).unwrap_or_else(|_| "".to_string()),
            None => {
                if let Some(ex) = res.get("exceptionDetails") {
                    return Err(format!("JS exception: {:?}", ex));
                }
                "".to_string()
            }
        };
        Ok(s)
    }

    /// Navigate the active page on `port` to `url`.
    pub async fn browser_navigate(&self, port: u16, url: &str) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let _ = cdp
            .navigate(url)
            .await
            .map_err(|e| format!("Navigate failed: {}", e))?;
        Ok(format!("Navigated to {}", url))
    }

    /// Navigate to `url`, wait for load, then fill username/password and submit using heuristics.
    pub async fn browser_login(
        &self,
        port: u16,
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let u = serde_json::to_string(username).map_err(|e| e.to_string())?;
        let p = serde_json::to_string(password).map_err(|e| e.to_string())?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let _ = cdp
            .navigate(url)
            .await
            .map_err(|e| format!("Navigate failed: {}", e))?;
        let _ = cdp
            .evaluate(
                "new Promise(function(r){ if(document.readyState==='complete') r(); else window.addEventListener('load', r); })",
                true,
            )
            .await
            .map_err(|e| format!("Wait for load failed: {}", e))?;
        let login_js = format!(
            r#"
(function(){{
  var u = {};
  var p = {};
  var userEl = document.querySelector('input[type=email]') || document.querySelector('input[name*="user"]') || document.querySelector('input[name*="email"]') || document.querySelector('input[id=username]') || document.querySelector('input[id=email]') || document.querySelector('input[id=user]') || document.querySelector('input[autocomplete=username]') || document.querySelector('form input[type=text]') || document.querySelector('input[type=text]');
  var passEl = document.querySelector('input[type=password]');
  var subEl = document.querySelector('button[type=submit]') || document.querySelector('input[type=submit]') || document.querySelector('button') || document.querySelector('[type=submit]') || (userEl && userEl.form && userEl.form.querySelector('button'));
  if (!userEl || !passEl) return JSON.stringify({{ok:false, err:'username or password field not found'}});
  userEl.focus(); userEl.value=u; userEl.dispatchEvent(new Event('input',{{bubbles:true}}));
  passEl.focus(); passEl.value=p; passEl.dispatchEvent(new Event('input',{{bubbles:true}}));
  if (subEl) {{ subEl.click(); return JSON.stringify({{ok:true}}); }}
  var f = userEl.form || passEl.form;
  if (f) {{ f.submit(); return JSON.stringify({{ok:true}}); }}
  return JSON.stringify({{ok:false, err:'submit not found'}});
}})()
"#,
            u, p
        );
        let res = cdp
            .evaluate(&login_js, false)
            .await
            .map_err(|e| format!("Login JS error: {}", e))?;
        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out: serde_json::Value =
            serde_json::from_str(val).unwrap_or(serde_json::json!({ "ok": false, "err": "parse" }));
        if out["ok"].as_bool() == Some(true) {
            Ok("Login submitted successfully.".to_string())
        } else {
            Err(out["err"].as_str().unwrap_or("Login failed").to_string())
        }
    }

    /// If `url` is non-empty, navigate to it then run `selector`. Return combined text of matching elements.
    pub async fn browser_scrape(
        &self,
        port: u16,
        url: &str,
        selector: &str,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        if !url.is_empty() {
            let _ = cdp
                .navigate(url)
                .await
                .map_err(|e| format!("Navigate failed: {}", e))?;
            let _ = cdp
                .evaluate(
                    "new Promise(function(r){ if(document.readyState==='complete') r(); else window.addEventListener('load', r); })",
                    true,
                )
                .await
                .map_err(|e| format!("Wait for load failed: {}", e))?;
        }
        let sel_esc = selector.replace('\\', "\\\\").replace('"', "\\\"");
        let code = format!(
            r#"Array.from(document.querySelectorAll("{}")).map(function(el){{ return el.innerText || el.textContent || ''; }}).join('\n')"#,
            sel_esc
        );
        let res = cdp
            .evaluate(&code, false)
            .await
            .map_err(|e| format!("Scrape JS error: {}", e))?;
        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        Ok(val.to_string())
    }

    /// Capture a screenshot from the active page on `port`.
    ///
    /// If `selector` is provided, capture only that element's bounding rect.
    /// Returns base64 (no data-url prefix) of a JPEG image.
    pub async fn browser_screenshot(
        &self,
        port: u16,
        selector: Option<&str>,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;

        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        // Enable Page domain (safe no-op if already enabled).
        let _ = cdp
            .send_message("Page.enable", serde_json::json!({}))
            .await
            .map_err(|e| format!("Page.enable failed: {}", e))?;

        let clip = if let Some(sel) = selector.map(str::trim).filter(|s| !s.is_empty()) {
            let sel_json = serde_json::to_string(sel).map_err(|e| e.to_string())?;
            let expr = format!(
                r#"(function(){{
  var sel = {sel_json};
  var el = document.querySelector(sel);
  if(!el) return {{ ok:false, err:'element not found', selector: sel }};
  var r = el.getBoundingClientRect();
  var scale = window.devicePixelRatio || 1;
  return {{
    ok:true,
    x: r.x + window.scrollX,
    y: r.y + window.scrollY,
    width: Math.max(1, r.width),
    height: Math.max(1, r.height),
    scale: scale
  }};
}})()"#
            );
            let res = cdp
                .evaluate(&expr, false)
                .await
                .map_err(|e| format!("Screenshot preflight JS error: {}", e))?;
            let val = res
                .get("result")
                .and_then(|r| r.get("value"))
                .cloned()
                .unwrap_or(serde_json::json!({}));

            if val.get("ok").and_then(|b| b.as_bool()) != Some(true) {
                let err = val
                    .get("err")
                    .and_then(|s| s.as_str())
                    .unwrap_or("selector lookup failed");
                return Err(err.to_string());
            }

            Some(serde_json::json!({
                "x": val.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0),
                "y": val.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0),
                "width": val.get("width").and_then(|v| v.as_f64()).unwrap_or(1.0),
                "height": val.get("height").and_then(|v| v.as_f64()).unwrap_or(1.0),
                "scale": val.get("scale").and_then(|v| v.as_f64()).unwrap_or(1.0),
            }))
        } else {
            None
        };

        let params = if let Some(clip) = clip {
            serde_json::json!({
                "format": "jpeg",
                "quality": 60,
                "fromSurface": true,
                "clip": clip,
            })
        } else {
            serde_json::json!({
                "format": "jpeg",
                "quality": 60,
                "fromSurface": true,
            })
        };

        let res = cdp
            .send_message("Page.captureScreenshot", params)
            .await
            .map_err(|e| format!("captureScreenshot failed: {}", e))?;

        let data = res
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "captureScreenshot returned no data".to_string())?;

        Ok(data.to_string())
    }

    /// Click the first element that matches `selector` on the active page.
    pub async fn browser_click(&self, port: u16, selector: &str) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let selector = selector.trim();
        if selector.is_empty() {
            return Err("click requires a selector".to_string());
        }
        let sel = serde_json::to_string(selector).map_err(|e| e.to_string())?;
        let expr = format!(
            r#"(function(){{
  var el = document.querySelector({sel});
  if(!el) return JSON.stringify({{ok:false, err:'element not found'}});
  el.click();
  return JSON.stringify({{ok:true}});
}})()"#
        );

        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let res = cdp
            .evaluate(&expr, false)
            .await
            .map_err(|e| format!("Click JS error: {}", e))?;

        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out: serde_json::Value =
            serde_json::from_str(val).unwrap_or(serde_json::json!({ "ok": false }));
        if out.get("ok").and_then(|b| b.as_bool()) == Some(true) {
            Ok(format!("Clicked {}", selector))
        } else {
            Err(out
                .get("err")
                .and_then(|s| s.as_str())
                .unwrap_or("click failed")
                .to_string())
        }
    }

    /// Type `text` into the first element that matches `selector`.
    /// Sets `.value` and dispatches an input event.
    pub async fn browser_type(
        &self,
        port: u16,
        selector: &str,
        text: &str,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let selector = selector.trim();
        if selector.is_empty() {
            return Err("type requires a selector".to_string());
        }
        let sel = serde_json::to_string(selector).map_err(|e| e.to_string())?;
        let txt = serde_json::to_string(text).map_err(|e| e.to_string())?;
        let expr = format!(
            r#"(function(){{
  var el = document.querySelector({sel});
  if(!el) return JSON.stringify({{ok:false, err:'element not found'}});
  el.focus();
  el.value = {txt};
  el.dispatchEvent(new Event('input', {{bubbles:true}}));
  el.dispatchEvent(new Event('change', {{bubbles:true}}));
  return JSON.stringify({{ok:true}});
}})()"#
        );

        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let res = cdp
            .evaluate(&expr, false)
            .await
            .map_err(|e| format!("Type JS error: {}", e))?;

        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out: serde_json::Value =
            serde_json::from_str(val).unwrap_or(serde_json::json!({ "ok": false }));
        if out.get("ok").and_then(|b| b.as_bool()) == Some(true) {
            Ok(format!("Typed into {}", selector))
        } else {
            Err(out
                .get("err")
                .and_then(|s| s.as_str())
                .unwrap_or("type failed")
                .to_string())
        }
    }

    /// Scroll the page by the given pixel deltas.
    pub async fn browser_scroll(&self, port: u16, dx: i64, dy: i64) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let expr = format!(
            "(function(){{ window.scrollBy({dx}, {dy}); return JSON.stringify({{ok:true, dx:{dx}, dy:{dy}}}); }})()"
        );
        let res = cdp
            .evaluate(&expr, false)
            .await
            .map_err(|e| format!("Scroll JS error: {}", e))?;
        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out: serde_json::Value =
            serde_json::from_str(val).unwrap_or(serde_json::json!({ "ok": false }));
        if out.get("ok").and_then(|b| b.as_bool()) == Some(true) {
            Ok(format!("Scrolled dx={} dy={}", dx, dy))
        } else {
            Err("scroll failed".to_string())
        }
    }

    fn key_to_cdp_params(key: &str) -> Result<(serde_json::Value, serde_json::Value), String> {
        // Accept forms like:
        // - Enter
        // - Tab
        // - Escape
        // - ArrowUp
        // - Ctrl+L
        // - Ctrl+Shift+I
        // - Alt+Left
        // - a
        let raw = key.trim();
        if raw.is_empty() {
            return Err("keypress requires a key".to_string());
        }

        let parts: Vec<&str> = raw
            .split('+')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        let (mods, base) = if parts.len() >= 2 {
            (parts[..parts.len() - 1].to_vec(), parts[parts.len() - 1])
        } else {
            (Vec::new(), raw)
        };

        let mut modifiers: i64 = 0;
        for m in mods {
            match m.to_ascii_lowercase().as_str() {
                "shift" => modifiers |= 8,
                "ctrl" | "control" => modifiers |= 2,
                "alt" => modifiers |= 1,
                "meta" | "cmd" | "command" | "super" => modifiers |= 4,
                _ => {}
            }
        }

        // Minimal mapping; CDP works with `key` alone for most keys.
        let base_norm = match base.to_ascii_lowercase().as_str() {
            "esc" => "Escape".to_string(),
            "return" => "Enter".to_string(),
            "left" => "ArrowLeft".to_string(),
            "right" => "ArrowRight".to_string(),
            "up" => "ArrowUp".to_string(),
            "down" => "ArrowDown".to_string(),
            other => {
                // Preserve original casing for named keys like ArrowUp, Enter
                if other.len() == 1 {
                    other.to_string()
                } else {
                    base.to_string()
                }
            }
        };

        let key_val = if base_norm.len() == 1 {
            // single char
            base_norm
        } else {
            base_norm
        };

        let down = serde_json::json!({
            "type": "keyDown",
            "key": key_val,
            "modifiers": modifiers,
        });
        let up = serde_json::json!({
            "type": "keyUp",
            "key": key_val,
            "modifiers": modifiers,
        });
        Ok((down, up))
    }

    /// Dispatch a keypress to the active page.
    pub async fn browser_keypress(&self, port: u16, key: &str) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;
        let _ = cdp
            .send_message("Input.enable", serde_json::json!({}))
            .await
            .map_err(|e| format!("Input.enable failed: {}", e))?;

        let (down, up) = Self::key_to_cdp_params(key)?;
        let _ = cdp
            .send_message("Input.dispatchKeyEvent", down)
            .await
            .map_err(|e| format!("dispatchKeyEvent down failed: {}", e))?;
        let _ = cdp
            .send_message("Input.dispatchKeyEvent", up)
            .await
            .map_err(|e| format!("dispatchKeyEvent up failed: {}", e))?;

        Ok(format!("Keypress: {}", key.trim()))
    }

    /// Wait until `document.querySelector(selector)` returns a node or timeout occurs.
    /// Returns a short status message.
    pub async fn browser_wait_for_selector(
        &self,
        port: u16,
        selector: &str,
        timeout_ms: u64,
    ) -> Result<String, String> {
        self.security_gate.lock().await.check_access()?;
        let selector = selector.trim();
        if selector.is_empty() {
            return Err("wait requires a selector".to_string());
        }
        let timeout_ms = timeout_ms.clamp(100, 120_000);
        let sel = serde_json::to_string(selector).map_err(|e| e.to_string())?;

        let mut cdp = CdpConnection::connect_to_page(port)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let expr = format!(
            r#"(function(){{
  var sel = {sel};
  var timeoutMs = {timeout_ms};
  return new Promise(function(resolve){{
    var start = Date.now();
    function tick(){{
      try {{
        var el = document.querySelector(sel);
        if (el) return resolve(JSON.stringify({{ok:true, selector: sel, elapsed_ms: Date.now()-start}}));
      }} catch (e) {{
        return resolve(JSON.stringify({{ok:false, err: String(e), selector: sel}}));
      }}
      if ((Date.now() - start) >= timeoutMs) return resolve(JSON.stringify({{ok:false, timeout:true, selector: sel, elapsed_ms: Date.now()-start}}));
      setTimeout(tick, 100);
    }}
    tick();
  }});
}})()"#
        );

        let res = cdp
            .evaluate(&expr, true)
            .await
            .map_err(|e| format!("Wait JS error: {}", e))?;
        let val = res
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out: serde_json::Value =
            serde_json::from_str(val).unwrap_or(serde_json::json!({ "ok": false, "err": "parse" }));

        if out.get("ok").and_then(|b| b.as_bool()) == Some(true) {
            let elapsed = out.get("elapsed_ms").and_then(|n| n.as_i64()).unwrap_or(0);
            Ok(format!("Selector found: {} ({}ms)", selector, elapsed))
        } else if out.get("timeout").and_then(|b| b.as_bool()) == Some(true) {
            let elapsed = out
                .get("elapsed_ms")
                .and_then(|n| n.as_i64())
                .unwrap_or(timeout_ms as i64);
            Err(format!(
                "Timeout waiting for selector: {} ({}ms)",
                selector, elapsed
            ))
        } else {
            Err(out
                .get("err")
                .and_then(|s| s.as_str())
                .unwrap_or("wait failed")
                .to_string())
        }
    }
}
