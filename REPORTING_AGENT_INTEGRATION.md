# Reporting Agent Integration Guide

## Overview

The Reporting Agent is a professional vulnerability and exploit reporting system that proactively creates comprehensive reports for security findings from WebGuard, Network Security Agent, Malware Sandbox, and user submissions.

## Backend Implementation ✅

### 1. Reporting Agent Crate (`reporting_agent/`)

**Files Created:**
- [`reporting_agent/Cargo.toml`](reporting_agent/Cargo.toml) - Dependencies and crate configuration
- [`reporting_agent/src/lib.rs`](reporting_agent/src/lib.rs) - Main agent implementation with report generation
- [`reporting_agent/src/mitre.rs`](reporting_agent/src/mitre.rs) - MITRE ATT&CK mapping
- [`reporting_agent/src/templates.rs`](reporting_agent/src/templates.rs) - Report templates

**Features:**
- Professional markdown-formatted reports
- Executive summary, findings, PoC, remediation sections
- MITRE ATT&CK technique mapping
- Risk scoring (0-10.0 scale)
- Severity classification (Info, Low, Medium, High, Critical)
- Memory storage (EPM/LTM integration)
- Proactive alerts for high-severity findings

### 2. Phoenix-Web Integration

**Files Modified:**
- [`phoenix-web/Cargo.toml`](phoenix-web/Cargo.toml) - Added reporting_agent dependency
- [`phoenix-web/src/main.rs`](phoenix-web/src/main.rs) - Added imports, AppState field, initialization, command routing
- [`phoenix-web/src/reporting_handler.rs`](phoenix-web/src/reporting_handler.rs) - Command handler for report operations

**Commands Available:**
```bash
report help                    # Show help
report list                    # List all stored reports
report get <report_id>         # Get specific report by ID
report last scan               # Generate report from last WebGuard/network scan
report vuln <scan_id>          # Generate report for specific vulnerability scan
report file <filename>         # Generate report for file submission
report url <url>               # Generate report for URL submission
```

### 3. Workspace Configuration

**Files Modified:**
- [`Cargo.toml`](Cargo.toml) - Added reporting_agent to workspace members

## Frontend Implementation ✅

### 1. Reports Panel Component

**Files Created:**
- [`frontend_desktop/components/ReportsPanel.tsx`](frontend_desktop/components/ReportsPanel.tsx) - Collapsible reports panel UI

**Features:**
- Latest report view
- Report history browser
- Rendered markdown view
- Raw markdown view
- Export to JSON/Markdown
- Severity badges and risk scores
- Quick actions (generate from last scan, refresh list)

### 2. App.tsx Integration (TODO)

**Required Changes:**

1. **Add Import** (line ~13):
```typescript
import ReportsPanel, { VulnerabilityReport } from './components/ReportsPanel';
```

2. **Add State** (after line ~234):
```typescript
// Reports panel state (hidden by default, toggle via "show reports")
const [showReportsPanel, setShowReportsPanel] = useState(false);
const [vulnerabilityReports, setVulnerabilityReports] = useState<VulnerabilityReport[]>([]);
```

3. **Add Command Handlers** (in `handleLocalCommand` function, after line ~270):
```typescript
if (lower === 'show reports' || lower === 'reports panel' || lower === 'open reports') {
  setShowReportsPanel(true);
  return { kind: 'handled', localAssistantMessage: '📊 Reports panel opened. View your vulnerability reports here.' };
}
if (lower === 'hide reports' || lower === 'close reports') {
  setShowReportsPanel(false);
  return { kind: 'handled', localAssistantMessage: 'Reports panel hidden.' };
}
```

4. **Add Report Command Detection** (in `handleSendMessage` function, after line ~3220):
```typescript
// Check if this is a report command
const isReportCommand = lower.startsWith('report ');
if (isReportCommand) {
  const result = await apiCommand(content);
  
  // Store report if generated
  if (result.type === 'report.generated' && result.report) {
    setVulnerabilityReports(prev => [result.report, ...prev.slice(0, 19)]); // Keep last 20
  }
  
  // Store report list if fetched
  if (result.type === 'report.list' && result.reports) {
    // Optionally update state with full report list
  }
  
  addMessage({
    role: 'assistant',
    content: result.message || 'Report command executed.',
    timestamp: Date.now(),
  });
  return;
}
```

5. **Add Panel Toggle Button** (in sidebar, after line ~3795):
```typescript
<button
  onClick={() => setShowReportsPanel(!showReportsPanel)}
  className={`p-2 rounded-lg transition-all ${showReportsPanel ? 'bg-cyan-500/20 text-cyan-400' : 'hover:bg-slate-800 text-slate-500'}`}
  title="Vulnerability Reports"
>
  <span className="material-symbols-outlined">description</span>
</button>
```

6. **Add Panel Component** (after WebGuardReportPanel, around line ~3882):
```typescript
<ReportsPanel
  isOpen={showReportsPanel}
  onClose={() => setShowReportsPanel(false)}
  reports={vulnerabilityReports}
  onCommand={(cmd) => {
    setInputValue(cmd);
    setShowReportsPanel(false);
  }}
/>
```

### 3. Command Registry

**Files Modified:**
- [`docs/frontend_command_registry.json`](docs/frontend_command_registry.json) - Added brain.report.* commands

## Testing Guide

### 1. Test Report Generation from WebGuard Scan

```bash
# Terminal 1: Start backend
cd phoenix-web
cargo run

# Terminal 2: Start frontend
cd frontend_desktop
npm run dev

# In UI chat:
webguard scan https://example.com
report last scan
```

**Expected Result:**
- Professional markdown report generated
- Report stored in EPM memory
- Report appears in Reports panel
- Severity badge and risk score displayed

### 2. Test Proactive Alerts

```bash
# Generate high-severity report
webguard test-xss https://vulnerable-site.com/search q
report last scan
```

**Expected Result:**
- Alert logged in backend console: `🚨 Proactive Alert: ...`
- High/Critical severity badge shown
- Risk score >= 7.0

### 3. Test Report List and Retrieval

```bash
report list
report get <report_id>
```

**Expected Result:**
- List shows all stored reports with metadata
- Get retrieves specific report by ID
- Reports persist in EPM memory

### 4. Test Panel UI

```bash
show reports
```

**Expected Result:**
- Panel opens with collapsible interface
- Latest report tab shows most recent
- History tab shows all reports
- Export buttons work (JSON/Markdown)
- View toggle works (Rendered/Markdown)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (React/TS)                      │
│  ┌────────────────┐  ┌──────────────┐  ┌─────────────────┐ │
│  │  ReportsPanel  │  │   App.tsx    │  │  Chat Commands  │ │
│  │   Component    │◄─┤   State      │◄─┤  report *       │ │
│  └────────────────┘  └──────────────┘  └─────────────────┘ │
└──────────────────────────────┬──────────────────────────────┘
                               │ HTTP/WS
┌──────────────────────────────▼──────────────────────────────┐
│                  Phoenix-Web (Actix/Rust)                    │
│  ┌────────────────────┐  ┌──────────────────────────────┐  │
│  │ reporting_handler  │  │      Command Router          │  │
│  │   .rs              │◄─┤  (main.rs)                   │  │
│  └─────────┬──────────┘  └──────────────────────────────┘  │
│            │                                                 │
│  ┌─────────▼──────────────────────────────────────────────┐ │
│  │           ReportingAgent (reporting_agent)             │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │ │
│  │  │   Report     │  │    MITRE     │  │  Templates  │ │ │
│  │  │  Generation  │  │   Mapping    │  │             │ │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘ │ │
│  └────────────────────────────────────────────────────────┘ │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                    Data Sources                              │
│  ┌──────────┐  ┌──────────────┐  ┌────────────────────┐    │
│  │ WebGuard │  │   Network    │  │  Malware Sandbox   │    │
│  │  Scans   │  │   Security   │  │     Agent          │    │
│  └──────────┘  └──────────────┘  └────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Report Format

### Markdown Structure

```markdown
# 🔴 Cross-Site Scripting (XSS) Vulnerability

**Report ID:** `abc-123-def`
**Generated:** 2026-01-23 04:00:00 UTC
**Severity:** 🔴 HIGH
**Risk Score:** 7.5/10.0

---

## 📋 Executive Summary

A Cross-Site Scripting vulnerability was identified...

## 🔍 Findings

### 🟠 Finding 1: Reflected XSS in Search Parameter

**Severity:** HIGH
**CVSS Score:** 7.3

Description of the vulnerability...

**Evidence:**
- Payload reflected in response
- No input sanitization detected

## 🎯 Affected Assets

- `https://example.com/search`
- `https://example.com/results`

## 🧪 Proof of Concept

```javascript
<script>alert('XSS')</script>
```

## 🛠️ Remediation Plan

**Priority:** 🟠 HIGH
**Estimated Effort:** 2-4 hours

### Steps

1. **Implement input validation**
   - Sanitize all user inputs before rendering
   - Tools: DOMPurify, OWASP Java Encoder

### Validation

- Re-test with XSS payloads
- Code review

## 🎯 MITRE ATT&CK Mapping

| Technique ID | Technique Name | Tactic | Description |
|--------------|----------------|--------|-------------|
| T1189 | Drive-by Compromise | Initial Access | XSS can be used for drive-by attacks |

## 📚 References

- https://owasp.org/www-community/attacks/xss/
- https://cwe.mitre.org/data/definitions/79.html

## 🏷️ Tags

`webguard` `xss` `injection`

---

*Generated by Phoenix AGI Reporting Agent*
```

## Next Steps

1. **Complete App.tsx Integration** - Add the 6 code snippets above
2. **Test End-to-End** - Run through all test scenarios
3. **Agent Spawner Template** (Optional) - Create template for spawning ReportingAgent as sub-agent
4. **Proactive Monitoring** (Future) - Auto-generate reports on new scans
5. **Self-Evolution** (Future) - Improve templates based on feedback

## Notes

- Reports are stored in EPM memory with key format: `report:<report_id>`
- WebGuard scans are stored with key format: `webguard:scan:<scan_id>`
- Proactive alerts trigger for severity >= High (configurable)
- Maximum 100 reports stored in memory (configurable)
- Panel is hidden by default, toggle with `show reports` / `hide reports`
- Export supports JSON and Markdown formats
