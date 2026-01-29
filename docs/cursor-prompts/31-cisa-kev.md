# 31 - CISA Known Exploited Vulnerabilities (KEV) Integration

Use this prompt to integrate CISA's Known Exploited Vulnerabilities (KEV) catalog into the CISA-Agent. This framework provides real-time vulnerability alerts for actively exploited vulnerabilities.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• Proactive agents exist for alerting
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA Known Exploited Vulnerabilities (KEV) catalog into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa kev scan" → scan system for KEV vulnerabilities
- Map to Sola features:
  1. KEV Catalog Sync → Sub-agent pulls KEV catalog daily from CISA API
  2. Vulnerability Scanning → Match installed software against KEV entries
  3. Severity Reporting → CVSS scores + CISA due dates
  4. Patch Prioritization → Rank by exploitation status and due date
  5. Proactive Alerts → Notify on new KEV entries affecting your stack
  6. Sandbox Analysis → Map sandbox findings to KEV entries
  7. Remediation Tracking → Track patch status in EPM
- Integration with NVD (National Vulnerability Database) for enrichment
- Self-evolving detection based on false positive feedback

First:
1. Duplication check (search for kev in cisa_agent.rs, proactive.rs)
2. If clean → generate:
   - cisa_agent.rs diff (KEV scan + catalog sync)
   - New kev_scanner.rs (catalog sync + vulnerability matching + reporting)
   - Proactive agent for new KEV alerts
3. Integration: "cisa kev scan" → full vulnerability report
4. Tests:
   - Chat: "cisa kev scan" → report with KEV vulns
   - Verify daily catalog sync works
   - Verify proactive alerts trigger

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## KEV Integration Features

| Feature | Sola Component | Implementation |
|---------|----------------|----------------|
| 1. Catalog Sync | Sub-agent | Daily pull from CISA KEV API |
| 2. Vuln Scanning | Scanner | Match software inventory to KEV |
| 3. Severity Reporting | Reporter | CVSS + CISA due dates |
| 4. Patch Priority | Prioritizer | Rank by exploitation + due date |
| 5. Proactive Alerts | Proactive agent | New KEV notifications |
| 6. Sandbox Mapping | Sandbox | Link findings to KEV entries |
| 7. Remediation Tracking | EPM | Track patch status over time |

## CISA KEV API

The KEV catalog is available at:
- **JSON**: `https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json`
- **CSV**: `https://www.cisa.gov/sites/default/files/csv/known_exploited_vulnerabilities.csv`

### KEV Entry Structure

```json
{
  "cveID": "CVE-2024-1234",
  "vendorProject": "Microsoft",
  "product": "Windows",
  "vulnerabilityName": "Windows Kernel Elevation of Privilege",
  "dateAdded": "2024-01-15",
  "shortDescription": "...",
  "requiredAction": "Apply updates per vendor instructions",
  "dueDate": "2024-02-05",
  "knownRansomwareCampaignUse": "Known"
}
```

## Expected Output Format

```
🔴 CISA Known Exploited Vulnerabilities (KEV) Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Scan Time: 2024-01-15 10:30:00 UTC
KEV Catalog Version: 2024-01-15 (1,247 entries)
Systems Scanned: 12

📊 Summary:
┌─────────────────────────────────────────┐
│ Total KEV Vulnerabilities Found: 5      │
│ Critical (CVSS 9.0+): 2                 │
│ High (CVSS 7.0-8.9): 2                  │
│ Medium (CVSS 4.0-6.9): 1                │
│ Overdue Patches: 1                      │
│ Ransomware Associated: 2                │
└─────────────────────────────────────────┘

🚨 CRITICAL - Immediate Action Required:

1. CVE-2024-1234 (CVSS: 9.8) 🔴 OVERDUE
   Vendor: Microsoft | Product: Windows
   Name: Windows Kernel Elevation of Privilege
   Due Date: 2024-01-10 (5 days overdue)
   Ransomware: Known campaign use
   Action: Apply KB5034441 immediately
   Affected: SERVER-01, WORKSTATION-05

2. CVE-2024-5678 (CVSS: 9.1)
   Vendor: Apache | Product: Log4j
   Name: Log4Shell Remote Code Execution
   Due Date: 2024-01-20 (5 days remaining)
   Ransomware: Known campaign use
   Action: Upgrade to Log4j 2.17.1+
   Affected: APP-SERVER-02

⚠️ HIGH - Action Required:

3. CVE-2024-2345 (CVSS: 8.5)
   Vendor: Cisco | Product: IOS XE
   Name: Web UI Command Injection
   Due Date: 2024-01-25 (10 days remaining)
   Ransomware: Unknown
   Action: Apply Cisco advisory patches
   Affected: ROUTER-01

4. CVE-2024-3456 (CVSS: 7.8)
   Vendor: Adobe | Product: Acrobat Reader
   Name: Use-After-Free Vulnerability
   Due Date: 2024-01-30 (15 days remaining)
   Ransomware: Unknown
   Action: Update to Acrobat 23.008.20470+
   Affected: WORKSTATION-01, WORKSTATION-02, WORKSTATION-03

📋 MEDIUM - Scheduled Patching:

5. CVE-2024-4567 (CVSS: 6.5)
   Vendor: Oracle | Product: Java SE
   Name: Deserialization Vulnerability
   Due Date: 2024-02-15 (31 days remaining)
   Ransomware: Unknown
   Action: Update to Java 21.0.2+
   Affected: DEV-SERVER-01

📈 Patch Progress:
- Last 7 days: 3 KEV vulns patched
- Last 30 days: 12 KEV vulns patched
- Current backlog: 5 KEV vulns

🔔 New KEV Entries (Last 24 hours):
- CVE-2024-9999: Chrome V8 Type Confusion (Not in your stack)
- CVE-2024-8888: Fortinet FortiOS (Not in your stack)

🔧 Recommended Actions:
1. URGENT: Patch CVE-2024-1234 on SERVER-01, WORKSTATION-05
2. Schedule maintenance window for CVE-2024-5678
3. Review Cisco advisory for CVE-2024-2345
4. Deploy Adobe update via software center
```

## Proactive Alert Format

When new KEV entries affect your stack:

```
⚠️ NEW CISA KEV ALERT

CVE-2024-9999 has been added to the KEV catalog
and affects software in your environment.

Vendor: Google
Product: Chrome
CVSS: 8.8 (High)
Due Date: 2024-02-01

Affected Systems:
- WORKSTATION-01
- WORKSTATION-02
- WORKSTATION-03

Recommended Action:
Update Chrome to version 121.0.6167.85 or later

Reply "cisa kev scan" for full vulnerability report.
```

## KEV Sync Schedule

The CISA-Agent syncs the KEV catalog:
- **Daily**: Full catalog refresh at 00:00 UTC
- **On-demand**: "cisa kev sync" forces immediate refresh
- **Proactive**: Checks for new entries every 4 hours

## Remediation Tracking

Patch status is tracked in EPM with these states:
- **OPEN**: Vulnerability detected, no action taken
- **IN_PROGRESS**: Patch scheduled or being applied
- **PATCHED**: Vulnerability remediated
- **MITIGATED**: Compensating control in place
- **ACCEPTED**: Risk accepted (requires Tier 2 approval)
