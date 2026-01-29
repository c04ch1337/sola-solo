# Network Security Agent

## Overview

The Network Security Agent is a sophisticated offensive security and network intelligence module for Phoenix AGI (SOLA). It provides OSCP/Kali-level capabilities for network scanning, vulnerability assessment, and security-gated exploit operations.

## Features

### 🔍 Network Scanning (nmap-like)
- **Host Discovery**: Ping sweep, ARP scan, TCP/UDP probing
- **Port Scanning**: TCP SYN, TCP Connect, UDP scanning
- **Service Detection**: Banner grabbing, version fingerprinting
- **OS Fingerprinting**: TTL-based and TCP/IP stack analysis
- **Target Parsing**: IP, CIDR notation, hostname, IP ranges

### 🛡️ Vulnerability Assessment
- **CVE Database**: Built-in vulnerability signatures
- **Service Mapping**: Automatic vulnerability detection based on services
- **Risk Scoring**: CVSS-based risk calculation
- **Remediation**: Actionable fix recommendations

### 🎯 MITRE ATT&CK Integration
- **Technique Mapping**: Map findings to ATT&CK techniques
- **Attack Path Analysis**: Generate potential attack chains
- **Threat Group Correlation**: Identify relevant threat actors
- **Mitigation Recommendations**: ATT&CK-based defensive guidance

### 📋 Security Playbooks
- **Network Reconnaissance**: Comprehensive network discovery
- **Web Application Assessment**: OWASP-based testing
- **Internal Penetration Test**: Full internal network assessment
- **Wireless Security**: WiFi security testing
- **Password Audit**: Credential strength assessment
- **Red Team Engagement**: Full adversary simulation

### ⚔️ Exploit Framework (Security-Gated)
- **Exploit Modules**: EternalBlue, BlueKeep, Log4Shell, etc.
- **Payload Generation**: Reverse shells, bind shells, Meterpreter
- **Session Management**: Track active sessions
- **CVE Search**: Find exploits by CVE ID

### 🔧 Kali Linux Tool Integration
- **Reconnaissance**: nmap, masscan, amass, subfinder
- **Web**: nikto, gobuster, ffuf, sqlmap, nuclei
- **Exploitation**: metasploit, crackmapexec, searchsploit
- **Password**: hashcat, john, hydra
- **Wireless**: aircrack-ng, airodump-ng
- **Post-Exploitation**: mimikatz, bloodhound, impacket

## Security Model

The agent implements a multi-tier security model to prevent unauthorized operations:

### Security Levels

| Level | Description | Operations |
|-------|-------------|------------|
| **Passive** | Read-only reconnaissance | Network discovery, passive OSINT |
| **Active** | Active scanning with consent | Port scanning, service detection |
| **Exploit** | Vulnerability exploitation | Exploit execution, credential attacks |
| **Offensive** | Full offensive capabilities | Red team operations, domain compromise |

### Authorization Flow

```
User Request → Security Gate Check → Authorization Validation → Operation Execution
                     ↓
              Audit Logging
```

### Security Gate Features
- **Time-limited Authorization**: Authorizations expire after specified duration
- **Target Restrictions**: Limit operations to specific IP ranges/hosts
- **Audit Logging**: All security decisions are logged
- **Multi-factor Confirmation**: Critical operations require explicit confirmation

## Usage

### Basic Network Scan

```rust
use network_security_agent::{NetworkSecurityAgent, ScanConfig, SecurityLevel};

#[tokio::main]
async fn main() {
    // Create agent
    let agent = NetworkSecurityAgent::awaken().await.unwrap();
    
    // Authorize active scanning
    agent.authorize(
        SecurityLevel::Active,
        "admin",
        Some(1), // 1 hour
        vec!["192.168.1.0/24".to_string()],
    ).await.unwrap();
    
    // Perform scan
    let config = ScanConfig::default()
        .with_target("192.168.1.0/24")
        .with_scan_type(ScanType::ServiceDetection);
    
    let result = agent.scan(&config).await.unwrap();
    
    // Analyze vulnerabilities
    let vuln_report = agent.analyze_vulnerabilities(result.id).await.unwrap();
    
    // Map to MITRE ATT&CK
    let mitre_mapping = agent.map_to_mitre(result.id).await.unwrap();
    
    println!("Found {} vulnerabilities", vuln_report.vulnerabilities.len());
    println!("Risk Score: {}", vuln_report.overall_risk_score);
}
```

### Execute Playbook

```rust
// Execute network reconnaissance playbook
let result = agent.execute_playbook("network-recon", "192.168.1.0/24").await.unwrap();

println!("Playbook completed: {}", result.summary);
for step in result.step_results {
    println!("  Step {}: {} - {:?}", step.step_id, step.step_name, step.status);
}
```

### Proactive Scanning

```rust
// Run proactive security scan
let report = agent.proactive_scan(vec![
    "192.168.1.0/24".to_string(),
    "10.0.0.0/24".to_string(),
]).await.unwrap();

println!("Scanned {} hosts, {} vulnerable", report.total_hosts, report.vulnerable_hosts);
```

### AI-Powered Analysis

```rust
// Get AI analysis of scan results
let analysis = agent.analyze_with_ai(scan_result.id).await.unwrap();
println!("{}", analysis.analysis);
```

## Chat Commands

SOLA can execute network security operations via chat:

```
User: "Scan my network for vulnerabilities"
SOLA: 🔍 Starting network scan on 192.168.1.0/24...
      Found 15 hosts, 3 with open ports
      Analyzing vulnerabilities...
      
      📊 Scan Summary:
      - Critical: 2 vulnerabilities
      - High: 5 vulnerabilities
      - Medium: 8 vulnerabilities
      
      Top Findings:
      1. CVE-2017-0144 (EternalBlue) on 192.168.1.50:445
      2. CVE-2019-0708 (BlueKeep) on 192.168.1.100:3389
      
      Recommendations:
      - Apply MS17-010 security update immediately
      - Disable RDP or apply Windows security updates
```

## Playbook Reference

### network-recon
**Category**: Reconnaissance  
**Required Level**: Active  
**Duration**: ~30 minutes

Steps:
1. Passive Information Gathering (whois, DNS)
2. Host Discovery (ping sweep)
3. Port Scanning (TCP/UDP)
4. Service Enumeration (banners, versions)

### web-app-assessment
**Category**: Web Application  
**Required Level**: Active  
**Duration**: ~60 minutes

Steps:
1. Technology Fingerprinting
2. Directory Enumeration
3. Vulnerability Scanning
4. SQL Injection Testing
5. XSS Testing

### internal-pentest
**Category**: Internal Network  
**Required Level**: Exploit  
**Duration**: ~4 hours

Steps:
1. Network Discovery
2. Service Enumeration
3. SMB Enumeration
4. LDAP Enumeration
5. Vulnerability Assessment
6. Credential Attacks
7. Exploitation

### wireless-assessment
**Category**: Wireless  
**Required Level**: Active  
**Duration**: ~2 hours

Steps:
1. Wireless Discovery
2. Client Discovery
3. WPA Handshake Capture
4. Password Cracking
5. Evil Twin Attack

### password-audit
**Category**: Credential Testing  
**Required Level**: Active  
**Duration**: ~3 hours

Steps:
1. Hash Extraction
2. Dictionary Attack
3. Rule-Based Attack
4. Mask Attack

### quick-vuln-scan
**Category**: Vulnerability Assessment  
**Required Level**: Active  
**Duration**: ~15 minutes

Steps:
1. Fast Port Scan
2. Vulnerability Scripts
3. CVE Check

### red-team-engagement
**Category**: Red Team  
**Required Level**: Offensive  
**Duration**: ~8 hours

Steps:
1. Reconnaissance
2. Initial Access
3. Persistence
4. Privilege Escalation
5. Lateral Movement
6. Data Exfiltration

## MITRE ATT&CK Coverage

### Tactics Covered
- TA0043: Reconnaissance
- TA0042: Resource Development
- TA0001: Initial Access
- TA0002: Execution
- TA0003: Persistence
- TA0004: Privilege Escalation
- TA0005: Defense Evasion
- TA0006: Credential Access
- TA0007: Discovery
- TA0008: Lateral Movement
- TA0011: Command and Control

### Key Techniques
- T1190: Exploit Public-Facing Application
- T1133: External Remote Services
- T1566: Phishing
- T1059: Command and Scripting Interpreter
- T1078: Valid Accounts
- T1068: Exploitation for Privilege Escalation
- T1110: Brute Force
- T1046: Network Service Discovery
- T1021: Remote Services
- T1210: Exploitation of Remote Services

## Vulnerability Database

### Critical Vulnerabilities
| CVE | Name | CVSS | Affected Services |
|-----|------|------|-------------------|
| CVE-2021-44228 | Log4Shell | 10.0 | Java, Tomcat, Elasticsearch |
| CVE-2017-0144 | EternalBlue | 9.8 | SMB (445) |
| CVE-2019-0708 | BlueKeep | 9.8 | RDP (3389) |
| CVE-2020-1472 | Zerologon | 10.0 | Netlogon |
| CVE-2021-26855 | ProxyLogon | 9.8 | Exchange |

### High Vulnerabilities
| CVE | Name | CVSS | Affected Services |
|-----|------|------|-------------------|
| CVE-2014-0160 | Heartbleed | 7.5 | SSL/TLS |
| CVE-2012-2122 | MySQL Auth Bypass | 7.5 | MySQL |
| TELNET-CLEARTEXT | Telnet Cleartext | 7.5 | Telnet |

## Kali Tool Reference

### Reconnaissance
| Tool | Description | Required Level |
|------|-------------|----------------|
| nmap | Network scanner | Active |
| masscan | Fast port scanner | Active |
| amass | Subdomain enumeration | Passive |
| subfinder | Subdomain discovery | Passive |
| theHarvester | Email harvester | Passive |

### Web Application
| Tool | Description | Required Level |
|------|-------------|----------------|
| nikto | Web server scanner | Active |
| gobuster | Directory brute-forcer | Active |
| ffuf | Web fuzzer | Active |
| sqlmap | SQL injection | Active |
| nuclei | Vulnerability scanner | Active |

### Exploitation
| Tool | Description | Required Level |
|------|-------------|----------------|
| msfconsole | Metasploit | Exploit |
| searchsploit | Exploit-DB search | Passive |
| crackmapexec | Network pentesting | Active |

### Password
| Tool | Description | Required Level |
|------|-------------|----------------|
| hashcat | Password recovery | Active |
| john | John the Ripper | Active |
| hydra | Login brute-forcer | Active |

### Wireless
| Tool | Description | Required Level |
|------|-------------|----------------|
| aircrack-ng | WiFi security | Active |
| airodump-ng | Packet capture | Active |

### Post-Exploitation
| Tool | Description | Required Level |
|------|-------------|----------------|
| mimikatz | Credential extraction | Exploit |
| bloodhound-python | AD enumeration | Active |
| impacket-secretsdump | Secrets dumping | Exploit |

## Configuration

### Environment Variables

```bash
# Security settings
NETWORK_SECURITY_AGENT_ENABLED=true
NETWORK_SECURITY_DEFAULT_LEVEL=passive
NETWORK_SECURITY_REQUIRE_CONFIRMATION=true

# Docker/Kali settings
KALI_DOCKER_ENABLED=true
KALI_DOCKER_IMAGE=kalilinux/kali-rolling

# Scan settings
SCAN_TIMEOUT_MS=3000
SCAN_MAX_THREADS=100
SCAN_DEFAULT_PORTS=top1000
```

## Self-Evolution

The Network Security Agent inherits SOLA's self-evolution capabilities:

### Memory Integration
- **STM/WM**: Per-session scan results and findings
- **LTM**: Historical vulnerability data, attack patterns
- **EPM**: Episodic memories of security assessments
- **RFM**: Relationship context for authorized users

### Skill Evolution
- Learns from successful/failed scans
- Improves detection accuracy over time
- Adapts playbooks based on environment
- Updates vulnerability signatures

### Playbook Evolution
- Tracks playbook effectiveness
- Adjusts step parameters based on results
- Adds new techniques discovered during assessments
- Reports improvements to Phoenix/SOLA

## Security Considerations

### Ethical Use
This agent is designed for **authorized security testing only**. Users must:
1. Have explicit written authorization for all targets
2. Comply with all applicable laws and regulations
3. Follow responsible disclosure practices
4. Document all testing activities

### Safety Features
- All operations require explicit authorization
- Exploit operations are simulated by default
- Audit logging for all security decisions
- Time-limited authorizations
- Target restrictions

### Disclaimer
This tool is provided for educational and authorized security testing purposes only. Unauthorized access to computer systems is illegal. The developers are not responsible for misuse of this software.

## Integration with Phoenix AGI

The Network Security Agent integrates seamlessly with the Phoenix AGI ecosystem:

```
Phoenix/SOLA (Queen)
       ↓
Network Security Agent (Specialized ORCH)
       ↓
   ┌───┴───┐
   ↓       ↓
Scanner  Exploit
   ↓       ↓
Vuln DB  Payloads
   ↓       ↓
MITRE KB Kali Tools
```

### Chat Integration
- Responds to natural language security requests
- Provides real-time scan progress updates
- Generates human-readable reports
- Offers AI-powered analysis and recommendations

### Proactive Capabilities
- Scheduled security scans
- Automatic vulnerability alerts
- Threat intelligence updates
- Compliance monitoring

## Future Enhancements

- [ ] Real-time threat intelligence feeds
- [ ] Cloud security assessment (AWS, Azure, GCP)
- [ ] Container security scanning
- [ ] API security testing
- [ ] Mobile application security
- [ ] IoT device security
- [ ] Compliance frameworks (PCI-DSS, HIPAA, SOC2)
- [ ] Custom exploit development
- [ ] Machine learning-based anomaly detection
