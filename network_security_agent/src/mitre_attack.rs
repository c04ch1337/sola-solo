//! MITRE ATT&CK Knowledge Base
//!
//! Comprehensive mapping of tactics, techniques, and procedures (TTPs)
//! from the MITRE ATT&CK framework for threat intelligence and attack analysis.

use crate::scanner::ScanResult;
use crate::SecurityAgentError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MITRE ATT&CK Knowledge Base
pub struct MitreAttackKB {
    /// Tactics (columns in the ATT&CK matrix)
    tactics: HashMap<String, Tactic>,
    /// Techniques (cells in the ATT&CK matrix)
    techniques: HashMap<String, Technique>,
    /// Sub-techniques
    sub_techniques: HashMap<String, SubTechnique>,
    /// Mitigations
    mitigations: HashMap<String, Mitigation>,
    /// Groups (threat actors)
    groups: HashMap<String, ThreatGroup>,
    /// Software/malware
    software: HashMap<String, Software>,
}

impl MitreAttackKB {
    /// Create a new MITRE ATT&CK knowledge base
    pub fn new() -> Self {
        let mut kb = Self {
            tactics: HashMap::new(),
            techniques: HashMap::new(),
            sub_techniques: HashMap::new(),
            mitigations: HashMap::new(),
            groups: HashMap::new(),
            software: HashMap::new(),
        };
        kb.load_attack_framework();
        kb
    }

    /// Load the ATT&CK framework data
    fn load_attack_framework(&mut self) {
        // Load tactics (Enterprise ATT&CK)
        self.load_tactics();
        // Load techniques
        self.load_techniques();
        // Load mitigations
        self.load_mitigations();
        // Load threat groups
        self.load_threat_groups();
        // Load software
        self.load_software();
    }

    /// Load tactics
    fn load_tactics(&mut self) {
        let tactics = vec![
            ("TA0043", "Reconnaissance", "The adversary is trying to gather information they can use to plan future operations."),
            ("TA0042", "Resource Development", "The adversary is trying to establish resources they can use to support operations."),
            ("TA0001", "Initial Access", "The adversary is trying to get into your network."),
            ("TA0002", "Execution", "The adversary is trying to run malicious code."),
            ("TA0003", "Persistence", "The adversary is trying to maintain their foothold."),
            ("TA0004", "Privilege Escalation", "The adversary is trying to gain higher-level permissions."),
            ("TA0005", "Defense Evasion", "The adversary is trying to avoid being detected."),
            ("TA0006", "Credential Access", "The adversary is trying to steal account names and passwords."),
            ("TA0007", "Discovery", "The adversary is trying to figure out your environment."),
            ("TA0008", "Lateral Movement", "The adversary is trying to move through your environment."),
            ("TA0009", "Collection", "The adversary is trying to gather data of interest to their goal."),
            ("TA0011", "Command and Control", "The adversary is trying to communicate with compromised systems."),
            ("TA0010", "Exfiltration", "The adversary is trying to steal data."),
            ("TA0040", "Impact", "The adversary is trying to manipulate, interrupt, or destroy your systems and data."),
        ];

        for (id, name, description) in tactics {
            self.tactics.insert(
                id.to_string(),
                Tactic {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: description.to_string(),
                    techniques: Vec::new(),
                },
            );
        }
    }

    /// Load techniques
    fn load_techniques(&mut self) {
        let techniques = vec![
            // Initial Access
            Technique {
                id: "T1190".to_string(),
                name: "Exploit Public-Facing Application".to_string(),
                description: "Adversaries may attempt to exploit a weakness in an Internet-facing host or system to initially access a network.".to_string(),
                tactic_ids: vec!["TA0001".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string(), "Containers".to_string()],
                detection: "Monitor application logs for abnormal behavior. Use web application firewalls.".to_string(),
                mitigations: vec!["M1048".to_string(), "M1050".to_string(), "M1051".to_string()],
                data_sources: vec!["Application Log".to_string(), "Network Traffic".to_string()],
                sub_techniques: Vec::new(),
            },
            Technique {
                id: "T1133".to_string(),
                name: "External Remote Services".to_string(),
                description: "Adversaries may leverage external-facing remote services to initially access and/or persist within a network.".to_string(),
                tactic_ids: vec!["TA0001".to_string(), "TA0003".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for unusual login attempts to remote services.".to_string(),
                mitigations: vec!["M1035".to_string(), "M1032".to_string()],
                data_sources: vec!["Logon Session".to_string(), "Network Traffic".to_string()],
                sub_techniques: Vec::new(),
            },
            Technique {
                id: "T1566".to_string(),
                name: "Phishing".to_string(),
                description: "Adversaries may send phishing messages to gain access to victim systems.".to_string(),
                tactic_ids: vec!["TA0001".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for suspicious email attachments and links.".to_string(),
                mitigations: vec!["M1049".to_string(), "M1017".to_string()],
                data_sources: vec!["Email".to_string(), "Network Traffic".to_string()],
                sub_techniques: vec!["T1566.001".to_string(), "T1566.002".to_string(), "T1566.003".to_string()],
            },
            // Execution
            Technique {
                id: "T1059".to_string(),
                name: "Command and Scripting Interpreter".to_string(),
                description: "Adversaries may abuse command and script interpreters to execute commands, scripts, or binaries.".to_string(),
                tactic_ids: vec!["TA0002".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor process execution and command-line arguments.".to_string(),
                mitigations: vec!["M1049".to_string(), "M1038".to_string()],
                data_sources: vec!["Process".to_string(), "Command".to_string()],
                sub_techniques: vec!["T1059.001".to_string(), "T1059.003".to_string(), "T1059.004".to_string()],
            },
            Technique {
                id: "T1203".to_string(),
                name: "Exploitation for Client Execution".to_string(),
                description: "Adversaries may exploit software vulnerabilities in client applications to execute code.".to_string(),
                tactic_ids: vec!["TA0002".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for abnormal process behavior after opening files.".to_string(),
                mitigations: vec!["M1048".to_string(), "M1050".to_string()],
                data_sources: vec!["Process".to_string(), "File".to_string()],
                sub_techniques: Vec::new(),
            },
            // Persistence
            Technique {
                id: "T1505".to_string(),
                name: "Server Software Component".to_string(),
                description: "Adversaries may abuse legitimate extensible development features of servers to establish persistent access.".to_string(),
                tactic_ids: vec!["TA0003".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for changes to server software components.".to_string(),
                mitigations: vec!["M1042".to_string(), "M1026".to_string()],
                data_sources: vec!["File".to_string(), "Application Log".to_string()],
                sub_techniques: vec!["T1505.003".to_string()],
            },
            Technique {
                id: "T1078".to_string(),
                name: "Valid Accounts".to_string(),
                description: "Adversaries may obtain and abuse credentials of existing accounts as a means of gaining Initial Access, Persistence, Privilege Escalation, or Defense Evasion.".to_string(),
                tactic_ids: vec!["TA0001".to_string(), "TA0003".to_string(), "TA0004".to_string(), "TA0005".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string(), "Azure AD".to_string()],
                detection: "Monitor for unusual account activity and login patterns.".to_string(),
                mitigations: vec!["M1027".to_string(), "M1026".to_string()],
                data_sources: vec!["Logon Session".to_string(), "User Account".to_string()],
                sub_techniques: vec!["T1078.001".to_string(), "T1078.002".to_string(), "T1078.003".to_string()],
            },
            // Privilege Escalation
            Technique {
                id: "T1068".to_string(),
                name: "Exploitation for Privilege Escalation".to_string(),
                description: "Adversaries may exploit software vulnerabilities in an attempt to elevate privileges.".to_string(),
                tactic_ids: vec!["TA0004".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for abnormal process behavior and privilege changes.".to_string(),
                mitigations: vec!["M1048".to_string(), "M1019".to_string()],
                data_sources: vec!["Process".to_string()],
                sub_techniques: Vec::new(),
            },
            // Credential Access
            Technique {
                id: "T1110".to_string(),
                name: "Brute Force".to_string(),
                description: "Adversaries may use brute force techniques to gain access to accounts when passwords are unknown or when password hashes are obtained.".to_string(),
                tactic_ids: vec!["TA0006".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string(), "Azure AD".to_string()],
                detection: "Monitor for multiple failed login attempts.".to_string(),
                mitigations: vec!["M1036".to_string(), "M1032".to_string()],
                data_sources: vec!["Logon Session".to_string(), "User Account".to_string()],
                sub_techniques: vec!["T1110.001".to_string(), "T1110.002".to_string(), "T1110.003".to_string()],
            },
            Technique {
                id: "T1552".to_string(),
                name: "Unsecured Credentials".to_string(),
                description: "Adversaries may search compromised systems to find and obtain insecurely stored credentials.".to_string(),
                tactic_ids: vec!["TA0006".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for access to credential stores and configuration files.".to_string(),
                mitigations: vec!["M1027".to_string(), "M1022".to_string()],
                data_sources: vec!["File".to_string(), "Process".to_string()],
                sub_techniques: vec!["T1552.001".to_string(), "T1552.004".to_string()],
            },
            // Discovery
            Technique {
                id: "T1046".to_string(),
                name: "Network Service Discovery".to_string(),
                description: "Adversaries may attempt to get a listing of services running on remote hosts and local network infrastructure devices.".to_string(),
                tactic_ids: vec!["TA0007".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for port scanning activity.".to_string(),
                mitigations: vec!["M1042".to_string(), "M1030".to_string()],
                data_sources: vec!["Network Traffic".to_string(), "Process".to_string()],
                sub_techniques: Vec::new(),
            },
            Technique {
                id: "T1018".to_string(),
                name: "Remote System Discovery".to_string(),
                description: "Adversaries may attempt to get a listing of other systems by IP address, hostname, or other logical identifier on a network.".to_string(),
                tactic_ids: vec!["TA0007".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for network enumeration commands.".to_string(),
                mitigations: vec!["M1042".to_string()],
                data_sources: vec!["Network Traffic".to_string(), "Process".to_string()],
                sub_techniques: Vec::new(),
            },
            // Lateral Movement
            Technique {
                id: "T1021".to_string(),
                name: "Remote Services".to_string(),
                description: "Adversaries may use Valid Accounts to log into a service that accepts remote connections.".to_string(),
                tactic_ids: vec!["TA0008".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for unusual remote service connections.".to_string(),
                mitigations: vec!["M1032".to_string(), "M1035".to_string()],
                data_sources: vec!["Logon Session".to_string(), "Network Traffic".to_string()],
                sub_techniques: vec!["T1021.001".to_string(), "T1021.002".to_string(), "T1021.004".to_string()],
            },
            Technique {
                id: "T1210".to_string(),
                name: "Exploitation of Remote Services".to_string(),
                description: "Adversaries may exploit remote services to gain unauthorized access to internal systems.".to_string(),
                tactic_ids: vec!["TA0008".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for exploitation attempts against remote services.".to_string(),
                mitigations: vec!["M1048".to_string(), "M1030".to_string()],
                data_sources: vec!["Network Traffic".to_string(), "Application Log".to_string()],
                sub_techniques: Vec::new(),
            },
            // Command and Control
            Technique {
                id: "T1071".to_string(),
                name: "Application Layer Protocol".to_string(),
                description: "Adversaries may communicate using OSI application layer protocols to avoid detection.".to_string(),
                tactic_ids: vec!["TA0011".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for unusual application layer traffic.".to_string(),
                mitigations: vec!["M1031".to_string()],
                data_sources: vec!["Network Traffic".to_string()],
                sub_techniques: vec!["T1071.001".to_string(), "T1071.004".to_string()],
            },
            // Defense Evasion
            Technique {
                id: "T1040".to_string(),
                name: "Network Sniffing".to_string(),
                description: "Adversaries may sniff network traffic to capture information about an environment.".to_string(),
                tactic_ids: vec!["TA0006".to_string(), "TA0007".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for promiscuous mode on network interfaces.".to_string(),
                mitigations: vec!["M1041".to_string(), "M1032".to_string()],
                data_sources: vec!["Process".to_string(), "Network Traffic".to_string()],
                sub_techniques: Vec::new(),
            },
            Technique {
                id: "T1557".to_string(),
                name: "Adversary-in-the-Middle".to_string(),
                description: "Adversaries may attempt to position themselves between two or more networked devices to support follow-on behaviors.".to_string(),
                tactic_ids: vec!["TA0006".to_string(), "TA0009".to_string()],
                platforms: vec!["Windows".to_string(), "Linux".to_string(), "macOS".to_string()],
                detection: "Monitor for ARP spoofing and unusual network traffic patterns.".to_string(),
                mitigations: vec!["M1041".to_string(), "M1037".to_string()],
                data_sources: vec!["Network Traffic".to_string()],
                sub_techniques: vec!["T1557.001".to_string(), "T1557.002".to_string()],
            },
            // Reconnaissance
            Technique {
                id: "T1589".to_string(),
                name: "Gather Victim Identity Information".to_string(),
                description: "Adversaries may gather information about the victim's identity that can be used during targeting.".to_string(),
                tactic_ids: vec!["TA0043".to_string()],
                platforms: vec!["PRE".to_string()],
                detection: "Monitor for social engineering attempts and data leaks.".to_string(),
                mitigations: vec!["M1056".to_string()],
                data_sources: vec!["Social Media".to_string()],
                sub_techniques: vec!["T1589.001".to_string(), "T1589.002".to_string()],
            },
        ];

        for technique in techniques {
            self.techniques.insert(technique.id.clone(), technique);
        }
    }

    /// Load mitigations
    fn load_mitigations(&mut self) {
        let mitigations = vec![
            ("M1048", "Application Isolation and Sandboxing", "Restrict execution of code to a virtual environment on or in transit to an endpoint system."),
            ("M1050", "Exploit Protection", "Use capabilities to detect and block conditions that may lead to or be indicative of a software exploit occurring."),
            ("M1051", "Update Software", "Perform regular software updates to mitigate exploitation risk."),
            ("M1035", "Limit Access to Resource Over Network", "Prevent access to file shares, remote access to systems, unnecessary services."),
            ("M1032", "Multi-factor Authentication", "Use two or more pieces of evidence to authenticate to a system."),
            ("M1049", "Antivirus/Antimalware", "Use signatures or heuristics to detect malicious software."),
            ("M1017", "User Training", "Train users to be aware of access or manipulation attempts by an adversary."),
            ("M1038", "Execution Prevention", "Block execution of code on a system through application control."),
            ("M1042", "Disable or Remove Feature or Program", "Remove or deny access to unnecessary and potentially vulnerable software."),
            ("M1026", "Privileged Account Management", "Manage the creation, modification, use, and permissions associated to privileged accounts."),
            ("M1027", "Password Policies", "Set and enforce secure password policies for accounts."),
            ("M1019", "Threat Intelligence Program", "A threat intelligence program helps an organization generate their own threat intelligence."),
            ("M1036", "Account Use Policies", "Configure features related to account use like login attempt lockouts."),
            ("M1022", "Restrict File and Directory Permissions", "Restrict access by setting directory and file permissions that are not specific to users or privileged accounts."),
            ("M1030", "Network Segmentation", "Architect sections of the network to isolate critical systems, functions, or resources."),
            ("M1031", "Network Intrusion Prevention", "Use intrusion detection signatures to block traffic at network boundaries."),
            ("M1041", "Encrypt Sensitive Information", "Protect sensitive information with strong encryption."),
            ("M1037", "Filter Network Traffic", "Use network appliances to filter ingress or egress traffic."),
            ("M1056", "Pre-compromise", "This category is used for any applicable mitigation activities that apply to techniques occurring before an adversary gains Initial Access."),
        ];

        for (id, name, description) in mitigations {
            self.mitigations.insert(
                id.to_string(),
                Mitigation {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: description.to_string(),
                },
            );
        }
    }

    /// Load threat groups
    fn load_threat_groups(&mut self) {
        let groups = vec![
            ThreatGroup {
                id: "G0016".to_string(),
                name: "APT29".to_string(),
                aliases: vec!["Cozy Bear".to_string(), "The Dukes".to_string()],
                description: "APT29 is a threat group attributed to Russia's Foreign Intelligence Service (SVR).".to_string(),
                techniques: vec!["T1190".to_string(), "T1566".to_string(), "T1078".to_string()],
            },
            ThreatGroup {
                id: "G0022".to_string(),
                name: "APT3".to_string(),
                aliases: vec!["Gothic Panda".to_string(), "UPS Team".to_string()],
                description: "APT3 is a China-based threat group attributed to the Chinese Ministry of State Security.".to_string(),
                techniques: vec!["T1190".to_string(), "T1059".to_string(), "T1068".to_string()],
            },
            ThreatGroup {
                id: "G0032".to_string(),
                name: "Lazarus Group".to_string(),
                aliases: vec!["Hidden Cobra".to_string(), "Guardians of Peace".to_string()],
                description: "Lazarus Group is a North Korean state-sponsored cyber threat group.".to_string(),
                techniques: vec!["T1566".to_string(), "T1059".to_string(), "T1110".to_string()],
            },
            ThreatGroup {
                id: "G0045".to_string(),
                name: "menuPass".to_string(),
                aliases: vec!["APT10".to_string(), "Stone Panda".to_string()],
                description: "menuPass is a threat group that has been active since at least 2006.".to_string(),
                techniques: vec!["T1133".to_string(), "T1078".to_string(), "T1021".to_string()],
            },
            ThreatGroup {
                id: "G0102".to_string(),
                name: "Wizard Spider".to_string(),
                aliases: vec!["UNC1878".to_string()],
                description: "Wizard Spider is a Russia-based financially motivated threat group.".to_string(),
                techniques: vec!["T1566".to_string(), "T1059".to_string(), "T1110".to_string()],
            },
        ];

        for group in groups {
            self.groups.insert(group.id.clone(), group);
        }
    }

    /// Load software/malware
    fn load_software(&mut self) {
        let software = vec![
            Software {
                id: "S0154".to_string(),
                name: "Cobalt Strike".to_string(),
                software_type: SoftwareType::Tool,
                description: "Cobalt Strike is a commercial, full-featured, remote access tool.".to_string(),
                techniques: vec!["T1059".to_string(), "T1071".to_string(), "T1021".to_string()],
            },
            Software {
                id: "S0002".to_string(),
                name: "Mimikatz".to_string(),
                software_type: SoftwareType::Tool,
                description: "Mimikatz is a credential dumper capable of obtaining plaintext Windows account logins and passwords.".to_string(),
                techniques: vec!["T1552".to_string(), "T1078".to_string()],
            },
            Software {
                id: "S0650".to_string(),
                name: "Emotet".to_string(),
                software_type: SoftwareType::Malware,
                description: "Emotet is a modular malware variant which is primarily used as a downloader for other malware variants.".to_string(),
                techniques: vec!["T1566".to_string(), "T1059".to_string()],
            },
            Software {
                id: "S0446".to_string(),
                name: "Ryuk".to_string(),
                software_type: SoftwareType::Malware,
                description: "Ryuk is a ransomware that has been used in targeted campaigns.".to_string(),
                techniques: vec!["T1486".to_string(), "T1059".to_string()],
            },
        ];

        for sw in software {
            self.software.insert(sw.id.clone(), sw);
        }
    }

    /// Map scan findings to MITRE ATT&CK techniques
    pub async fn map_findings(&self, scan_result: &ScanResult) -> Result<MitreMapping, SecurityAgentError> {
        let mut mapped_techniques = Vec::new();

        for host in &scan_result.hosts {
            for port in &host.ports {
                if port.state != crate::scanner::PortState::Open {
                    continue;
                }

                // Map open ports to potential techniques
                let techniques = self.map_port_to_techniques(port.port, port.service.as_deref());
                for tech_id in techniques {
                    if let Some(technique) = self.techniques.get(&tech_id) {
                        mapped_techniques.push(MappedTechnique {
                            technique_id: technique.id.clone(),
                            technique_name: technique.name.clone(),
                            tactic_ids: technique.tactic_ids.clone(),
                            confidence: 0.7,
                            evidence: format!(
                                "Port {} ({}) open on {}",
                                port.port,
                                port.service.as_deref().unwrap_or("unknown"),
                                host.ip
                            ),
                            mitigations: technique.mitigations.clone(),
                        });
                    }
                }
            }
        }

        // Generate potential attack paths
        let attack_paths = self.generate_attack_paths(&mapped_techniques);

        // Get relevant threat groups
        let relevant_groups = self.find_relevant_groups(&mapped_techniques);

        // Generate recommendations before moving attack_paths
        let recommendations = self.generate_mitre_recommendations(&attack_paths);

        Ok(MitreMapping {
            scan_id: scan_result.id,
            techniques: mapped_techniques,
            attack_paths,
            relevant_groups,
            recommendations,
        })
    }

    /// Map a port/service to potential ATT&CK techniques
    fn map_port_to_techniques(&self, port: u16, service: Option<&str>) -> Vec<String> {
        let mut techniques = Vec::new();

        match port {
            21 => {
                techniques.push("T1133".to_string()); // External Remote Services
                techniques.push("T1110".to_string()); // Brute Force
            }
            22 => {
                techniques.push("T1133".to_string()); // External Remote Services
                techniques.push("T1021".to_string()); // Remote Services
                techniques.push("T1110".to_string()); // Brute Force
            }
            23 => {
                techniques.push("T1133".to_string()); // External Remote Services
                techniques.push("T1040".to_string()); // Network Sniffing
                techniques.push("T1557".to_string()); // Adversary-in-the-Middle
            }
            25 | 587 | 465 => {
                techniques.push("T1566".to_string()); // Phishing
            }
            80 | 443 | 8080 | 8443 => {
                techniques.push("T1190".to_string()); // Exploit Public-Facing Application
                techniques.push("T1071".to_string()); // Application Layer Protocol
            }
            135 | 139 | 445 => {
                techniques.push("T1210".to_string()); // Exploitation of Remote Services
                techniques.push("T1021".to_string()); // Remote Services
            }
            389 | 636 => {
                techniques.push("T1018".to_string()); // Remote System Discovery
                techniques.push("T1078".to_string()); // Valid Accounts
            }
            1433 | 3306 | 5432 | 27017 => {
                techniques.push("T1190".to_string()); // Exploit Public-Facing Application
                techniques.push("T1110".to_string()); // Brute Force
            }
            3389 => {
                techniques.push("T1133".to_string()); // External Remote Services
                techniques.push("T1021".to_string()); // Remote Services
                techniques.push("T1210".to_string()); // Exploitation of Remote Services
            }
            5900 => {
                techniques.push("T1021".to_string()); // Remote Services
                techniques.push("T1110".to_string()); // Brute Force
            }
            _ => {}
        }

        // Add techniques based on service name
        if let Some(svc) = service {
            let svc_lower = svc.to_lowercase();
            if svc_lower.contains("ssh") {
                techniques.push("T1021".to_string());
            }
            if svc_lower.contains("http") || svc_lower.contains("web") {
                techniques.push("T1190".to_string());
            }
            if svc_lower.contains("smb") || svc_lower.contains("microsoft-ds") {
                techniques.push("T1210".to_string());
            }
        }

        techniques.sort();
        techniques.dedup();
        techniques
    }

    /// Generate potential attack paths
    fn generate_attack_paths(&self, techniques: &[MappedTechnique]) -> Vec<AttackPath> {
        let mut paths = Vec::new();

        // Check for Initial Access -> Execution -> Persistence patterns
        let has_initial_access = techniques.iter().any(|t| t.tactic_ids.contains(&"TA0001".to_string()));
        let has_execution = techniques.iter().any(|t| t.tactic_ids.contains(&"TA0002".to_string()));
        let has_lateral = techniques.iter().any(|t| t.tactic_ids.contains(&"TA0008".to_string()));

        if has_initial_access {
            let mut path = AttackPath {
                name: "External Compromise".to_string(),
                description: "Adversary gains initial access through external-facing services".to_string(),
                stages: vec![
                    AttackStage {
                        tactic: "Initial Access".to_string(),
                        techniques: techniques
                            .iter()
                            .filter(|t| t.tactic_ids.contains(&"TA0001".to_string()))
                            .map(|t| t.technique_id.clone())
                            .collect(),
                    },
                ],
                risk_level: "High".to_string(),
            };

            if has_execution {
                path.stages.push(AttackStage {
                    tactic: "Execution".to_string(),
                    techniques: techniques
                        .iter()
                        .filter(|t| t.tactic_ids.contains(&"TA0002".to_string()))
                        .map(|t| t.technique_id.clone())
                        .collect(),
                });
            }

            if has_lateral {
                path.stages.push(AttackStage {
                    tactic: "Lateral Movement".to_string(),
                    techniques: techniques
                        .iter()
                        .filter(|t| t.tactic_ids.contains(&"TA0008".to_string()))
                        .map(|t| t.technique_id.clone())
                        .collect(),
                });
                path.risk_level = "Critical".to_string();
            }

            paths.push(path);
        }

        paths
    }

    /// Find relevant threat groups based on techniques
    fn find_relevant_groups(&self, techniques: &[MappedTechnique]) -> Vec<String> {
        let technique_ids: Vec<&str> = techniques.iter().map(|t| t.technique_id.as_str()).collect();

        self.groups
            .values()
            .filter(|g| g.techniques.iter().any(|t| technique_ids.contains(&t.as_str())))
            .map(|g| format!("{} ({})", g.name, g.aliases.join(", ")))
            .collect()
    }

    /// Generate recommendations based on attack paths
    fn generate_mitre_recommendations(&self, paths: &[AttackPath]) -> Vec<String> {
        let mut recommendations = Vec::new();

        for path in paths {
            for stage in &path.stages {
                for tech_id in &stage.techniques {
                    if let Some(technique) = self.techniques.get(tech_id) {
                        for mit_id in &technique.mitigations {
                            if let Some(mitigation) = self.mitigations.get(mit_id) {
                                let rec = format!(
                                    "[{}] {}: {}",
                                    mit_id, mitigation.name, mitigation.description
                                );
                                if !recommendations.contains(&rec) {
                                    recommendations.push(rec);
                                }
                            }
                        }
                    }
                }
            }
        }

        recommendations
    }

    /// Get technique by ID
    pub fn get_technique(&self, id: &str) -> Option<&Technique> {
        self.techniques.get(id)
    }

    /// Get tactic by ID
    pub fn get_tactic(&self, id: &str) -> Option<&Tactic> {
        self.tactics.get(id)
    }

    /// Get mitigation by ID
    pub fn get_mitigation(&self, id: &str) -> Option<&Mitigation> {
        self.mitigations.get(id)
    }

    /// Search techniques by keyword
    pub fn search_techniques(&self, keyword: &str) -> Vec<&Technique> {
        let keyword_lower = keyword.to_lowercase();
        self.techniques
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&keyword_lower)
                    || t.description.to_lowercase().contains(&keyword_lower)
                    || t.id.to_lowercase().contains(&keyword_lower)
            })
            .collect()
    }

    /// Get all tactics
    pub fn get_tactics(&self) -> Vec<&Tactic> {
        self.tactics.values().collect()
    }

    /// Get all techniques
    pub fn get_techniques(&self) -> Vec<&Technique> {
        self.techniques.values().collect()
    }

    /// Get all threat groups
    pub fn get_threat_groups(&self) -> Vec<&ThreatGroup> {
        self.groups.values().collect()
    }

    /// Get all mitigations
    pub fn get_mitigations(&self) -> Vec<&Mitigation> {
        self.mitigations.values().collect()
    }

    /// Get all software
    pub fn get_software(&self) -> Vec<&Software> {
        self.software.values().collect()
    }
}

impl Default for MitreAttackKB {
    fn default() -> Self {
        Self::new()
    }
}

/// MITRE ATT&CK Tactic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactic {
    pub id: String,
    pub name: String,
    pub description: String,
    pub techniques: Vec<String>,
}

/// MITRE ATT&CK Technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technique {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tactic_ids: Vec<String>,
    pub platforms: Vec<String>,
    pub detection: String,
    pub mitigations: Vec<String>,
    pub data_sources: Vec<String>,
    pub sub_techniques: Vec<String>,
}

/// MITRE ATT&CK Sub-Technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTechnique {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parent_id: String,
}

/// MITRE ATT&CK Mitigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mitigation {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// Threat Group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatGroup {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub techniques: Vec<String>,
}

/// Software/Malware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Software {
    pub id: String,
    pub name: String,
    pub software_type: SoftwareType,
    pub description: String,
    pub techniques: Vec<String>,
}

/// Software type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoftwareType {
    Tool,
    Malware,
}

/// MITRE mapping result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMapping {
    pub scan_id: uuid::Uuid,
    pub techniques: Vec<MappedTechnique>,
    pub attack_paths: Vec<AttackPath>,
    pub relevant_groups: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Mapped technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic_ids: Vec<String>,
    pub confidence: f32,
    pub evidence: String,
    pub mitigations: Vec<String>,
}

/// Attack path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPath {
    pub name: String,
    pub description: String,
    pub stages: Vec<AttackStage>,
    pub risk_level: String,
}

/// Attack stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackStage {
    pub tactic: String,
    pub techniques: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mitre_kb_creation() {
        let kb = MitreAttackKB::new();
        assert!(!kb.tactics.is_empty());
        assert!(!kb.techniques.is_empty());
        assert!(!kb.mitigations.is_empty());
    }

    #[test]
    fn test_search_techniques() {
        let kb = MitreAttackKB::new();
        let results = kb.search_techniques("phishing");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_get_technique() {
        let kb = MitreAttackKB::new();
        let technique = kb.get_technique("T1190");
        assert!(technique.is_some());
        assert_eq!(technique.unwrap().name, "Exploit Public-Facing Application");
    }
}
