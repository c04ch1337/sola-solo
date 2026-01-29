//! Reporting Agent command handler for phoenix-web

use crate::AppState;
use reporting_agent::{ReportRequest, ReportType};
use serde_json::json;

/// Handle reporting agent commands
///
/// Commands:
/// - report vuln <scan_id> - Generate report for specific vulnerability scan
/// - report last scan - Generate report for last WebGuard/network scan
/// - report file <filename> - Generate report for file submission
/// - report url <url> - Generate report for URL submission
/// - report list - List all stored reports
/// - report get <report_id> - Get specific report by ID
/// - report help - Show help
pub async fn handle_reporting_command(state: &AppState, cmd: &str) -> serde_json::Value {
    let Some(reporting_agent) = &state.reporting_agent else {
        return json!({
            "type": "error",
            "message": "Reporting Agent not available"
        });
    };

    let rest = cmd
        .strip_prefix("report")
        .map(|s| s.trim())
        .unwrap_or("");

    let parts: Vec<&str> = rest.split_whitespace().collect();

    if parts.is_empty() || parts[0] == "help" {
        return json!({
            "type": "report.help",
            "message": "📊 **Reporting Agent - Professional Vulnerability Reporting**\n\n\
                Commands:\n\
                - `report vuln <scan_id>` - Generate report for specific vulnerability scan\n\
                - `report last scan` - Generate report for last WebGuard/network scan\n\
                - `report file <filename>` - Generate report for file submission\n\
                - `report url <url>` - Generate report for URL submission\n\
                - `report list` - List all stored reports\n\
                - `report get <report_id>` - Get specific report by ID\n\
                - `report help` - Show this help\n\n\
                **Features:**\n\
                - Professional markdown-formatted reports\n\
                - Executive summary, findings, PoC, remediation\n\
                - MITRE ATT&CK mapping\n\
                - Risk scoring and severity classification\n\
                - Proactive alerts for high-severity findings\n\n\
                **Examples:**\n\
                ```\n\
                report last scan\n\
                report vuln XSS-001\n\
                report file suspicious.exe\n\
                report list\n\
                ```"
        });
    }

    match parts[0] {
        "list" => {
            let agent = reporting_agent.lock().await;
            let reports = agent.list_reports().await;
            
            if reports.is_empty() {
                return json!({
                    "type": "report.list",
                    "message": "No reports available yet. Generate reports using `report last scan` or other commands.",
                    "reports": []
                });
            }

            let report_summaries: Vec<_> = reports.iter().map(|r| {
                json!({
                    "id": r.id,
                    "title": r.title,
                    "severity": format!("{:?}", r.severity),
                    "risk_score": r.risk_score,
                    "generated_at": r.generated_at.to_rfc3339(),
                    "findings_count": r.findings.len(),
                })
            }).collect();

            let mut message = format!("📊 **Stored Reports ({}):**\n\n", reports.len());
            for r in &reports {
                message.push_str(&format!(
                    "- **{}** `{}` - {} (Risk: {:.1}/10.0) - {} findings\n",
                    r.severity.emoji(),
                    r.id,
                    r.title,
                    r.risk_score,
                    r.findings.len()
                ));
            }

            json!({
                "type": "report.list",
                "message": message,
                "reports": report_summaries
            })
        }

        "get" => {
            if parts.len() < 2 {
                return json!({
                    "type": "error",
                    "message": "Usage: report get <report_id>\nExample: report get abc-123"
                });
            }

            let report_id = parts[1];
            let agent = reporting_agent.lock().await;
            
            match agent.get_report(report_id).await {
                Ok(report) => {
                    // Store in EPM for persistence
                    if let Err(e) = state.vaults.store_soul(
                        &format!("report:{}", report.id),
                        &serde_json::to_string(&report).unwrap_or_default(),
                    ) {
                        tracing::warn!("Failed to store report in EPM: {}", e);
                    }

                    json!({
                        "type": "report.get",
                        "report_id": report.id,
                        "message": report.markdown,
                        "report": report
                    })
                }
                Err(e) => json!({
                    "type": "error",
                    "message": format!("Report not found: {}. Use `report list` to see available reports.", e)
                })
            }
        }

        "last" => {
            if parts.len() < 2 || parts[1] != "scan" {
                return json!({
                    "type": "error",
                    "message": "Usage: report last scan"
                });
            }

            // Try to get last WebGuard scan
            let last_webguard = state.webguard_last_report.lock().await;
            if let Some(ref scan) = *last_webguard {
                let scan_id = scan.id.clone();
                drop(last_webguard);
                
                let request = ReportRequest {
                    report_type: ReportType::WebGuardPassive { scan_id },
                    include_remediation: true,
                    include_mitre: true,
                    include_poc: true,
                };

                let mut agent = reporting_agent.lock().await;
                match agent.generate_report(request).await {
                    Ok(report) => {
                        // Store in EPM
                        if let Err(e) = state.vaults.store_soul(
                            &format!("report:{}", report.id),
                            &serde_json::to_string(&report).unwrap_or_default(),
                        ) {
                            tracing::warn!("Failed to store report in EPM: {}", e);
                        }

                        // Check if proactive alert needed
                        if agent.should_alert(&report) {
                            let alert = agent.generate_alert_summary(&report);
                            tracing::info!("🚨 Proactive Alert: {}", alert);
                            // Could send via proactive_tx here
                        }

                        json!({
                            "type": "report.generated",
                            "report_id": report.id,
                            "message": report.markdown,
                            "report": report
                        })
                    }
                    Err(e) => json!({
                        "type": "error",
                        "message": format!("Failed to generate report: {}", e)
                    })
                }
            } else {
                json!({
                    "type": "error",
                    "message": "No recent scan available. Run `webguard scan <url>` first."
                })
            }
        }

        "vuln" => {
            if parts.len() < 2 {
                return json!({
                    "type": "error",
                    "message": "Usage: report vuln <scan_id>\nExample: report vuln XSS-001"
                });
            }

            let scan_id = parts[1].to_string();
            
            // Determine report type based on scan_id prefix or try to load from memory
            let report_type = if scan_id.contains("xss") || scan_id.contains("XSS") {
                ReportType::WebGuardXss { scan_id: scan_id.clone() }
            } else if scan_id.contains("sqli") || scan_id.contains("SQLi") {
                ReportType::WebGuardSqli { scan_id: scan_id.clone() }
            } else if scan_id.contains("redirect") {
                ReportType::WebGuardRedirect { scan_id: scan_id.clone() }
            } else if scan_id.contains("cmdinj") {
                ReportType::WebGuardCmdInj { scan_id: scan_id.clone() }
            } else {
                ReportType::WebGuardPassive { scan_id: scan_id.clone() }
            };

            let request = ReportRequest {
                report_type,
                include_remediation: true,
                include_mitre: true,
                include_poc: true,
            };

            let mut agent = reporting_agent.lock().await;
            match agent.generate_report(request).await {
                Ok(report) => {
                    // Store in EPM
                    if let Err(e) = state.vaults.store_soul(
                        &format!("report:{}", report.id),
                        &serde_json::to_string(&report).unwrap_or_default(),
                    ) {
                        tracing::warn!("Failed to store report in EPM: {}", e);
                    }

                    // Check if proactive alert needed
                    if agent.should_alert(&report) {
                        let alert = agent.generate_alert_summary(&report);
                        tracing::info!("🚨 Proactive Alert: {}", alert);
                    }

                    json!({
                        "type": "report.generated",
                        "report_id": report.id,
                        "message": report.markdown,
                        "report": report
                    })
                }
                Err(e) => json!({
                    "type": "error",
                    "message": format!("Failed to generate report: {}", e)
                })
            }
        }

        "file" => {
            if parts.len() < 2 {
                return json!({
                    "type": "error",
                    "message": "Usage: report file <filename>\nExample: report file suspicious.exe"
                });
            }

            let filename = parts[1..].join(" ");
            let request = ReportRequest {
                report_type: ReportType::FileSubmission { filename: filename.clone() },
                include_remediation: true,
                include_mitre: false,
                include_poc: false,
            };

            let mut agent = reporting_agent.lock().await;
            match agent.generate_report(request).await {
                Ok(report) => {
                    // Store in EPM
                    if let Err(e) = state.vaults.store_soul(
                        &format!("report:{}", report.id),
                        &serde_json::to_string(&report).unwrap_or_default(),
                    ) {
                        tracing::warn!("Failed to store report in EPM: {}", e);
                    }

                    json!({
                        "type": "report.generated",
                        "report_id": report.id,
                        "message": report.markdown,
                        "report": report
                    })
                }
                Err(e) => json!({
                    "type": "error",
                    "message": format!("Failed to generate report: {}", e)
                })
            }
        }

        "url" => {
            if parts.len() < 2 {
                return json!({
                    "type": "error",
                    "message": "Usage: report url <url>\nExample: report url https://example.com"
                });
            }

            let url = parts[1].to_string();
            let request = ReportRequest {
                report_type: ReportType::UrlSubmission { url: url.clone() },
                include_remediation: true,
                include_mitre: false,
                include_poc: false,
            };

            let mut agent = reporting_agent.lock().await;
            match agent.generate_report(request).await {
                Ok(report) => {
                    // Store in EPM
                    if let Err(e) = state.vaults.store_soul(
                        &format!("report:{}", report.id),
                        &serde_json::to_string(&report).unwrap_or_default(),
                    ) {
                        tracing::warn!("Failed to store report in EPM: {}", e);
                    }

                    json!({
                        "type": "report.generated",
                        "report_id": report.id,
                        "message": report.markdown,
                        "report": report
                    })
                }
                Err(e) => json!({
                    "type": "error",
                    "message": format!("Failed to generate report: {}", e)
                })
            }
        }

        _ => json!({
            "type": "error",
            "message": format!("Unknown report command: {}. Use `report help` for available commands.", parts[0])
        })
    }
}
