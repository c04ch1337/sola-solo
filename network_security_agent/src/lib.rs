//! Network Security Agent for Phoenix AGI
//!
//! A sophisticated offensive security and network intelligence agent with OSCP/Kali-level capabilities.
//! This agent provides network scanning, vulnerability assessment, and security-gated exploit capabilities.
//!
//! # Features
//!
//! - **Network Scanning**: nmap-like port scanning, service detection, OS fingerprinting
//! - **Vulnerability Assessment**: CVE lookup, vulnerability scoring, risk analysis
//! - **MITRE ATT&CK Integration**: TTP mapping, attack chain analysis
//! - **Security Playbooks**: Automated penetration testing workflows
//! - **Exploit Framework**: Security-gated exploit capabilities (requires explicit authorization)
//! - **Kali Tool Wrappers**: Integration with common security tools
//!
//! # Security Model
//!
//! This agent implements a multi-tier security model:
//! - **Passive Mode** (default): Read-only reconnaissance, no active probing
//! - **Active Mode**: Active scanning with explicit user consent
//! - **Exploit Mode**: Requires security gate approval and explicit authorization
//!
//! # Example
//!
//! ```rust,no_run
//! use network_security_agent::{NetworkSecurityAgent, ScanConfig, SecurityGate};
//!
//! #[tokio::main]
//! async fn main() {
//!     let agent = NetworkSecurityAgent::awaken().await.unwrap();
//!     
//!     // Passive network discovery
//!     let networks = agent.discover_networks().await.unwrap();
//!     
//!     // Active port scan (requires consent)
//!     let config = ScanConfig::default().with_target("192.168.1.0/24");
//!     let results = agent.scan_network(&config).await.unwrap();
//! }
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod exploit;
pub mod kali_tools;
pub mod mitre_attack;
pub mod playbooks;
pub mod scanner;
pub mod vulnerability;

pub use exploit::*;
pub use kali_tools::*;
pub use mitre_attack::*;
pub use playbooks::*;
pub use scanner::*;
pub use vulnerability::*;

/// Errors that can occur in the Network Security Agent
#[derive(Debug, Error)]
pub enum SecurityAgentError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Security gate blocked: {0}")]
    SecurityGateBlocked(String),

    #[error("Tool not available: {0}")]
    ToolNotAvailable(String),

    #[error("Scan failed: {0}")]
    ScanFailed(String),

    #[error("Exploit blocked: {0}")]
    ExploitBlocked(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

/// Security authorization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Read-only reconnaissance, no active probing
    Passive = 0,
    /// Active scanning with user consent
    Active = 1,
    /// Vulnerability exploitation with explicit authorization
    Exploit = 2,
    /// Full offensive capabilities (requires admin + explicit consent)
    Offensive = 3,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Passive
    }
}

/// Security gate for controlling access to dangerous operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGate {
    /// Current authorization level
    pub level: SecurityLevel,
    /// User who authorized the operation
    pub authorized_by: Option<String>,
    /// Timestamp of authorization
    pub authorized_at: Option<DateTime<Utc>>,
    /// Expiration time for authorization
    pub expires_at: Option<DateTime<Utc>>,
    /// Specific targets authorized for scanning/exploitation
    pub authorized_targets: Vec<String>,
    /// Audit log of security decisions
    pub audit_log: Vec<SecurityAuditEntry>,
}

impl Default for SecurityGate {
    fn default() -> Self {
        Self {
            level: SecurityLevel::Passive,
            authorized_by: None,
            authorized_at: None,
            expires_at: None,
            authorized_targets: Vec::new(),
            audit_log: Vec::new(),
        }
    }
}

impl SecurityGate {
    /// Create a new security gate with passive level
    pub fn new() -> Self {
        Self::default()
    }

    /// Authorize a specific security level
    pub fn authorize(
        &mut self,
        level: SecurityLevel,
        user: &str,
        duration_hours: Option<u64>,
        targets: Vec<String>,
    ) -> Result<(), SecurityAgentError> {
        let now = Utc::now();

        // Log the authorization attempt
        self.audit_log.push(SecurityAuditEntry {
            timestamp: now,
            action: format!("Authorization requested for level {:?}", level),
            user: user.to_string(),
            result: "pending".to_string(),
            details: serde_json::json!({
                "targets": targets,
                "duration_hours": duration_hours,
            }),
        });

        // Require explicit confirmation for exploit/offensive levels
        if level >= SecurityLevel::Exploit {
            // In production, this would require multi-factor authentication
            // and explicit user confirmation through a secure channel
            eprintln!(
                "⚠️  WARNING: Authorizing {:?} level operations. This enables potentially dangerous capabilities.",
                level
            );
            eprintln!("⚠️  Authorized by: {}", user);
            eprintln!("⚠️  Targets: {:?}", targets);
        }

        self.level = level;
        self.authorized_by = Some(user.to_string());
        self.authorized_at = Some(now);
        self.expires_at = duration_hours.map(|h| now + chrono::Duration::hours(h as i64));
        self.authorized_targets = targets;

        // Log successful authorization
        self.audit_log.push(SecurityAuditEntry {
            timestamp: Utc::now(),
            action: format!("Authorization granted for level {:?}", level),
            user: user.to_string(),
            result: "success".to_string(),
            details: serde_json::json!({}),
        });

        Ok(())
    }

    /// Check if an operation is authorized
    pub fn check_authorization(
        &self,
        required_level: SecurityLevel,
        target: Option<&str>,
    ) -> Result<(), SecurityAgentError> {
        // Check if authorization has expired
        if let Some(expires) = self.expires_at {
            if Utc::now() > expires {
                return Err(SecurityAgentError::SecurityGateBlocked(
                    "Authorization has expired".to_string(),
                ));
            }
        }

        // Check security level
        if self.level < required_level {
            return Err(SecurityAgentError::SecurityGateBlocked(format!(
                "Operation requires {:?} level, current level is {:?}",
                required_level, self.level
            )));
        }

        // Check target authorization for active/exploit operations
        if required_level >= SecurityLevel::Active {
            if let Some(t) = target {
                if !self.authorized_targets.is_empty()
                    && !self.authorized_targets.iter().any(|at| {
                        t.starts_with(at) || at == "*" || Self::ip_in_range(t, at)
                    })
                {
                    return Err(SecurityAgentError::SecurityGateBlocked(format!(
                        "Target {} is not in authorized targets list",
                        t
                    )));
                }
            }
        }

        Ok(())
    }

    /// Check if an IP is within an authorized CIDR range
    fn ip_in_range(ip: &str, cidr: &str) -> bool {
        if let Ok(network) = cidr.parse::<ipnetwork::IpNetwork>() {
            if let Ok(addr) = ip.parse::<IpAddr>() {
                return network.contains(addr);
            }
        }
        false
    }

    /// Revoke all authorizations
    pub fn revoke(&mut self) {
        self.audit_log.push(SecurityAuditEntry {
            timestamp: Utc::now(),
            action: "Authorization revoked".to_string(),
            user: self.authorized_by.clone().unwrap_or_default(),
            result: "success".to_string(),
            details: serde_json::json!({}),
        });

        self.level = SecurityLevel::Passive;
        self.authorized_by = None;
        self.authorized_at = None;
        self.expires_at = None;
        self.authorized_targets.clear();
    }
}

/// Audit log entry for security operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub user: String,
    pub result: String,
    pub details: serde_json::Value,
}

/// Network Security Agent - The main agent struct
pub struct NetworkSecurityAgent {
    /// Unique agent ID
    pub id: Uuid,
    /// Security gate for authorization
    pub security_gate: Arc<RwLock<SecurityGate>>,
    /// Network scanner
    scanner: Arc<scanner::NetworkScanner>,
    /// Vulnerability engine
    vuln_engine: Arc<vulnerability::VulnerabilityEngine>,
    /// MITRE ATT&CK knowledge base
    mitre_kb: Arc<mitre_attack::MitreAttackKB>,
    /// Playbook engine
    playbook_engine: Arc<playbooks::PlaybookEngine>,
    /// Exploit framework (security-gated)
    exploit_framework: Arc<exploit::ExploitFramework>,
    /// Kali tool wrappers
    kali_tools: Arc<kali_tools::KaliToolWrapper>,
    /// LLM for intelligent analysis
    llm: Option<Arc<llm_orchestrator::LLMOrchestrator>>,
    /// Scan results cache
    scan_cache: Arc<RwLock<HashMap<Uuid, ScanResult>>>,
}

impl NetworkSecurityAgent {
    /// Awaken the Network Security Agent
    pub async fn awaken() -> Result<Self, SecurityAgentError> {
        println!("🔒 Network Security Agent awakening — Offensive security capabilities online.");

        let security_gate = Arc::new(RwLock::new(SecurityGate::new()));

        // Initialize LLM if available
        let llm = match llm_orchestrator::LLMOrchestrator::awaken() {
            Ok(l) => {
                println!("  ✓ LLM integration enabled for intelligent analysis");
                Some(Arc::new(l))
            }
            Err(e) => {
                eprintln!("  ⚠ LLM not available: {} - continuing without AI analysis", e);
                None
            }
        };

        Ok(Self {
            id: Uuid::new_v4(),
            security_gate: security_gate.clone(),
            scanner: Arc::new(scanner::NetworkScanner::new(security_gate.clone())),
            vuln_engine: Arc::new(vulnerability::VulnerabilityEngine::new()),
            mitre_kb: Arc::new(mitre_attack::MitreAttackKB::new()),
            playbook_engine: Arc::new(playbooks::PlaybookEngine::new(security_gate.clone())),
            exploit_framework: Arc::new(exploit::ExploitFramework::new(security_gate.clone())),
            kali_tools: Arc::new(kali_tools::KaliToolWrapper::new(security_gate.clone())),
            llm,
            scan_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Authorize security operations
    pub async fn authorize(
        &self,
        level: SecurityLevel,
        user: &str,
        duration_hours: Option<u64>,
        targets: Vec<String>,
    ) -> Result<(), SecurityAgentError> {
        let mut gate = self.security_gate.write().await;
        gate.authorize(level, user, duration_hours, targets)
    }

    /// Revoke all authorizations
    pub async fn revoke_authorization(&self) {
        let mut gate = self.security_gate.write().await;
        gate.revoke();
    }

    /// Discover networks (passive)
    pub async fn discover_networks(&self) -> Result<Vec<NetworkDiscovery>, SecurityAgentError> {
        self.scanner.discover_networks().await
    }

    /// Scan a network or host
    pub async fn scan(&self, config: &ScanConfig) -> Result<ScanResult, SecurityAgentError> {
        let result = self.scanner.scan(config).await?;

        // Cache the result
        let mut cache = self.scan_cache.write().await;
        cache.insert(result.id, result.clone());

        Ok(result)
    }

    /// Analyze vulnerabilities in scan results
    pub async fn analyze_vulnerabilities(
        &self,
        scan_id: Uuid,
    ) -> Result<VulnerabilityReport, SecurityAgentError> {
        let cache = self.scan_cache.read().await;
        let scan_result = cache
            .get(&scan_id)
            .ok_or_else(|| SecurityAgentError::ScanFailed("Scan result not found".to_string()))?;

        self.vuln_engine.analyze(scan_result).await
    }

    /// Map findings to MITRE ATT&CK framework
    pub async fn map_to_mitre(
        &self,
        scan_id: Uuid,
    ) -> Result<MitreMapping, SecurityAgentError> {
        let cache = self.scan_cache.read().await;
        let scan_result = cache
            .get(&scan_id)
            .ok_or_else(|| SecurityAgentError::ScanFailed("Scan result not found".to_string()))?;

        self.mitre_kb.map_findings(scan_result).await
    }

    /// Execute a security playbook
    pub async fn execute_playbook(
        &self,
        playbook_name: &str,
        target: &str,
    ) -> Result<PlaybookResult, SecurityAgentError> {
        self.playbook_engine.execute(playbook_name, target).await
    }

    /// List available playbooks
    pub fn list_playbooks(&self) -> Vec<PlaybookInfo> {
        self.playbook_engine.list_playbooks()
    }

    /// Attempt an exploit (requires Exploit security level)
    pub async fn exploit(
        &self,
        exploit_id: &str,
        target: &str,
        options: HashMap<String, String>,
    ) -> Result<ExploitResult, SecurityAgentError> {
        self.exploit_framework
            .execute(exploit_id, target, options)
            .await
    }

    /// List available exploits
    pub fn list_exploits(&self) -> Vec<ExploitInfo> {
        self.exploit_framework.list_exploits()
    }

    /// Run a Kali tool
    pub async fn run_kali_tool(
        &self,
        tool: &str,
        args: Vec<String>,
    ) -> Result<ToolOutput, SecurityAgentError> {
        self.kali_tools.run(tool, args).await
    }

    /// List available Kali tools
    pub fn list_kali_tools(&self) -> Vec<KaliToolInfo> {
        self.kali_tools.list_tools()
    }

    /// Get intelligent analysis of scan results using LLM
    pub async fn analyze_with_ai(
        &self,
        scan_id: Uuid,
    ) -> Result<AIAnalysis, SecurityAgentError> {
        let llm = self.llm.as_ref().ok_or_else(|| {
            SecurityAgentError::Configuration("LLM not available".to_string())
        })?;

        let cache = self.scan_cache.read().await;
        let scan_result = cache
            .get(&scan_id)
            .ok_or_else(|| SecurityAgentError::ScanFailed("Scan result not found".to_string()))?;

        // Get vulnerability report
        let vuln_report = self.vuln_engine.analyze(scan_result).await?;

        // Get MITRE mapping
        let mitre_mapping = self.mitre_kb.map_findings(scan_result).await?;

        // Build analysis prompt
        let prompt = format!(
            r#"You are SOLA, an expert offensive security analyst with OSCP certification and extensive penetration testing experience.

Analyze the following network scan results and provide a comprehensive security assessment:

## Scan Results
{}

## Vulnerability Report
{}

## MITRE ATT&CK Mapping
{}

Provide your analysis in the following format:

### Executive Summary
[Brief overview of security posture]

### Critical Findings
[List critical vulnerabilities and risks]

### Attack Vectors
[Potential attack paths an adversary could take]

### Recommendations
[Prioritized remediation steps]

### MITRE ATT&CK Analysis
[Relevant TTPs and defensive recommendations]

Be specific, technical, and actionable. Reference CVEs and MITRE techniques where applicable."#,
            serde_json::to_string_pretty(scan_result).unwrap_or_default(),
            serde_json::to_string_pretty(&vuln_report).unwrap_or_default(),
            serde_json::to_string_pretty(&mitre_mapping).unwrap_or_default(),
        );

        let analysis = llm
            .speak(&prompt, None)
            .await
            .map_err(|e| SecurityAgentError::Network(e))?;

        Ok(AIAnalysis {
            scan_id,
            analysis,
            generated_at: Utc::now(),
            model: "llm_orchestrator".to_string(),
        })
    }

    /// Proactive security scan - runs automatically and reports findings
    pub async fn proactive_scan(&self, targets: Vec<String>) -> Result<ProactiveScanReport, SecurityAgentError> {
        println!("🔍 Starting proactive security scan...");

        let mut findings = Vec::new();
        let mut total_hosts = 0;
        let mut vulnerable_hosts = 0;

        for target in &targets {
            println!("  Scanning: {}", target);

            let config = ScanConfig::default()
                .with_target(target)
                .with_scan_type(ScanType::ServiceDetection);

            match self.scan(&config).await {
                Ok(result) => {
                    total_hosts += result.hosts.len();

                    // Analyze vulnerabilities
                    if let Ok(vuln_report) = self.analyze_vulnerabilities(result.id).await {
                        if !vuln_report.vulnerabilities.is_empty() {
                            vulnerable_hosts += 1;
                            findings.push(ProactiveFinding {
                                target: target.clone(),
                                scan_id: result.id,
                                vulnerabilities: vuln_report.vulnerabilities,
                                risk_score: vuln_report.overall_risk_score,
                            });
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  ⚠ Failed to scan {}: {}", target, e);
                }
            }
        }

        let report = ProactiveScanReport {
            id: Uuid::new_v4(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            targets_scanned: targets,
            total_hosts,
            vulnerable_hosts,
            findings,
        };

        // If LLM is available, generate AI summary
        if let Some(llm) = &self.llm {
            let summary_prompt = format!(
                "Summarize this security scan report in 2-3 sentences for a chat message:\n{}",
                serde_json::to_string_pretty(&report).unwrap_or_default()
            );

            if let Ok(summary) = llm.speak(&summary_prompt, None).await {
                println!("\n📊 Scan Summary: {}", summary);
            }
        }

        Ok(report)
    }

    /// Get current security gate status
    pub async fn get_security_status(&self) -> SecurityGate {
        self.security_gate.read().await.clone()
    }

    /// Get audit log
    pub async fn get_audit_log(&self) -> Vec<SecurityAuditEntry> {
        self.security_gate.read().await.audit_log.clone()
    }

    // ========================================================================
    // API-compatible methods for phoenix-web integration
    // ========================================================================

    /// Get current authorization level
    pub fn current_authorization_level(&self) -> SecurityLevel {
        // Use blocking read for sync context
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.security_gate.read().await.level
            })
        })
    }

    /// Check if exploit operations are authorized
    pub fn is_exploit_authorized(&self) -> bool {
        self.current_authorization_level() >= SecurityLevel::Exploit
    }

    /// Scan network with simple parameters (API-friendly)
    pub async fn scan_network(
        &mut self,
        target: &str,
        ports: Option<&[u16]>,
    ) -> Result<serde_json::Value, SecurityAgentError> {
        let mut config = ScanConfig::default().with_target(target);
        
        if let Some(p) = ports {
            config = config.with_ports(p.to_vec());
        }
        
        let result = self.scan(&config).await?;
        Ok(serde_json::to_value(&result).unwrap_or_default())
    }

    /// Quick scan of local network or specified target
    pub async fn quick_scan(&mut self, target: &str) -> Result<serde_json::Value, SecurityAgentError> {
        let actual_target = if target == "local" {
            // Discover local network
            let networks = self.discover_networks().await?;
            if let Some(net) = networks.first() {
                // Use the IP address from the network discovery
                net.ip_address.map(|ip| ip.to_string()).unwrap_or_else(|| "127.0.0.1".to_string())
            } else {
                "127.0.0.1".to_string()
            }
        } else {
            target.to_string()
        };

        let config = ScanConfig::default()
            .with_target(&actual_target)
            .with_scan_type(ScanType::TcpConnect)  // Use TcpConnect for quick scan
            .with_ports(vec![21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 3306, 3389, 5432, 8080, 8443]);

        let result = self.scan(&config).await?;
        Ok(serde_json::to_value(&result).unwrap_or_default())
    }

    /// Get vulnerability database
    pub fn get_vulnerability_database(&self) -> Vec<serde_json::Value> {
        self.vuln_engine.get_known_vulnerabilities()
            .iter()
            .map(|v| serde_json::to_value(v).unwrap_or_default())
            .collect()
    }

    /// Check vulnerabilities for a target
    pub async fn check_vulnerabilities(
        &mut self,
        target: &str,
        services: Option<&[String]>,
    ) -> Result<serde_json::Value, SecurityAgentError> {
        // First scan the target
        let config = ScanConfig::default()
            .with_target(target)
            .with_scan_type(ScanType::ServiceDetection);
        
        let scan_result = self.scan(&config).await?;
        
        // Analyze vulnerabilities
        let vuln_report = self.analyze_vulnerabilities(scan_result.id).await?;
        
        // Filter by services if specified
        let filtered = if let Some(svc_filter) = services {
            let filtered_vulns: Vec<_> = vuln_report.vulnerabilities
                .into_iter()
                .filter(|v| {
                    if let Some(ref svc) = v.service {
                        svc_filter.iter().any(|s| svc.contains(s))
                    } else {
                        false
                    }
                })
                .collect();
            VulnerabilityReport {
                vulnerabilities: filtered_vulns,
                ..vuln_report
            }
        } else {
            vuln_report
        };
        
        Ok(serde_json::to_value(&filtered).unwrap_or_default())
    }

    /// Execute playbook with options (API-friendly)
    pub async fn execute_playbook_with_options(
        &mut self,
        playbook_id: &str,
        target: &str,
        _options: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, SecurityAgentError> {
        let result = self.execute_playbook(playbook_id, target).await?;
        Ok(serde_json::to_value(&result).unwrap_or_default())
    }

    /// Authorize with string level (API-friendly)
    pub async fn authorize_str(
        &mut self,
        level_str: &str,
        targets: Option<&[String]>,
        duration: std::time::Duration,
        reason: Option<&str>,
    ) -> Result<AuthorizationInfo, SecurityAgentError> {
        let level = match level_str.to_lowercase().as_str() {
            "passive" => SecurityLevel::Passive,
            "active" => SecurityLevel::Active,
            "exploit" => SecurityLevel::Exploit,
            "offensive" => SecurityLevel::Offensive,
            _ => return Err(SecurityAgentError::Configuration(format!("Unknown security level: {}", level_str))),
        };

        let target_vec = targets.map(|t| t.to_vec()).unwrap_or_else(|| vec!["*".to_string()]);
        let duration_hours = Some(duration.as_secs() / 3600);
        let user = reason.unwrap_or("API request");

        self.authorize(level, user, duration_hours, target_vec).await?;

        let gate = self.security_gate.read().await;
        Ok(AuthorizationInfo {
            level: format!("{:?}", gate.level),
            expires_at: gate.expires_at.map(|t| t.to_rfc3339()),
            allowed_targets: gate.authorized_targets.clone(),
        })
    }

    /// Get MITRE tactics
    pub fn get_mitre_tactics(&self) -> Vec<serde_json::Value> {
        self.mitre_kb.get_tactics()
            .iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect()
    }

    /// Get MITRE techniques
    pub fn get_mitre_techniques(&self) -> Vec<serde_json::Value> {
        self.mitre_kb.get_techniques()
            .iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect()
    }

    /// Get MITRE threat groups
    pub fn get_mitre_threat_groups(&self) -> Vec<serde_json::Value> {
        self.mitre_kb.get_threat_groups()
            .iter()
            .map(|g| serde_json::to_value(g).unwrap_or_default())
            .collect()
    }

    /// List available tools
    pub fn list_available_tools(&self) -> Vec<serde_json::Value> {
        self.list_kali_tools()
            .iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect()
    }

    /// Execute a tool (API-friendly)
    pub async fn execute_tool(
        &mut self,
        tool: &str,
        args: Option<&[String]>,
        target: Option<&str>,
    ) -> Result<serde_json::Value, SecurityAgentError> {
        let mut tool_args = args.map(|a| a.to_vec()).unwrap_or_default();
        
        // Add target to args if provided
        if let Some(t) = target {
            tool_args.push(t.to_string());
        }
        
        let output = self.run_kali_tool(tool, tool_args).await?;
        Ok(serde_json::to_value(&output).unwrap_or_default())
    }

    /// Execute exploit (API-friendly)
    pub async fn execute_exploit(
        &mut self,
        exploit_id: &str,
        target: &str,
        options: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, SecurityAgentError> {
        let opts: HashMap<String, String> = options
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        
        let result = self.exploit(exploit_id, target, opts).await?;
        Ok(serde_json::to_value(&result).unwrap_or_default())
    }

    /// Generate security report
    pub fn generate_security_report(&self) -> serde_json::Value {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let gate = self.security_gate.read().await;
                let cache = self.scan_cache.read().await;
                
                serde_json::json!({
                    "agent_id": self.id.to_string(),
                    "security_level": format!("{:?}", gate.level),
                    "authorized_by": gate.authorized_by,
                    "authorized_at": gate.authorized_at,
                    "expires_at": gate.expires_at,
                    "authorized_targets": gate.authorized_targets,
                    "cached_scans": cache.len(),
                    "audit_log_entries": gate.audit_log.len(),
                    "capabilities": {
                        "scanner": true,
                        "vulnerability_engine": true,
                        "mitre_kb": true,
                        "playbooks": true,
                        "exploit_framework": true,
                        "kali_tools": true,
                        "llm_analysis": self.llm.is_some(),
                    }
                })
            })
        })
    }
}

/// Authorization info returned from authorize operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationInfo {
    pub level: String,
    pub expires_at: Option<String>,
    pub allowed_targets: Vec<String>,
}

/// AI-generated security analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub scan_id: Uuid,
    pub analysis: String,
    pub generated_at: DateTime<Utc>,
    pub model: String,
}

/// Proactive scan finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveFinding {
    pub target: String,
    pub scan_id: Uuid,
    pub vulnerabilities: Vec<Vulnerability>,
    pub risk_score: f32,
}

/// Proactive scan report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveScanReport {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub targets_scanned: Vec<String>,
    pub total_hosts: usize,
    pub vulnerable_hosts: usize,
    pub findings: Vec<ProactiveFinding>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_gate_default() {
        let gate = SecurityGate::new();
        assert_eq!(gate.level, SecurityLevel::Passive);
        assert!(gate.authorized_by.is_none());
    }

    #[tokio::test]
    async fn test_security_gate_authorization() {
        let mut gate = SecurityGate::new();
        gate.authorize(
            SecurityLevel::Active,
            "test_user",
            Some(1),
            vec!["192.168.1.0/24".to_string()],
        )
        .unwrap();

        assert_eq!(gate.level, SecurityLevel::Active);
        assert_eq!(gate.authorized_by, Some("test_user".to_string()));
        assert!(!gate.authorized_targets.is_empty());
    }

    #[tokio::test]
    async fn test_security_gate_check() {
        let mut gate = SecurityGate::new();
        gate.authorize(
            SecurityLevel::Active,
            "test_user",
            Some(1),
            vec!["192.168.1.0/24".to_string()],
        )
        .unwrap();

        // Should pass - authorized target
        assert!(gate
            .check_authorization(SecurityLevel::Active, Some("192.168.1.100"))
            .is_ok());

        // Should fail - unauthorized target
        assert!(gate
            .check_authorization(SecurityLevel::Active, Some("10.0.0.1"))
            .is_err());

        // Should fail - higher level required
        assert!(gate
            .check_authorization(SecurityLevel::Exploit, Some("192.168.1.100"))
            .is_err());
    }

    #[tokio::test]
    async fn test_security_gate_revoke() {
        let mut gate = SecurityGate::new();
        gate.authorize(
            SecurityLevel::Exploit,
            "admin",
            Some(24),
            vec!["*".to_string()],
        )
        .unwrap();

        gate.revoke();

        assert_eq!(gate.level, SecurityLevel::Passive);
        assert!(gate.authorized_by.is_none());
        assert!(gate.authorized_targets.is_empty());
    }
}
