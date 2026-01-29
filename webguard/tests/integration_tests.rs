//! Integration tests for WebGuard web vulnerability scanner

use webguard::{
    format_notification_summary, format_report_markdown, CorsAnalysis, Finding, HeaderStatus,
    PassiveScanReport, ScanSummary, SecurityHeadersReport, SensitivePathResult, ServerFingerprint,
    Severity, WebGuard, WebGuardConfig,
    // Phase 28b: XSS Testing
    XssTester, XssTestConfig, XssTestReport, XssSummary, XssFinding, XssPayloadResult, XssType,
    format_xss_report_markdown, format_xss_notification_summary, SAFE_XSS_PAYLOADS,
};

#[tokio::test]
async fn test_webguard_creation_default() {
    let guard = WebGuard::new();
    assert!(guard.is_ok(), "WebGuard should create with default config");
}

#[tokio::test]
async fn test_webguard_creation_custom_config() {
    let config = WebGuardConfig {
        timeout_secs: 10,
        user_agent: "TestAgent/1.0".to_string(),
        check_sensitive_paths: false,
        follow_redirects: false,
        max_redirects: 3,
    };
    let guard = WebGuard::with_config(config);
    assert!(guard.is_ok(), "WebGuard should create with custom config");
}

#[tokio::test]
async fn test_severity_levels() {
    // Test emoji mapping
    assert_eq!(Severity::Critical.emoji(), "🔴");
    assert_eq!(Severity::High.emoji(), "🟠");
    assert_eq!(Severity::Medium.emoji(), "🟡");
    assert_eq!(Severity::Low.emoji(), "🔵");
    assert_eq!(Severity::Info.emoji(), "⚪");

    // Test badge mapping
    assert_eq!(Severity::Critical.badge(), "CRITICAL");
    assert_eq!(Severity::High.badge(), "HIGH");
    assert_eq!(Severity::Medium.badge(), "MEDIUM");
    assert_eq!(Severity::Low.badge(), "LOW");
    assert_eq!(Severity::Info.badge(), "INFO");
}

#[tokio::test]
async fn test_format_report_markdown_basic() {
    let report = create_test_report();
    let markdown = format_report_markdown(&report);

    // Verify markdown contains expected sections
    assert!(markdown.contains("WebGuard Scan Report"));
    assert!(markdown.contains("https://test.example.com"));
    assert!(markdown.contains("Summary"));
    assert!(markdown.contains("Security Headers"));
}

#[tokio::test]
async fn test_format_report_markdown_with_findings() {
    let mut report = create_test_report();
    report.findings.push(Finding {
        id: "TEST-001".to_string(),
        category: "Test Category".to_string(),
        title: "Test Finding".to_string(),
        description: "This is a test finding".to_string(),
        severity: Severity::High,
        evidence: Some("test evidence".to_string()),
        remediation: Some("Fix the issue".to_string()),
    });
    report.summary.total_findings = 1;
    report.summary.high_count = 1;
    report.summary.overall_risk = Severity::High;

    let markdown = format_report_markdown(&report);

    assert!(markdown.contains("Findings"));
    assert!(markdown.contains("TEST-001"));
    assert!(markdown.contains("Test Finding"));
    assert!(markdown.contains("test evidence"));
}

#[tokio::test]
async fn test_format_notification_summary() {
    let mut report = create_test_report();
    report.summary.total_findings = 5;
    report.summary.critical_count = 1;
    report.summary.high_count = 2;

    let summary = format_notification_summary(&report);

    assert!(summary.contains("https://test.example.com"));
    assert!(summary.contains("5 findings"));
    assert!(summary.contains("1 critical"));
    assert!(summary.contains("2 high"));
}

#[tokio::test]
async fn test_scan_summary_calculation() {
    let summary = ScanSummary {
        total_findings: 10,
        critical_count: 1,
        high_count: 2,
        medium_count: 3,
        low_count: 2,
        info_count: 2,
        overall_risk: Severity::Critical,
    };

    assert_eq!(summary.total_findings, 10);
    assert_eq!(summary.overall_risk, Severity::Critical);
}

#[tokio::test]
async fn test_header_status_present() {
    let status = HeaderStatus {
        present: true,
        value: Some("max-age=31536000; includeSubDomains".to_string()),
        severity: Severity::Info,
        issue: None,
    };

    assert!(status.present);
    assert!(status.value.is_some());
    assert!(status.issue.is_none());
}

#[tokio::test]
async fn test_header_status_missing() {
    let status = HeaderStatus {
        present: false,
        value: None,
        severity: Severity::High,
        issue: Some("Header is missing".to_string()),
    };

    assert!(!status.present);
    assert!(status.value.is_none());
    assert!(status.issue.is_some());
}

#[tokio::test]
async fn test_cors_analysis_safe() {
    let cors = CorsAnalysis {
        allow_origin: Some("https://trusted.example.com".to_string()),
        allow_credentials: Some(true),
        allow_methods: Some("GET, POST".to_string()),
        allow_headers: Some("Content-Type".to_string()),
        expose_headers: None,
        max_age: Some("3600".to_string()),
        is_misconfigured: false,
        issues: vec![],
    };

    assert!(!cors.is_misconfigured);
    assert!(cors.issues.is_empty());
}

#[tokio::test]
async fn test_cors_analysis_misconfigured() {
    let cors = CorsAnalysis {
        allow_origin: Some("*".to_string()),
        allow_credentials: Some(true),
        allow_methods: None,
        allow_headers: None,
        expose_headers: None,
        max_age: None,
        is_misconfigured: true,
        issues: vec!["Wildcard origin with credentials".to_string()],
    };

    assert!(cors.is_misconfigured);
    assert!(!cors.issues.is_empty());
}

#[tokio::test]
async fn test_sensitive_path_result() {
    let path = SensitivePathResult {
        path: "/.git/config".to_string(),
        status: 200,
        accessible: true,
        severity: Severity::Critical,
    };

    assert!(path.accessible);
    assert_eq!(path.severity, Severity::Critical);
}

#[tokio::test]
async fn test_server_fingerprint() {
    let fingerprint = ServerFingerprint {
        server: Some("nginx/1.18.0".to_string()),
        x_powered_by: Some("PHP/7.4".to_string()),
        x_aspnet_version: None,
        x_generator: None,
        via: None,
        detected_tech: vec!["Nginx".to_string(), "PHP".to_string()],
    };

    assert!(fingerprint.server.is_some());
    assert!(fingerprint.x_powered_by.is_some());
    assert_eq!(fingerprint.detected_tech.len(), 2);
}

#[tokio::test]
async fn test_finding_structure() {
    let finding = Finding {
        id: "HDR-001".to_string(),
        category: "Security Headers".to_string(),
        title: "Missing CSP".to_string(),
        description: "Content-Security-Policy header is not set".to_string(),
        severity: Severity::Medium,
        evidence: None,
        remediation: Some("Add Content-Security-Policy header".to_string()),
    };

    assert_eq!(finding.id, "HDR-001");
    assert_eq!(finding.severity, Severity::Medium);
    assert!(finding.remediation.is_some());
}

#[tokio::test]
async fn test_report_serialization() {
    let report = create_test_report();
    let json = serde_json::to_string(&report);
    assert!(json.is_ok(), "Report should serialize to JSON");

    let json_str = json.unwrap();
    assert!(json_str.contains("test.example.com"));
}

#[tokio::test]
async fn test_report_deserialization() {
    let report = create_test_report();
    let json = serde_json::to_string(&report).unwrap();
    let deserialized: Result<PassiveScanReport, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok(), "Report should deserialize from JSON");

    let restored = deserialized.unwrap();
    assert_eq!(restored.target_url, report.target_url);
}

// Helper function to create a test report
fn create_test_report() -> PassiveScanReport {
    PassiveScanReport {
        id: "test-scan-123".to_string(),
        target_url: "https://test.example.com".to_string(),
        scan_time: chrono::Utc::now(),
        duration_ms: 150,
        status_code: Some(200),
        security_headers: SecurityHeadersReport {
            csp: Some(HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Medium,
                issue: Some("CSP header missing".to_string()),
            }),
            hsts: Some(HeaderStatus {
                present: true,
                value: Some("max-age=31536000".to_string()),
                severity: Severity::Info,
                issue: None,
            }),
            x_frame_options: Some(HeaderStatus {
                present: true,
                value: Some("DENY".to_string()),
                severity: Severity::Info,
                issue: None,
            }),
            x_content_type_options: Some(HeaderStatus {
                present: true,
                value: Some("nosniff".to_string()),
                severity: Severity::Info,
                issue: None,
            }),
            referrer_policy: Some(HeaderStatus {
                present: false,
                value: None,
                severity: Severity::Low,
                issue: Some("Referrer-Policy missing".to_string()),
            }),
            permissions_policy: None,
            x_xss_protection: None,
        },
        server_fingerprint: ServerFingerprint {
            server: Some("nginx".to_string()),
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
    }
}

// ============================================================================
// Phase 28b: XSS Testing Tests
// ============================================================================

#[tokio::test]
async fn test_xss_tester_creation_default() {
    let tester = XssTester::new();
    assert!(tester.is_ok(), "XssTester should create with default config");
}

#[tokio::test]
async fn test_xss_tester_creation_custom_config() {
    let config = XssTestConfig {
        timeout_secs: 15,
        user_agent: "XSSTestAgent/1.0".to_string(),
        max_payloads: 5,
        check_reflection: true,
        check_execution: true,
        sandbox_mode: true,
    };
    let tester = XssTester::with_config(config);
    assert!(tester.is_ok(), "XssTester should create with custom config");
}

#[tokio::test]
async fn test_xss_type_descriptions() {
    assert!(XssType::Reflected.description().contains("Reflected"));
    assert!(XssType::Stored.description().contains("Stored"));
    assert!(XssType::DomBased.description().contains("DOM"));
}

#[tokio::test]
async fn test_safe_xss_payloads_exist() {
    assert!(!SAFE_XSS_PAYLOADS.is_empty(), "Should have safe XSS payloads");
    assert!(SAFE_XSS_PAYLOADS.len() >= 10, "Should have at least 10 payloads");
    
    // Verify all payloads have descriptions
    for (payload, description) in SAFE_XSS_PAYLOADS {
        assert!(!payload.is_empty(), "Payload should not be empty");
        assert!(!description.is_empty(), "Description should not be empty");
    }
}

#[tokio::test]
async fn test_format_xss_report_markdown_vulnerable() {
    let report = create_vulnerable_xss_report();
    let markdown = format_xss_report_markdown(&report);
    
    // Verify markdown contains expected sections
    assert!(markdown.contains("WebGuard XSS Test Report"), "Should have title");
    assert!(markdown.contains("VULNERABLE TO XSS"), "Should indicate vulnerability");
    assert!(markdown.contains("Payloads Tested"), "Should show payloads tested");
    assert!(markdown.contains("Vulnerabilities Found"), "Should have findings section");
    assert!(markdown.contains("Proof of Concept"), "Should have PoC");
    assert!(markdown.contains("Remediation"), "Should have remediation");
}

#[tokio::test]
async fn test_format_xss_report_markdown_clean() {
    let report = create_clean_xss_report();
    let markdown = format_xss_report_markdown(&report);
    
    // Verify markdown indicates no vulnerabilities
    assert!(markdown.contains("WebGuard XSS Test Report"), "Should have title");
    assert!(markdown.contains("No XSS vulnerabilities detected"), "Should indicate clean");
    assert!(!markdown.contains("VULNERABLE TO XSS"), "Should not indicate vulnerability");
}

#[tokio::test]
async fn test_format_xss_notification_summary_vulnerable() {
    let report = create_vulnerable_xss_report();
    let summary = format_xss_notification_summary(&report);
    
    assert!(summary.contains("XSS VULNERABILITY DETECTED"), "Should indicate vulnerability");
    assert!(summary.contains(&report.target_url), "Should contain target URL");
    assert!(summary.contains(&report.parameter), "Should contain parameter");
}

#[tokio::test]
async fn test_format_xss_notification_summary_clean() {
    let report = create_clean_xss_report();
    let summary = format_xss_notification_summary(&report);
    
    assert!(summary.contains("No vulnerabilities found"), "Should indicate clean");
    assert!(summary.contains(&report.target_url), "Should contain target URL");
}

// Helper function to create a vulnerable XSS test report
fn create_vulnerable_xss_report() -> XssTestReport {
    use chrono::Utc;
    
    XssTestReport {
        id: "xss-test-123".to_string(),
        target_url: "https://example.com/search".to_string(),
        parameter: "q".to_string(),
        scan_time: Utc::now(),
        duration_ms: 500,
        payloads_tested: 17,
        payloads_reflected: 3,
        payloads_executed: 1,
        findings: vec![
            XssFinding {
                id: "XSS-001".to_string(),
                xss_type: XssType::Reflected,
                severity: Severity::Critical,
                parameter: "q".to_string(),
                payload: "<script>alert(1)</script>".to_string(),
                payload_description: "Basic script injection".to_string(),
                proof_of_concept: "https://example.com/search?q=<script>alert(1)</script>".to_string(),
                evidence: "Payload reflected unescaped in response".to_string(),
                remediation: "Implement proper output encoding".to_string(),
            },
        ],
        payload_results: vec![
            XssPayloadResult {
                payload: "<script>alert(1)</script>".to_string(),
                payload_description: "Basic script injection".to_string(),
                reflected: true,
                executed: true,
                context: Some("HTML body context".to_string()),
                response_snippet: Some("...<script>alert(1)</script>...".to_string()),
            },
        ],
        summary: XssSummary {
            vulnerable: true,
            total_findings: 1,
            critical_count: 1,
            high_count: 0,
            overall_risk: Severity::Critical,
        },
    }
}

// Helper function to create a clean XSS test report
fn create_clean_xss_report() -> XssTestReport {
    use chrono::Utc;
    
    XssTestReport {
        id: "xss-test-456".to_string(),
        target_url: "https://secure-example.com/search".to_string(),
        parameter: "q".to_string(),
        scan_time: Utc::now(),
        duration_ms: 450,
        payloads_tested: 17,
        payloads_reflected: 0,
        payloads_executed: 0,
        findings: vec![],
        payload_results: vec![
            XssPayloadResult {
                payload: "<script>alert(1)</script>".to_string(),
                payload_description: "Basic script injection".to_string(),
                reflected: false,
                executed: false,
                context: None,
                response_snippet: None,
            },
        ],
        summary: XssSummary {
            vulnerable: false,
            total_findings: 0,
            critical_count: 0,
            high_count: 0,
            overall_risk: Severity::Info,
        },
    }
}
