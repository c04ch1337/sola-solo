# 29 - CISA #StopRansomware Integration

Use this prompt to integrate CISA's #StopRansomware framework into the CISA-Agent. This framework provides prevention and response playbooks for ransomware attacks.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• VirusTotal integration exists for malware analysis
• Memory vaults exist for backup/restore
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA #StopRansomware into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa stop-ransomware check <file/path>" → scan for ransomware indicators
- Map to Sola features:
  1. File Analysis → Sandbox + VirusTotal for malware detection
  2. MITRE Mapping → Map findings to MITRE ATT&CK ransomware TTPs
  3. Incident Playbook → Sub-agent with response steps
  4. Backup/Restore → Memory vault verification and restore capability
  5. Network Isolation → Recommendations for containment
  6. IOC Detection → Scan for known ransomware indicators
  7. Encryption Detection → Detect file encryption patterns
  8. Ransom Note Detection → Scan for common ransom note patterns
- Incident response playbook in sub-agents
- Proactive alerts on ransomware indicators
- Integration with CISA's ransomware vulnerability warnings

First:
1. Duplication check (search for ransomware in cisa_agent.rs, sandbox)
2. If clean → generate:
   - cisa_agent.rs diff (ransomware check + playbook trigger)
   - New ransomware_assessor.rs (file analysis + IOC detection + playbook)
   - Sandbox integration for safe file analysis
   - VirusTotal integration for threat intelligence
3. Integration: "cisa stop-ransomware check <file>" → full analysis + playbook
4. Tests:
   - Chat: "cisa stop-ransomware check <file>" → analysis report
   - Verify sandbox isolation works
   - Verify playbook triggers on detection

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## #StopRansomware Checks Mapping

| Check Area | Sola Feature | Assessment Method |
|------------|--------------|-------------------|
| 1. File Analysis | Sandbox + VirusTotal | Isolated execution + hash lookup |
| 2. MITRE Mapping | MITRE ATT&CK API | Map behaviors to TTPs |
| 3. Incident Playbook | Sub-agent system | Automated response steps |
| 4. Backup/Restore | Memory vaults | Verify backup integrity |
| 5. Network Isolation | Proactive agents | Containment recommendations |
| 6. IOC Detection | Threat intel feeds | Known ransomware indicators |
| 7. Encryption Detection | File analysis | Entropy analysis, extension changes |
| 8. Ransom Note Detection | Pattern matching | Common ransom note signatures |

## Ransomware MITRE ATT&CK TTPs

Key TTPs mapped for ransomware detection:

- **T1486**: Data Encrypted for Impact
- **T1490**: Inhibit System Recovery
- **T1489**: Service Stop
- **T1562**: Impair Defenses
- **T1070**: Indicator Removal
- **T1059**: Command and Scripting Interpreter
- **T1547**: Boot or Logon Autostart Execution
- **T1055**: Process Injection

## Expected Output Format

```
🚨 CISA #StopRansomware Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Target: suspicious_file.exe
Status: ⚠️ POTENTIAL THREAT DETECTED

📊 Analysis Results:
┌─────────────────────────────────────────┐
│ VirusTotal: 47/72 engines detected      │
│ Sandbox: Malicious behavior observed    │
│ Threat Family: LockBit 3.0              │
│ Confidence: 94%                         │
└─────────────────────────────────────────┘

🎯 MITRE ATT&CK Mapping:
- T1486: Data Encrypted for Impact ✓
- T1490: Inhibit System Recovery ✓
- T1489: Service Stop ✓
- T1562: Impair Defenses ✓

🔍 Indicators of Compromise (IOCs):
- SHA256: a1b2c3d4e5f6...
- C2 Domain: malicious-domain[.]com
- Registry Key: HKLM\SOFTWARE\LockBit
- File Extension: .lockbit

📋 Incident Response Playbook Activated:
1. ⏳ Isolate affected system (PENDING USER APPROVAL)
2. ⏳ Preserve forensic evidence
3. ⏳ Check backup integrity
4. ⏳ Scan connected systems
5. ⏳ Report to CISA (cisa.gov/report)

💾 Backup Status:
- Last backup: 2 hours ago
- Backup integrity: VERIFIED
- Restore point available: YES

🔧 Immediate Actions Required:
1. DO NOT pay ransom
2. Disconnect from network immediately
3. Preserve all logs and evidence
4. Contact incident response team
5. Report to CISA: cisa.gov/report
```

## Incident Response Playbook

The #StopRansomware playbook includes these automated steps:

### Detection Phase
1. Sandbox analysis of suspicious file
2. VirusTotal hash lookup
3. IOC comparison against threat feeds
4. MITRE ATT&CK TTP mapping

### Containment Phase (Requires User Approval)
1. Network isolation recommendation
2. Process termination guidance
3. Account lockdown suggestions

### Eradication Phase
1. Malware removal guidance
2. System restoration from backup
3. Credential rotation recommendations

### Recovery Phase
1. Backup integrity verification
2. System restoration steps
3. Monitoring for re-infection

### Lessons Learned
1. Store incident details in EPM
2. Update detection rules
3. Improve backup procedures
