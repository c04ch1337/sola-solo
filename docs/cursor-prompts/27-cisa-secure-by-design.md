# 27 - CISA Secure by Design Integration

Use this prompt to integrate CISA's Secure by Design Pledge into the CISA-Agent. This framework focuses on secure defaults, vulnerability disclosure, and building security into products from the start.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• Code analysis tools exist for vulnerability detection
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA Secure by Design Pledge into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa secure-design check <code/path>" → analyze for secure-by-design principles
- Map to Sola features:
  1. MFA by Default → Check authentication implementations
  2. Eliminate Default Passwords → Scan for hardcoded credentials
  3. Reduce Vulnerability Classes → Static analysis for common vulns (SQLi, XSS, etc.)
  4. Security Patches → Verify auto-update mechanisms
  5. Vulnerability Disclosure Policy → Check for SECURITY.md, CVE process
  6. CVE Transparency → Verify CWE root cause documentation
  7. Evidence of Intrusions → Logging and audit trail verification
- Generate SBOM (Software Bill of Materials) for analyzed code
- Code review via tool agents for vulnerability disclosure
- Self-modification bounded by ethical guardrails
- Store analysis results in EPM for pattern learning

First:
1. Duplication check (search for secure-design/sbom in cisa_agent.rs, code_analysis)
2. If clean → generate:
   - cisa_agent.rs diff (secure-design assessment + SBOM generation)
   - New secure_design_assessor.rs (7 pledge goals + code analysis)
   - Integration with code_analysis crate for static analysis
3. Integration: "cisa secure-design check src/" → full analysis report
4. Tests:
   - Chat: "cisa secure-design check <path>" → report with findings
   - Verify SBOM generation works

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## Secure by Design Pledge Goals Mapping

| Pledge Goal | Sola Feature | Assessment Check |
|-------------|--------------|------------------|
| 1. MFA by Default | Auth analysis | Scan for MFA implementation |
| 2. No Default Passwords | Credential scanner | Detect hardcoded secrets |
| 3. Reduce Vuln Classes | Static analysis | SQLi, XSS, SSRF, path traversal |
| 4. Security Patches | Update checker | Verify auto-update mechanism |
| 5. Vuln Disclosure Policy | File scanner | Check SECURITY.md exists |
| 6. CVE Transparency | Documentation check | Verify CWE documentation |
| 7. Intrusion Evidence | Logging audit | Check audit trail completeness |

## Expected Output Format

```
🔒 CISA Secure by Design Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Target: src/
Files Analyzed: 47
SBOM Generated: sbom-2024-01-15.json

Pledge Compliance:
✅ MFA by Default: PASS (MFA enforced in auth.rs:45)
❌ No Default Passwords: FAIL (Hardcoded credential found)
   └─ src/config.rs:23 - API_KEY = "default123"
⚠️ Reduce Vuln Classes: PARTIAL (2 potential issues)
   └─ src/api/handler.rs:89 - Potential SQL injection
   └─ src/web/render.rs:34 - Potential XSS
✅ Security Patches: PASS (Auto-update enabled)
✅ Vuln Disclosure: PASS (SECURITY.md present)
⚠️ CVE Transparency: PARTIAL (CWE mapping incomplete)
✅ Intrusion Evidence: PASS (Audit logging enabled)

📦 SBOM Summary:
- Dependencies: 127
- Direct: 23
- Transitive: 104
- Known Vulnerabilities: 2 (lodash@4.17.20, axios@0.21.1)

🔧 Recommendations:
1. Remove hardcoded credential in src/config.rs:23
2. Use parameterized queries in src/api/handler.rs:89
3. Sanitize output in src/web/render.rs:34
4. Update vulnerable dependencies
```

## SBOM Format (CycloneDX)

The generated SBOM follows CycloneDX format for compatibility with vulnerability scanners:

```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.4",
  "version": 1,
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "tools": [{ "name": "Sola CISA-Agent", "version": "1.0.0" }]
  },
  "components": [...]
}
```
