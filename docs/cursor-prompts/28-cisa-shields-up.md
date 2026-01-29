# 28 - CISA Shields Up Integration

Use this prompt to integrate CISA's Shields Up 2.0 framework into the CISA-Agent. This framework focuses on hardening against nation-state threats through patching, MFA, and network monitoring.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• Proactive agents exist for anomaly detection
• KEV catalog integration exists
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA Shields Up 2.0 into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa shields-up scan" → assess hardening against nation-state threats
- Map to Sola features:
  1. Patch Management → KEV catalog integration for critical patches
  2. MFA Enforcement → Verify MFA on all privileged accounts
  3. Network Monitoring → Proactive agent for anomalous activity detection
  4. Incident Response → Pre-staged playbook for rapid response
  5. Backup Verification → Memory vault backup status check
  6. Threat Simulation → Sandbox for threat analysis
  7. Access Control → Tiered access verification
  8. Logging & Detection → Audit log completeness check
- Sandbox threat simulation for known nation-state TTPs
- Proactive alerts on new threat intelligence
- Integration with MITRE ATT&CK for TTP mapping

First:
1. Duplication check (search for shields-up in cisa_agent.rs, proactive.rs)
2. If clean → generate:
   - cisa_agent.rs diff (shields-up assessment + threat simulation)
   - New shields_up_assessor.rs (8 hardening checks + TTP mapping)
   - Sandbox integration for threat simulation
3. Integration: "cisa shields-up scan" → full hardening report
4. Tests:
   - Chat: "cisa shields-up scan" → report with threat posture
   - Verify sandbox threat simulation works

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## Shields Up Hardening Checks Mapping

| Hardening Area | Sola Feature | Assessment Check |
|----------------|--------------|------------------|
| 1. Patch Management | KEV scanner | Check for unpatched KEV vulns |
| 2. MFA Enforcement | Auth analysis | Verify MFA on privileged accounts |
| 3. Network Monitoring | Proactive agents | Check monitoring coverage |
| 4. Incident Response | Playbook system | Verify playbook readiness |
| 5. Backup Verification | Memory vaults | Check backup freshness |
| 6. Threat Simulation | Sandbox | Run TTP simulations |
| 7. Access Control | Tiered access | Verify least privilege |
| 8. Logging & Detection | Audit system | Check log completeness |

## Nation-State TTP Categories (MITRE ATT&CK)

The Shields Up assessment maps to these MITRE ATT&CK categories:

- **Initial Access**: Phishing, exploit public-facing apps
- **Execution**: Command/scripting interpreter abuse
- **Persistence**: Account manipulation, scheduled tasks
- **Privilege Escalation**: Valid accounts, exploitation
- **Defense Evasion**: Indicator removal, masquerading
- **Credential Access**: Brute force, credential dumping
- **Discovery**: Network/system information discovery
- **Lateral Movement**: Remote services, internal spearphishing
- **Collection**: Data from local system, email collection
- **Exfiltration**: Exfiltration over C2, web service

## Expected Output Format

```
🛡️ CISA Shields Up 2.0 Assessment
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Threat Posture: ELEVATED RISK
Overall Score: 72/100

Hardening Status:
✅ Patch Management: PASS (0 KEV vulns, last scan: 2h ago)
✅ MFA Enforcement: PASS (All privileged accounts MFA-enabled)
⚠️ Network Monitoring: PARTIAL (85% coverage, gaps in subnet 10.0.3.x)
✅ Incident Response: PASS (Playbook tested 7 days ago)
❌ Backup Verification: FAIL (Last backup: 14 days ago)
✅ Threat Simulation: PASS (Last TTP test: 3 days ago)
⚠️ Access Control: PARTIAL (2 over-privileged accounts detected)
✅ Logging & Detection: PASS (All critical events logged)

🎯 Threat Simulation Results:
- APT29 (Cozy Bear) TTPs: 8/10 blocked
- APT28 (Fancy Bear) TTPs: 9/10 blocked
- Lazarus Group TTPs: 7/10 blocked

⚠️ Active Threat Intelligence:
- New KEV: CVE-2024-1234 (Critical) - Patch available
- CISA Alert AA24-015A: Active exploitation of VPN vulns

🔧 Recommendations:
1. URGENT: Run backup immediately (14 days overdue)
2. Extend network monitoring to subnet 10.0.3.x
3. Review privileges for: admin_backup, svc_deploy
4. Apply patch for CVE-2024-1234
```

## Proactive Alerts

The Shields Up integration enables proactive alerts for:

- New KEV vulnerabilities affecting your stack
- CISA alerts for active exploitation campaigns
- Anomalous network activity matching nation-state TTPs
- Backup age exceeding threshold (default: 7 days)
- Failed threat simulation tests
