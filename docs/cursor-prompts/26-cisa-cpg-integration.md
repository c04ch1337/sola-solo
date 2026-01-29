# 26 - CISA CPG Integration (Cross-Sector Cybersecurity Performance Goals)

Use this prompt to integrate CISA Cross-Sector CPGs into the CISA-Agent. CPGs are baseline cybersecurity practices for account security, vulnerability management, and incident response.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA Cross-Sector CPGs into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa cpg scan" → assess system against 9 CPG goals
- Map to Sola features:
  1. Account Security → Tiered access control (Tier 0–2 with consent gating)
  2. Device Security → Device/browser control integration
  3. Data Security → Memory vault encryption + backup
  4. Governance & Training → Audit logging + user guidance
  5. Vulnerability Management → KEV scanner integration
  6. Supply Chain/Third Party → SBOM generation + dependency scanning
  7. Response & Recovery → Incident playbook + memory restore
  8. Network Security → Network monitoring via proactive agents
  9. Email Security → Outlook COM integration + phishing detection
- Generate report in chat (compliance score 0-100, gaps, recommendations)
- Self-evolve: Update playbook based on assessment accuracy after X runs
- Store assessment results in EPM for historical tracking

First:
1. Duplication check (search for cpg in cisa_agent.rs, phoenix-web)
2. If clean → generate:
   - cisa_agent.rs diff (CPG assessment steps + mapping to Sola features)
   - New cpg_assessor.rs (detailed CPG goal checks)
   - playbook YAML update (CPG-specific steps)
3. Integration: "cisa cpg scan" → full compliance report in chat
4. Tests:
   - Chat: "cisa cpg scan" → report with score + gaps
   - Verify each CPG goal maps to Sola feature

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## CPG Goals Mapping

| CPG Goal | Sola Feature | Assessment Check |
|----------|--------------|------------------|
| 1. Account Security | Tiered access (Tier 0-2) | Verify consent gating active |
| 2. Device Security | Browser control | Check device trust status |
| 3. Data Security | Memory vault encryption | Verify encryption enabled |
| 4. Governance & Training | Audit logging | Check log retention policy |
| 5. Vulnerability Management | KEV scanner | Run KEV scan, check patch status |
| 6. Supply Chain | SBOM generation | Verify SBOM exists for agents |
| 7. Response & Recovery | Incident playbook | Check playbook completeness |
| 8. Network Security | Proactive agents | Verify network monitoring active |
| 9. Email Security | Outlook COM | Check phishing detection enabled |

## Expected Output Format

```
📊 CISA CPG Compliance Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Score: 78/100

✅ Account Security: PASS (Tier 2 gating active)
✅ Device Security: PASS (Browser control enabled)
⚠️ Data Security: PARTIAL (Encryption enabled, backup not configured)
✅ Governance: PASS (Audit logging active, 90-day retention)
❌ Vulnerability Management: FAIL (3 KEV vulns unpatched)
⚠️ Supply Chain: PARTIAL (SBOM exists, 2 deps outdated)
✅ Response & Recovery: PASS (Playbook complete)
✅ Network Security: PASS (Proactive monitoring active)
✅ Email Security: PASS (Phishing detection enabled)

🔧 Recommendations:
1. Patch KEV vulnerabilities: CVE-2024-1234, CVE-2024-5678, CVE-2024-9012
2. Configure automated backup for memory vaults
3. Update outdated dependencies: lodash@4.17.20, axios@0.21.1
```
