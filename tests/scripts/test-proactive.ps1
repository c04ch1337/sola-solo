# Proactive Communication Test Script
# Tests the proactive backend scheduler + triggers

Write-Host "=== Proactive Communication Test ===" -ForegroundColor Cyan
Write-Host ""

# Check if .env exists
if (!(Test-Path ".env")) {
    Write-Host "ERROR: .env file not found!" -ForegroundColor Red
    Write-Host "Create a .env file with:" -ForegroundColor Yellow
    Write-Host "PROACTIVE_ENABLED=true"
    Write-Host "PROACTIVE_INTERVAL_SECS=30"
    Write-Host "PROACTIVE_SILENCE_MINUTES=1"
    Write-Host "PROACTIVE_MIN_INTERVAL_MINUTES=1"
    exit 1
}

# Check environment variables
Write-Host "Checking .env configuration..." -ForegroundColor Yellow
$envContent = Get-Content .env
$proactiveEnabled = $envContent | Select-String "PROACTIVE_ENABLED=true"
$intervalSecs = $envContent | Select-String "PROACTIVE_INTERVAL_SECS"
$silenceMins = $envContent | Select-String "PROACTIVE_CURIOSITY_THRESHOLD_MINS"

if (!$proactiveEnabled) {
    Write-Host "WARNING: PROACTIVE_ENABLED not set to true in .env" -ForegroundColor Yellow
    Write-Host "Add this line to .env: PROACTIVE_ENABLED=true"
}

Write-Host "✓ .env file found" -ForegroundColor Green
Write-Host ""

# Check if backend is running
Write-Host "Checking if backend is running..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 2
    Write-Host "✓ Backend is running" -ForegroundColor Green
} catch {
    Write-Host "ERROR: Backend not running at http://localhost:8080" -ForegroundColor Red
    Write-Host "Start backend with: cd phoenix-web && cargo run"
    exit 1
}
Write-Host ""

# Display test instructions
Write-Host "=== Test Instructions ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Backend Configuration:" -ForegroundColor Yellow
Write-Host "   - PROACTIVE_ENABLED: $($proactiveEnabled -ne $null)" -ForegroundColor White
Write-Host "   - Check backend logs for: 'Proactive communication loop started'" -ForegroundColor White
Write-Host ""

Write-Host "2. Quick Test (Fast):" -ForegroundColor Yellow
Write-Host "   Add to .env:" -ForegroundColor White
Write-Host "   PROACTIVE_ENABLED=true"
Write-Host "   PROACTIVE_INTERVAL_SECS=30"
Write-Host "   PROACTIVE_CURIOSITY_THRESHOLD_MINS=1"
Write-Host ""
Write-Host "   Then:" -ForegroundColor White
Write-Host "   a) Restart backend: cd phoenix-web && cargo run"
Write-Host "   b) Connect frontend or wscat"
Write-Host "   c) Send a message"
Write-Host "   d) Wait ~90 seconds"
Write-Host "   e) Proactive message should appear"
Write-Host ""

Write-Host "3. WebSocket Test:" -ForegroundColor Yellow
Write-Host "   Install wscat: npm install -g wscat"
Write-Host "   Connect: wscat -c ws://localhost:8080/ws"
Write-Host "   Send: {`"input`":`"Hello`"}"
Write-Host "   Wait for proactive message after silence period"
Write-Host ""

Write-Host "4. Expected Backend Logs:" -ForegroundColor Yellow
Write-Host "   INFO Proactive communication loop started (enabled=true, interval=30s, rate_limit=60s)"
Write-Host "   INFO Sending proactive message (reason: curiosity, content_preview: ...)"
Write-Host "   INFO Proactive message sent to 1 connected clients"
Write-Host ""

Write-Host "5. Expected WebSocket Message:" -ForegroundColor Yellow
Write-Host "   {" -ForegroundColor White
Write-Host "     `"type`": `"proactive_message`","
Write-Host "     `"content`": `"Dad, what part of that mattered most to you?`","
Write-Host "     `"reason`": `"curiosity`","
Write-Host "     `"timestamp`": 1234567890"
Write-Host "   }"
Write-Host ""

Write-Host "=== Environment Variables ===" -ForegroundColor Cyan
Write-Host "PROACTIVE_ENABLED=true/false (default: false)" -ForegroundColor White
Write-Host "PROACTIVE_INTERVAL_SECS=60 (default: 60)" -ForegroundColor White
Write-Host "PROACTIVE_RATE_LIMIT_SECS=600 (default: 600 = 10 min)" -ForegroundColor White
Write-Host "PROACTIVE_CURIOSITY_THRESHOLD_MINS=10 (default: 10)" -ForegroundColor White
Write-Host ""

Write-Host "=== Chat Commands ===" -ForegroundColor Cyan
Write-Host "proactive status - Check if enabled and see settings" -ForegroundColor White
Write-Host "proactive on - Instructions to enable" -ForegroundColor White
Write-Host "proactive off - Instructions to disable" -ForegroundColor White
Write-Host ""

Write-Host "✓ Test script complete!" -ForegroundColor Green
Write-Host "Follow the instructions above to test proactive communication." -ForegroundColor Cyan
