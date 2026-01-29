# Phase 28b: WebGuard XSS Testing

## Overview

Phase 28b extends WebGuard with safe active XSS (Cross-Site Scripting) vulnerability testing capabilities. This builds directly on Phase 28a's passive scanning foundation.

## Features Implemented

### XSS Testing Module (`webguard/src/lib.rs`)

1. **Safe XSS Payloads** (`SAFE_XSS_PAYLOADS`)
   - 17 safe, non-destructive payloads
   - Basic script injection (`<script>alert(1)</script>`)
   - Event handler payloads (`onerror`, `onload`, `onclick`, etc.)
   - Encoded payloads (HTML entities, CharCode)
   - JavaScript URI payloads
   - Polyglot payloads

2. **XSS Types** (`XssType`)
   - `Reflected` - Payload reflected in response
   - `Stored` - Payload stored and executed later
   - `DomBased` - Client-side JavaScript execution

3. **XSS Tester** (`XssTester`)
   - Configurable timeout, user agent, max payloads
   - Reflection detection (exact and encoded)
   - Execution indicator detection
   - Context-aware analysis (HTML, attribute, JavaScript)
   - Proof-of-concept URL generation
   - Remediation advice per XSS type

4. **Report Structures**
   - `XssTestReport` - Complete test results
   - `XssFinding` - Individual vulnerability finding
   - `XssPayloadResult` - Per-payload test result
   - `XssSummary` - Summary statistics

5. **Markdown Formatting**
   - `format_xss_report_markdown()` - Full report for chat
   - `format_xss_notification_summary()` - Brief notification

### Backend Integration (`phoenix-web/src/main.rs`)

1. **New Commands**
   - `webguard test-xss <url> <param>` - Test URL parameter for XSS
   - `webguard xss-report last` - Show last XSS test report

2. **State Management**
   - `xss_tester: Option<Arc<XssTester>>` - XSS tester instance
   - `xss_last_report: Arc<Mutex<Option<XssTestReport>>>` - Last report cache

3. **EPM Storage**
   - Reports stored with key `webguard:xss:{id}`
   - Retrievable by ID or "last"

### Frontend Help (`frontend_desktop/App.tsx`)

Updated `help webguard` to include:
- XSS testing commands
- Safe payload information
- Detection types
- Usage examples

## Chat Commands

```
# Test a URL parameter for XSS
webguard test-xss https://example.com/search q

# View last XSS test report
webguard xss-report last

# View XSS report by ID
webguard xss-report <scan-id>

# Show help (includes XSS commands)
webguard help
```

## Example Output

```markdown
## 🔴 WebGuard XSS Test Report

**Target:** `https://example.com/search`
**Parameter:** `q`
**Scan Time:** 2026-01-23 02:30:00 UTC
**Duration:** 500ms

---

### 📊 Summary

⚠️ **VULNERABLE TO XSS**

| Metric | Value |
|--------|-------|
| Payloads Tested | 17 |
| Payloads Reflected | 3 |
| Payloads Executed | 1 |
| Total Findings | 1 |
| 🔴 Critical | 1 |
| 🟠 High | 0 |

### 🚨 Vulnerabilities Found

#### 🔴 XSS-001 - Reflected XSS
**Parameter:** `q`
**Payload:** `<script>alert(1)</script>`
**Description:** Basic script injection
**Evidence:** Payload reflected unescaped in response

**Proof of Concept:**
```
https://example.com/search?q=<script>alert(1)</script>
```

**Remediation:**
1. Implement proper output encoding based on context
2. Use Content-Security-Policy header
3. Validate and sanitize all user input
4. Use HTTPOnly and Secure flags on cookies
5. Consider using a Web Application Firewall (WAF)
```

## Safety Features

1. **Safe Payloads Only**
   - No destructive actions (no `document.cookie` theft)
   - No stored XSS attacks
   - No persistent modifications

2. **Sandbox Mode**
   - `sandbox_mode: true` by default
   - Tests run in isolated context

3. **Rate Limiting**
   - Configurable timeout
   - Max payloads limit

## Testing

Run the integration tests:

```bash
cd webguard
cargo test
```

Key tests:
- `test_xss_tester_creation_default`
- `test_xss_tester_creation_custom_config`
- `test_xss_type_descriptions`
- `test_safe_xss_payloads_exist`
- `test_format_xss_report_markdown_vulnerable`
- `test_format_xss_report_markdown_clean`
- `test_format_xss_notification_summary_vulnerable`
- `test_format_xss_notification_summary_clean`

## Files Modified

1. `webguard/Cargo.toml` - Added dependencies (tokio-tungstenite, futures, scraper)
2. `webguard/src/lib.rs` - Added XSS testing module
3. `webguard/tests/integration_tests.rs` - Added XSS tests
4. `phoenix-web/src/main.rs` - Added XSS command handling
5. `frontend_desktop/App.tsx` - Updated help text

## Dependencies Added

```toml
# WebSocket for CDP communication (XSS testing)
tokio-tungstenite = "0.21"
futures = "0.3"

# HTML parsing for XSS detection
scraper = "0.18"
```

## Future Enhancements (Phase 28c+)

1. **CDP Integration** - Real browser execution via Chrome DevTools Protocol
2. **DOM-based XSS Detection** - Client-side JavaScript analysis
3. **Stored XSS Detection** - Multi-request correlation
4. **Custom Payloads** - User-defined payload lists
5. **Report UI** - Visual report panel in frontend

## Related Documentation

- [Phase 28a: WebGuard Passive Scan](./28a-webguard-passive-scan.md)
- [Browser Control Integration](../BROWSER_CONTROL_INTEGRATION.md)
- [Sandbox Architecture](../MALWARE_SANDBOX_INTEGRATION.md)
