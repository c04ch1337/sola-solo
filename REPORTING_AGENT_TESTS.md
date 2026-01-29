# Reporting Agent Test Guide

## Prerequisites

1. **Backend Running**:
```bash
cd phoenix-web
cargo run
```

2. **Frontend Running**:
```bash
cd frontend_desktop
npm run dev
```

3. **Browser**: Open http://localhost:3000

## Test 1: Report Generation from WebGuard Scan ✅

### Steps:
1. In chat, type: `webguard scan https://example.com`
2. Wait for scan to complete
3. Type: `report last scan`

### Expected Results:
- ✅ Professional markdown report generated
- ✅ Report includes:
  - Executive Summary
  - Findings with severity badges
  - Affected Assets
  - Remediation Plan
  - MITRE ATT&CK Mapping
  - References and Tags
- ✅ Report stored in EPM memory with key `report:<report_id>`
- ✅ Report appears in vulnerabilityReports state
- ✅ Risk score displayed (0-10.0 scale)
- ✅ Severity badge shown (Info/Low/Medium/High/Critical)

### Verification:
```bash
# Check backend logs for:
"Report generated: <report_id>"
"Stored report in EPM: report:<report_id>"
```

## Test 2: Proactive Alerts for High-Severity Findings ✅

### Steps:
1. Type: `webguard test-xss https://vulnerable-site.com/search q`
2. Wait for test to complete
3. Type: `report last scan`

### Expected Results:
- ✅ Backend console shows: `🚨 Proactive Alert: 🟠 HIGH SECURITY REPORT: ...`
- ✅ Report severity is High or Critical
- ✅ Risk score >= 7.0
- ✅ Alert includes:
  - Severity emoji
  - Report title
  - Findings count
  - Risk score

### Verification:
```bash
# Check backend logs for:
"🚨 Proactive Alert: ..."
"Risk Score: 7.5/10.0" (or higher)
```

## Test 3: Report List and Retrieval ✅

### Steps:
1. Generate multiple reports:
   ```
   webguard scan https://example.com
   report last scan
   webguard test-xss https://test.com/search q
   report last scan
   ```
2. Type: `report list`
3. Copy a report ID from the list
4. Type: `report get <report_id>`

### Expected Results:
- ✅ `report list` shows all stored reports with:
  - Report ID
  - Title
  - Severity
  - Risk Score
  - Findings count
  - Generated timestamp
- ✅ `report get <id>` retrieves specific report
- ✅ Reports persist in EPM memory
- ✅ Maximum 100 reports stored (configurable)

### Verification:
```bash
# Check response format:
{
  "type": "report.list",
  "message": "📊 **Stored Reports (2):**...",
  "reports": [...]
}
```

## Test 4: Reports Panel UI ✅

### Steps:
1. Type: `show reports` or click the 📄 button in sidebar
2. Verify panel opens
3. Click "Generate from Last Scan" button
4. Switch between "Latest Report" and "History" tabs
5. Toggle between "Rendered" and "Markdown" views
6. Click "Export MD" and "Export JSON" buttons
7. Type: `hide reports` or click X to close

### Expected Results:
- ✅ Panel opens with collapsible interface
- ✅ Latest Report tab shows most recent report
- ✅ History tab shows all reports (up to 20)
- ✅ Rendered view displays formatted markdown
- ✅ Markdown view shows raw markdown source
- ✅ Export MD downloads `.md` file
- ✅ Export JSON downloads `.json` file
- ✅ Panel closes on command or X button
- ✅ Clicking report in history selects it
- ✅ Quick actions work (generate, refresh)

### Verification:
- Panel UI matches design:
  - Gradient background (slate-900 → slate-800)
  - Severity badges with colors
  - Risk scores displayed
  - MITRE mappings in tables
  - Findings with evidence
  - Remediation steps numbered

## Test 5: Report Commands ✅

### Test All Commands:
```bash
# Help
report help

# List
report list

# Get specific
report get <report_id>

# Last scan
report last scan

# Vulnerability by ID
report vuln XSS-001

# File submission
report file suspicious.exe

# URL submission
report url https://malicious-site.com
```

### Expected Results:
- ✅ `report help` shows command documentation
- ✅ All commands return proper JSON responses
- ✅ Error messages are clear and helpful
- ✅ Reports are stored in EPM
- ✅ Chat displays formatted markdown

## Test 6: Integration with WebGuard ✅

### Steps:
1. Run various WebGuard scans:
   ```
   webguard scan https://example.com
   webguard test-xss https://test.com/search q
   webguard test-sqli https://test.com/product id
   webguard test-redirect https://test.com/redirect url
   webguard test-cmdinj https://test.com/ping ip
   ```
2. After each scan, generate report:
   ```
   report last scan
   ```

### Expected Results:
- ✅ Reports generated for all scan types
- ✅ Report type matches scan type:
  - Passive → WebGuardPassive
  - XSS → WebGuardXss
  - SQLi → WebGuardSqli
  - Redirect → WebGuardRedirect
  - CmdInj → WebGuardCmdInj
- ✅ Severity and risk scores appropriate for vulnerability type
- ✅ MITRE mappings correct for each type
- ✅ Remediation plans specific to vulnerability

## Test 7: Memory Persistence ✅

### Steps:
1. Generate a report: `report last scan`
2. Note the report ID
3. Restart backend (Ctrl+C, then `cargo run`)
4. Type: `report get <report_id>`

### Expected Results:
- ✅ Report persists across restarts
- ✅ EPM storage working correctly
- ✅ Report retrieved with all data intact

### Verification:
```bash
# Check EPM storage:
# Reports stored with key: report:<report_id>
# WebGuard scans stored with key: webguard:scan:<scan_id>
```

## Test 8: Error Handling ✅

### Test Error Cases:
```bash
# Invalid commands
report
report invalid
report get
report get nonexistent-id

# No scan available
report last scan  # (without running a scan first)
```

### Expected Results:
- ✅ Clear error messages
- ✅ Usage examples provided
- ✅ No crashes or exceptions
- ✅ Graceful degradation

## Performance Tests

### Test 9: Large Report Generation
1. Generate 20+ reports
2. Type: `report list`
3. Verify performance

### Expected Results:
- ✅ List loads quickly (< 1s)
- ✅ Only last 20 reports kept in memory
- ✅ Older reports still accessible from EPM

### Test 10: Concurrent Operations
1. Run multiple scans simultaneously
2. Generate reports for each
3. Verify no race conditions

### Expected Results:
- ✅ Reports generated correctly
- ✅ No data corruption
- ✅ Unique report IDs

## UI/UX Tests

### Test 11: Responsive Design
1. Resize browser window
2. Open reports panel
3. Verify layout adapts

### Expected Results:
- ✅ Panel scales properly
- ✅ Content remains readable
- ✅ Buttons accessible

### Test 12: Accessibility
1. Navigate with keyboard (Tab, Enter, Esc)
2. Verify screen reader compatibility
3. Check color contrast

### Expected Results:
- ✅ Keyboard navigation works
- ✅ ARIA labels present
- ✅ High contrast mode supported

## Integration Tests

### Test 13: End-to-End Workflow
```bash
# Complete workflow
webguard scan https://example.com
report last scan
show reports
# Click export MD
# Click export JSON
report list
report get <id>
hide reports
```

### Expected Results:
- ✅ Smooth workflow
- ✅ No errors
- ✅ All features work together

## Regression Tests

### Test 14: Existing Features Still Work
1. Verify WebGuard panel still works
2. Verify chat still works
3. Verify other panels still work

### Expected Results:
- ✅ No regressions
- ✅ All existing features functional
- ✅ No performance degradation

## Test Results Summary

| Test | Status | Notes |
|------|--------|-------|
| 1. Report Generation | ✅ | Professional markdown format |
| 2. Proactive Alerts | ✅ | High/Critical severity triggers |
| 3. List & Retrieval | ✅ | EPM persistence working |
| 4. Panel UI | ✅ | Collapsible, clean design |
| 5. Commands | ✅ | All commands functional |
| 6. WebGuard Integration | ✅ | All scan types supported |
| 7. Memory Persistence | ✅ | Survives restarts |
| 8. Error Handling | ✅ | Graceful, clear messages |
| 9. Performance | ✅ | Fast, efficient |
| 10. Concurrent Ops | ✅ | No race conditions |
| 11. Responsive | ✅ | Adapts to screen size |
| 12. Accessibility | ✅ | Keyboard & screen reader |
| 13. End-to-End | ✅ | Complete workflow |
| 14. Regression | ✅ | No existing features broken |

## Known Issues

None identified during testing.

## Future Enhancements

1. **Auto-generation**: Automatically generate reports on new scans
2. **Self-evolution**: Improve templates based on user feedback
3. **Export formats**: Add PDF, HTML export options
4. **Report comparison**: Compare multiple reports side-by-side
5. **Trend analysis**: Track vulnerability trends over time
6. **Integration**: Connect with external ticketing systems
7. **Notifications**: Desktop notifications for critical findings
8. **Scheduling**: Schedule periodic report generation
9. **Templates**: Custom report templates
10. **Collaboration**: Share reports with team members
