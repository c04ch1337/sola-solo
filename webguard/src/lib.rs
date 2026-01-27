//! WebGuard - Lightweight Web Vulnerability Scanner for Sola
//!
//! Phase 28a: Passive scanning (read-only checks)
//! - Security headers analysis (CSP, HSTS, X-Frame-Options, etc.)
//! - Server fingerprinting (Server, X-Powered-By, etc.)
//! - CORS misconfiguration detection
//! - Exposed sensitive paths detection
//! - Tech stack detection

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::{header::HeaderMap, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};
use url::Url;
use uuid::Uuid;

/// Severity levels for security findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🔵",
            Severity::Info => "⚪",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

/// A single security finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub evidence: Option<String>,
    pub remediation: Option<String>,
}

/// Security header analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersReport {
    pub csp: Option<HeaderStatus>,
    pub hsts: Option<HeaderStatus>,
    pub x_frame_options: Option<HeaderStatus>,
    pub x_content_type_options: Option<HeaderStatus>,
    pub referrer_policy: Option<HeaderStatus>,
    pub permissions_policy: Option<HeaderStatus>,
    pub x_xss_protection: Option<HeaderStatus>,
}

/// Status of a security header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderStatus {
    pub present: bool,
    pub value: Option<String>,
    pub severity: Severity,
    pub issue: Option<String>,
}

/// Server fingerprint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerFingerprint {
    pub server: Option<String>,
    pub x_powered_by: Option<String>,
    pub x_aspnet_version: Option<String>,
    pub x_generator: Option<String>,
    pub via: Option<String>,
    pub detected_tech: Vec<String>,
}

/// CORS configuration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsAnalysis {
    pub allow_origin: Option<String>,
    pub allow_credentials: Option<bool>,
    pub allow_methods: Option<String>,
    pub allow_headers: Option<String>,
    pub expose_headers: Option<String>,
    pub max_age: Option<String>,
    pub is_misconfigured: bool,
    pub issues: Vec<String>,
}

/// Sensitive path check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivePathResult {
    pub path: String,
    pub status: u16,
    pub accessible: bool,
    pub severity: Severity,
}

/// Complete passive scan report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveScanReport {
    pub id: String,
    pub target_url: String,
    pub scan_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub status_code: Option<u16>,
    pub security_headers: SecurityHeadersReport,
    pub server_fingerprint: ServerFingerprint,
    pub cors_analysis: CorsAnalysis,
    pub sensitive_paths: Vec<SensitivePathResult>,
    pub findings: Vec<Finding>,
    pub summary: ScanSummary,
}

/// Scan summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub info_count: usize,
    pub overall_risk: Severity,
}

/// WebGuard scanner configuration
#[derive(Debug, Clone)]
pub struct WebGuardConfig {
    pub timeout_secs: u64,
    pub user_agent: String,
    pub check_sensitive_paths: bool,
    pub follow_redirects: bool,
    pub max_redirects: usize,
}

impl Default for WebGuardConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            check_sensitive_paths: true,
            follow_redirects: true,
            max_redirects: 5,
        }
    }
}

/// Sensitive paths to check
const SENSITIVE_PATHS: &[(&str, Severity)] = &[
    ("/.git/config", Severity::Critical),
    ("/.git/HEAD", Severity::Critical),
    ("/.env", Severity::Critical),
    ("/.env.local", Severity::Critical),
    ("/.env.production", Severity::Critical),
    ("/config.php", Severity::High),
    ("/wp-config.php", Severity::Critical),
    ("/web.config", Severity::High),
    ("/.htaccess", Severity::Medium),
    ("/.htpasswd", Severity::Critical),
    ("/admin", Severity::Medium),
    ("/admin/", Severity::Medium),
    ("/administrator", Severity::Medium),
    ("/phpmyadmin", Severity::High),
    ("/phpMyAdmin", Severity::High),
    ("/backup", Severity::High),
    ("/backup/", Severity::High),
    ("/backups", Severity::High),
    ("/wp-admin", Severity::Low),
    ("/wp-login.php", Severity::Low),
    ("/server-status", Severity::Medium),
    ("/server-info", Severity::Medium),
    ("/.svn/entries", Severity::High),
    ("/.DS_Store", Severity::Low),
    ("/robots.txt", Severity::Info),
    ("/sitemap.xml", Severity::Info),
    ("/crossdomain.xml", Severity::Medium),
    ("/clientaccesspolicy.xml", Severity::Medium),
    ("/.well-known/security.txt", Severity::Info),
    ("/api/", Severity::Info),
    ("/api/v1/", Severity::Info),
    ("/graphql", Severity::Info),
    ("/swagger.json", Severity::Medium),
    ("/openapi.json", Severity::Medium),
    ("/api-docs", Severity::Medium),
];

/// WebGuard passive scanner
pub struct WebGuard {
    client: Client,
    config: WebGuardConfig,
}

impl WebGuard {
    /// Create a new WebGuard scanner with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(WebGuardConfig::default())
    }

    /// Create a new WebGuard scanner with custom configuration
    pub fn with_config(config: WebGuardConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .redirect(if config.follow_redirects {
                reqwest::redirect::Policy::limited(config.max_redirects)
            } else {
                reqwest::redirect::Policy::none()
            })
            .danger_accept_invalid_certs(false)
            .build()?;

        Ok(Self { client, config })
    }

    /// Run a passive scan on the target URL
    pub async fn passive_scan(&self, target_url: &str) -> Result<PassiveScanReport> {
        let start_time = std::time::Instant::now();
        let scan_id = Uuid::new_v4().to_string();

        // Validate and normalize URL
        let url = Url::parse(target_url)
            .map_err(|e| anyhow!("Invalid URL '{}': {}", target_url, e))?;

        info!("🔍 WebGuard passive scan starting for: {}", url);

        // Fetch the main page
        let response = self.client.get(url.as_str()).send().await?;
        let status_code = response.status().as_u16();
        let headers = response.headers().clone();

        debug!("Response status: {}", status_code);

        // Analyze security headers
        let security_headers = self.analyze_security_headers(&headers);

        // Extract server fingerprint
        let server_fingerprint = self.extract_fingerprint(&headers);

        // Analyze CORS configuration
        let cors_analysis = self.analyze_cors(&headers);

        // Check sensitive paths
        let sensitive_paths = if self.config.check_sensitive_paths {
            self.check_sensitive_paths(&url).await
        } else {
            Vec::new()
        };

        // Compile all findings
        let mut findings = Vec::new();
        self.add_header_findings(&security_headers, &mut findings);
        self.add_fingerprint_findings(&server_fingerprint, &mut findings);
        self.add_cors_findings(&cors_analysis, &mut findings);
        self.add_path_findings(&sensitive_paths, &mut findings);

        // Calculate summary
        let summary = self.calculate_summary(&findings);

        let duration_ms = start_time.elapsed().as_millis() as u64;

        info!(
            "✅ WebGuard scan complete: {} findings ({} critical, {} high)",
            summary.total_findings, summary.critical_count, summary.high_count
        );

        Ok(PassiveScanReport {
            id: scan_id,
            target_url: target_url.to_string(),
            scan_time: Utc::now(),
            duration_ms,
            status_code: Some(status_code),
            security_headers,
            server_fingerprint,
            cors_analysis,
            sensitive_paths,
            findings,
            summary,
        })
    }

    /// Analyze security headers
    fn analyze_security_headers(&self, headers: &HeaderMap) -> SecurityHeadersReport {
        SecurityHeadersReport {
            csp: self.check_csp(headers),
            hsts: self.check_hsts(headers),
            x_frame_options: self.check_x_frame_options(headers),
            x_content_type_options: self.check_x_content_type_options(headers),
            referrer_policy: self.check_referrer_policy(headers),
            permissions_policy: self.check_permissions_policy(headers),
            x_xss_protection: self.check_x_xss_protection(headers),
        }
    }

    fn check_csp(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("content-security-policy")
            .or_else(|| headers.get("Content-Security-Policy"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            let mut issues = Vec::new();
            if v.contains("unsafe-inline") {
                issues.push("Contains 'unsafe-inline'");
            }
            if v.contains("unsafe-eval") {
                issues.push("Contains 'unsafe-eval'");
            }
            if v.contains("*") && !v.contains("*.") {
                issues.push("Contains wildcard origin");
            }

            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: if issues.is_empty() {
                    Severity::Info
                } else {
                    Severity::Medium
                },
                issue: if issues.is_empty() {
                    None
                } else {
                    Some(issues.join("; "))
                },
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Medium,
                issue: Some("Content-Security-Policy header is missing".to_string()),
            }
        })
    }

    fn check_hsts(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("strict-transport-security")
            .or_else(|| headers.get("Strict-Transport-Security"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            let mut issues = Vec::new();

            // Check max-age
            if let Some(caps) = Regex::new(r"max-age=(\d+)")
                .ok()
                .and_then(|re| re.captures(v))
            {
                if let Some(age) = caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
                    if age < 31536000 {
                        // Less than 1 year
                        issues.push("max-age should be at least 31536000 (1 year)");
                    }
                }
            }

            if !v.to_lowercase().contains("includesubdomains") {
                issues.push("Missing includeSubDomains directive");
            }

            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: if issues.is_empty() {
                    Severity::Info
                } else {
                    Severity::Low
                },
                issue: if issues.is_empty() {
                    None
                } else {
                    Some(issues.join("; "))
                },
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::High,
                issue: Some("Strict-Transport-Security header is missing".to_string()),
            }
        })
    }

    fn check_x_frame_options(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("x-frame-options")
            .or_else(|| headers.get("X-Frame-Options"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            let upper = v.to_uppercase();
            let issue = if upper != "DENY" && upper != "SAMEORIGIN" {
                Some(format!("Unexpected value: {}. Use DENY or SAMEORIGIN", v))
            } else {
                None
            };

            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: if issue.is_some() {
                    Severity::Medium
                } else {
                    Severity::Info
                },
                issue,
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Medium,
                issue: Some("X-Frame-Options header is missing (clickjacking risk)".to_string()),
            }
        })
    }

    fn check_x_content_type_options(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("x-content-type-options")
            .or_else(|| headers.get("X-Content-Type-Options"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            let issue = if v.to_lowercase() != "nosniff" {
                Some(format!("Expected 'nosniff', got '{}'", v))
            } else {
                None
            };

            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: if issue.is_some() {
                    Severity::Low
                } else {
                    Severity::Info
                },
                issue,
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Low,
                issue: Some("X-Content-Type-Options header is missing".to_string()),
            }
        })
    }

    fn check_referrer_policy(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("referrer-policy")
            .or_else(|| headers.get("Referrer-Policy"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            let lower = v.to_lowercase();
            let weak_policies = ["unsafe-url", "no-referrer-when-downgrade"];
            let issue = if weak_policies.iter().any(|p| lower.contains(p)) {
                Some(format!("Weak referrer policy: {}", v))
            } else {
                None
            };

            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: if issue.is_some() {
                    Severity::Low
                } else {
                    Severity::Info
                },
                issue,
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Low,
                issue: Some("Referrer-Policy header is missing".to_string()),
            }
        })
    }

    fn check_permissions_policy(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("permissions-policy")
            .or_else(|| headers.get("Permissions-Policy"))
            .or_else(|| headers.get("feature-policy"))
            .or_else(|| headers.get("Feature-Policy"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: Severity::Info,
                issue: None,
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Low,
                issue: Some("Permissions-Policy header is missing".to_string()),
            }
        })
    }

    fn check_x_xss_protection(&self, headers: &HeaderMap) -> Option<HeaderStatus> {
        let value = headers
            .get("x-xss-protection")
            .or_else(|| headers.get("X-XSS-Protection"))
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Some(if let Some(ref v) = value {
            // Note: X-XSS-Protection is deprecated in modern browsers
            // but having it set to "0" is actually recommended now
            HeaderStatus {
                present: true,
                value: Some(v.clone()),
                severity: Severity::Info,
                issue: Some(
                    "X-XSS-Protection is deprecated; rely on CSP instead".to_string(),
                ),
            }
        } else {
            HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Info,
                issue: None, // Not having it is fine
            }
        })
    }

    /// Extract server fingerprint from headers
    fn extract_fingerprint(&self, headers: &HeaderMap) -> ServerFingerprint {
        let get_header = |name: &str| -> Option<String> {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(String::from)
        };

        let server = get_header("server").or_else(|| get_header("Server"));
        let x_powered_by = get_header("x-powered-by").or_else(|| get_header("X-Powered-By"));
        let x_aspnet_version =
            get_header("x-aspnet-version").or_else(|| get_header("X-AspNet-Version"));
        let x_generator = get_header("x-generator").or_else(|| get_header("X-Generator"));
        let via = get_header("via").or_else(|| get_header("Via"));

        // Detect technologies from headers
        let mut detected_tech = Vec::new();

        if let Some(ref s) = server {
            let lower = s.to_lowercase();
            if lower.contains("nginx") {
                detected_tech.push("Nginx".to_string());
            }
            if lower.contains("apache") {
                detected_tech.push("Apache".to_string());
            }
            if lower.contains("iis") || lower.contains("microsoft") {
                detected_tech.push("IIS".to_string());
            }
            if lower.contains("cloudflare") {
                detected_tech.push("Cloudflare".to_string());
            }
            if lower.contains("openresty") {
                detected_tech.push("OpenResty".to_string());
            }
        }

        if let Some(ref p) = x_powered_by {
            let lower = p.to_lowercase();
            if lower.contains("php") {
                detected_tech.push("PHP".to_string());
            }
            if lower.contains("asp.net") || lower.contains("aspnet") {
                detected_tech.push("ASP.NET".to_string());
            }
            if lower.contains("express") {
                detected_tech.push("Express.js".to_string());
            }
            if lower.contains("next.js") {
                detected_tech.push("Next.js".to_string());
            }
        }

        // Check for framework-specific headers
        if headers.contains_key("x-drupal-cache")
            || headers.contains_key("X-Drupal-Cache")
        {
            detected_tech.push("Drupal".to_string());
        }
        if headers.contains_key("x-shopify-stage")
            || headers.contains_key("X-Shopify-Stage")
        {
            detected_tech.push("Shopify".to_string());
        }
        if headers.contains_key("x-wix-request-id")
            || headers.contains_key("X-Wix-Request-Id")
        {
            detected_tech.push("Wix".to_string());
        }

        ServerFingerprint {
            server,
            x_powered_by,
            x_aspnet_version,
            x_generator,
            via,
            detected_tech,
        }
    }

    /// Analyze CORS configuration
    fn analyze_cors(&self, headers: &HeaderMap) -> CorsAnalysis {
        let get_header = |name: &str| -> Option<String> {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(String::from)
        };

        let allow_origin = get_header("access-control-allow-origin")
            .or_else(|| get_header("Access-Control-Allow-Origin"));
        let allow_credentials_str = get_header("access-control-allow-credentials")
            .or_else(|| get_header("Access-Control-Allow-Credentials"));
        let allow_credentials = allow_credentials_str
            .as_ref()
            .map(|v| v.to_lowercase() == "true");
        let allow_methods = get_header("access-control-allow-methods")
            .or_else(|| get_header("Access-Control-Allow-Methods"));
        let allow_headers = get_header("access-control-allow-headers")
            .or_else(|| get_header("Access-Control-Allow-Headers"));
        let expose_headers = get_header("access-control-expose-headers")
            .or_else(|| get_header("Access-Control-Expose-Headers"));
        let max_age = get_header("access-control-max-age")
            .or_else(|| get_header("Access-Control-Max-Age"));

        let mut issues = Vec::new();
        let mut is_misconfigured = false;

        // Check for dangerous CORS configurations
        if let Some(ref origin) = allow_origin {
            if origin == "*" {
                if allow_credentials == Some(true) {
                    issues.push(
                        "CRITICAL: Access-Control-Allow-Origin: * with credentials=true is invalid but may indicate misconfiguration".to_string(),
                    );
                    is_misconfigured = true;
                } else {
                    issues.push(
                        "Access-Control-Allow-Origin: * allows any origin".to_string(),
                    );
                }
            }
            if origin.contains("null") {
                issues.push(
                    "Access-Control-Allow-Origin: null can be exploited".to_string(),
                );
                is_misconfigured = true;
            }
        }

        if allow_credentials == Some(true) && allow_origin.is_some() {
            let origin = allow_origin.as_ref().unwrap();
            if origin != "*" {
                // This is actually the correct way, but we note it
                debug!("CORS credentials enabled with specific origin: {}", origin);
            }
        }

        CorsAnalysis {
            allow_origin,
            allow_credentials,
            allow_methods,
            allow_headers,
            expose_headers,
            max_age,
            is_misconfigured,
            issues,
        }
    }

    /// Check for exposed sensitive paths
    async fn check_sensitive_paths(&self, base_url: &Url) -> Vec<SensitivePathResult> {
        let mut results = Vec::new();

        for (path, severity) in SENSITIVE_PATHS {
            let full_url = match base_url.join(path) {
                Ok(u) => u,
                Err(_) => continue,
            };

            // Use HEAD request to minimize bandwidth
            let response = match self.client.head(full_url.as_str()).send().await {
                Ok(r) => r,
                Err(e) => {
                    debug!("Failed to check {}: {}", path, e);
                    continue;
                }
            };

            let status = response.status().as_u16();
            let accessible = matches!(
                response.status(),
                StatusCode::OK
                    | StatusCode::MOVED_PERMANENTLY
                    | StatusCode::FOUND
                    | StatusCode::TEMPORARY_REDIRECT
                    | StatusCode::PERMANENT_REDIRECT
            );

            // Only report if accessible or returns interesting status
            if accessible || status == 403 {
                results.push(SensitivePathResult {
                    path: path.to_string(),
                    status,
                    accessible,
                    severity: if accessible { *severity } else { Severity::Info },
                });
            }
        }

        results
    }

    /// Add findings from security headers analysis
    fn add_header_findings(&self, headers: &SecurityHeadersReport, findings: &mut Vec<Finding>) {
        let mut finding_id = 1;

        if let Some(ref csp) = headers.csp {
            if !csp.present || csp.issue.is_some() {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "Content-Security-Policy".to_string(),
                    description: csp
                        .issue
                        .clone()
                        .unwrap_or_else(|| "CSP header issue".to_string()),
                    severity: csp.severity,
                    evidence: csp.value.clone(),
                    remediation: Some(
                        "Implement a strict Content-Security-Policy header to prevent XSS and data injection attacks."
                            .to_string(),
                    ),
                });
                finding_id += 1;
            }
        }

        if let Some(ref hsts) = headers.hsts {
            if !hsts.present || hsts.issue.is_some() {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "Strict-Transport-Security".to_string(),
                    description: hsts
                        .issue
                        .clone()
                        .unwrap_or_else(|| "HSTS header issue".to_string()),
                    severity: hsts.severity,
                    evidence: hsts.value.clone(),
                    remediation: Some(
                        "Add Strict-Transport-Security: max-age=31536000; includeSubDomains; preload"
                            .to_string(),
                    ),
                });
                finding_id += 1;
            }
        }

        if let Some(ref xfo) = headers.x_frame_options {
            if !xfo.present || xfo.issue.is_some() {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "X-Frame-Options".to_string(),
                    description: xfo
                        .issue
                        .clone()
                        .unwrap_or_else(|| "X-Frame-Options header issue".to_string()),
                    severity: xfo.severity,
                    evidence: xfo.value.clone(),
                    remediation: Some(
                        "Add X-Frame-Options: DENY or SAMEORIGIN to prevent clickjacking".to_string(),
                    ),
                });
                finding_id += 1;
            }
        }

        if let Some(ref xcto) = headers.x_content_type_options {
            if !xcto.present {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "X-Content-Type-Options".to_string(),
                    description: xcto
                        .issue
                        .clone()
                        .unwrap_or_else(|| "X-Content-Type-Options header missing".to_string()),
                    severity: xcto.severity,
                    evidence: None,
                    remediation: Some(
                        "Add X-Content-Type-Options: nosniff to prevent MIME-type sniffing"
                            .to_string(),
                    ),
                });
                finding_id += 1;
            }
        }

        if let Some(ref rp) = headers.referrer_policy {
            if !rp.present || rp.issue.is_some() {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "Referrer-Policy".to_string(),
                    description: rp
                        .issue
                        .clone()
                        .unwrap_or_else(|| "Referrer-Policy header issue".to_string()),
                    severity: rp.severity,
                    evidence: rp.value.clone(),
                    remediation: Some(
                        "Add Referrer-Policy: strict-origin-when-cross-origin or no-referrer"
                            .to_string(),
                    ),
                });
                finding_id += 1;
            }
        }

        if let Some(ref pp) = headers.permissions_policy {
            if !pp.present {
                findings.push(Finding {
                    id: format!("HDR-{:03}", finding_id),
                    category: "Security Headers".to_string(),
                    title: "Permissions-Policy".to_string(),
                    description: pp
                        .issue
                        .clone()
                        .unwrap_or_else(|| "Permissions-Policy header missing".to_string()),
                    severity: pp.severity,
                    evidence: None,
                    remediation: Some(
                        "Add Permissions-Policy header to control browser features".to_string(),
                    ),
                });
            }
        }
    }

    /// Add findings from server fingerprint
    fn add_fingerprint_findings(
        &self,
        fingerprint: &ServerFingerprint,
        findings: &mut Vec<Finding>,
    ) {
        let mut finding_id = 100;

        if fingerprint.server.is_some() {
            findings.push(Finding {
                id: format!("FP-{:03}", finding_id),
                category: "Information Disclosure".to_string(),
                title: "Server Header Exposed".to_string(),
                description: "Server header reveals web server software and version".to_string(),
                severity: Severity::Low,
                evidence: fingerprint.server.clone(),
                remediation: Some(
                    "Remove or obfuscate the Server header to reduce information leakage"
                        .to_string(),
                ),
            });
            finding_id += 1;
        }

        if fingerprint.x_powered_by.is_some() {
            findings.push(Finding {
                id: format!("FP-{:03}", finding_id),
                category: "Information Disclosure".to_string(),
                title: "X-Powered-By Header Exposed".to_string(),
                description: "X-Powered-By header reveals backend technology".to_string(),
                severity: Severity::Low,
                evidence: fingerprint.x_powered_by.clone(),
                remediation: Some("Remove the X-Powered-By header".to_string()),
            });
            finding_id += 1;
        }

        if fingerprint.x_aspnet_version.is_some() {
            findings.push(Finding {
                id: format!("FP-{:03}", finding_id),
                category: "Information Disclosure".to_string(),
                title: "ASP.NET Version Exposed".to_string(),
                description: "X-AspNet-Version header reveals ASP.NET version".to_string(),
                severity: Severity::Medium,
                evidence: fingerprint.x_aspnet_version.clone(),
                remediation: Some(
                    "Remove X-AspNet-Version header in web.config: <httpRuntime enableVersionHeader=\"false\" />"
                        .to_string(),
                ),
            });
        }
    }

    /// Add findings from CORS analysis
    fn add_cors_findings(&self, cors: &CorsAnalysis, findings: &mut Vec<Finding>) {
        if cors.is_misconfigured {
            for (i, issue) in cors.issues.iter().enumerate() {
                let severity = if issue.contains("CRITICAL") {
                    Severity::Critical
                } else if issue.contains("null") {
                    Severity::High
                } else {
                    Severity::Medium
                };

                findings.push(Finding {
                    id: format!("CORS-{:03}", i + 1),
                    category: "CORS Misconfiguration".to_string(),
                    title: "CORS Policy Issue".to_string(),
                    description: issue.clone(),
                    severity,
                    evidence: cors.allow_origin.clone(),
                    remediation: Some(
                        "Configure CORS to allow only trusted origins. Never use * with credentials."
                            .to_string(),
                    ),
                });
            }
        }
    }

    /// Add findings from sensitive path checks
    fn add_path_findings(&self, paths: &[SensitivePathResult], findings: &mut Vec<Finding>) {
        for (i, path) in paths.iter().enumerate() {
            if path.accessible {
                findings.push(Finding {
                    id: format!("PATH-{:03}", i + 1),
                    category: "Sensitive Path Exposure".to_string(),
                    title: format!("Accessible: {}", path.path),
                    description: format!(
                        "Sensitive path {} is accessible (HTTP {})",
                        path.path, path.status
                    ),
                    severity: path.severity,
                    evidence: Some(format!("HTTP {}", path.status)),
                    remediation: Some(format!(
                        "Restrict access to {} or remove it from the web root",
                        path.path
                    )),
                });
            }
        }
    }

    /// Calculate scan summary
    fn calculate_summary(&self, findings: &[Finding]) -> ScanSummary {
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;
        let mut info_count = 0;

        for finding in findings {
            match finding.severity {
                Severity::Critical => critical_count += 1,
                Severity::High => high_count += 1,
                Severity::Medium => medium_count += 1,
                Severity::Low => low_count += 1,
                Severity::Info => info_count += 1,
            }
        }

        let overall_risk = if critical_count > 0 {
            Severity::Critical
        } else if high_count > 0 {
            Severity::High
        } else if medium_count > 0 {
            Severity::Medium
        } else if low_count > 0 {
            Severity::Low
        } else {
            Severity::Info
        };

        ScanSummary {
            total_findings: findings.len(),
            critical_count,
            high_count,
            medium_count,
            low_count,
            info_count,
            overall_risk,
        }
    }
}

impl Default for WebGuard {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebGuard instance")
    }
}

/// Format a PassiveScanReport as Markdown for chat display
pub fn format_report_markdown(report: &PassiveScanReport) -> String {
    let mut md = String::new();

    // Header
    md.push_str(&format!(
        "## {} WebGuard Scan Report\n\n",
        report.summary.overall_risk.emoji()
    ));
    md.push_str(&format!("**Target:** `{}`\n", report.target_url));
    md.push_str(&format!(
        "**Scan Time:** {}\n",
        report.scan_time.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Duration:** {}ms\n", report.duration_ms));
    if let Some(status) = report.status_code {
        md.push_str(&format!("**HTTP Status:** {}\n", status));
    }
    md.push_str("\n---\n\n");

    // Summary
    md.push_str("### 📊 Summary\n\n");
    md.push_str(&format!(
        "| Severity | Count |\n|----------|-------|\n"
    ));
    md.push_str(&format!(
        "| {} Critical | {} |\n",
        Severity::Critical.emoji(),
        report.summary.critical_count
    ));
    md.push_str(&format!(
        "| {} High | {} |\n",
        Severity::High.emoji(),
        report.summary.high_count
    ));
    md.push_str(&format!(
        "| {} Medium | {} |\n",
        Severity::Medium.emoji(),
        report.summary.medium_count
    ));
    md.push_str(&format!(
        "| {} Low | {} |\n",
        Severity::Low.emoji(),
        report.summary.low_count
    ));
    md.push_str(&format!(
        "| {} Info | {} |\n",
        Severity::Info.emoji(),
        report.summary.info_count
    ));
    md.push_str(&format!(
        "\n**Total Findings:** {}\n\n",
        report.summary.total_findings
    ));

    // Security Headers Table
    md.push_str("### 🔒 Security Headers\n\n");
    md.push_str("| Header | Status | Value |\n|--------|--------|-------|\n");

    let headers = &report.security_headers;
    if let Some(ref h) = headers.csp {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-").chars().take(50).collect::<String>();
        md.push_str(&format!("| Content-Security-Policy | {} | `{}` |\n", status, value));
    }
    if let Some(ref h) = headers.hsts {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-");
        md.push_str(&format!(
            "| Strict-Transport-Security | {} | `{}` |\n",
            status, value
        ));
    }
    if let Some(ref h) = headers.x_frame_options {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-");
        md.push_str(&format!("| X-Frame-Options | {} | `{}` |\n", status, value));
    }
    if let Some(ref h) = headers.x_content_type_options {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-");
        md.push_str(&format!(
            "| X-Content-Type-Options | {} | `{}` |\n",
            status, value
        ));
    }
    if let Some(ref h) = headers.referrer_policy {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-");
        md.push_str(&format!("| Referrer-Policy | {} | `{}` |\n", status, value));
    }
    if let Some(ref h) = headers.permissions_policy {
        let status = if h.present { "✅" } else { "❌" };
        let value = h.value.as_deref().unwrap_or("-").chars().take(50).collect::<String>();
        md.push_str(&format!("| Permissions-Policy | {} | `{}` |\n", status, value));
    }
    md.push_str("\n");

    // Server Fingerprint
    if report.server_fingerprint.server.is_some()
        || report.server_fingerprint.x_powered_by.is_some()
        || !report.server_fingerprint.detected_tech.is_empty()
    {
        md.push_str("### 🔍 Server Fingerprint\n\n");
        if let Some(ref s) = report.server_fingerprint.server {
            md.push_str(&format!("- **Server:** `{}`\n", s));
        }
        if let Some(ref p) = report.server_fingerprint.x_powered_by {
            md.push_str(&format!("- **X-Powered-By:** `{}`\n", p));
        }
        if !report.server_fingerprint.detected_tech.is_empty() {
            md.push_str(&format!(
                "- **Detected Technologies:** {}\n",
                report.server_fingerprint.detected_tech.join(", ")
            ));
        }
        md.push_str("\n");
    }

    // CORS Analysis
    if report.cors_analysis.allow_origin.is_some() || report.cors_analysis.is_misconfigured {
        md.push_str("### 🌐 CORS Configuration\n\n");
        if let Some(ref origin) = report.cors_analysis.allow_origin {
            md.push_str(&format!("- **Allow-Origin:** `{}`\n", origin));
        }
        if let Some(creds) = report.cors_analysis.allow_credentials {
            md.push_str(&format!("- **Allow-Credentials:** `{}`\n", creds));
        }
        if report.cors_analysis.is_misconfigured {
            md.push_str("\n⚠️ **CORS Issues:**\n");
            for issue in &report.cors_analysis.issues {
                md.push_str(&format!("- {}\n", issue));
            }
        }
        md.push_str("\n");
    }

    // Sensitive Paths
    let accessible_paths: Vec<_> = report
        .sensitive_paths
        .iter()
        .filter(|p| p.accessible)
        .collect();
    if !accessible_paths.is_empty() {
        md.push_str("### 📁 Exposed Sensitive Paths\n\n");
        md.push_str("| Path | Status | Severity |\n|------|--------|----------|\n");
        for path in accessible_paths {
            md.push_str(&format!(
                "| `{}` | {} | {} {} |\n",
                path.path,
                path.status,
                path.severity.emoji(),
                path.severity.badge()
            ));
        }
        md.push_str("\n");
    }

    // Findings
    if !report.findings.is_empty() {
        md.push_str("### 🚨 Findings\n\n");
        for finding in &report.findings {
            md.push_str(&format!(
                "#### {} {} - {}\n",
                finding.severity.emoji(),
                finding.id,
                finding.title
            ));
            md.push_str(&format!("**Category:** {}\n", finding.category));
            md.push_str(&format!("**Description:** {}\n", finding.description));
            if let Some(ref evidence) = finding.evidence {
                md.push_str(&format!("**Evidence:** `{}`\n", evidence));
            }
            if let Some(ref remediation) = finding.remediation {
                md.push_str(&format!("**Remediation:** {}\n", remediation));
            }
            md.push_str("\n");
        }
    }

    md.push_str("---\n");
    md.push_str(&format!("*Scan ID: {}*\n", report.id));

    md
}

/// Format a brief summary for notifications
pub fn format_notification_summary(report: &PassiveScanReport) -> String {
    format!(
        "WebGuard scan of {} complete: {} findings ({} critical, {} high)",
        report.target_url,
        report.summary.total_findings,
        report.summary.critical_count,
        report.summary.high_count
    )
}

// ============================================================================
// Phase 28b: Active XSS Testing Module
// ============================================================================

/// Safe XSS payloads for testing (no destructive actions)
pub const SAFE_XSS_PAYLOADS: &[(&str, &str)] = &[
    // Basic alert payloads
    ("<script>alert(1)</script>", "Basic script injection"),
    ("<script>alert('XSS')</script>", "Script with string"),
    ("<img src=x onerror=alert(1)>", "IMG onerror handler"),
    ("<svg onload=alert(1)>", "SVG onload handler"),
    ("<body onload=alert(1)>", "Body onload handler"),
    // Event handler payloads
    ("<div onmouseover=alert(1)>hover</div>", "Mouseover event"),
    ("<input onfocus=alert(1) autofocus>", "Input autofocus"),
    ("<marquee onstart=alert(1)>", "Marquee onstart"),
    ("<video><source onerror=alert(1)>", "Video source error"),
    // Encoded payloads
    ("<script>alert(String.fromCharCode(88,83,83))</script>", "CharCode encoding"),
    ("<img src=x onerror=&#97;&#108;&#101;&#114;&#116;(1)>", "HTML entity encoding"),
    // JavaScript URI payloads
    ("<a href=javascript:alert(1)>click</a>", "JavaScript URI"),
    ("<iframe src=javascript:alert(1)>", "Iframe JavaScript URI"),
    // Template literal payloads
    ("<script>alert`1`</script>", "Template literal"),
    // DOM-based payloads
    ("<img src=1 onerror=alert(document.domain)>", "Document domain leak"),
    // Polyglot payloads
    ("jaVasCript:/*-/*`/*\\`/*'/*\"/**/(/* */oNcLiCk=alert() )//", "Polyglot payload"),
];

/// XSS vulnerability type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum XssType {
    Reflected,
    Stored,
    DomBased,
}

impl XssType {
    pub fn description(&self) -> &'static str {
        match self {
            XssType::Reflected => "Reflected XSS - payload is reflected in the response",
            XssType::Stored => "Stored XSS - payload is stored and executed on subsequent requests",
            XssType::DomBased => "DOM-based XSS - payload is executed via client-side JavaScript",
        }
    }
}

/// Result of a single XSS payload test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssPayloadResult {
    pub payload: String,
    pub payload_description: String,
    pub reflected: bool,
    pub executed: bool,
    pub context: Option<String>,
    pub response_snippet: Option<String>,
}

/// XSS vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssFinding {
    pub id: String,
    pub xss_type: XssType,
    pub severity: Severity,
    pub parameter: String,
    pub payload: String,
    pub payload_description: String,
    pub proof_of_concept: String,
    pub evidence: String,
    pub remediation: String,
}

/// Complete XSS test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssTestReport {
    pub id: String,
    pub target_url: String,
    pub parameter: String,
    pub scan_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub payloads_tested: usize,
    pub payloads_reflected: usize,
    pub payloads_executed: usize,
    pub findings: Vec<XssFinding>,
    pub payload_results: Vec<XssPayloadResult>,
    pub summary: XssSummary,
}

/// XSS test summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssSummary {
    pub vulnerable: bool,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub overall_risk: Severity,
}

/// XSS test configuration
#[derive(Debug, Clone)]
pub struct XssTestConfig {
    pub timeout_secs: u64,
    pub user_agent: String,
    pub max_payloads: usize,
    pub check_reflection: bool,
    pub check_execution: bool,
    pub sandbox_mode: bool,
}

impl Default for XssTestConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            max_payloads: 17, // All safe payloads
            check_reflection: true,
            check_execution: true,
            sandbox_mode: true, // Always run in sandbox mode
        }
    }
}

/// XSS Tester - Active XSS vulnerability testing
pub struct XssTester {
    client: Client,
    config: XssTestConfig,
}

impl XssTester {
    /// Create a new XSS tester with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(XssTestConfig::default())
    }

    /// Create a new XSS tester with custom configuration
    pub fn with_config(config: XssTestConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(5))
            .danger_accept_invalid_certs(false)
            .build()?;

        Ok(Self { client, config })
    }

    /// Test a URL parameter for XSS vulnerabilities
    /// 
    /// # Arguments
    /// * `target_url` - The base URL to test (e.g., "https://example.com/search")
    /// * `parameter` - The parameter name to inject payloads into (e.g., "q")
    /// 
    /// # Returns
    /// An XssTestReport containing all findings
    pub async fn test_xss(&self, target_url: &str, parameter: &str) -> Result<XssTestReport> {
        let start_time = std::time::Instant::now();
        let scan_id = Uuid::new_v4().to_string();

        // Validate URL
        let base_url = Url::parse(target_url)
            .map_err(|e| anyhow!("Invalid URL '{}': {}", target_url, e))?;

        info!("🔍 WebGuard XSS test starting for: {} (param: {})", base_url, parameter);

        let mut payload_results = Vec::new();
        let mut findings = Vec::new();
        let mut finding_id = 1;

        // Test each safe payload
        let payloads_to_test = SAFE_XSS_PAYLOADS
            .iter()
            .take(self.config.max_payloads);

        for (payload, description) in payloads_to_test {
            let result = self.test_single_payload(&base_url, parameter, payload, description).await;
            
            // Check if this payload found a vulnerability
            if result.reflected || result.executed {
                let severity = if result.executed {
                    Severity::Critical
                } else {
                    Severity::High
                };

                let xss_type = if result.executed {
                    XssType::Reflected
                } else {
                    XssType::Reflected // Could be DOM-based, but we detect reflected first
                };

                findings.push(XssFinding {
                    id: format!("XSS-{:03}", finding_id),
                    xss_type,
                    severity,
                    parameter: parameter.to_string(),
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    proof_of_concept: self.generate_poc(&base_url, parameter, payload),
                    evidence: result.response_snippet.clone().unwrap_or_else(|| "Payload reflected in response".to_string()),
                    remediation: self.get_remediation(xss_type),
                });
                finding_id += 1;
            }

            payload_results.push(result);
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Calculate summary
        let payloads_reflected = payload_results.iter().filter(|r| r.reflected).count();
        let payloads_executed = payload_results.iter().filter(|r| r.executed).count();
        let critical_count = findings.iter().filter(|f| f.severity == Severity::Critical).count();
        let high_count = findings.iter().filter(|f| f.severity == Severity::High).count();

        let overall_risk = if critical_count > 0 {
            Severity::Critical
        } else if high_count > 0 {
            Severity::High
        } else if payloads_reflected > 0 {
            Severity::Medium
        } else {
            Severity::Info
        };

        let summary = XssSummary {
            vulnerable: !findings.is_empty(),
            total_findings: findings.len(),
            critical_count,
            high_count,
            overall_risk,
        };

        info!(
            "✅ WebGuard XSS test complete: {} findings ({} critical, {} high)",
            summary.total_findings, summary.critical_count, summary.high_count
        );

        Ok(XssTestReport {
            id: scan_id,
            target_url: target_url.to_string(),
            parameter: parameter.to_string(),
            scan_time: Utc::now(),
            duration_ms,
            payloads_tested: payload_results.len(),
            payloads_reflected,
            payloads_executed,
            findings,
            payload_results,
            summary,
        })
    }

    /// Test a single XSS payload
    async fn test_single_payload(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> XssPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing payload: {} -> {}", description, test_url);

        // Make request
        let response = match self.client.get(test_url.as_str()).send().await {
            Ok(resp) => resp,
            Err(e) => {
                debug!("Request failed for payload '{}': {}", description, e);
                return XssPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    reflected: false,
                    executed: false,
                    context: None,
                    response_snippet: None,
                };
            }
        };

        // Get response body
        let body = match response.text().await {
            Ok(text) => text,
            Err(_) => {
                return XssPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    reflected: false,
                    executed: false,
                    context: None,
                    response_snippet: None,
                };
            }
        };

        // Check if payload is reflected in response
        let reflected = body.contains(payload) || self.check_encoded_reflection(&body, payload);

        // Check for execution indicators (simplified - real execution would need CDP)
        // We look for signs that the payload might execute
        let executed = self.check_execution_indicators(&body, payload);

        // Extract context around the reflection
        let (context, snippet) = if reflected {
            self.extract_reflection_context(&body, payload)
        } else {
            (None, None)
        };

        XssPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            reflected,
            executed,
            context,
            response_snippet: snippet,
        }
    }

    /// Check if payload is reflected in encoded form
    fn check_encoded_reflection(&self, body: &str, payload: &str) -> bool {
        // Check URL encoding
        let url_encoded = urlencoding::encode(payload);
        if body.contains(url_encoded.as_str()) {
            return true;
        }

        // Check HTML entity encoding
        let html_encoded = payload
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");
        if body.contains(&html_encoded) {
            return true;
        }

        false
    }

    /// Check for indicators that XSS might execute
    fn check_execution_indicators(&self, body: &str, payload: &str) -> bool {
        // If the payload contains script tags and they appear unescaped
        if payload.contains("<script>") && body.contains("<script>alert") {
            return true;
        }

        // If event handlers appear unescaped
        let event_handlers = ["onerror=", "onload=", "onclick=", "onmouseover=", "onfocus="];
        for handler in event_handlers {
            if payload.contains(handler) && body.contains(handler) {
                // Check if it's not escaped
                let escaped = format!("&quot;{}", handler);
                if !body.contains(&escaped) {
                    return true;
                }
            }
        }

        // Check for javascript: URI
        if payload.contains("javascript:") && body.contains("javascript:alert") {
            return true;
        }

        false
    }

    /// Extract context around where payload is reflected
    fn extract_reflection_context(&self, body: &str, payload: &str) -> (Option<String>, Option<String>) {
        if let Some(pos) = body.find(payload) {
            let start = pos.saturating_sub(50);
            let end = (pos + payload.len() + 50).min(body.len());
            let snippet = &body[start..end];
            
            // Determine context (HTML, attribute, script, etc.)
            let context = self.determine_context(body, pos);
            
            (Some(context), Some(format!("...{}...", snippet)))
        } else {
            (None, None)
        }
    }

    /// Determine the context where payload is reflected
    fn determine_context(&self, body: &str, position: usize) -> String {
        let before = &body[..position];
        
        // Check if inside a script tag
        if let Some(script_pos) = before.rfind("<script") {
            if before[script_pos..].find("</script>").is_none() {
                return "JavaScript context".to_string();
            }
        }

        // Check if inside an attribute
        if let Some(attr_pos) = before.rfind('=') {
            let after_eq = &before[attr_pos..];
            if after_eq.contains('"') && !after_eq.contains("\" ") {
                return "HTML attribute (double-quoted)".to_string();
            }
            if after_eq.contains('\'') && !after_eq.contains("' ") {
                return "HTML attribute (single-quoted)".to_string();
            }
        }

        // Check if inside a tag
        if let Some(tag_pos) = before.rfind('<') {
            if before[tag_pos..].find('>').is_none() {
                return "HTML tag context".to_string();
            }
        }

        "HTML body context".to_string()
    }

    /// Generate a proof-of-concept URL
    fn generate_poc(&self, base_url: &Url, parameter: &str, payload: &str) -> String {
        let mut poc_url = base_url.clone();
        poc_url.query_pairs_mut().append_pair(parameter, payload);
        poc_url.to_string()
    }

    /// Get remediation advice for XSS type
    fn get_remediation(&self, xss_type: XssType) -> String {
        match xss_type {
            XssType::Reflected => {
                "1. Implement proper output encoding based on context (HTML, JavaScript, URL, CSS)\n\
                 2. Use Content-Security-Policy header to restrict script execution\n\
                 3. Validate and sanitize all user input on the server side\n\
                 4. Use HTTPOnly and Secure flags on session cookies\n\
                 5. Consider using a Web Application Firewall (WAF)".to_string()
            }
            XssType::Stored => {
                "1. Sanitize all user input before storing in database\n\
                 2. Encode output when displaying stored content\n\
                 3. Implement Content-Security-Policy header\n\
                 4. Use parameterized queries to prevent injection\n\
                 5. Implement input validation with allowlists".to_string()
            }
            XssType::DomBased => {
                "1. Avoid using dangerous DOM methods (innerHTML, document.write)\n\
                 2. Use textContent instead of innerHTML when possible\n\
                 3. Sanitize data from URL parameters before DOM insertion\n\
                 4. Implement Content-Security-Policy with strict-dynamic\n\
                 5. Use DOMPurify or similar library for HTML sanitization".to_string()
            }
        }
    }
}

impl Default for XssTester {
    fn default() -> Self {
        Self::new().expect("Failed to create default XssTester instance")
    }
}

/// Format an XSS test report as Markdown for chat display
pub fn format_xss_report_markdown(report: &XssTestReport) -> String {
    let mut md = String::new();

    // Header
    let status_emoji = if report.summary.vulnerable { "🔴" } else { "✅" };
    md.push_str(&format!(
        "## {} WebGuard XSS Test Report\n\n",
        status_emoji
    ));
    md.push_str(&format!("**Target:** `{}`\n", report.target_url));
    md.push_str(&format!("**Parameter:** `{}`\n", report.parameter));
    md.push_str(&format!(
        "**Scan Time:** {}\n",
        report.scan_time.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Duration:** {}ms\n", report.duration_ms));
    md.push_str("\n---\n\n");

    // Summary
    md.push_str("### 📊 Summary\n\n");
    if report.summary.vulnerable {
        md.push_str("⚠️ **VULNERABLE TO XSS**\n\n");
    } else {
        md.push_str("✅ **No XSS vulnerabilities detected**\n\n");
    }

    md.push_str(&format!("| Metric | Value |\n|--------|-------|\n"));
    md.push_str(&format!("| Payloads Tested | {} |\n", report.payloads_tested));
    md.push_str(&format!("| Payloads Reflected | {} |\n", report.payloads_reflected));
    md.push_str(&format!("| Payloads Executed | {} |\n", report.payloads_executed));
    md.push_str(&format!("| Total Findings | {} |\n", report.summary.total_findings));
    md.push_str(&format!(
        "| {} Critical | {} |\n",
        Severity::Critical.emoji(),
        report.summary.critical_count
    ));
    md.push_str(&format!(
        "| {} High | {} |\n",
        Severity::High.emoji(),
        report.summary.high_count
    ));
    md.push_str("\n");

    // Findings
    if !report.findings.is_empty() {
        md.push_str("### 🚨 Vulnerabilities Found\n\n");
        for finding in &report.findings {
            md.push_str(&format!(
                "#### {} {} - {} XSS\n",
                finding.severity.emoji(),
                finding.id,
                match finding.xss_type {
                    XssType::Reflected => "Reflected",
                    XssType::Stored => "Stored",
                    XssType::DomBased => "DOM-based",
                }
            ));
            md.push_str(&format!("**Parameter:** `{}`\n", finding.parameter));
            md.push_str(&format!("**Payload:** `{}`\n", finding.payload));
            md.push_str(&format!("**Description:** {}\n", finding.payload_description));
            md.push_str(&format!("**Evidence:** {}\n", finding.evidence));
            md.push_str(&format!("\n**Proof of Concept:**\n```\n{}\n```\n", finding.proof_of_concept));
            md.push_str(&format!("\n**Remediation:**\n{}\n\n", finding.remediation));
        }
    }

    // Payload Results Table
    md.push_str("### 📋 Payload Test Results\n\n");
    md.push_str("| Payload | Reflected | Executed | Context |\n|---------|-----------|----------|----------|\n");
    for result in &report.payload_results {
        let reflected = if result.reflected { "✅" } else { "❌" };
        let executed = if result.executed { "⚠️ Yes" } else { "No" };
        let context = result.context.as_deref().unwrap_or("-");
        // Truncate payload for display
        let payload_display: String = result.payload.chars().take(30).collect();
        let payload_display = if result.payload.len() > 30 {
            format!("{}...", payload_display)
        } else {
            payload_display
        };
        md.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            payload_display.replace('|', "\\|"),
            reflected,
            executed,
            context
        ));
    }
    md.push_str("\n");

    md.push_str("---\n");
    md.push_str(&format!("*Scan ID: {}*\n", report.id));
    md.push_str("\n⚠️ **Note:** This is a safe, passive XSS test. No destructive payloads were used.\n");

    md
}

/// Format a brief XSS summary for notifications
pub fn format_xss_notification_summary(report: &XssTestReport) -> String {
    if report.summary.vulnerable {
        format!(
            "🔴 XSS VULNERABILITY DETECTED on {} (param: {}) - {} findings ({} critical)",
            report.target_url,
            report.parameter,
            report.summary.total_findings,
            report.summary.critical_count
        )
    } else {
        format!(
            "✅ XSS test complete for {} (param: {}) - No vulnerabilities found",
            report.target_url,
            report.parameter
        )
    }
}

// Add urlencoding for payload encoding checks
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut encoded = String::new();
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    encoded.push(byte as char);
                }
                _ => {
                    encoded.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        encoded
    }
}

// ============================================================================
// Phase 28d: SQL Injection (SQLi) Testing Module
// ============================================================================

/// Safe SQLi payloads for testing (no destructive actions)
/// These payloads are designed to detect vulnerabilities without causing harm
pub const SAFE_SQLI_PAYLOADS: &[(&str, &str, SqliPayloadType)] = &[
    // Error-based payloads
    ("'", "Single quote - basic error trigger", SqliPayloadType::ErrorBased),
    ("\"", "Double quote - basic error trigger", SqliPayloadType::ErrorBased),
    ("' OR '1'='1", "Classic OR injection", SqliPayloadType::ErrorBased),
    ("\" OR \"\"=\"", "Double quote OR injection", SqliPayloadType::ErrorBased),
    ("' OR '1'='1' --", "OR injection with comment", SqliPayloadType::ErrorBased),
    ("' OR '1'='1' #", "OR injection with hash comment", SqliPayloadType::ErrorBased),
    ("1' ORDER BY 1--", "ORDER BY enumeration", SqliPayloadType::ErrorBased),
    ("1' ORDER BY 100--", "ORDER BY high number (error)", SqliPayloadType::ErrorBased),
    ("' UNION SELECT NULL--", "UNION SELECT probe", SqliPayloadType::ErrorBased),
    ("' AND '1'='1", "AND true condition", SqliPayloadType::BooleanBased),
    ("' AND '1'='2", "AND false condition", SqliPayloadType::BooleanBased),
    
    // Boolean-based blind payloads
    ("1 AND 1=1", "Boolean true (numeric)", SqliPayloadType::BooleanBased),
    ("1 AND 1=2", "Boolean false (numeric)", SqliPayloadType::BooleanBased),
    ("1' AND '1'='1", "Boolean true (string)", SqliPayloadType::BooleanBased),
    ("1' AND '1'='2", "Boolean false (string)", SqliPayloadType::BooleanBased),
    ("1 OR 1=1", "OR true (numeric)", SqliPayloadType::BooleanBased),
    ("1 OR 1=2", "OR false (numeric)", SqliPayloadType::BooleanBased),
    
    // Time-based blind payloads (safe delays)
    ("' WAITFOR DELAY '0:0:3'--", "MSSQL time delay (3s)", SqliPayloadType::TimeBased),
    ("1' AND SLEEP(3)--", "MySQL time delay (3s)", SqliPayloadType::TimeBased),
    ("1'; WAITFOR DELAY '0:0:3'--", "MSSQL stacked delay", SqliPayloadType::TimeBased),
    ("1' AND (SELECT * FROM (SELECT(SLEEP(3)))a)--", "MySQL nested sleep", SqliPayloadType::TimeBased),
    ("1' AND pg_sleep(3)--", "PostgreSQL time delay (3s)", SqliPayloadType::TimeBased),
    ("1'; SELECT pg_sleep(3);--", "PostgreSQL stacked delay", SqliPayloadType::TimeBased),
    
    // Database fingerprinting (safe)
    ("' AND @@version--", "MSSQL version probe", SqliPayloadType::ErrorBased),
    ("' AND version()--", "MySQL/PostgreSQL version probe", SqliPayloadType::ErrorBased),
    ("' AND sqlite_version()--", "SQLite version probe", SqliPayloadType::ErrorBased),
];

/// SQL error keywords that indicate SQLi vulnerability
pub const SQL_ERROR_KEYWORDS: &[&str] = &[
    // MySQL errors
    "mysql_fetch",
    "mysql_num_rows",
    "mysql_query",
    "mysqli_",
    "You have an error in your SQL syntax",
    "Warning: mysql_",
    "Warning: mysqli_",
    "MySQL server version",
    "supplied argument is not a valid MySQL",
    
    // PostgreSQL errors
    "pg_query",
    "pg_exec",
    "PostgreSQL query failed",
    "ERROR: syntax error at or near",
    "unterminated quoted string",
    
    // MSSQL errors
    "Microsoft SQL Native Client error",
    "ODBC SQL Server Driver",
    "SQLServer JDBC Driver",
    "Unclosed quotation mark",
    "mssql_query",
    "Incorrect syntax near",
    
    // Oracle errors
    "ORA-00933",
    "ORA-00921",
    "ORA-01756",
    "Oracle error",
    "quoted string not properly terminated",
    
    // SQLite errors
    "SQLite3::",
    "SQLITE_ERROR",
    "sqlite3_",
    "unrecognized token",
    
    // Generic SQL errors
    "SQL syntax",
    "SQL error",
    "syntax error",
    "unexpected end of SQL",
    "invalid query",
    "Query failed",
    "database error",
    "SQLSTATE",
];

/// SQLi payload type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SqliPayloadType {
    ErrorBased,
    BooleanBased,
    TimeBased,
    UnionBased,
}

impl SqliPayloadType {
    pub fn description(&self) -> &'static str {
        match self {
            SqliPayloadType::ErrorBased => "Error-based SQLi - triggers database error messages",
            SqliPayloadType::BooleanBased => "Boolean-based blind SQLi - detects true/false response differences",
            SqliPayloadType::TimeBased => "Time-based blind SQLi - detects response time delays",
            SqliPayloadType::UnionBased => "UNION-based SQLi - extracts data via UNION queries",
        }
    }
    
    pub fn badge(&self) -> &'static str {
        match self {
            SqliPayloadType::ErrorBased => "ERROR",
            SqliPayloadType::BooleanBased => "BOOLEAN",
            SqliPayloadType::TimeBased => "TIME",
            SqliPayloadType::UnionBased => "UNION",
        }
    }
}

/// SQLi vulnerability type detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SqliType {
    ErrorBased,
    BooleanBlind,
    TimeBlind,
    UnionBased,
}

impl SqliType {
    pub fn description(&self) -> &'static str {
        match self {
            SqliType::ErrorBased => "Error-based SQLi - database errors reveal injection point",
            SqliType::BooleanBlind => "Boolean-based blind SQLi - response differences indicate injection",
            SqliType::TimeBlind => "Time-based blind SQLi - response delays confirm injection",
            SqliType::UnionBased => "UNION-based SQLi - data extraction via UNION queries possible",
        }
    }
}

/// Result of a single SQLi payload test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliPayloadResult {
    pub payload: String,
    pub payload_description: String,
    pub payload_type: SqliPayloadType,
    pub error_detected: bool,
    pub error_message: Option<String>,
    pub response_time_ms: u64,
    pub time_delay_detected: bool,
    pub boolean_difference: Option<bool>,
    pub response_length: usize,
    pub status_code: u16,
}

/// SQLi vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliFinding {
    pub id: String,
    pub sqli_type: SqliType,
    pub severity: Severity,
    pub parameter: String,
    pub payload: String,
    pub payload_description: String,
    pub proof_of_concept: String,
    pub evidence: String,
    pub database_type: Option<String>,
    pub remediation: String,
}

/// Complete SQLi test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliTestReport {
    pub id: String,
    pub target_url: String,
    pub parameter: String,
    pub scan_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub payloads_tested: usize,
    pub errors_detected: usize,
    pub time_delays_detected: usize,
    pub boolean_differences: usize,
    pub findings: Vec<SqliFinding>,
    pub payload_results: Vec<SqliPayloadResult>,
    pub summary: SqliSummary,
}

/// SQLi test summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliSummary {
    pub vulnerable: bool,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub overall_risk: Severity,
    pub detected_database: Option<String>,
}

/// SQLi test configuration
#[derive(Debug, Clone)]
pub struct SqliTestConfig {
    pub timeout_secs: u64,
    pub user_agent: String,
    pub max_payloads: usize,
    pub time_delay_threshold_ms: u64,
    pub check_error_based: bool,
    pub check_boolean_based: bool,
    pub check_time_based: bool,
    pub sandbox_mode: bool,
}

impl Default for SqliTestConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            max_payloads: 30, // All safe payloads
            time_delay_threshold_ms: 2500, // 2.5 seconds threshold for time-based detection
            check_error_based: true,
            check_boolean_based: true,
            check_time_based: true,
            sandbox_mode: true, // Always run in sandbox mode
        }
    }
}

/// SQLi Tester - SQL Injection vulnerability testing
pub struct SqliTester {
    client: Client,
    config: SqliTestConfig,
    cdp_port: Option<u16>, // Chrome DevTools Protocol port for sandbox sessions
}

impl SqliTester {
    /// Create a new SQLi tester with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(SqliTestConfig::default())
    }

    /// Create a new SQLi tester with custom configuration
    pub fn with_config(config: SqliTestConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(5))
            .danger_accept_invalid_certs(false)
            .build()?;

        // Get CDP port from environment if sandbox mode is enabled
        let cdp_port = if config.sandbox_mode {
            std::env::var("CHROME_DEBUG_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .or_else(|| {
                    std::env::var("BROWSER_DEBUG_PORT")
                        .ok()
                        .and_then(|s| s.parse().ok())
                })
                .or(Some(9222)) // Default CDP port
        } else {
            None
        };

        Ok(Self { 
            client, 
            config,
            cdp_port,
        })
    }

    /// Test a URL parameter for SQLi vulnerabilities
    /// 
    /// # Arguments
    /// * `target_url` - The base URL to test (e.g., "https://example.com/search")
    /// * `parameter` - The parameter name to inject payloads into (e.g., "id")
    /// 
    /// # Returns
    /// A SqliTestReport containing all findings
    pub async fn test_sqli(&self, target_url: &str, parameter: &str) -> Result<SqliTestReport> {
        let start_time = std::time::Instant::now();
        let scan_id = Uuid::new_v4().to_string();

        // Validate URL
        let base_url = Url::parse(target_url)
            .map_err(|e| anyhow!("Invalid URL '{}': {}", target_url, e))?;

        info!("🔍 WebGuard SQLi test starting for: {} (param: {})", base_url, parameter);

        // First, get baseline response for comparison
        let baseline = self.get_baseline_response(&base_url, parameter).await;

        let mut payload_results = Vec::new();
        let mut findings = Vec::new();
        let mut finding_id = 1;
        let mut detected_database: Option<String> = None;

        // Test each safe payload
        let payloads_to_test = SAFE_SQLI_PAYLOADS
            .iter()
            .take(self.config.max_payloads);

        for (payload, description, payload_type) in payloads_to_test {
            // Skip payload types based on config
            match payload_type {
                SqliPayloadType::ErrorBased if !self.config.check_error_based => continue,
                SqliPayloadType::BooleanBased if !self.config.check_boolean_based => continue,
                SqliPayloadType::TimeBased if !self.config.check_time_based => continue,
                _ => {}
            }

            let result = self.test_single_payload(&base_url, parameter, payload, description, *payload_type, &baseline).await;
            
            // Check if this payload found a vulnerability
            if result.error_detected || result.time_delay_detected || result.boolean_difference == Some(true) {
                let sqli_type = if result.error_detected {
                    SqliType::ErrorBased
                } else if result.time_delay_detected {
                    SqliType::TimeBlind
                } else {
                    SqliType::BooleanBlind
                };

                let severity = match sqli_type {
                    SqliType::ErrorBased => Severity::High,
                    SqliType::TimeBlind => Severity::Critical,
                    SqliType::BooleanBlind => Severity::High,
                    SqliType::UnionBased => Severity::Critical,
                };

                // Try to detect database type from error message
                if let Some(ref error_msg) = result.error_message {
                    if detected_database.is_none() {
                        detected_database = self.detect_database_type(error_msg);
                    }
                }

                findings.push(SqliFinding {
                    id: format!("SQLI-{:03}", finding_id),
                    sqli_type,
                    severity,
                    parameter: parameter.to_string(),
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    proof_of_concept: self.generate_poc(&base_url, parameter, payload),
                    evidence: self.format_evidence(&result),
                    database_type: detected_database.clone(),
                    remediation: self.get_remediation(sqli_type),
                });
                finding_id += 1;
            }

            payload_results.push(result);
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Calculate summary
        let errors_detected = payload_results.iter().filter(|r| r.error_detected).count();
        let time_delays_detected = payload_results.iter().filter(|r| r.time_delay_detected).count();
        let boolean_differences = payload_results.iter().filter(|r| r.boolean_difference == Some(true)).count();
        let critical_count = findings.iter().filter(|f| f.severity == Severity::Critical).count();
        let high_count = findings.iter().filter(|f| f.severity == Severity::High).count();
        let medium_count = findings.iter().filter(|f| f.severity == Severity::Medium).count();

        let overall_risk = if critical_count > 0 {
            Severity::Critical
        } else if high_count > 0 {
            Severity::High
        } else if medium_count > 0 {
            Severity::Medium
        } else if errors_detected > 0 || time_delays_detected > 0 {
            Severity::Medium
        } else {
            Severity::Info
        };

        let summary = SqliSummary {
            vulnerable: !findings.is_empty(),
            total_findings: findings.len(),
            critical_count,
            high_count,
            medium_count,
            overall_risk,
            detected_database: detected_database.clone(),
        };

        info!(
            "✅ WebGuard SQLi test complete: {} findings ({} critical, {} high)",
            summary.total_findings, summary.critical_count, summary.high_count
        );

        Ok(SqliTestReport {
            id: scan_id,
            target_url: target_url.to_string(),
            parameter: parameter.to_string(),
            scan_time: Utc::now(),
            duration_ms,
            payloads_tested: payload_results.len(),
            errors_detected,
            time_delays_detected,
            boolean_differences,
            findings,
            payload_results,
            summary,
        })
    }

    /// Get baseline response for comparison
    async fn get_baseline_response(&self, base_url: &Url, parameter: &str) -> Option<(u64, usize, String)> {
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, "1");

        let start = std::time::Instant::now();
        match self.client.get(test_url.as_str()).send().await {
            Ok(resp) => {
                let _status = resp.status().as_u16();
                match resp.text().await {
                    Ok(body) => Some((start.elapsed().as_millis() as u64, body.len(), body)),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Test a single SQLi payload
    async fn test_single_payload(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
        payload_type: SqliPayloadType,
        baseline: &Option<(u64, usize, String)>,
    ) -> SqliPayloadResult {
        // Use CDP sandbox session if enabled, otherwise use HTTP
        if self.config.sandbox_mode && self.cdp_port.is_some() {
            self.test_single_payload_cdp(base_url, parameter, payload, description, payload_type, baseline).await
        } else {
            self.test_single_payload_http(base_url, parameter, payload, description, payload_type, baseline).await
        }
    }

    /// Test a single SQLi payload via HTTP (fallback mode)
    async fn test_single_payload_http(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
        payload_type: SqliPayloadType,
        baseline: &Option<(u64, usize, String)>,
    ) -> SqliPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing SQLi payload (HTTP): {} -> {}", description, test_url);

        let start = std::time::Instant::now();

        // Make request
        let response = match self.client.get(test_url.as_str()).send().await {
            Ok(resp) => resp,
            Err(e) => {
                debug!("Request failed for payload '{}': {}", description, e);
                return SqliPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    payload_type,
                    error_detected: false,
                    error_message: None,
                    response_time_ms: start.elapsed().as_millis() as u64,
                    time_delay_detected: false,
                    boolean_difference: None,
                    response_length: 0,
                    status_code: 0,
                };
            }
        };

        let response_time_ms = start.elapsed().as_millis() as u64;
        let status_code = response.status().as_u16();

        // Get response body
        let body = match response.text().await {
            Ok(text) => text,
            Err(_) => {
                return SqliPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    payload_type,
                    error_detected: false,
                    error_message: None,
                    response_time_ms,
                    time_delay_detected: false,
                    boolean_difference: None,
                    response_length: 0,
                    status_code,
                };
            }
        };

        let response_length = body.len();

        // Check for SQL error messages
        let (error_detected, error_message) = self.check_sql_errors(&body);

        // Check for time-based detection
        let time_delay_detected = payload_type == SqliPayloadType::TimeBased 
            && response_time_ms >= self.config.time_delay_threshold_ms;

        // Check for boolean-based detection
        let boolean_difference = if payload_type == SqliPayloadType::BooleanBased {
            if let Some((_, baseline_len, _)) = baseline {
                // Significant length difference indicates boolean injection
                let diff = (response_length as i64 - *baseline_len as i64).abs();
                let threshold = (*baseline_len as f64 * 0.1) as i64; // 10% difference
                Some(diff > threshold || status_code != 200)
            } else {
                None
            }
        } else {
            None
        };

        SqliPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            payload_type,
            error_detected,
            error_message,
            response_time_ms,
            time_delay_detected,
            boolean_difference,
            response_length,
            status_code,
        }
    }

    /// Test a single SQLi payload via CDP sandbox session (isolated, safe)
    async fn test_single_payload_cdp(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
        payload_type: SqliPayloadType,
        baseline: &Option<(u64, usize, String)>,
    ) -> SqliPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing SQLi payload (CDP sandbox): {} -> {}", description, test_url);

        let start = std::time::Instant::now();
        let cdp_port = self.cdp_port.unwrap_or(9222);

        // Connect to CDP sandbox session using browser_orch_ext
        use browser_orch_ext::orchestrator::cdp::CdpConnection;
        
        let mut cdp = match CdpConnection::connect_to_page(cdp_port).await {
            Ok(conn) => conn,
            Err(e) => {
                warn!("Failed to connect to CDP on port {}: {}. Falling back to HTTP.", cdp_port, e);
                return self.test_single_payload_http(base_url, parameter, payload, description, payload_type, baseline).await;
            }
        };

        // Enable Page and Runtime domains for DOM access
        let _ = cdp.send_message("Page.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("Runtime.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("DOM.enable", serde_json::json!({})).await;

        // Navigate to URL with payload (isolated sandbox session)
        let nav_result = cdp.navigate(&test_url.to_string()).await;
        if nav_result.is_err() {
            warn!("CDP navigation failed, falling back to HTTP");
            return self.test_single_payload_http(base_url, parameter, payload, description, payload_type, baseline).await;
        }

        // Wait for page to load (with timeout for time-based SQLi detection)
        // For time-based SQLi, we need to wait longer to detect delays
        if payload_type == SqliPayloadType::TimeBased {
            tokio::time::sleep(Duration::from_millis(self.config.time_delay_threshold_ms + 500)).await;
        } else {
            tokio::time::sleep(Duration::from_millis(1000)).await; // Standard wait
        }

        // Get page content via CDP
        let body = match cdp.evaluate(
            "document.documentElement.outerHTML",
            false
        ).await {
            Ok(result) => {
                // Extract HTML content from CDP result
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            }
            Err(e) => {
                debug!("Failed to get DOM via CDP: {}", e);
                // Fallback: try to get text content
                match cdp.evaluate("document.body ? document.body.innerText : ''", false).await {
                    Ok(result) => {
                        result.get("result")
                            .and_then(|r| r.get("value"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string()
                    }
                    Err(_) => String::new(),
                }
            }
        };

        let response_time_ms = start.elapsed().as_millis() as u64;
        let response_length = body.len();

        // Check for SQL error messages in DOM
        let (error_detected, error_message) = self.check_sql_errors(&body);

        // Also check for SQL errors in visible text content
        let console_errors = match cdp.evaluate(
            "JSON.stringify(Array.from(document.querySelectorAll('*')).map(el => el.textContent).filter(t => t && (t.toLowerCase().includes('sql') || t.toLowerCase().includes('mysql') || t.toLowerCase().includes('syntax error') || t.toLowerCase().includes('database error'))).slice(0, 5))",
            false
        ).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            }
            Err(_) => String::new(),
        };

        // If no error detected in body, check console errors
        let (error_detected, error_message) = if !error_detected && !console_errors.is_empty() {
            (true, Some(format!("Potential SQL error in DOM: {}", console_errors)))
        } else {
            (error_detected, error_message)
        };

        // Check for time-based detection
        let time_delay_detected = payload_type == SqliPayloadType::TimeBased 
            && response_time_ms >= self.config.time_delay_threshold_ms;

        // Check for boolean-based detection
        let boolean_difference = if payload_type == SqliPayloadType::BooleanBased {
            if let Some((_, baseline_len, _)) = baseline {
                // Significant length difference indicates boolean injection
                let diff = (response_length as i64 - *baseline_len as i64).abs();
                let threshold = (*baseline_len as f64 * 0.1) as i64; // 10% difference
                Some(diff > threshold)
            } else {
                None
            }
        } else {
            None
        };

        // Get HTTP status code if available (CDP doesn't always provide this easily)
        let status_code = match cdp.evaluate(
            "window.performance && window.performance.getEntriesByType && window.performance.getEntriesByType('navigation')[0] ? window.performance.getEntriesByType('navigation')[0].responseStatus || 200 : 200",
            false
        ).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(200) as u16
            }
            Err(_) => 200, // Default to 200 if we can't determine
        };

        SqliPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            payload_type,
            error_detected,
            error_message,
            response_time_ms,
            time_delay_detected,
            boolean_difference,
            response_length,
            status_code,
        }
    }

    /// Check response body for SQL error messages
    fn check_sql_errors(&self, body: &str) -> (bool, Option<String>) {
        let body_lower = body.to_lowercase();
        
        for keyword in SQL_ERROR_KEYWORDS {
            if body_lower.contains(&keyword.to_lowercase()) {
                // Extract context around the error
                if let Some(pos) = body_lower.find(&keyword.to_lowercase()) {
                    let start = pos.saturating_sub(50);
                    let end = (pos + keyword.len() + 100).min(body.len());
                    let snippet = &body[start..end];
                    return (true, Some(format!("...{}...", snippet.trim())));
                }
                return (true, Some(keyword.to_string()));
            }
        }
        
        (false, None)
    }

    /// Detect database type from error message
    fn detect_database_type(&self, error_msg: &str) -> Option<String> {
        let lower = error_msg.to_lowercase();
        
        if lower.contains("mysql") || lower.contains("mysqli") {
            Some("MySQL".to_string())
        } else if lower.contains("postgresql") || lower.contains("pg_") {
            Some("PostgreSQL".to_string())
        } else if lower.contains("mssql") || lower.contains("sql server") || lower.contains("sqlserver") {
            Some("Microsoft SQL Server".to_string())
        } else if lower.contains("oracle") || lower.contains("ora-") {
            Some("Oracle".to_string())
        } else if lower.contains("sqlite") {
            Some("SQLite".to_string())
        } else {
            None
        }
    }

    /// Format evidence string from payload result
    fn format_evidence(&self, result: &SqliPayloadResult) -> String {
        let mut evidence = Vec::new();
        
        if result.error_detected {
            if let Some(ref msg) = result.error_message {
                evidence.push(format!("SQL error detected: {}", msg));
            } else {
                evidence.push("SQL error detected in response".to_string());
            }
        }
        
        if result.time_delay_detected {
            evidence.push(format!("Time delay detected: {}ms (threshold: {}ms)", 
                result.response_time_ms, self.config.time_delay_threshold_ms));
        }
        
        if result.boolean_difference == Some(true) {
            evidence.push(format!("Response difference detected (length: {} bytes)", result.response_length));
        }
        
        if evidence.is_empty() {
            "Potential vulnerability indicator".to_string()
        } else {
            evidence.join("; ")
        }
    }

    /// Generate a proof-of-concept URL
    fn generate_poc(&self, base_url: &Url, parameter: &str, payload: &str) -> String {
        let mut poc_url = base_url.clone();
        poc_url.query_pairs_mut().append_pair(parameter, payload);
        poc_url.to_string()
    }

    /// Get remediation advice for SQLi type
    fn get_remediation(&self, sqli_type: SqliType) -> String {
        match sqli_type {
            SqliType::ErrorBased | SqliType::UnionBased => {
                "1. Use parameterized queries (prepared statements) for all database operations\n\
                 2. Implement proper input validation with allowlists\n\
                 3. Use stored procedures with parameterized inputs\n\
                 4. Apply the principle of least privilege to database accounts\n\
                 5. Disable detailed error messages in production\n\
                 6. Consider using an ORM with built-in SQL injection protection\n\
                 7. Deploy a Web Application Firewall (WAF) as defense-in-depth".to_string()
            }
            SqliType::BooleanBlind | SqliType::TimeBlind => {
                "1. Use parameterized queries (prepared statements) - this is critical\n\
                 2. Implement strict input validation and sanitization\n\
                 3. Use stored procedures with parameterized inputs\n\
                 4. Apply rate limiting to prevent automated exploitation\n\
                 5. Monitor for unusual query patterns and response times\n\
                 6. Use database activity monitoring (DAM) solutions\n\
                 7. Deploy a Web Application Firewall (WAF) with SQLi rules".to_string()
            }
        }
    }
}

impl Default for SqliTester {
    fn default() -> Self {
        Self::new().expect("Failed to create default SqliTester instance")
    }
}

/// Format a SQLi test report as Markdown for chat display
pub fn format_sqli_report_markdown(report: &SqliTestReport) -> String {
    let mut md = String::new();

    // Header
    let status_emoji = if report.summary.vulnerable { "🔴" } else { "✅" };
    md.push_str(&format!(
        "## {} WebGuard SQLi Test Report\n\n",
        status_emoji
    ));
    md.push_str(&format!("**Target:** `{}`\n", report.target_url));
    md.push_str(&format!("**Parameter:** `{}`\n", report.parameter));
    md.push_str(&format!(
        "**Scan Time:** {}\n",
        report.scan_time.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Duration:** {}ms\n", report.duration_ms));
    if let Some(ref db) = report.summary.detected_database {
        md.push_str(&format!("**Detected Database:** {}\n", db));
    }
    md.push_str("\n---\n\n");

    // Summary
    md.push_str("### 📊 Summary\n\n");
    if report.summary.vulnerable {
        md.push_str("⚠️ **VULNERABLE TO SQL INJECTION**\n\n");
    } else {
        md.push_str("✅ **No SQL injection vulnerabilities detected**\n\n");
    }

    md.push_str("| Metric | Value |\n|--------|-------|\n");
    md.push_str(&format!("| Payloads Tested | {} |\n", report.payloads_tested));
    md.push_str(&format!("| Errors Detected | {} |\n", report.errors_detected));
    md.push_str(&format!("| Time Delays Detected | {} |\n", report.time_delays_detected));
    md.push_str(&format!("| Boolean Differences | {} |\n", report.boolean_differences));
    md.push_str(&format!("| Total Findings | {} |\n", report.summary.total_findings));
    md.push_str(&format!(
        "| {} Critical | {} |\n",
        Severity::Critical.emoji(),
        report.summary.critical_count
    ));
    md.push_str(&format!(
        "| {} High | {} |\n",
        Severity::High.emoji(),
        report.summary.high_count
    ));
    md.push_str(&format!(
        "| {} Medium | {} |\n",
        Severity::Medium.emoji(),
        report.summary.medium_count
    ));
    md.push_str("\n");

    // Findings
    if !report.findings.is_empty() {
        md.push_str("### 🚨 Vulnerabilities Found\n\n");
        for finding in &report.findings {
            md.push_str(&format!(
                "#### {} {} - {} SQLi\n",
                finding.severity.emoji(),
                finding.id,
                match finding.sqli_type {
                    SqliType::ErrorBased => "Error-based",
                    SqliType::BooleanBlind => "Boolean-blind",
                    SqliType::TimeBlind => "Time-blind",
                    SqliType::UnionBased => "UNION-based",
                }
            ));
            md.push_str(&format!("**Parameter:** `{}`\n", finding.parameter));
            md.push_str(&format!("**Payload:** `{}`\n", finding.payload));
            md.push_str(&format!("**Description:** {}\n", finding.payload_description));
            md.push_str(&format!("**Evidence:** {}\n", finding.evidence));
            if let Some(ref db) = finding.database_type {
                md.push_str(&format!("**Database Type:** {}\n", db));
            }
            md.push_str(&format!("\n**Proof of Concept:**\n```\n{}\n```\n", finding.proof_of_concept));
            md.push_str(&format!("\n**Remediation:**\n{}\n\n", finding.remediation));
        }
    }

    // Payload Results Table
    md.push_str("### 📋 Payload Test Results\n\n");
    md.push_str("| Type | Payload | Error | Time (ms) | Status |\n|------|---------|-------|-----------|--------|\n");
    for result in &report.payload_results {
        let type_badge = result.payload_type.badge();
        let error = if result.error_detected { "⚠️ Yes" } else { "No" };
        let time_indicator = if result.time_delay_detected { 
            format!("⏱️ {}", result.response_time_ms) 
        } else { 
            result.response_time_ms.to_string() 
        };
        // Truncate payload for display
        let payload_display: String = result.payload.chars().take(25).collect();
        let payload_display = if result.payload.len() > 25 {
            format!("{}...", payload_display)
        } else {
            payload_display
        };
        md.push_str(&format!(
            "| {} | `{}` | {} | {} | {} |\n",
            type_badge,
            payload_display.replace('|', "\\|"),
            error,
            time_indicator,
            result.status_code
        ));
    }
    md.push_str("\n");

    md.push_str("---\n");
    md.push_str(&format!("*Scan ID: {}*\n", report.id));
    md.push_str("\n⚠️ **Note:** This is a safe SQLi test. No destructive payloads were used and no data was modified.\n");

    md
}

/// Format a brief SQLi summary for notifications
pub fn format_sqli_notification_summary(report: &SqliTestReport) -> String {
    if report.summary.vulnerable {
        let db_info = report.summary.detected_database
            .as_ref()
            .map(|db| format!(" [{}]", db))
            .unwrap_or_default();
        format!(
            "🔴 SQL INJECTION VULNERABILITY DETECTED on {} (param: {}){} - {} findings ({} critical)",
            report.target_url,
            report.parameter,
            db_info,
            report.summary.total_findings,
            report.summary.critical_count
        )
    } else {
        format!(
            "✅ SQLi test complete for {} (param: {}) - No vulnerabilities found",
            report.target_url,
            report.parameter
        )
    }
}

// ============================================================================
// Phase 28f: Open Redirect Testing
// ============================================================================

/// Safe open redirect test payloads
const SAFE_REDIRECT_PAYLOADS: &[(&str, &str)] = &[
    ("//evil.com", "Protocol-relative URL to external domain"),
    ("//google.com", "Protocol-relative URL to known domain"),
    ("///evil.com", "Triple-slash redirect"),
    ("/\\evil.com", "Mixed slash redirect"),
    ("javascript:alert(1)", "JavaScript protocol"),
    ("data:text/html,<script>alert(1)</script>", "Data URI"),
    ("//evil.com%00.example.com", "Null byte bypass"),
    ("//evil.com%0d%0a.example.com", "CRLF bypass"),
    ("@evil.com", "At-sign redirect"),
    ("//evil.com#@example.com", "Fragment bypass"),
];

/// Open redirect test result for a single payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectPayloadResult {
    pub payload: String,
    pub payload_description: String,
    pub redirect_detected: bool,
    pub final_location: Option<String>,
    pub redirect_chain: Vec<String>,
    pub status_codes: Vec<u16>,
    pub is_external: bool,
    pub is_javascript: bool,
}

/// Open redirect vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectFinding {
    pub id: String,
    pub severity: Severity,
    pub parameter: String,
    pub payload: String,
    pub payload_description: String,
    pub proof_of_concept: String,
    pub evidence: String,
    pub final_location: String,
    pub remediation: String,
}

/// Complete open redirect test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectTestReport {
    pub id: String,
    pub target_url: String,
    pub parameter: String,
    pub scan_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub payloads_tested: usize,
    pub redirects_detected: usize,
    pub external_redirects: usize,
    pub javascript_redirects: usize,
    pub findings: Vec<RedirectFinding>,
    pub payload_results: Vec<RedirectPayloadResult>,
    pub summary: RedirectSummary,
}

/// Open redirect test summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectSummary {
    pub vulnerable: bool,
    pub total_findings: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub overall_risk: Severity,
}

/// Open redirect test configuration
#[derive(Debug, Clone)]
pub struct RedirectTestConfig {
    pub timeout_secs: u64,
    pub user_agent: String,
    pub max_payloads: usize,
    pub sandbox_mode: bool,
}

impl Default for RedirectTestConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            max_payloads: 10,
            sandbox_mode: true,
        }
    }
}

/// Open Redirect Tester
pub struct RedirectTester {
    client: Client,
    config: RedirectTestConfig,
    cdp_port: Option<u16>, // Chrome DevTools Protocol port for sandbox sessions
}

impl RedirectTester {
    /// Create a new redirect tester with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(RedirectTestConfig::default())
    }

    /// Create a new redirect tester with custom configuration
    pub fn with_config(config: RedirectTestConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::none()) // Don't follow redirects automatically
            .danger_accept_invalid_certs(false)
            .build()?;

        // Get CDP port from environment if sandbox mode is enabled
        let cdp_port = if config.sandbox_mode {
            std::env::var("CHROME_DEBUG_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .or_else(|| {
                    std::env::var("BROWSER_DEBUG_PORT")
                        .ok()
                        .and_then(|s| s.parse().ok())
                })
                .or(Some(9222)) // Default CDP port
        } else {
            None
        };

        Ok(Self { 
            client, 
            config,
            cdp_port,
        })
    }

    /// Test a URL parameter for open redirect vulnerabilities
    pub async fn test_redirect(&self, target_url: &str, parameter: &str) -> Result<RedirectTestReport> {
        let start_time = std::time::Instant::now();
        let scan_id = Uuid::new_v4().to_string();

        // Validate URL
        let base_url = Url::parse(target_url)
            .map_err(|e| anyhow!("Invalid URL '{}': {}", target_url, e))?;

        info!("🔍 WebGuard Open Redirect test starting for: {} (param: {})", base_url, parameter);

        let mut payload_results = Vec::new();
        let mut findings = Vec::new();
        let mut finding_id = 1;

        // Test each safe payload
        let payloads_to_test = SAFE_REDIRECT_PAYLOADS
            .iter()
            .take(self.config.max_payloads);

        for (payload, description) in payloads_to_test {
            let result = self.test_single_redirect_payload(&base_url, parameter, payload, description).await;
            
            // Check if this payload found a vulnerability
            if result.redirect_detected && (result.is_external || result.is_javascript) {
                let severity = if result.is_javascript {
                    Severity::High
                } else if result.is_external {
                    Severity::Medium
                } else {
                    Severity::Low
                };

                findings.push(RedirectFinding {
                    id: format!("REDIR-{:03}", finding_id),
                    severity,
                    parameter: parameter.to_string(),
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    proof_of_concept: self.generate_redirect_poc(&base_url, parameter, payload),
                    evidence: self.format_redirect_evidence(&result),
                    final_location: result.final_location.clone().unwrap_or_default(),
                    remediation: self.get_redirect_remediation(),
                });
                finding_id += 1;
            }

            payload_results.push(result);
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Calculate summary
        let redirects_detected = payload_results.iter().filter(|r| r.redirect_detected).count();
        let external_redirects = payload_results.iter().filter(|r| r.is_external).count();
        let javascript_redirects = payload_results.iter().filter(|r| r.is_javascript).count();
        let high_count = findings.iter().filter(|f| f.severity == Severity::High).count();
        let medium_count = findings.iter().filter(|f| f.severity == Severity::Medium).count();

        let overall_risk = if high_count > 0 {
            Severity::High
        } else if medium_count > 0 {
            Severity::Medium
        } else if redirects_detected > 0 {
            Severity::Low
        } else {
            Severity::Info
        };

        let summary = RedirectSummary {
            vulnerable: !findings.is_empty(),
            total_findings: findings.len(),
            high_count,
            medium_count,
            overall_risk,
        };

        info!(
            "✅ WebGuard Open Redirect test complete: {} findings ({} high, {} medium)",
            summary.total_findings, summary.high_count, summary.medium_count
        );

        Ok(RedirectTestReport {
            id: scan_id,
            target_url: target_url.to_string(),
            parameter: parameter.to_string(),
            scan_time: Utc::now(),
            duration_ms,
            payloads_tested: payload_results.len(),
            redirects_detected,
            external_redirects,
            javascript_redirects,
            findings,
            payload_results,
            summary,
        })
    }

    /// Test a single redirect payload
    async fn test_single_redirect_payload(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> RedirectPayloadResult {
        // Use CDP sandbox session if enabled, otherwise use HTTP
        if self.config.sandbox_mode && self.cdp_port.is_some() {
            self.test_single_redirect_payload_cdp(base_url, parameter, payload, description).await
        } else {
            self.test_single_redirect_payload_http(base_url, parameter, payload, description).await
        }
    }

    /// Test a single redirect payload via HTTP (fallback mode)
    async fn test_single_redirect_payload_http(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> RedirectPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing redirect payload: {} -> {}", description, test_url);

        let mut redirect_chain = vec![test_url.to_string()];
        let mut status_codes = Vec::new();
        let mut current_url = test_url.to_string();
        let mut redirect_detected = false;
        let mut is_external = false;
        let mut is_javascript = false;

        // Follow redirect chain manually (max 5 hops)
        for _ in 0..5 {
            match self.client.get(&current_url).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    status_codes.push(status.as_u16());

                    // Check for redirect status codes
                    if status.is_redirection() {
                        redirect_detected = true;
                        if let Some(location) = resp.headers().get("location") {
                            if let Ok(location_str) = location.to_str() {
                                redirect_chain.push(location_str.to_string());
                                
                                // Check if redirect is external
                                if let Ok(redirect_url) = Url::parse(location_str) {
                                    if redirect_url.host_str() != base_url.host_str() {
                                        is_external = true;
                                    }
                                    current_url = redirect_url.to_string();
                                } else {
                                    // Relative URL - resolve it
                                    if let Ok(resolved) = base_url.join(location_str) {
                                        current_url = resolved.to_string();
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        // Not a redirect, check response body for meta refresh or javascript redirect
                        if let Ok(body) = resp.text().await {
                            let body_lower = body.to_lowercase();
                            if body_lower.contains("window.location") ||
                               body_lower.contains("location.href") ||
                               body_lower.contains("location.replace") ||
                               body_lower.contains("<meta") && body_lower.contains("refresh") {
                                redirect_detected = true;
                                is_javascript = true;
                            }
                        }
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        // Check if payload itself is javascript protocol
        if payload.starts_with("javascript:") || payload.starts_with("data:") {
            is_javascript = true;
            redirect_detected = true;
        }

        RedirectPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            redirect_detected,
            final_location: redirect_chain.last().cloned(),
            redirect_chain,
            status_codes,
            is_external,
            is_javascript,
        }
    }

    /// Test a single redirect payload via CDP sandbox session (isolated, safe)
    async fn test_single_redirect_payload_cdp(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> RedirectPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing redirect payload (CDP sandbox): {} -> {}", description, test_url);

        let mut redirect_chain = vec![test_url.to_string()];
        let mut status_codes = Vec::new();
        let mut redirect_detected = false;
        let mut is_external = false;
        let mut is_javascript = false;
        let cdp_port = self.cdp_port.unwrap_or(9222);

        // Connect to CDP sandbox session
        use browser_orch_ext::orchestrator::cdp::CdpConnection;
        
        let mut cdp = match CdpConnection::connect_to_page(cdp_port).await {
            Ok(conn) => conn,
            Err(e) => {
                warn!("Failed to connect to CDP on port {}: {}. Falling back to HTTP.", cdp_port, e);
                return self.test_single_redirect_payload_http(base_url, parameter, payload, description).await;
            }
        };

        // Enable Page and Network domains for redirect tracking
        let _ = cdp.send_message("Page.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("Runtime.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("Network.enable", serde_json::json!({})).await;

        // Navigate to URL with payload (isolated sandbox session)
        let nav_result = cdp.navigate(&test_url.to_string()).await;
        if nav_result.is_err() {
            warn!("CDP navigation failed, falling back to HTTP");
            return self.test_single_redirect_payload_http(base_url, parameter, payload, description).await;
        }

        // Wait for navigation to complete and capture redirects
        tokio::time::sleep(Duration::from_millis(2000)).await;

        // Get final URL after navigation (CDP tracks redirects automatically)
        let final_url = match cdp.evaluate("window.location.href", false).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            }
            Err(_) => String::new(),
        };

        // Get HTTP status code
        let status_code = match cdp.evaluate(
            "window.performance && window.performance.getEntriesByType && window.performance.getEntriesByType('navigation')[0] ? window.performance.getEntriesByType('navigation')[0].responseStatus || 200 : 200",
            false
        ).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(200) as u16
            }
            Err(_) => 200,
        };

        status_codes.push(status_code);

        // Check if we were redirected
        if !final_url.is_empty() && final_url != test_url.to_string() {
            redirect_detected = true;
            redirect_chain.push(final_url.clone());

            // Check if redirect is external
            if let Ok(redirect_url) = Url::parse(&final_url) {
                if redirect_url.host_str() != base_url.host_str() {
                    is_external = true;
                }
            }
        }

        // Check for JavaScript redirects in the page
        let page_content = match cdp.evaluate("document.documentElement.outerHTML", false).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            }
            Err(_) => String::new(),
        };

        let content_lower = page_content.to_lowercase();
        if content_lower.contains("window.location") ||
           content_lower.contains("location.href") ||
           content_lower.contains("location.replace") ||
           (content_lower.contains("<meta") && content_lower.contains("refresh")) {
            redirect_detected = true;
            is_javascript = true;
        }

        // Check if payload itself is javascript protocol
        if payload.starts_with("javascript:") || payload.starts_with("data:") {
            is_javascript = true;
            redirect_detected = true;
        }

        // Check for redirect status codes (3xx)
        if status_code >= 300 && status_code < 400 {
            redirect_detected = true;
        }

        RedirectPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            redirect_detected,
            final_location: if !final_url.is_empty() && final_url != test_url.to_string() {
                Some(final_url)
            } else {
                redirect_chain.last().cloned()
            },
            redirect_chain,
            status_codes,
            is_external,
            is_javascript,
        }
    }

    /// Format evidence string from redirect result
    fn format_redirect_evidence(&self, result: &RedirectPayloadResult) -> String {
        let mut evidence = Vec::new();
        
        if result.is_javascript {
            evidence.push("JavaScript/Data URI redirect detected".to_string());
        }
        
        if result.is_external {
            evidence.push(format!("External redirect to: {}", result.final_location.as_ref().unwrap_or(&"unknown".to_string())));
        }
        
        if result.redirect_chain.len() > 1 {
            evidence.push(format!("Redirect chain: {} hops", result.redirect_chain.len() - 1));
        }
        
        if !result.status_codes.is_empty() {
            evidence.push(format!("Status codes: {:?}", result.status_codes));
        }
        
        if evidence.is_empty() {
            "Potential open redirect vulnerability".to_string()
        } else {
            evidence.join("; ")
        }
    }

    /// Generate a proof-of-concept URL
    fn generate_redirect_poc(&self, base_url: &Url, parameter: &str, payload: &str) -> String {
        let mut poc_url = base_url.clone();
        poc_url.query_pairs_mut().append_pair(parameter, payload);
        poc_url.to_string()
    }

    /// Get remediation advice for open redirect
    fn get_redirect_remediation(&self) -> String {
        "1. Validate all redirect URLs against an allowlist of trusted domains\n\
         2. Use relative paths for internal redirects instead of full URLs\n\
         3. Implement a redirect token/hash system to verify legitimate redirects\n\
         4. Reject redirects to external domains or javascript: protocols\n\
         5. Display a warning page before redirecting to external sites\n\
         6. Use the Referer-Policy header to control referrer information\n\
         7. Implement Content Security Policy (CSP) to prevent malicious redirects".to_string()
    }
}

impl Default for RedirectTester {
    fn default() -> Self {
        Self::new().expect("Failed to create default RedirectTester instance")
    }
}

/// Format a redirect test report as Markdown for chat display
pub fn format_redirect_report_markdown(report: &RedirectTestReport) -> String {
    let mut md = String::new();

    // Header
    let status_emoji = if report.summary.vulnerable { "🔴" } else { "✅" };
    md.push_str(&format!(
        "## {} WebGuard Open Redirect Test Report\n\n",
        status_emoji
    ));
    md.push_str(&format!("**Target:** `{}`\n", report.target_url));
    md.push_str(&format!("**Parameter:** `{}`\n", report.parameter));
    md.push_str(&format!(
        "**Scan Time:** {}\n",
        report.scan_time.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Duration:** {}ms\n", report.duration_ms));
    md.push_str("\n---\n\n");

    // Summary
    md.push_str("### 📊 Summary\n\n");
    if report.summary.vulnerable {
        md.push_str("⚠️ **VULNERABLE TO OPEN REDIRECT**\n\n");
    } else {
        md.push_str("✅ **No open redirect vulnerabilities detected**\n\n");
    }

    md.push_str("| Metric | Value |\n|--------|-------|\n");
    md.push_str(&format!("| Payloads Tested | {} |\n", report.payloads_tested));
    md.push_str(&format!("| Redirects Detected | {} |\n", report.redirects_detected));
    md.push_str(&format!("| External Redirects | {} |\n", report.external_redirects));
    md.push_str(&format!("| JavaScript Redirects | {} |\n", report.javascript_redirects));
    md.push_str(&format!("| Total Findings | {} |\n", report.summary.total_findings));
    md.push_str(&format!(
        "| {} High | {} |\n",
        Severity::High.emoji(),
        report.summary.high_count
    ));
    md.push_str(&format!(
        "| {} Medium | {} |\n",
        Severity::Medium.emoji(),
        report.summary.medium_count
    ));
    md.push_str("\n");

    // Findings
    if !report.findings.is_empty() {
        md.push_str("### 🔍 Vulnerabilities Found\n\n");
        for finding in &report.findings {
            md.push_str(&format!(
                "#### {} {} - {}\n\n",
                finding.severity.emoji(),
                finding.id,
                finding.severity.badge()
            ));
            md.push_str(&format!("**Parameter:** `{}`\n", finding.parameter));
            md.push_str(&format!("**Payload:** `{}`\n", finding.payload));
            md.push_str(&format!("**Description:** {}\n", finding.payload_description));
            md.push_str(&format!("**Final Location:** `{}`\n", finding.final_location));
            md.push_str(&format!("**Evidence:** {}\n\n", finding.evidence));
            md.push_str(&format!("**Proof of Concept:**\n```\n{}\n```\n\n", finding.proof_of_concept));
            md.push_str(&format!("**Remediation:**\n{}\n\n", finding.remediation));
            md.push_str("---\n\n");
        }
    }

    // Payload Results Table
    md.push_str("### 📋 Payload Test Results\n\n");
    md.push_str("| Payload | Description | Redirect | External | JavaScript | Status |\n");
    md.push_str("|---------|-------------|----------|----------|------------|--------|\n");
    for result in &report.payload_results {
        let payload_display = result.payload.replace('|', "\\|");
        let redirect = if result.redirect_detected { "✅" } else { "❌" };
        let external = if result.is_external { "⚠️" } else { "-" };
        let javascript = if result.is_javascript { "🔴" } else { "-" };
        let status = if !result.status_codes.is_empty() {
            format!("{}", result.status_codes[0])
        } else {
            "-".to_string()
        };
        md.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            payload_display,
            result.payload_description,
            redirect,
            external,
            javascript,
            status
        ));
    }
    md.push_str("\n");

    md.push_str("---\n");
    md.push_str(&format!("*Scan ID: {}*\n", report.id));
    md.push_str("\n⚠️ **Note:** This is a safe open redirect test. No malicious redirects were executed.\n");

    md
}

/// Format a brief redirect summary for notifications
pub fn format_redirect_notification_summary(report: &RedirectTestReport) -> String {
    if report.summary.vulnerable {
        format!(
            "🔴 OPEN REDIRECT VULNERABILITY DETECTED on {} (param: {}) - {} findings ({} high)",
            report.target_url,
            report.parameter,
            report.summary.total_findings,
            report.summary.high_count
        )
    } else {
        format!(
            "✅ Open redirect test complete for {} (param: {}) - No vulnerabilities found",
            report.target_url,
            report.parameter
        )
    }
}

// ============================================================================
// Phase 28g: Command Injection Testing (Very Limited/Safe)
// ============================================================================

/// Extremely safe command injection detection payloads (no real execution)
const SAFE_CMDINJ_PAYLOADS: &[(&str, &str)] = &[
    (";echo vulnerable", "Semicolon command separator"),
    ("|echo vulnerable", "Pipe command separator"),
    ("&&echo vulnerable", "AND command separator"),
    ("`echo vulnerable`", "Backtick command substitution"),
    ("$(echo vulnerable)", "Dollar-paren command substitution"),
    (";id", "Semicolon with id command"),
    ("|id", "Pipe with id command"),
    ("&&id", "AND with id command"),
    ("`id`", "Backtick with id command"),
    ("$(id)", "Dollar-paren with id command"),
];

/// Command injection error keywords (indicators of injection)
const CMDINJ_ERROR_KEYWORDS: &[&str] = &[
    "command not found",
    "sh: ",
    "bash: ",
    "/bin/sh",
    "/bin/bash",
    "syntax error",
    "unexpected token",
    "Permission denied",
    "cannot execute",
    "No such file or directory",
    "vulnerable", // Our test echo output
];

/// Command injection test result for a single payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdInjPayloadResult {
    pub payload: String,
    pub payload_description: String,
    pub injection_detected: bool,
    pub error_message: Option<String>,
    pub response_length: usize,
    pub status_code: u16,
}

/// Command injection vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdInjFinding {
    pub id: String,
    pub severity: Severity,
    pub parameter: String,
    pub payload: String,
    pub payload_description: String,
    pub proof_of_concept: String,
    pub evidence: String,
    pub remediation: String,
}

/// Complete command injection test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdInjTestReport {
    pub id: String,
    pub target_url: String,
    pub parameter: String,
    pub scan_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub payloads_tested: usize,
    pub injections_detected: usize,
    pub findings: Vec<CmdInjFinding>,
    pub payload_results: Vec<CmdInjPayloadResult>,
    pub summary: CmdInjSummary,
}

/// Command injection test summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdInjSummary {
    pub vulnerable: bool,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub overall_risk: Severity,
}

/// Command injection test configuration
#[derive(Debug, Clone)]
pub struct CmdInjTestConfig {
    pub timeout_secs: u64,
    pub user_agent: String,
    pub max_payloads: usize,
    pub sandbox_mode: bool,
}

impl Default for CmdInjTestConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            max_payloads: 10,
            sandbox_mode: true,
        }
    }
}

/// Command Injection Tester (Very Limited/Safe)
pub struct CmdInjTester {
    client: Client,
    config: CmdInjTestConfig,
    cdp_port: Option<u16>, // Chrome DevTools Protocol port for sandbox sessions
}

impl CmdInjTester {
    /// Create a new command injection tester with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(CmdInjTestConfig::default())
    }

    /// Create a new command injection tester with custom configuration
    pub fn with_config(config: CmdInjTestConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(5))
            .danger_accept_invalid_certs(false)
            .build()?;

        // Get CDP port from environment if sandbox mode is enabled
        let cdp_port = if config.sandbox_mode {
            std::env::var("CHROME_DEBUG_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .or_else(|| {
                    std::env::var("BROWSER_DEBUG_PORT")
                        .ok()
                        .and_then(|s| s.parse().ok())
                })
                .or(Some(9222)) // Default CDP port
        } else {
            None
        };

        Ok(Self { 
            client, 
            config,
            cdp_port,
        })
    }

    /// Test a URL parameter for command injection vulnerabilities
    pub async fn test_cmdinj(&self, target_url: &str, parameter: &str) -> Result<CmdInjTestReport> {
        let start_time = std::time::Instant::now();
        let scan_id = Uuid::new_v4().to_string();

        // Validate URL
        let base_url = Url::parse(target_url)
            .map_err(|e| anyhow!("Invalid URL '{}': {}", target_url, e))?;

        info!("🔍 WebGuard Command Injection test starting for: {} (param: {})", base_url, parameter);

        let mut payload_results = Vec::new();
        let mut findings = Vec::new();
        let mut finding_id = 1;

        // Test each safe payload
        let payloads_to_test = SAFE_CMDINJ_PAYLOADS
            .iter()
            .take(self.config.max_payloads);

        for (payload, description) in payloads_to_test {
            let result = self.test_single_cmdinj_payload(&base_url, parameter, payload, description).await;
            
            // Check if this payload found a vulnerability
            if result.injection_detected {
                findings.push(CmdInjFinding {
                    id: format!("CMDINJ-{:03}", finding_id),
                    severity: Severity::Critical,
                    parameter: parameter.to_string(),
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    proof_of_concept: self.generate_cmdinj_poc(&base_url, parameter, payload),
                    evidence: self.format_cmdinj_evidence(&result),
                    remediation: self.get_cmdinj_remediation(),
                });
                finding_id += 1;
            }

            payload_results.push(result);
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Calculate summary
        let injections_detected = payload_results.iter().filter(|r| r.injection_detected).count();
        let critical_count = findings.iter().filter(|f| f.severity == Severity::Critical).count();
        let high_count = findings.iter().filter(|f| f.severity == Severity::High).count();

        let overall_risk = if critical_count > 0 {
            Severity::Critical
        } else if high_count > 0 {
            Severity::High
        } else if injections_detected > 0 {
            Severity::Medium
        } else {
            Severity::Info
        };

        let summary = CmdInjSummary {
            vulnerable: !findings.is_empty(),
            total_findings: findings.len(),
            critical_count,
            high_count,
            overall_risk,
        };

        info!(
            "✅ WebGuard Command Injection test complete: {} findings ({} critical, {} high)",
            summary.total_findings, summary.critical_count, summary.high_count
        );

        Ok(CmdInjTestReport {
            id: scan_id,
            target_url: target_url.to_string(),
            parameter: parameter.to_string(),
            scan_time: Utc::now(),
            duration_ms,
            payloads_tested: payload_results.len(),
            injections_detected,
            findings,
            payload_results,
            summary,
        })
    }

    /// Test a single command injection payload
    async fn test_single_cmdinj_payload(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> CmdInjPayloadResult {
        // Use CDP sandbox session if enabled, otherwise use HTTP
        if self.config.sandbox_mode && self.cdp_port.is_some() {
            self.test_single_cmdinj_payload_cdp(base_url, parameter, payload, description).await
        } else {
            self.test_single_cmdinj_payload_http(base_url, parameter, payload, description).await
        }
    }

    /// Test a single command injection payload via HTTP (fallback mode)
    async fn test_single_cmdinj_payload_http(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> CmdInjPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing command injection payload (HTTP): {} -> {}", description, test_url);

        // Make request
        let response = match self.client.get(test_url.as_str()).send().await {
            Ok(resp) => resp,
            Err(e) => {
                debug!("Request failed for payload '{}': {}", description, e);
                return CmdInjPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    injection_detected: false,
                    error_message: None,
                    response_length: 0,
                    status_code: 0,
                };
            }
        };

        let status_code = response.status().as_u16();

        // Get response body
        let body = match response.text().await {
            Ok(text) => text,
            Err(_) => {
                return CmdInjPayloadResult {
                    payload: payload.to_string(),
                    payload_description: description.to_string(),
                    injection_detected: false,
                    error_message: None,
                    response_length: 0,
                    status_code,
                };
            }
        };

        let response_length = body.len();

        // Check for command injection error messages
        let (injection_detected, error_message) = self.check_cmdinj_errors(&body);

        CmdInjPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            injection_detected,
            error_message,
            response_length,
            status_code,
        }
    }

    /// Test a single command injection payload via CDP sandbox session (isolated, safe)
    async fn test_single_cmdinj_payload_cdp(
        &self,
        base_url: &Url,
        parameter: &str,
        payload: &str,
        description: &str,
    ) -> CmdInjPayloadResult {
        // Build URL with payload
        let mut test_url = base_url.clone();
        test_url.query_pairs_mut().append_pair(parameter, payload);

        debug!("Testing command injection payload (CDP sandbox): {} -> {}", description, test_url);

        let cdp_port = self.cdp_port.unwrap_or(9222);

        // Connect to CDP sandbox session
        use browser_orch_ext::orchestrator::cdp::CdpConnection;
        
        let mut cdp = match CdpConnection::connect_to_page(cdp_port).await {
            Ok(conn) => conn,
            Err(e) => {
                warn!("Failed to connect to CDP on port {}: {}. Falling back to HTTP.", cdp_port, e);
                return self.test_single_cmdinj_payload_http(base_url, parameter, payload, description).await;
            }
        };

        // Enable Page and Runtime domains for DOM access
        let _ = cdp.send_message("Page.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("Runtime.enable", serde_json::json!({})).await;
        let _ = cdp.send_message("DOM.enable", serde_json::json!({})).await;

        // Navigate to URL with payload (isolated sandbox session)
        let nav_result = cdp.navigate(&test_url.to_string()).await;
        if nav_result.is_err() {
            warn!("CDP navigation failed, falling back to HTTP");
            return self.test_single_cmdinj_payload_http(base_url, parameter, payload, description).await;
        }

        // Wait for page to load
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Get page content via CDP
        let body = match cdp.evaluate(
            "document.documentElement.outerHTML",
            false
        ).await {
            Ok(result) => {
                // Extract HTML content from CDP result
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            }
            Err(e) => {
                debug!("Failed to get DOM via CDP: {}", e);
                // Fallback: try to get text content
                match cdp.evaluate("document.body ? document.body.innerText : ''", false).await {
                    Ok(result) => {
                        result.get("result")
                            .and_then(|r| r.get("value"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string()
                    }
                    Err(_) => String::new(),
                }
            }
        };

        let response_length = body.len();

        // Check for command injection error messages in DOM
        let (injection_detected, error_message) = self.check_cmdinj_errors(&body);

        // Also check for command injection indicators in visible text
        let cmd_indicators = match cdp.evaluate(
            "JSON.stringify(Array.from(document.querySelectorAll('*')).map(el => el.textContent).filter(t => t && (t.toLowerCase().includes('command not found') || t.toLowerCase().includes('sh:') || t.toLowerCase().includes('bash:') || t.toLowerCase().includes('vulnerable'))).slice(0, 5))",
            false
        ).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            }
            Err(_) => String::new(),
        };

        // If no injection detected in body, check command indicators
        let (injection_detected, error_message) = if !injection_detected && !cmd_indicators.is_empty() {
            (true, Some(format!("Potential command injection indicator in DOM: {}", cmd_indicators)))
        } else {
            (injection_detected, error_message)
        };

        // Get HTTP status code if available
        let status_code = match cdp.evaluate(
            "window.performance && window.performance.getEntriesByType && window.performance.getEntriesByType('navigation')[0] ? window.performance.getEntriesByType('navigation')[0].responseStatus || 200 : 200",
            false
        ).await {
            Ok(result) => {
                result.get("result")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(200) as u16
            }
            Err(_) => 200, // Default to 200 if we can't determine
        };

        CmdInjPayloadResult {
            payload: payload.to_string(),
            payload_description: description.to_string(),
            injection_detected,
            error_message,
            response_length,
            status_code,
        }
    }

    /// Check response body for command injection error messages
    fn check_cmdinj_errors(&self, body: &str) -> (bool, Option<String>) {
        let body_lower = body.to_lowercase();
        
        for keyword in CMDINJ_ERROR_KEYWORDS {
            if body_lower.contains(&keyword.to_lowercase()) {
                // Extract context around the error
                if let Some(pos) = body_lower.find(&keyword.to_lowercase()) {
                    let start = pos.saturating_sub(50);
                    let end = (pos + keyword.len() + 100).min(body.len());
                    let snippet = &body[start..end];
                    return (true, Some(format!("...{}...", snippet.trim())));
                }
                return (true, Some(keyword.to_string()));
            }
        }
        
        (false, None)
    }

    /// Format evidence string from command injection result
    fn format_cmdinj_evidence(&self, result: &CmdInjPayloadResult) -> String {
        if let Some(ref msg) = result.error_message {
            format!("Command injection indicator detected: {}", msg)
        } else {
            "Potential command injection vulnerability".to_string()
        }
    }

    /// Generate a proof-of-concept URL
    fn generate_cmdinj_poc(&self, base_url: &Url, parameter: &str, payload: &str) -> String {
        let mut poc_url = base_url.clone();
        poc_url.query_pairs_mut().append_pair(parameter, payload);
        poc_url.to_string()
    }

    /// Get remediation advice for command injection
    fn get_cmdinj_remediation(&self) -> String {
        "1. NEVER pass user input directly to system commands or shell functions\n\
         2. Use parameterized APIs instead of shell commands when possible\n\
         3. Implement strict input validation with allowlists (not blocklists)\n\
         4. Escape all special shell characters if shell execution is unavoidable\n\
         5. Use language-specific safe APIs (e.g., subprocess with shell=False in Python)\n\
         6. Apply the principle of least privilege to application processes\n\
         7. Implement application sandboxing and containerization\n\
         8. Deploy a Web Application Firewall (WAF) with command injection rules".to_string()
    }
}

impl Default for CmdInjTester {
    fn default() -> Self {
        Self::new().expect("Failed to create default CmdInjTester instance")
    }
}

/// Format a command injection test report as Markdown for chat display
pub fn format_cmdinj_report_markdown(report: &CmdInjTestReport) -> String {
    let mut md = String::new();

    // Header
    let status_emoji = if report.summary.vulnerable { "🔴" } else { "✅" };
    md.push_str(&format!(
        "## {} WebGuard Command Injection Test Report\n\n",
        status_emoji
    ));
    md.push_str(&format!("**Target:** `{}`\n", report.target_url));
    md.push_str(&format!("**Parameter:** `{}`\n", report.parameter));
    md.push_str(&format!(
        "**Scan Time:** {}\n",
        report.scan_time.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Duration:** {}ms\n", report.duration_ms));
    md.push_str("\n---\n\n");

    // Summary
    md.push_str("### 📊 Summary\n\n");
    if report.summary.vulnerable {
        md.push_str("⚠️ **VULNERABLE TO COMMAND INJECTION**\n\n");
    } else {
        md.push_str("✅ **No command injection vulnerabilities detected**\n\n");
    }

    md.push_str("| Metric | Value |\n|--------|-------|\n");
    md.push_str(&format!("| Payloads Tested | {} |\n", report.payloads_tested));
    md.push_str(&format!("| Injections Detected | {} |\n", report.injections_detected));
    md.push_str(&format!("| Total Findings | {} |\n", report.summary.total_findings));
    md.push_str(&format!(
        "| {} Critical | {} |\n",
        Severity::Critical.emoji(),
        report.summary.critical_count
    ));
    md.push_str(&format!(
        "| {} High | {} |\n",
        Severity::High.emoji(),
        report.summary.high_count
    ));
    md.push_str("\n");

    // Findings
    if !report.findings.is_empty() {
        md.push_str("### 🔍 Vulnerabilities Found\n\n");
        for finding in &report.findings {
            md.push_str(&format!(
                "#### {} {} - {}\n\n",
                finding.severity.emoji(),
                finding.id,
                finding.severity.badge()
            ));
            md.push_str(&format!("**Parameter:** `{}`\n", finding.parameter));
            md.push_str(&format!("**Payload:** `{}`\n", finding.payload));
            md.push_str(&format!("**Description:** {}\n", finding.payload_description));
            md.push_str(&format!("**Evidence:** {}\n\n", finding.evidence));
            md.push_str(&format!("**Proof of Concept:**\n```\n{}\n```\n\n", finding.proof_of_concept));
            md.push_str(&format!("**Remediation:**\n{}\n\n", finding.remediation));
            md.push_str("---\n\n");
        }
    }

    // Payload Results Table
    md.push_str("### 📋 Payload Test Results\n\n");
    md.push_str("| Payload | Description | Injection Detected | Status |\n");
    md.push_str("|---------|-------------|-------------------|--------|\n");
    for result in &report.payload_results {
        let payload_display = result.payload.replace('|', "\\|");
        let injection = if result.injection_detected { "🔴 YES" } else { "✅ NO" };
        md.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            payload_display,
            result.payload_description,
            injection,
            result.status_code
        ));
    }
    md.push_str("\n");

    md.push_str("---\n");
    md.push_str(&format!("*Scan ID: {}*\n", report.id));
    md.push_str("\n⚠️ **Note:** This is a safe command injection test. No actual commands were executed on the host system.\n");

    md
}

/// Format a brief command injection summary for notifications
pub fn format_cmdinj_notification_summary(report: &CmdInjTestReport) -> String {
    if report.summary.vulnerable {
        format!(
            "🔴 COMMAND INJECTION VULNERABILITY DETECTED on {} (param: {}) - {} findings ({} critical)",
            report.target_url,
            report.parameter,
            report.summary.total_findings,
            report.summary.critical_count
        )
    } else {
        format!(
            "✅ Command injection test complete for {} (param: {}) - No vulnerabilities found",
            report.target_url,
            report.parameter
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webguard_creation() {
        let guard = WebGuard::new();
        assert!(guard.is_ok());
    }

    #[tokio::test]
    async fn test_severity_emoji() {
        assert_eq!(Severity::Critical.emoji(), "🔴");
        assert_eq!(Severity::High.emoji(), "🟠");
        assert_eq!(Severity::Medium.emoji(), "🟡");
        assert_eq!(Severity::Low.emoji(), "🔵");
        assert_eq!(Severity::Info.emoji(), "⚪");
    }

    #[tokio::test]
    async fn test_format_report_markdown() {
        let report = PassiveScanReport {
            id: "test-123".to_string(),
            target_url: "https://example.com".to_string(),
            scan_time: Utc::now(),
            duration_ms: 100,
            status_code: Some(200),
            security_headers: SecurityHeadersReport {
                csp: Some(HeaderStatus {
                    present: false,
                    value: None,
                    severity: Severity::Medium,
                    issue: Some("Missing CSP".to_string()),
                }),
                hsts: None,
                x_frame_options: None,
                x_content_type_options: None,
                referrer_policy: None,
                permissions_policy: None,
                x_xss_protection: None,
            },
            server_fingerprint: ServerFingerprint {
                server: Some("nginx/1.18.0".to_string()),
                x_powered_by: None,
                x_aspnet_version: None,
                x_generator: None,
                via: None,
                detected_tech: vec!["Nginx".to_string()],
            },
            cors_analysis: CorsAnalysis {
                allow_origin: None,
                allow_credentials: None,
                allow_methods: None,
                allow_headers: None,
                expose_headers: None,
                max_age: None,
                is_misconfigured: false,
                issues: vec![],
            },
            sensitive_paths: vec![],
            findings: vec![],
            summary: ScanSummary {
                total_findings: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                low_count: 0,
                info_count: 0,
                overall_risk: Severity::Info,
            },
        };

        let md = format_report_markdown(&report);
        assert!(md.contains("WebGuard Scan Report"));
        assert!(md.contains("https://example.com"));
    }

    #[tokio::test]
    async fn test_sqli_tester_creation() {
        let tester = SqliTester::new();
        assert!(tester.is_ok());
    }

    #[tokio::test]
    async fn test_sqli_tester_custom_config() {
        let config = SqliTestConfig {
            timeout_secs: 60,
            time_delay_threshold_ms: 5000,
            ..Default::default()
        };
        let tester = SqliTester::with_config(config);
        assert!(tester.is_ok());
    }

    #[tokio::test]
    async fn test_sqli_type_descriptions() {
        assert!(SqliType::ErrorBased.description().contains("Error-based"));
        assert!(SqliType::BooleanBlind.description().contains("Boolean"));
        assert!(SqliType::TimeBlind.description().contains("Time"));
        assert!(SqliType::UnionBased.description().contains("UNION"));
    }

    #[tokio::test]
    async fn test_sqli_payload_type_badges() {
        assert_eq!(SqliPayloadType::ErrorBased.badge(), "ERROR");
        assert_eq!(SqliPayloadType::BooleanBased.badge(), "BOOLEAN");
        assert_eq!(SqliPayloadType::TimeBased.badge(), "TIME");
        assert_eq!(SqliPayloadType::UnionBased.badge(), "UNION");
    }

    #[tokio::test]
    async fn test_safe_sqli_payloads_exist() {
        assert!(!SAFE_SQLI_PAYLOADS.is_empty());
        // Verify we have different payload types
        let has_error = SAFE_SQLI_PAYLOADS.iter().any(|(_, _, t)| *t == SqliPayloadType::ErrorBased);
        let has_boolean = SAFE_SQLI_PAYLOADS.iter().any(|(_, _, t)| *t == SqliPayloadType::BooleanBased);
        let has_time = SAFE_SQLI_PAYLOADS.iter().any(|(_, _, t)| *t == SqliPayloadType::TimeBased);
        assert!(has_error);
        assert!(has_boolean);
        assert!(has_time);
    }

    #[tokio::test]
    async fn test_format_sqli_report_markdown_vulnerable() {
        let report = SqliTestReport {
            id: "test-sqli-123".to_string(),
            target_url: "https://example.com/search".to_string(),
            parameter: "id".to_string(),
            scan_time: Utc::now(),
            duration_ms: 500,
            payloads_tested: 10,
            errors_detected: 2,
            time_delays_detected: 1,
            boolean_differences: 1,
            findings: vec![SqliFinding {
                id: "SQLI-001".to_string(),
                sqli_type: SqliType::ErrorBased,
                severity: Severity::High,
                parameter: "id".to_string(),
                payload: "' OR '1'='1".to_string(),
                payload_description: "Classic OR injection".to_string(),
                proof_of_concept: "https://example.com/search?id=' OR '1'='1".to_string(),
                evidence: "SQL error detected: mysql_fetch".to_string(),
                database_type: Some("MySQL".to_string()),
                remediation: "Use prepared statements".to_string(),
            }],
            payload_results: vec![],
            summary: SqliSummary {
                vulnerable: true,
                total_findings: 1,
                critical_count: 0,
                high_count: 1,
                medium_count: 0,
                overall_risk: Severity::High,
                detected_database: Some("MySQL".to_string()),
            },
        };

        let md = format_sqli_report_markdown(&report);
        assert!(md.contains("WebGuard SQLi Test Report"));
        assert!(md.contains("VULNERABLE TO SQL INJECTION"));
        assert!(md.contains("MySQL"));
        assert!(md.contains("SQLI-001"));
    }

    #[tokio::test]
    async fn test_format_sqli_report_markdown_clean() {
        let report = SqliTestReport {
            id: "test-sqli-456".to_string(),
            target_url: "https://secure.example.com/api".to_string(),
            parameter: "query".to_string(),
            scan_time: Utc::now(),
            duration_ms: 300,
            payloads_tested: 10,
            errors_detected: 0,
            time_delays_detected: 0,
            boolean_differences: 0,
            findings: vec![],
            payload_results: vec![],
            summary: SqliSummary {
                vulnerable: false,
                total_findings: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                overall_risk: Severity::Info,
                detected_database: None,
            },
        };

        let md = format_sqli_report_markdown(&report);
        assert!(md.contains("WebGuard SQLi Test Report"));
        assert!(md.contains("No SQL injection vulnerabilities detected"));
    }

    #[tokio::test]
    async fn test_format_sqli_notification_summary_vulnerable() {
        let report = SqliTestReport {
            id: "test".to_string(),
            target_url: "https://example.com".to_string(),
            parameter: "id".to_string(),
            scan_time: Utc::now(),
            duration_ms: 100,
            payloads_tested: 5,
            errors_detected: 1,
            time_delays_detected: 0,
            boolean_differences: 0,
            findings: vec![],
            payload_results: vec![],
            summary: SqliSummary {
                vulnerable: true,
                total_findings: 1,
                critical_count: 1,
                high_count: 0,
                medium_count: 0,
                overall_risk: Severity::Critical,
                detected_database: Some("MySQL".to_string()),
            },
        };

        let summary = format_sqli_notification_summary(&report);
        assert!(summary.contains("SQL INJECTION VULNERABILITY DETECTED"));
        assert!(summary.contains("MySQL"));
    }

    #[tokio::test]
    async fn test_format_sqli_notification_summary_clean() {
        let report = SqliTestReport {
            id: "test".to_string(),
            target_url: "https://example.com".to_string(),
            parameter: "id".to_string(),
            scan_time: Utc::now(),
            duration_ms: 100,
            payloads_tested: 5,
            errors_detected: 0,
            time_delays_detected: 0,
            boolean_differences: 0,
            findings: vec![],
            payload_results: vec![],
            summary: SqliSummary {
                vulnerable: false,
                total_findings: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                overall_risk: Severity::Info,
                detected_database: None,
            },
        };

        let summary = format_sqli_notification_summary(&report);
        assert!(summary.contains("No vulnerabilities found"));
    }
}
