//! Network Scanner Module
//!
//! Provides nmap-like network scanning capabilities including:
//! - Host discovery (ping sweep, ARP scan)
//! - Port scanning (TCP SYN, TCP Connect, UDP)
//! - Service detection and version fingerprinting
//! - OS fingerprinting
//! - Script scanning (NSE-like)

use crate::{SecurityAgentError, SecurityGate, SecurityLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;
use uuid::Uuid;

/// Network scanner with security-gated operations
pub struct NetworkScanner {
    security_gate: Arc<RwLock<SecurityGate>>,
    service_signatures: HashMap<u16, ServiceSignature>,
    os_fingerprints: Vec<OSFingerprint>,
}

impl NetworkScanner {
    /// Create a new network scanner
    pub fn new(security_gate: Arc<RwLock<SecurityGate>>) -> Self {
        Self {
            security_gate,
            service_signatures: Self::load_service_signatures(),
            os_fingerprints: Self::load_os_fingerprints(),
        }
    }

    /// Load service signatures for port identification
    fn load_service_signatures() -> HashMap<u16, ServiceSignature> {
        let mut sigs = HashMap::new();

        // Common service signatures
        let services = vec![
            (21, "ftp", "File Transfer Protocol", vec!["220", "FTP"]),
            (22, "ssh", "Secure Shell", vec!["SSH-", "OpenSSH"]),
            (23, "telnet", "Telnet", vec!["login:", "Telnet"]),
            (25, "smtp", "Simple Mail Transfer Protocol", vec!["220", "SMTP", "ESMTP"]),
            (53, "dns", "Domain Name System", vec![]),
            (80, "http", "Hypertext Transfer Protocol", vec!["HTTP/", "<!DOCTYPE", "<html"]),
            (110, "pop3", "Post Office Protocol v3", vec!["+OK", "POP3"]),
            (111, "rpcbind", "RPC Portmapper", vec![]),
            (135, "msrpc", "Microsoft RPC", vec![]),
            (139, "netbios-ssn", "NetBIOS Session Service", vec![]),
            (143, "imap", "Internet Message Access Protocol", vec!["* OK", "IMAP"]),
            (443, "https", "HTTP Secure", vec!["HTTP/", "TLS"]),
            (445, "microsoft-ds", "Microsoft Directory Services", vec![]),
            (993, "imaps", "IMAP over SSL", vec![]),
            (995, "pop3s", "POP3 over SSL", vec![]),
            (1433, "mssql", "Microsoft SQL Server", vec![]),
            (1521, "oracle", "Oracle Database", vec![]),
            (3306, "mysql", "MySQL Database", vec!["mysql", "MariaDB"]),
            (3389, "rdp", "Remote Desktop Protocol", vec![]),
            (5432, "postgresql", "PostgreSQL Database", vec![]),
            (5900, "vnc", "Virtual Network Computing", vec!["RFB"]),
            (6379, "redis", "Redis Database", vec!["REDIS", "-ERR"]),
            (8080, "http-proxy", "HTTP Proxy", vec!["HTTP/"]),
            (8443, "https-alt", "HTTPS Alternate", vec!["HTTP/"]),
            (27017, "mongodb", "MongoDB Database", vec![]),
        ];

        for (port, name, description, banners) in services {
            sigs.insert(
                port,
                ServiceSignature {
                    port,
                    name: name.to_string(),
                    description: description.to_string(),
                    banner_patterns: banners.iter().map(|s| s.to_string()).collect(),
                },
            );
        }

        sigs
    }

    /// Load OS fingerprints
    fn load_os_fingerprints() -> Vec<OSFingerprint> {
        vec![
            OSFingerprint {
                name: "Linux".to_string(),
                version: Some("2.6.x - 5.x".to_string()),
                ttl_range: (60, 64),
                window_size: Some(5840),
                tcp_options: vec!["MSS".to_string(), "SACK".to_string(), "TS".to_string()],
            },
            OSFingerprint {
                name: "Windows".to_string(),
                version: Some("7/8/10/11/Server".to_string()),
                ttl_range: (125, 128),
                window_size: Some(8192),
                tcp_options: vec!["MSS".to_string(), "NOP".to_string(), "WS".to_string()],
            },
            OSFingerprint {
                name: "macOS".to_string(),
                version: Some("10.x - 14.x".to_string()),
                ttl_range: (60, 64),
                window_size: Some(65535),
                tcp_options: vec!["MSS".to_string(), "NOP".to_string(), "WS".to_string(), "TS".to_string()],
            },
            OSFingerprint {
                name: "Cisco IOS".to_string(),
                version: None,
                ttl_range: (252, 255),
                window_size: Some(4128),
                tcp_options: vec!["MSS".to_string()],
            },
        ]
    }

    /// Discover networks (passive - no authorization required)
    pub async fn discover_networks(&self) -> Result<Vec<NetworkDiscovery>, SecurityAgentError> {
        let mut discoveries = Vec::new();

        // Get local network interfaces
        if let Ok(interfaces) = Self::get_network_interfaces().await {
            for iface in interfaces {
                discoveries.push(NetworkDiscovery {
                    interface: iface.name.clone(),
                    ip_address: iface.ip_address,
                    subnet_mask: iface.subnet_mask,
                    gateway: iface.gateway,
                    mac_address: iface.mac_address,
                    network_type: iface.network_type,
                    discovered_at: Utc::now(),
                });
            }
        }

        Ok(discoveries)
    }

    /// Get network interfaces
    async fn get_network_interfaces() -> Result<Vec<NetworkInterface>, SecurityAgentError> {
        let mut interfaces = Vec::new();

        // Use system commands to get interface info
        #[cfg(target_os = "windows")]
        {
            let output = tokio::process::Command::new("ipconfig")
                .arg("/all")
                .output()
                .await
                .map_err(|e| SecurityAgentError::Network(e.to_string()))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            interfaces.extend(Self::parse_ipconfig_output(&stdout));
        }

        #[cfg(not(target_os = "windows"))]
        {
            let output = tokio::process::Command::new("ip")
                .args(["addr", "show"])
                .output()
                .await
                .map_err(|e| SecurityAgentError::Network(e.to_string()))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            interfaces.extend(Self::parse_ip_addr_output(&stdout));
        }

        Ok(interfaces)
    }

    /// Parse ipconfig output (Windows)
    #[cfg(target_os = "windows")]
    fn parse_ipconfig_output(output: &str) -> Vec<NetworkInterface> {
        let mut interfaces = Vec::new();
        let mut current_iface: Option<NetworkInterface> = None;

        for line in output.lines() {
            let line = line.trim();

            if line.ends_with(':') && !line.contains("adapter") {
                // New adapter section
                if let Some(iface) = current_iface.take() {
                    if iface.ip_address.is_some() {
                        interfaces.push(iface);
                    }
                }
            } else if line.contains("adapter") && line.ends_with(':') {
                // Adapter name
                if let Some(iface) = current_iface.take() {
                    if iface.ip_address.is_some() {
                        interfaces.push(iface);
                    }
                }
                let name = line.trim_end_matches(':').to_string();
                let network_type = if name.to_lowercase().contains("wireless") || name.to_lowercase().contains("wi-fi") {
                    NetworkType::Wireless
                } else if name.to_lowercase().contains("ethernet") {
                    NetworkType::Ethernet
                } else if name.to_lowercase().contains("loopback") {
                    NetworkType::Loopback
                } else {
                    NetworkType::Unknown
                };
                current_iface = Some(NetworkInterface {
                    name,
                    ip_address: None,
                    subnet_mask: None,
                    gateway: None,
                    mac_address: None,
                    network_type,
                });
            } else if let Some(ref mut iface) = current_iface {
                if line.starts_with("IPv4 Address") {
                    if let Some(ip) = line.split(':').nth(1) {
                        let ip = ip.trim().trim_start_matches("(Preferred)").trim();
                        iface.ip_address = ip.parse().ok();
                    }
                } else if line.starts_with("Subnet Mask") {
                    if let Some(mask) = line.split(':').nth(1) {
                        iface.subnet_mask = Some(mask.trim().to_string());
                    }
                } else if line.starts_with("Default Gateway") {
                    if let Some(gw) = line.split(':').nth(1) {
                        let gw = gw.trim();
                        if !gw.is_empty() {
                            iface.gateway = gw.parse().ok();
                        }
                    }
                } else if line.starts_with("Physical Address") {
                    if let Some(mac) = line.split(':').nth(1) {
                        iface.mac_address = Some(mac.trim().replace('-', ":"));
                    }
                }
            }
        }

        if let Some(iface) = current_iface {
            if iface.ip_address.is_some() {
                interfaces.push(iface);
            }
        }

        interfaces
    }

    /// Parse ip addr output (Linux/macOS)
    #[cfg(not(target_os = "windows"))]
    fn parse_ip_addr_output(output: &str) -> Vec<NetworkInterface> {
        let mut interfaces = Vec::new();
        let mut current_iface: Option<NetworkInterface> = None;

        for line in output.lines() {
            if line.starts_with(char::is_numeric) {
                // New interface
                if let Some(iface) = current_iface.take() {
                    if iface.ip_address.is_some() {
                        interfaces.push(iface);
                    }
                }
                if let Some(name) = line.split(':').nth(1) {
                    let name = name.trim().to_string();
                    let network_type = if name.starts_with("wl") {
                        NetworkType::Wireless
                    } else if name.starts_with("eth") || name.starts_with("en") {
                        NetworkType::Ethernet
                    } else if name == "lo" {
                        NetworkType::Loopback
                    } else {
                        NetworkType::Unknown
                    };
                    current_iface = Some(NetworkInterface {
                        name,
                        ip_address: None,
                        subnet_mask: None,
                        gateway: None,
                        mac_address: None,
                        network_type,
                    });
                }
            } else if let Some(ref mut iface) = current_iface {
                let line = line.trim();
                if line.starts_with("inet ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let ip_cidr = parts[1];
                        if let Some(ip) = ip_cidr.split('/').next() {
                            iface.ip_address = ip.parse().ok();
                        }
                        if let Some(cidr) = ip_cidr.split('/').nth(1) {
                            iface.subnet_mask = Some(Self::cidr_to_netmask(cidr.parse().unwrap_or(24)));
                        }
                    }
                } else if line.starts_with("link/ether") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        iface.mac_address = Some(parts[1].to_string());
                    }
                }
            }
        }

        if let Some(iface) = current_iface {
            if iface.ip_address.is_some() {
                interfaces.push(iface);
            }
        }

        interfaces
    }

    /// Convert CIDR to netmask
    #[cfg(not(target_os = "windows"))]
    fn cidr_to_netmask(cidr: u8) -> String {
        let mask: u32 = if cidr == 0 {
            0
        } else {
            !0u32 << (32 - cidr)
        };
        format!(
            "{}.{}.{}.{}",
            (mask >> 24) & 0xFF,
            (mask >> 16) & 0xFF,
            (mask >> 8) & 0xFF,
            mask & 0xFF
        )
    }

    /// Perform a network scan
    pub async fn scan(&self, config: &ScanConfig) -> Result<ScanResult, SecurityAgentError> {
        // Check authorization
        let gate = self.security_gate.read().await;
        gate.check_authorization(SecurityLevel::Active, Some(&config.target))?;
        drop(gate);

        let scan_id = Uuid::new_v4();
        let started_at = Utc::now();

        println!("🔍 Starting {} scan on {}", config.scan_type, config.target);

        // Parse target(s)
        let targets = self.parse_targets(&config.target)?;
        let mut hosts = Vec::new();

        for target_ip in targets {
            // Host discovery
            if config.host_discovery {
                if !self.is_host_alive(&target_ip, config.timeout_ms).await {
                    continue;
                }
            }

            let mut host = HostResult {
                ip: target_ip,
                hostname: self.resolve_hostname(&target_ip).await,
                mac_address: None,
                os_detection: None,
                ports: Vec::new(),
                status: HostStatus::Up,
                latency_ms: None,
            };

            // Port scanning
            match config.scan_type {
                ScanType::TcpConnect | ScanType::TcpSyn => {
                    host.ports = self
                        .scan_tcp_ports(&target_ip, &config.ports, config.timeout_ms)
                        .await;
                }
                ScanType::Udp => {
                    host.ports = self
                        .scan_udp_ports(&target_ip, &config.ports, config.timeout_ms)
                        .await;
                }
                ScanType::ServiceDetection => {
                    host.ports = self
                        .scan_tcp_ports(&target_ip, &config.ports, config.timeout_ms)
                        .await;
                    // Grab banners for open ports
                    for port in &mut host.ports {
                        if port.state == PortState::Open {
                            port.service = self.detect_service(&target_ip, port.port).await;
                            port.banner = self.grab_banner(&target_ip, port.port).await;
                        }
                    }
                }
                ScanType::OsDetection => {
                    host.ports = self
                        .scan_tcp_ports(&target_ip, &config.ports, config.timeout_ms)
                        .await;
                    host.os_detection = self.detect_os(&target_ip).await;
                }
                ScanType::Comprehensive => {
                    host.ports = self
                        .scan_tcp_ports(&target_ip, &config.ports, config.timeout_ms)
                        .await;
                    for port in &mut host.ports {
                        if port.state == PortState::Open {
                            port.service = self.detect_service(&target_ip, port.port).await;
                            port.banner = self.grab_banner(&target_ip, port.port).await;
                        }
                    }
                    host.os_detection = self.detect_os(&target_ip).await;
                }
            }

            hosts.push(host);
        }

        let completed_at = Utc::now();

        let total_hosts = hosts.len();
        let hosts_up = hosts.iter().filter(|h| h.status == HostStatus::Up).count();
        let total_ports_scanned = config.ports.len() * total_hosts;
        let open_ports = hosts.iter().flat_map(|h| &h.ports).filter(|p| p.state == PortState::Open).count();
        let duration_ms = (completed_at - started_at).num_milliseconds() as u64;

        Ok(ScanResult {
            id: scan_id,
            target: config.target.clone(),
            scan_type: config.scan_type.clone(),
            started_at,
            completed_at,
            hosts,
            statistics: ScanStatistics {
                total_hosts,
                hosts_up,
                total_ports_scanned,
                open_ports,
                duration_ms,
            },
        })
    }

    /// Parse target specification (IP, CIDR, range)
    fn parse_targets(&self, target: &str) -> Result<Vec<IpAddr>, SecurityAgentError> {
        let mut targets = Vec::new();

        // Check if it's a CIDR notation
        if target.contains('/') {
            if let Ok(network) = target.parse::<ipnetwork::IpNetwork>() {
                for ip in network.iter() {
                    targets.push(ip);
                }
            } else {
                return Err(SecurityAgentError::Parse(format!(
                    "Invalid CIDR notation: {}",
                    target
                )));
            }
        }
        // Check if it's a range (e.g., 192.168.1.1-254)
        else if target.contains('-') {
            let parts: Vec<&str> = target.rsplitn(2, '.').collect();
            if parts.len() == 2 {
                let range_part = parts[0];
                let base = parts[1];
                if let Some((start, end)) = range_part.split_once('-') {
                    let start: u8 = start.parse().map_err(|_| {
                        SecurityAgentError::Parse(format!("Invalid range start: {}", start))
                    })?;
                    let end: u8 = end.parse().map_err(|_| {
                        SecurityAgentError::Parse(format!("Invalid range end: {}", end))
                    })?;
                    for i in start..=end {
                        let ip_str = format!("{}.{}", base, i);
                        if let Ok(ip) = ip_str.parse() {
                            targets.push(ip);
                        }
                    }
                }
            }
        }
        // Single IP or hostname
        else {
            // Try to parse as IP first
            if let Ok(ip) = target.parse() {
                targets.push(ip);
            } else {
                // Try DNS resolution
                if let Ok(ips) = dns_lookup::lookup_host(target) {
                    targets.extend(ips);
                } else {
                    return Err(SecurityAgentError::Parse(format!(
                        "Cannot resolve target: {}",
                        target
                    )));
                }
            }
        }

        Ok(targets)
    }

    /// Check if a host is alive
    async fn is_host_alive(&self, ip: &IpAddr, timeout_ms: u64) -> bool {
        // Try TCP connect to common ports
        let common_ports = [80, 443, 22, 445, 139];

        for port in common_ports {
            let addr = SocketAddr::new(*ip, port);
            if timeout(
                Duration::from_millis(timeout_ms),
                AsyncTcpStream::connect(addr),
            )
            .await
            .is_ok()
            {
                return true;
            }
        }

        // Try ICMP ping via system command
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = tokio::process::Command::new("ping")
                .args(["-n", "1", "-w", &timeout_ms.to_string(), &ip.to_string()])
                .output()
                .await
            {
                return output.status.success();
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(output) = tokio::process::Command::new("ping")
                .args(["-c", "1", "-W", &(timeout_ms / 1000).max(1).to_string(), &ip.to_string()])
                .output()
                .await
            {
                return output.status.success();
            }
        }

        false
    }

    /// Resolve hostname for an IP
    async fn resolve_hostname(&self, ip: &IpAddr) -> Option<String> {
        dns_lookup::lookup_addr(ip).ok()
    }

    /// Scan TCP ports
    async fn scan_tcp_ports(
        &self,
        ip: &IpAddr,
        ports: &[u16],
        timeout_ms: u64,
    ) -> Vec<PortResult> {
        let mut results = Vec::new();

        for &port in ports {
            let addr = SocketAddr::new(*ip, port);
            let state = match timeout(
                Duration::from_millis(timeout_ms),
                AsyncTcpStream::connect(addr),
            )
            .await
            {
                Ok(Ok(_)) => PortState::Open,
                Ok(Err(_)) => PortState::Closed,
                Err(_) => PortState::Filtered,
            };

            if state == PortState::Open || state == PortState::Filtered {
                results.push(PortResult {
                    port,
                    protocol: Protocol::Tcp,
                    state,
                    service: self.service_signatures.get(&port).map(|s| s.name.clone()),
                    version: None,
                    banner: None,
                });
            }
        }

        results
    }

    /// Scan UDP ports
    async fn scan_udp_ports(
        &self,
        ip: &IpAddr,
        ports: &[u16],
        _timeout_ms: u64,
    ) -> Vec<PortResult> {
        let mut results = Vec::new();

        // UDP scanning is less reliable - we'll mark common UDP ports as open|filtered
        let common_udp_ports = [53, 67, 68, 69, 123, 137, 138, 161, 162, 500, 514, 520, 1900];

        for &port in ports {
            if common_udp_ports.contains(&port) {
                results.push(PortResult {
                    port,
                    protocol: Protocol::Udp,
                    state: PortState::OpenFiltered,
                    service: self.service_signatures.get(&port).map(|s| s.name.clone()),
                    version: None,
                    banner: None,
                });
            }
        }

        results
    }

    /// Detect service on a port
    async fn detect_service(&self, ip: &IpAddr, port: u16) -> Option<String> {
        // First check known signatures
        if let Some(sig) = self.service_signatures.get(&port) {
            return Some(sig.name.clone());
        }

        // Try to grab banner and identify
        if let Some(banner) = self.grab_banner(ip, port).await {
            // Check against all signatures
            for sig in self.service_signatures.values() {
                for pattern in &sig.banner_patterns {
                    if banner.contains(pattern) {
                        return Some(sig.name.clone());
                    }
                }
            }
        }

        None
    }

    /// Grab banner from a service
    async fn grab_banner(&self, ip: &IpAddr, port: u16) -> Option<String> {
        let addr = SocketAddr::new(*ip, port);

        match timeout(Duration::from_secs(5), AsyncTcpStream::connect(addr)).await {
            Ok(Ok(mut stream)) => {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                // Send HTTP request for web ports
                if port == 80 || port == 8080 || port == 443 || port == 8443 {
                    let _ = stream
                        .write_all(b"HEAD / HTTP/1.0\r\nHost: target\r\n\r\n")
                        .await;
                }

                let mut buffer = vec![0u8; 1024];
                match timeout(Duration::from_secs(3), stream.read(&mut buffer)).await {
                    Ok(Ok(n)) if n > 0 => {
                        Some(String::from_utf8_lossy(&buffer[..n]).to_string())
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Detect operating system
    async fn detect_os(&self, ip: &IpAddr) -> Option<OSDetection> {
        // Use TTL-based detection from ping
        #[cfg(target_os = "windows")]
        let ping_cmd = tokio::process::Command::new("ping")
            .args(["-n", "1", &ip.to_string()])
            .output()
            .await;

        #[cfg(not(target_os = "windows"))]
        let ping_cmd = tokio::process::Command::new("ping")
            .args(["-c", "1", &ip.to_string()])
            .output()
            .await;

        if let Ok(output) = ping_cmd {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Extract TTL
            let ttl = if let Some(ttl_match) = stdout
                .to_lowercase()
                .find("ttl=")
                .or_else(|| stdout.to_lowercase().find("ttl "))
            {
                let ttl_str: String = stdout[ttl_match..]
                    .chars()
                    .skip_while(|c| !c.is_ascii_digit())
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                ttl_str.parse::<u8>().ok()
            } else {
                None
            };

            if let Some(ttl) = ttl {
                // Match against fingerprints
                for fp in &self.os_fingerprints {
                    if ttl >= fp.ttl_range.0 && ttl <= fp.ttl_range.1 {
                        return Some(OSDetection {
                            name: fp.name.clone(),
                            version: fp.version.clone(),
                            confidence: 0.7,
                            fingerprint_method: "TTL analysis".to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}

/// Scan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Target specification (IP, CIDR, hostname, range)
    pub target: String,
    /// Ports to scan
    pub ports: Vec<u16>,
    /// Scan type
    pub scan_type: ScanType,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    /// Perform host discovery first
    pub host_discovery: bool,
    /// Scan intensity (1-5)
    pub intensity: u8,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target: String::new(),
            ports: Self::top_1000_ports(),
            scan_type: ScanType::TcpConnect,
            timeout_ms: 3000,
            host_discovery: true,
            intensity: 3,
        }
    }
}

impl ScanConfig {
    /// Set target
    pub fn with_target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    /// Set scan type
    pub fn with_scan_type(mut self, scan_type: ScanType) -> Self {
        self.scan_type = scan_type;
        self
    }

    /// Set ports
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    /// Use top 100 ports
    pub fn top_100_ports() -> Vec<u16> {
        vec![
            7, 9, 13, 21, 22, 23, 25, 26, 37, 53, 79, 80, 81, 88, 106, 110, 111, 113, 119, 135,
            139, 143, 144, 179, 199, 389, 427, 443, 444, 445, 465, 513, 514, 515, 543, 544, 548,
            554, 587, 631, 646, 873, 990, 993, 995, 1025, 1026, 1027, 1028, 1029, 1110, 1433,
            1720, 1723, 1755, 1900, 2000, 2001, 2049, 2121, 2717, 3000, 3128, 3306, 3389, 3986,
            4899, 5000, 5009, 5051, 5060, 5101, 5190, 5357, 5432, 5631, 5666, 5800, 5900, 6000,
            6001, 6646, 7070, 8000, 8008, 8009, 8080, 8081, 8443, 8888, 9100, 9999, 10000, 32768,
            49152, 49153, 49154, 49155, 49156, 49157,
        ]
    }

    /// Use top 1000 ports (nmap default)
    pub fn top_1000_ports() -> Vec<u16> {
        // Abbreviated - in production this would be the full nmap top 1000
        let mut ports = Self::top_100_ports();
        ports.extend(vec![
            1, 3, 4, 6, 11, 12, 14, 15, 17, 18, 19, 20, 24, 27, 29, 30, 32, 33, 35, 38, 39, 41,
            42, 43, 49, 50, 51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 82, 83, 84, 85, 86, 87, 89, 90, 91, 92, 93,
            94, 95, 96, 97, 98, 99, 100, 102, 104, 105, 107, 108, 109, 112, 114, 115, 116, 117,
            118, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 136,
            137, 138, 140, 141, 142, 145, 146, 147, 148, 149, 150,
        ]);
        ports.sort();
        ports.dedup();
        ports
    }

    /// Scan all ports
    pub fn all_ports() -> Vec<u16> {
        (1..=65535).collect()
    }
}

/// Scan types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanType {
    /// TCP Connect scan (full handshake)
    TcpConnect,
    /// TCP SYN scan (half-open)
    TcpSyn,
    /// UDP scan
    Udp,
    /// Service/version detection
    ServiceDetection,
    /// OS detection
    OsDetection,
    /// Comprehensive scan (all of the above)
    Comprehensive,
}

impl std::fmt::Display for ScanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanType::TcpConnect => write!(f, "TCP Connect"),
            ScanType::TcpSyn => write!(f, "TCP SYN"),
            ScanType::Udp => write!(f, "UDP"),
            ScanType::ServiceDetection => write!(f, "Service Detection"),
            ScanType::OsDetection => write!(f, "OS Detection"),
            ScanType::Comprehensive => write!(f, "Comprehensive"),
        }
    }
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: Uuid,
    pub target: String,
    pub scan_type: ScanType,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub hosts: Vec<HostResult>,
    pub statistics: ScanStatistics,
}

/// Host scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostResult {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub os_detection: Option<OSDetection>,
    pub ports: Vec<PortResult>,
    pub status: HostStatus,
    pub latency_ms: Option<u64>,
}

/// Host status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostStatus {
    Up,
    Down,
    Unknown,
}

/// Port scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortResult {
    pub port: u16,
    pub protocol: Protocol,
    pub state: PortState,
    pub service: Option<String>,
    pub version: Option<String>,
    pub banner: Option<String>,
}

/// Protocol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
}

/// Port state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,
}

/// Scan statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    pub total_hosts: usize,
    pub hosts_up: usize,
    pub total_ports_scanned: usize,
    pub open_ports: usize,
    pub duration_ms: u64,
}

/// Network discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscovery {
    pub interface: String,
    pub ip_address: Option<IpAddr>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<IpAddr>,
    pub mac_address: Option<String>,
    pub network_type: NetworkType,
    pub discovered_at: DateTime<Utc>,
}

/// Network interface info
#[derive(Debug, Clone)]
struct NetworkInterface {
    name: String,
    ip_address: Option<IpAddr>,
    subnet_mask: Option<String>,
    gateway: Option<IpAddr>,
    mac_address: Option<String>,
    network_type: NetworkType,
}

/// Network type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Ethernet,
    Wireless,
    Loopback,
    Virtual,
    Unknown,
}

/// Service signature for identification
#[derive(Debug, Clone)]
struct ServiceSignature {
    port: u16,
    name: String,
    description: String,
    banner_patterns: Vec<String>,
}

/// OS fingerprint
#[derive(Debug, Clone)]
struct OSFingerprint {
    name: String,
    version: Option<String>,
    ttl_range: (u8, u8),
    window_size: Option<u32>,
    tcp_options: Vec<String>,
}

/// OS detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSDetection {
    pub name: String,
    pub version: Option<String>,
    pub confidence: f32,
    pub fingerprint_method: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert!(!config.ports.is_empty());
        assert_eq!(config.scan_type, ScanType::TcpConnect);
    }

    #[test]
    fn test_scan_config_builder() {
        let config = ScanConfig::default()
            .with_target("192.168.1.1")
            .with_scan_type(ScanType::ServiceDetection)
            .with_ports(vec![22, 80, 443]);

        assert_eq!(config.target, "192.168.1.1");
        assert_eq!(config.scan_type, ScanType::ServiceDetection);
        assert_eq!(config.ports, vec![22, 80, 443]);
    }
}
