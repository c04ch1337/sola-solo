# Malware Sandbox Agent Test Script
# Tests the Malware Sandbox Agent API endpoints

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Malware Sandbox Agent Test Suite" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$baseUrl = "http://localhost:8888/api/sandbox"
$testsPassed = 0
$testsFailed = 0

# Function to make API request
function Invoke-ApiTest {
    param(
        [string]$Method,
        [string]$Endpoint,
        [object]$Body = $null,
        [string]$Description
    )
    
    Write-Host "Testing: $Description" -ForegroundColor Yellow
    Write-Host "  → $Method $Endpoint" -ForegroundColor Gray
    
    try {
        $params = @{
            Method = $Method
            Uri = "$baseUrl$Endpoint"
            ContentType = "application/json"
            ErrorAction = "Stop"
        }
        
        if ($Body) {
            $params.Body = ($Body | ConvertTo-Json -Depth 10)
        }
        
        $response = Invoke-RestMethod @params
        Write-Host "  ✓ SUCCESS" -ForegroundColor Green
        $script:testsPassed++
        return $response
    }
    catch {
        Write-Host "  ✗ FAILED: $($_.Exception.Message)" -ForegroundColor Red
        $script:testsFailed++
        return $null
    }
}

# Test 1: Check sandbox status
Write-Host ""
Write-Host "[Test 1] Checking Sandbox Status..." -ForegroundColor Cyan
$status = Invoke-ApiTest -Method "GET" -Endpoint "/status" -Description "Get sandbox status"
if ($status) {
    Write-Host "  Agent Name: $($status.agent_name)" -ForegroundColor White
    Write-Host "  Enabled: $($status.enabled)" -ForegroundColor White
    Write-Host "  Capabilities: $($status.capabilities.Count)" -ForegroundColor White
}

# Test 2: Create session
Write-Host ""
Write-Host "[Test 2] Creating Sandbox Session..." -ForegroundColor Cyan
$session = Invoke-ApiTest -Method "POST" -Endpoint "/session/create" -Body @{} -Description "Create new session"
if ($session) {
    $sessionId = $session.session_id
    Write-Host "  Session ID: $sessionId" -ForegroundColor White
}

# Test 3: Upload test file
if ($sessionId) {
    Write-Host ""
    Write-Host "[Test 3] Uploading Test File..." -ForegroundColor Cyan
    
    # Create a test file
    $testContent = "This is a test file for malware sandbox testing."
    $testBytes = [System.Text.Encoding]::UTF8.GetBytes($testContent)
    $testBase64 = [Convert]::ToBase64String($testBytes)
    
    $uploadBody = @{
        session_id = $sessionId
        file_name = "test.txt"
        file_data_base64 = $testBase64
    }
    
    $upload = Invoke-ApiTest -Method "POST" -Endpoint "/upload" -Body $uploadBody -Description "Upload test file"
    if ($upload) {
        $fileId = $upload.file_id
        Write-Host "  File ID: $fileId" -ForegroundColor White
        Write-Host "  File Size: $($upload.file_size) bytes" -ForegroundColor White
    }
}

# Test 4: List files
if ($sessionId) {
    Write-Host ""
    Write-Host "[Test 4] Listing Files..." -ForegroundColor Cyan
    $listBody = @{
        session_id = $sessionId
    }
    $files = Invoke-ApiTest -Method "POST" -Endpoint "/files/list" -Body $listBody -Description "List session files"
    if ($files) {
        Write-Host "  Files Count: $($files.count)" -ForegroundColor White
    }
}

# Test 5: List playbooks
Write-Host ""
Write-Host "[Test 5] Listing Playbooks..." -ForegroundColor Cyan
$playbooks = Invoke-ApiTest -Method "GET" -Endpoint "/playbooks" -Description "Get available playbooks"
if ($playbooks) {
    Write-Host "  Playbooks Count: $($playbooks.count)" -ForegroundColor White
    foreach ($playbook in $playbooks.playbooks) {
        Write-Host "    • $($playbook.name)" -ForegroundColor Gray
    }
}

# Test 6: Analyze file (if uploaded)
if ($sessionId -and $fileId) {
    Write-Host ""
    Write-Host "[Test 6] Analyzing File..." -ForegroundColor Cyan
    $analyzeBody = @{
        session_id = $sessionId
        file_id = $fileId
    }
    $analysis = Invoke-ApiTest -Method "POST" -Endpoint "/analyze" -Body $analyzeBody -Description "Analyze uploaded file"
    if ($analysis) {
        Write-Host "  Threat Level: $($analysis.analysis.threat_level)" -ForegroundColor White
        Write-Host "  MITRE Techniques: $($analysis.analysis.mitre_techniques.Count)" -ForegroundColor White
        Write-Host "  Behavioral Indicators: $($analysis.analysis.behavioral_indicators.Count)" -ForegroundColor White
    }
}

# Test 7: Clear session
if ($sessionId) {
    Write-Host ""
    Write-Host "[Test 7] Clearing Session..." -ForegroundColor Cyan
    $clearBody = @{
        session_id = $sessionId
    }
    $clear = Invoke-ApiTest -Method "POST" -Endpoint "/clear" -Body $clearBody -Description "Clear session files"
}

# Summary
Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Test Summary" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Tests Passed: $testsPassed" -ForegroundColor Green
Write-Host "Tests Failed: $testsFailed" -ForegroundColor $(if ($testsFailed -gt 0) { "Red" } else { "Green" })
Write-Host ""

if ($testsFailed -eq 0) {
    Write-Host "✓ All tests passed!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ Some tests failed. Check configuration and ensure:" -ForegroundColor Red
    Write-Host "  1. Phoenix server is running (cargo run --bin pagi-sola-web)" -ForegroundColor White
    Write-Host "  2. MALWARE_SANDBOX_ENABLED=true in .env" -ForegroundColor White
    Write-Host "  3. Sandbox directory exists and is writable" -ForegroundColor White
    exit 1
}
