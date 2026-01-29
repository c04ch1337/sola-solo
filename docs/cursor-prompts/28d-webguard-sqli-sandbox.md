# Phase 28d – WebGuard SQLi Sandbox Testing

You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WebGuard passive scan implemented (headers, fingerprinting, CORS, sensitive paths)
• WebGuard XSS testing implemented (safe payloads in sandbox CDP)
• Browser CDP control exists (navigate, screenshot, etc.) using local Chrome on port 9222
• Sandbox exists (isolated folder, path validation, symlink rejection, no execution)
• UI goal: moderate, clean, chat-centric — features via chat commands

Task: Add safe SQL injection (SQLi) testing to WebGuard (phase 28d: SQLi sandbox).

Requirements:
- Chat command: "webguard test-sqli <url> <param>" (e.g. "webguard test-sqli https://example.com/search q")
- Use safe SQLi payloads only (error-based, boolean-based, time-based — no destructive ones)
  - Examples: ' OR '1'='1, " OR ""=" , ' WAITFOR DELAY '0:0:5'--, 1' OR SLEEP(5)--
- Run in isolated sandbox CDP session (never on host browser/database)
- Capture response time, DOM changes, error messages after each payload
- Detect blind/reflected SQLi (time delays > threshold, boolean changes, error keywords like "SQL syntax", "mysql_fetch")
- Report findings (severity, PoC payload, remediation advice: prepared statements, input validation, WAF)
- All tests run in sandbox folder (no host OS/database impact)
- Store report in EPM memory
- Optional: tray notification for confirmed SQLi

First:
1. Duplication check (search for sqli/webguard in codebase)
2. If clean → generate:
   - webguard/src/lib.rs diff (add SQLi test logic + sandbox CDP session)
   - Use existing sandbox path validation
   - App.tsx diff (chat parser for test-sqli)
3. Integration: "webguard test-sqli https://example.com/search q" → runs safe payloads, reports findings
4. Tests:
   - Chat command → safe payloads tested, report generated
   - Path escape attempt → rejected with error

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
