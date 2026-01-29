//! Report templates for different vulnerability types

/// Get a professional report template for a given vulnerability type
pub fn get_report_template(vuln_type: &str) -> ReportTemplate {
    match vuln_type.to_lowercase().as_str() {
        "xss" => ReportTemplate {
            title: "Cross-Site Scripting (XSS) Vulnerability".to_string(),
            summary_template: "A Cross-Site Scripting vulnerability was identified that allows attackers to inject malicious scripts into web pages viewed by other users.".to_string(),
            impact: "Attackers can steal session cookies, perform actions on behalf of users, deface websites, or redirect users to malicious sites.".to_string(),
            recommendation: "Implement proper input validation and output encoding. Use Content Security Policy (CSP) headers.".to_string(),
        },
        "sqli" => ReportTemplate {
            title: "SQL Injection Vulnerability".to_string(),
            summary_template: "A SQL Injection vulnerability was discovered that allows attackers to manipulate database queries.".to_string(),
            impact: "Attackers can read sensitive data, modify database contents, execute administrative operations, or in some cases issue commands to the operating system.".to_string(),
            recommendation: "Use parameterized queries or prepared statements. Implement proper input validation and least privilege database access.".to_string(),
        },
        "cmdinj" => ReportTemplate {
            title: "Command Injection Vulnerability".to_string(),
            summary_template: "A Command Injection vulnerability was found that allows attackers to execute arbitrary system commands.".to_string(),
            impact: "Attackers can execute arbitrary commands on the host operating system, potentially leading to full system compromise.".to_string(),
            recommendation: "Avoid using system calls with user input. If necessary, use strict input validation and sandboxing.".to_string(),
        },
        "redirect" => ReportTemplate {
            title: "Open Redirect Vulnerability".to_string(),
            summary_template: "An Open Redirect vulnerability was identified that allows attackers to redirect users to arbitrary URLs.".to_string(),
            impact: "Attackers can use this for phishing attacks by redirecting users to malicious sites that appear to come from a trusted source.".to_string(),
            recommendation: "Validate and whitelist redirect destinations. Avoid using user-controlled input for redirects.".to_string(),
        },
        _ => ReportTemplate {
            title: "Security Vulnerability".to_string(),
            summary_template: "A security vulnerability was identified.".to_string(),
            impact: "This vulnerability may allow attackers to compromise system security.".to_string(),
            recommendation: "Review and remediate the identified vulnerability according to security best practices.".to_string(),
        },
    }
}

/// Report template structure
pub struct ReportTemplate {
    pub title: String,
    pub summary_template: String,
    pub impact: String,
    pub recommendation: String,
}
