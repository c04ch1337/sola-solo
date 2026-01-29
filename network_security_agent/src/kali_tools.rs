//! Kali Linux Tool Wrappers
//!
//! Provides integration with common Kali Linux security tools.
//! Supports both native tool execution and Docker-based Kali containers.
//!
//! # Supported Tools
//!
//! - **Reconnaissance**: nmap, masscan, amass, subfinder, theHarvester
//! - **Web**: nikto, gobuster, ffuf, sqlmap, nuclei, burpsuite
//! - **Exploitation**: metasploit, searchsploit, crackmapexec
//! - **Password**: hashcat, john, hydra
//! - **Wireless**: aircrack-ng, wifite, kismet
//! - **Post-Exploitation**: mimikatz, bloodhound, impacket

use crate::{SecurityAgentError, SecurityGate, SecurityLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Kali tool wrapper with security-gated execution
pub struct KaliToolWrapper {
    security_gate: Arc<RwLock<SecurityGate>>,
    tools: HashMap<String, KaliTool>,
    docker_available: bool,
    kali_container: Option<String>,
}

impl KaliToolWrapper {
    /// Create a new Kali tool wrapper
    pub fn new(security_gate: Arc<RwLock<SecurityGate>>) -> Self {
        let mut wrapper = Self {
            security_gate,
            tools: HashMap::new(),
            docker_available: false,
            kali_container: None,
        };
        wrapper.load_tools();
        wrapper
    }

    /// Check if Docker is available
    pub async fn check_docker(&mut self) -> bool {
        match Command::new("docker").arg("--version").output().await {
            Ok(output) => {
                self.docker_available = output.status.success();
                self.docker_available
            }
            Err(_) => {
                self.docker_available = false;
                false
            }
        }
    }

    /// Start a Kali Linux Docker container
    pub async fn start_kali_container(&mut self) -> Result<String, SecurityAgentError> {
        if !self.docker_available {
            return Err(SecurityAgentError::ToolNotAvailable(
                "Docker is not available".to_string(),
            ));
        }

        let container_name = format!("sola-kali-{}", Uuid::new_v4().to_string()[..8].to_string());

        let output = Command::new("docker")
            .args([
                "run",
                "-d",
                "--name",
                &container_name,
                "--network",
                "host",
                "--cap-add",
                "NET_ADMIN",
                "--cap-add",
                "NET_RAW",
                "kalilinux/kali-rolling",
                "tail",
                "-f",
                "/dev/null",
            ])
            .output()
            .await
            .map_err(|e| SecurityAgentError::ToolNotAvailable(e.to_string()))?;

        if output.status.success() {
            self.kali_container = Some(container_name.clone());
            Ok(container_name)
        } else {
            Err(SecurityAgentError::ToolNotAvailable(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    /// Stop the Kali container
    pub async fn stop_kali_container(&mut self) -> Result<(), SecurityAgentError> {
        if let Some(container) = &self.kali_container {
            let _ = Command::new("docker")
                .args(["stop", container])
                .output()
                .await;
            let _ = Command::new("docker")
                .args(["rm", container])
                .output()
                .await;
            self.kali_container = None;
        }
        Ok(())
    }

    /// Load tool definitions
    fn load_tools(&mut self) {
        // Reconnaissance Tools
        self.tools.insert(
            "nmap".to_string(),
            KaliTool {
                name: "nmap".to_string(),
                description: "Network exploration and security auditing".to_string(),
                category: ToolCategory::Reconnaissance,
                required_level: SecurityLevel::Active,
                binary: "nmap".to_string(),
                common_args: vec![
                    ("-sS".to_string(), "TCP SYN scan".to_string()),
                    ("-sV".to_string(), "Service version detection".to_string()),
                    ("-O".to_string(), "OS detection".to_string()),
                    ("-A".to_string(), "Aggressive scan".to_string()),
                    ("-p-".to_string(), "All ports".to_string()),
                    ("--script".to_string(), "NSE scripts".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        self.tools.insert(
            "masscan".to_string(),
            KaliTool {
                name: "masscan".to_string(),
                description: "Fast port scanner".to_string(),
                category: ToolCategory::Reconnaissance,
                required_level: SecurityLevel::Active,
                binary: "masscan".to_string(),
                common_args: vec![
                    ("-p".to_string(), "Ports to scan".to_string()),
                    ("--rate".to_string(), "Packets per second".to_string()),
                    ("--banners".to_string(), "Grab banners".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        self.tools.insert(
            "amass".to_string(),
            KaliTool {
                name: "amass".to_string(),
                description: "Subdomain enumeration".to_string(),
                category: ToolCategory::Reconnaissance,
                required_level: SecurityLevel::Passive,
                binary: "amass".to_string(),
                common_args: vec![
                    ("enum".to_string(), "Enumeration mode".to_string()),
                    ("-d".to_string(), "Target domain".to_string()),
                    ("-passive".to_string(), "Passive only".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "subfinder".to_string(),
            KaliTool {
                name: "subfinder".to_string(),
                description: "Subdomain discovery tool".to_string(),
                category: ToolCategory::Reconnaissance,
                required_level: SecurityLevel::Passive,
                binary: "subfinder".to_string(),
                common_args: vec![
                    ("-d".to_string(), "Target domain".to_string()),
                    ("-silent".to_string(), "Silent mode".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "theHarvester".to_string(),
            KaliTool {
                name: "theHarvester".to_string(),
                description: "Email and subdomain harvester".to_string(),
                category: ToolCategory::Reconnaissance,
                required_level: SecurityLevel::Passive,
                binary: "theHarvester".to_string(),
                common_args: vec![
                    ("-d".to_string(), "Target domain".to_string()),
                    ("-b".to_string(), "Data source".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        // Web Application Tools
        self.tools.insert(
            "nikto".to_string(),
            KaliTool {
                name: "nikto".to_string(),
                description: "Web server scanner".to_string(),
                category: ToolCategory::WebApplication,
                required_level: SecurityLevel::Active,
                binary: "nikto".to_string(),
                common_args: vec![
                    ("-h".to_string(), "Target host".to_string()),
                    ("-p".to_string(), "Target port".to_string()),
                    ("-ssl".to_string(), "Use SSL".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "gobuster".to_string(),
            KaliTool {
                name: "gobuster".to_string(),
                description: "Directory/file brute-forcer".to_string(),
                category: ToolCategory::WebApplication,
                required_level: SecurityLevel::Active,
                binary: "gobuster".to_string(),
                common_args: vec![
                    ("dir".to_string(), "Directory mode".to_string()),
                    ("-u".to_string(), "Target URL".to_string()),
                    ("-w".to_string(), "Wordlist".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "ffuf".to_string(),
            KaliTool {
                name: "ffuf".to_string(),
                description: "Fast web fuzzer".to_string(),
                category: ToolCategory::WebApplication,
                required_level: SecurityLevel::Active,
                binary: "ffuf".to_string(),
                common_args: vec![
                    ("-u".to_string(), "Target URL with FUZZ".to_string()),
                    ("-w".to_string(), "Wordlist".to_string()),
                    ("-mc".to_string(), "Match HTTP codes".to_string()),
                ],
                output_format: OutputFormat::Json,
                requires_root: false,
            },
        );

        self.tools.insert(
            "sqlmap".to_string(),
            KaliTool {
                name: "sqlmap".to_string(),
                description: "SQL injection tool".to_string(),
                category: ToolCategory::WebApplication,
                required_level: SecurityLevel::Active,
                binary: "sqlmap".to_string(),
                common_args: vec![
                    ("-u".to_string(), "Target URL".to_string()),
                    ("--batch".to_string(), "Non-interactive".to_string()),
                    ("--dbs".to_string(), "Enumerate databases".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "nuclei".to_string(),
            KaliTool {
                name: "nuclei".to_string(),
                description: "Vulnerability scanner".to_string(),
                category: ToolCategory::WebApplication,
                required_level: SecurityLevel::Active,
                binary: "nuclei".to_string(),
                common_args: vec![
                    ("-u".to_string(), "Target URL".to_string()),
                    ("-t".to_string(), "Template path".to_string()),
                    ("-severity".to_string(), "Severity filter".to_string()),
                ],
                output_format: OutputFormat::Json,
                requires_root: false,
            },
        );

        // Exploitation Tools
        self.tools.insert(
            "msfconsole".to_string(),
            KaliTool {
                name: "msfconsole".to_string(),
                description: "Metasploit Framework".to_string(),
                category: ToolCategory::Exploitation,
                required_level: SecurityLevel::Exploit,
                binary: "msfconsole".to_string(),
                common_args: vec![
                    ("-q".to_string(), "Quiet mode".to_string()),
                    ("-x".to_string(), "Execute commands".to_string()),
                    ("-r".to_string(), "Resource script".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "searchsploit".to_string(),
            KaliTool {
                name: "searchsploit".to_string(),
                description: "Exploit-DB search".to_string(),
                category: ToolCategory::Exploitation,
                required_level: SecurityLevel::Passive,
                binary: "searchsploit".to_string(),
                common_args: vec![
                    ("-w".to_string(), "Show URLs".to_string()),
                    ("-j".to_string(), "JSON output".to_string()),
                ],
                output_format: OutputFormat::Json,
                requires_root: false,
            },
        );

        self.tools.insert(
            "crackmapexec".to_string(),
            KaliTool {
                name: "crackmapexec".to_string(),
                description: "Network pentesting tool".to_string(),
                category: ToolCategory::Exploitation,
                required_level: SecurityLevel::Active,
                binary: "crackmapexec".to_string(),
                common_args: vec![
                    ("smb".to_string(), "SMB protocol".to_string()),
                    ("--shares".to_string(), "Enumerate shares".to_string()),
                    ("-u".to_string(), "Username".to_string()),
                    ("-p".to_string(), "Password".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        // Password Tools
        self.tools.insert(
            "hashcat".to_string(),
            KaliTool {
                name: "hashcat".to_string(),
                description: "Password recovery tool".to_string(),
                category: ToolCategory::Password,
                required_level: SecurityLevel::Active,
                binary: "hashcat".to_string(),
                common_args: vec![
                    ("-m".to_string(), "Hash type".to_string()),
                    ("-a".to_string(), "Attack mode".to_string()),
                    ("-w".to_string(), "Workload profile".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "john".to_string(),
            KaliTool {
                name: "john".to_string(),
                description: "John the Ripper".to_string(),
                category: ToolCategory::Password,
                required_level: SecurityLevel::Active,
                binary: "john".to_string(),
                common_args: vec![
                    ("--wordlist".to_string(), "Wordlist file".to_string()),
                    ("--format".to_string(), "Hash format".to_string()),
                    ("--show".to_string(), "Show cracked".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        self.tools.insert(
            "hydra".to_string(),
            KaliTool {
                name: "hydra".to_string(),
                description: "Login brute-forcer".to_string(),
                category: ToolCategory::Password,
                required_level: SecurityLevel::Active,
                binary: "hydra".to_string(),
                common_args: vec![
                    ("-l".to_string(), "Login name".to_string()),
                    ("-L".to_string(), "Login list".to_string()),
                    ("-p".to_string(), "Password".to_string()),
                    ("-P".to_string(), "Password list".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        // Wireless Tools
        self.tools.insert(
            "aircrack-ng".to_string(),
            KaliTool {
                name: "aircrack-ng".to_string(),
                description: "WiFi security auditing".to_string(),
                category: ToolCategory::Wireless,
                required_level: SecurityLevel::Active,
                binary: "aircrack-ng".to_string(),
                common_args: vec![
                    ("-w".to_string(), "Wordlist".to_string()),
                    ("-b".to_string(), "Target BSSID".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        self.tools.insert(
            "airodump-ng".to_string(),
            KaliTool {
                name: "airodump-ng".to_string(),
                description: "WiFi packet capture".to_string(),
                category: ToolCategory::Wireless,
                required_level: SecurityLevel::Active,
                binary: "airodump-ng".to_string(),
                common_args: vec![
                    ("-c".to_string(), "Channel".to_string()),
                    ("--bssid".to_string(), "Target BSSID".to_string()),
                    ("-w".to_string(), "Output prefix".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        // Post-Exploitation Tools
        self.tools.insert(
            "mimikatz".to_string(),
            KaliTool {
                name: "mimikatz".to_string(),
                description: "Windows credential extraction".to_string(),
                category: ToolCategory::PostExploitation,
                required_level: SecurityLevel::Exploit,
                binary: "mimikatz".to_string(),
                common_args: vec![
                    ("sekurlsa::logonpasswords".to_string(), "Dump passwords".to_string()),
                    ("lsadump::sam".to_string(), "Dump SAM".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        self.tools.insert(
            "bloodhound-python".to_string(),
            KaliTool {
                name: "bloodhound-python".to_string(),
                description: "Active Directory enumeration".to_string(),
                category: ToolCategory::PostExploitation,
                required_level: SecurityLevel::Active,
                binary: "bloodhound-python".to_string(),
                common_args: vec![
                    ("-d".to_string(), "Domain".to_string()),
                    ("-u".to_string(), "Username".to_string()),
                    ("-p".to_string(), "Password".to_string()),
                    ("-c".to_string(), "Collection method".to_string()),
                ],
                output_format: OutputFormat::Json,
                requires_root: false,
            },
        );

        self.tools.insert(
            "impacket-secretsdump".to_string(),
            KaliTool {
                name: "impacket-secretsdump".to_string(),
                description: "Remote secrets dumping".to_string(),
                category: ToolCategory::PostExploitation,
                required_level: SecurityLevel::Exploit,
                binary: "secretsdump.py".to_string(),
                common_args: vec![],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );

        // Network Tools
        self.tools.insert(
            "responder".to_string(),
            KaliTool {
                name: "responder".to_string(),
                description: "LLMNR/NBT-NS/mDNS poisoner".to_string(),
                category: ToolCategory::Network,
                required_level: SecurityLevel::Exploit,
                binary: "responder".to_string(),
                common_args: vec![
                    ("-I".to_string(), "Interface".to_string()),
                    ("-w".to_string(), "WPAD rogue".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: true,
            },
        );

        self.tools.insert(
            "enum4linux".to_string(),
            KaliTool {
                name: "enum4linux".to_string(),
                description: "Windows/Samba enumeration".to_string(),
                category: ToolCategory::Network,
                required_level: SecurityLevel::Active,
                binary: "enum4linux".to_string(),
                common_args: vec![
                    ("-a".to_string(), "All enumeration".to_string()),
                    ("-u".to_string(), "Username".to_string()),
                    ("-p".to_string(), "Password".to_string()),
                ],
                output_format: OutputFormat::Text,
                requires_root: false,
            },
        );
    }

    /// Run a Kali tool
    pub async fn run(
        &self,
        tool_name: &str,
        args: Vec<String>,
    ) -> Result<ToolOutput, SecurityAgentError> {
        let tool = self.tools.get(tool_name).ok_or_else(|| {
            SecurityAgentError::ToolNotAvailable(format!("Tool '{}' not found", tool_name))
        })?;

        // Check authorization
        let gate = self.security_gate.read().await;
        gate.check_authorization(tool.required_level, None)?;
        drop(gate);

        println!("🔧 Running tool: {}", tool.name);
        println!("   Category: {:?}", tool.category);
        println!("   Args: {:?}", args);

        let started_at = Utc::now();

        // Try to run the tool natively first
        let output = self.execute_tool(&tool.binary, &args).await;

        let completed_at = Utc::now();

        match output {
            Ok((stdout, stderr, success)) => Ok(ToolOutput {
                id: Uuid::new_v4(),
                tool_name: tool_name.to_string(),
                command: format!("{} {}", tool.binary, args.join(" ")),
                started_at,
                completed_at,
                success,
                stdout,
                stderr,
                parsed_results: None,
            }),
            Err(e) => {
                // If native execution fails, try Docker if available
                if self.docker_available {
                    self.run_in_docker(tool, args).await
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Execute a tool natively
    async fn execute_tool(
        &self,
        binary: &str,
        args: &[String],
    ) -> Result<(String, String, bool), SecurityAgentError> {
        let mut cmd = Command::new(binary);
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| SecurityAgentError::ToolNotAvailable(e.to_string()))?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        let mut stdout_output = String::new();
        let mut stderr_output = String::new();

        // Read output streams
        loop {
            tokio::select! {
                line = stdout_reader.next_line() => {
                    match line {
                        Ok(Some(l)) => {
                            println!("   [stdout] {}", l);
                            stdout_output.push_str(&l);
                            stdout_output.push('\n');
                        }
                        Ok(None) => break,
                        Err(e) => {
                            stderr_output.push_str(&format!("Error reading stdout: {}\n", e));
                            break;
                        }
                    }
                }
                line = stderr_reader.next_line() => {
                    match line {
                        Ok(Some(l)) => {
                            eprintln!("   [stderr] {}", l);
                            stderr_output.push_str(&l);
                            stderr_output.push('\n');
                        }
                        Ok(None) => {}
                        Err(e) => {
                            stderr_output.push_str(&format!("Error reading stderr: {}\n", e));
                        }
                    }
                }
            }
        }

        let status = child
            .wait()
            .await
            .map_err(|e| SecurityAgentError::ToolNotAvailable(e.to_string()))?;

        Ok((stdout_output, stderr_output, status.success()))
    }

    /// Run a tool in Docker container
    async fn run_in_docker(
        &self,
        tool: &KaliTool,
        args: Vec<String>,
    ) -> Result<ToolOutput, SecurityAgentError> {
        let container = self.kali_container.as_ref().ok_or_else(|| {
            SecurityAgentError::ToolNotAvailable("Kali container not running".to_string())
        })?;

        let started_at = Utc::now();

        let mut docker_args = vec![
            "exec".to_string(),
            container.clone(),
            tool.binary.clone(),
        ];
        docker_args.extend(args.clone());

        let output = Command::new("docker")
            .args(&docker_args)
            .output()
            .await
            .map_err(|e| SecurityAgentError::ToolNotAvailable(e.to_string()))?;

        let completed_at = Utc::now();

        Ok(ToolOutput {
            id: Uuid::new_v4(),
            tool_name: tool.name.clone(),
            command: format!("docker exec {} {} {}", container, tool.binary, args.join(" ")),
            started_at,
            completed_at,
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            parsed_results: None,
        })
    }

    /// List available tools
    pub fn list_tools(&self) -> Vec<KaliToolInfo> {
        self.tools
            .values()
            .map(|t| KaliToolInfo {
                name: t.name.clone(),
                description: t.description.clone(),
                category: t.category.clone(),
                required_level: t.required_level,
                requires_root: t.requires_root,
            })
            .collect()
    }

    /// Get tool details
    pub fn get_tool(&self, name: &str) -> Option<&KaliTool> {
        self.tools.get(name)
    }

    /// Search tools by category
    pub fn search_by_category(&self, category: &ToolCategory) -> Vec<&KaliTool> {
        self.tools
            .values()
            .filter(|t| &t.category == category)
            .collect()
    }

    /// Generate tool command
    pub fn generate_command(&self, tool_name: &str, options: &HashMap<String, String>) -> Option<String> {
        let tool = self.tools.get(tool_name)?;
        let mut cmd = tool.binary.clone();

        for (key, value) in options {
            cmd.push_str(&format!(" {} {}", key, value));
        }

        Some(cmd)
    }
}

/// Kali tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliTool {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub required_level: SecurityLevel,
    pub binary: String,
    pub common_args: Vec<(String, String)>,
    pub output_format: OutputFormat,
    pub requires_root: bool,
}

/// Tool category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    Reconnaissance,
    WebApplication,
    Exploitation,
    Password,
    Wireless,
    PostExploitation,
    Network,
    Forensics,
    Reporting,
}

/// Output format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Xml,
    Binary,
}

/// Tool execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub id: Uuid,
    pub tool_name: String,
    pub command: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub parsed_results: Option<serde_json::Value>,
}

/// Tool info for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliToolInfo {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub required_level: SecurityLevel,
    pub requires_root: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kali_wrapper_creation() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let wrapper = KaliToolWrapper::new(gate);
        assert!(!wrapper.tools.is_empty());
    }

    #[test]
    fn test_list_tools() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let wrapper = KaliToolWrapper::new(gate);
        let tools = wrapper.list_tools();
        assert!(!tools.is_empty());
    }

    #[test]
    fn test_search_by_category() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let wrapper = KaliToolWrapper::new(gate);
        let recon_tools = wrapper.search_by_category(&ToolCategory::Reconnaissance);
        assert!(!recon_tools.is_empty());
    }

    #[test]
    fn test_generate_command() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let wrapper = KaliToolWrapper::new(gate);
        let mut options = HashMap::new();
        options.insert("-sV".to_string(), "".to_string());
        options.insert("-p".to_string(), "80,443".to_string());
        let cmd = wrapper.generate_command("nmap", &options);
        assert!(cmd.is_some());
    }
}
