# 28a - WebGuard Passive Scan Implementation

## Overview

WebGuard is a lightweight web vulnerability scanner integrated into Sola, providing passive security analysis similar to Burp Suite but simpler, safer, and fully chat-integrated.

## Implementation Status: ✅ Complete

### Components Created

1. **webguard crate** (`webguard/src/lib.rs`)
   - Passive scan logic for security header analysis
   - Server fingerprinting
   - CORS misconfiguration detection
   - Sensitive path exposure detection
   - Tech stack detection
   - Markdown report formatting

2. **phoenix-web integration** (`phoenix-web/src/main.rs`)
   - `handle_webguard_command()` function
   - Command routing for `webguard scan`, `webguard passive`, `webguard report`
   - EPM memory storage for scan reports
   - WebGuard instance in AppState

3. **Frontend integration** (`frontend_desktop/App.tsx`)
   - Help topic for `help webguard`
   - Added to help topics list

4. **Tests** (`webguard/tests/integration_tests.rs`)
   - Comprehensive unit and integration tests
   - Report serialization/deserialization tests
   - Severity level tests
   - Markdown formatting tests

## Chat Commands

```
webguard scan <url>      - Run passive security scan
webguard passive <url>   - Same as scan
webguard report last     - Show last scan report
webguard help            - Show help
```

## Passive Scan Checks

### Security Headers
- Content-Security-Policy (CSP)
- Strict-Transport-Security (HSTS)
- X-Frame-Options
- X-Content-Type-Options
- Referrer-Policy
- Permissions-Policy
- X-XSS-Protection (deprecated check)

### Server Fingerprinting
- Server header
- X-Powered-By
- X-AspNet-Version
- X-Generator
- Via header
- Technology detection (Nginx, Apache, PHP, ASP.NET, etc.)

### CORS Analysis
- Access-Control-Allow-Origin
- Access-Control-Allow-Credentials
- Wildcard origin detection
- Credentials misconfiguration

### Sensitive Paths (HEAD requests)
- /.git/config, /.git/HEAD
- /.env, /.env.local, /.env.production
- /admin, /administrator
- /backup, /backups
- /wp-admin, /wp-config.php
- /phpmyadmin
- /.htaccess, /.htpasswd
- /server-status, /server-info
- /swagger.json, /openapi.json
- And more...

## Severity Levels

| Level | Emoji | Description |
|-------|-------|-------------|
| Critical | 🔴 | Immediate action required |
| High | 🟠 | Significant security risk |
| Medium | 🟡 | Moderate concern |
| Low | 🔵 | Minor issue |
| Info | ⚪ | Informational |

## Report Format

Reports are displayed as formatted Markdown in chat with:
- Summary table (findings by severity)
- Security headers table (present/missing status)
- Server fingerprint section
- CORS configuration analysis
- Exposed sensitive paths
- Detailed findings with remediation advice

## Storage

- Reports stored in EPM memory with key `webguard:scan:{scan_id}`
- Last report cached in `webguard_last_report` for quick access
- Reports persist across sessions

## Future Phases

- **28b**: Active XSS/reflected testing in sandbox
- **28c**: Rich report UI panel (collapsible, exportable)

## Usage Example

```
User: webguard scan https://example.com

Sola: ## 🟡 WebGuard Scan Report

**Target:** `https://example.com`
**Scan Time:** 2026-01-23 02:00:00 UTC
**Duration:** 150ms
**HTTP Status:** 200

---

### 📊 Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | 0 |
| 🟠 High | 1 |
| 🟡 Medium | 2 |
| 🔵 Low | 3 |
| ⚪ Info | 2 |

**Total Findings:** 8

### 🔒 Security Headers

| Header | Status | Value |
|--------|--------|-------|
| Content-Security-Policy | ❌ | `-` |
| Strict-Transport-Security | ✅ | `max-age=31536000` |
...
```

## Dependencies

- reqwest (HTTP client)
- serde/serde_json (serialization)
- chrono (timestamps)
- uuid (scan IDs)
- regex (pattern matching)
- url (URL parsing)
- tracing (logging)
- anyhow/thiserror (error handling)
