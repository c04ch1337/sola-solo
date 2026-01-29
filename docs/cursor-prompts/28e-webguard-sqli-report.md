# Phase 28e – WebGuard SQLi Report Display

You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WebGuard passive scan + XSS testing implemented
• SQLi testing (28d) implemented
• UI goal: moderate, clean, chat-centric — features via chat commands

Task: Add rich UI report display for SQLi scan results (phase 28e).

Requirements:
- Show SQLi scan results in chat as formatted Markdown (tables, severity badges, PoC payloads, error messages)
- Optional collapsible "WebGuard SQLi Report" panel (hidden by default, toggle via "show webguard sqli" or header icon)
- Panel shows:
  - Scan summary
  - Detected SQLi types (reflected, blind, error-based)
  - PoC payloads that triggered
  - Remediation advice (prepared statements, input validation, WAF)
  - Export as JSON/Markdown button
- Chat commands: "webguard sqli report last" → shows latest SQLi scan
- Keep UI moderate: report in chat primary, panel optional

First:
1. Duplication check (search for sqli/report in App.tsx, components/)
2. If clean → generate:
   - components/WebGuardSQLiReportPanel.tsx (collapsible report)
   - App.tsx diff (toggle + chat command for sqli report)
   - Update chat rendering for SQLi Markdown tables/badges
3. Integration: "webguard test-sqli" → rich report in chat; "show webguard sqli" → opens panel
4. Tests:
   - Run SQLi scan → report appears formatted in chat
   - "show webguard sqli" → panel opens with details

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
