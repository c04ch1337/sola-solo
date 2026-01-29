# Malware Sandbox Agent Setup Script
# This script configures the Malware Sandbox Agent for Phoenix AGI

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Malware Sandbox Agent Setup" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Check if .env file exists
$envFile = ".env"
if (-not (Test-Path $envFile)) {
    Write-Host "Creating .env file..." -ForegroundColor Yellow
    New-Item -Path $envFile -ItemType File | Out-Null
}

# Function to update or add environment variable in .env
function Set-EnvVariable {
    param(
        [string]$Key,
        [string]$Value
    )
    
    $content = Get-Content $envFile -ErrorAction SilentlyContinue
    $found = $false
    
    $newContent = $content | ForEach-Object {
        if ($_ -match "^$Key=") {
            $found = $true
            "$Key=$Value"
        } else {
            $_
        }
    }
    
    if (-not $found) {
        $newContent += "$Key=$Value"
    }
    
    $newContent | Set-Content $envFile
}

# Enable Malware Sandbox Agent
Write-Host "Configuring Malware Sandbox Agent..." -ForegroundColor Green
Set-EnvVariable "MALWARE_SANDBOX_ENABLED" "true"

# Sandbox configuration
Write-Host ""
Write-Host "Sandbox Configuration:" -ForegroundColor Yellow
$sandboxPath = Read-Host "Sandbox path (default: ./data/sandbox)"
if ([string]::IsNullOrWhiteSpace($sandboxPath)) {
    $sandboxPath = "./data/sandbox"
}
Set-EnvVariable "SANDBOX_PATH" $sandboxPath

$maxFileSize = Read-Host "Max file size in MB (default: 50)"
if ([string]::IsNullOrWhiteSpace($maxFileSize)) {
    $maxFileSize = "50"
}
Set-EnvVariable "SANDBOX_MAX_FILE_SIZE_MB" $maxFileSize

$maxTotalSize = Read-Host "Max total size in MB (default: 500)"
if ([string]::IsNullOrWhiteSpace($maxTotalSize)) {
    $maxTotalSize = "500"
}
Set-EnvVariable "SANDBOX_MAX_TOTAL_SIZE_MB" $maxTotalSize

$cleanupDays = Read-Host "Cleanup after days (default: 7)"
if ([string]::IsNullOrWhiteSpace($cleanupDays)) {
    $cleanupDays = "7"
}
Set-EnvVariable "SANDBOX_CLEANUP_DAYS" $cleanupDays

# VirusTotal API configuration
Write-Host ""
Write-Host "VirusTotal Integration:" -ForegroundColor Yellow
Write-Host "Get your free API key at: https://www.virustotal.com/gui/join-us" -ForegroundColor Cyan
$vtApiKey = Read-Host "VirusTotal API key (leave empty to skip)"
if (-not [string]::IsNullOrWhiteSpace($vtApiKey)) {
    Set-EnvVariable "VIRUSTOTAL_API_KEY" $vtApiKey
    Set-EnvVariable "VIRUSTOTAL_ENABLED" "true"
    Write-Host "  ✓ VirusTotal integration enabled" -ForegroundColor Green
} else {
    Write-Host "  ⊘ VirusTotal integration skipped" -ForegroundColor Gray
}

# Network Security Agent collaboration
Write-Host ""
Write-Host "Network Security Agent Collaboration:" -ForegroundColor Yellow
$enableNSA = Read-Host "Enable NSA collaboration? (y/n, default: y)"
if ($enableNSA -eq "" -or $enableNSA -eq "y" -or $enableNSA -eq "Y") {
    Set-EnvVariable "NETWORK_SECURITY_AGENT_ENABLED" "true"
    Write-Host "  ✓ NSA collaboration enabled" -ForegroundColor Green
} else {
    Write-Host "  ⊘ NSA collaboration disabled" -ForegroundColor Gray
}

# Create sandbox directory
Write-Host ""
Write-Host "Creating sandbox directory..." -ForegroundColor Green
if (-not (Test-Path $sandboxPath)) {
    New-Item -Path $sandboxPath -ItemType Directory -Force | Out-Null
    Write-Host "  ✓ Created: $sandboxPath" -ForegroundColor Green
} else {
    Write-Host "  ✓ Directory already exists: $sandboxPath" -ForegroundColor Green
}

# Summary
Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Configuration Summary:" -ForegroundColor Yellow
Write-Host "  • Malware Sandbox: ENABLED" -ForegroundColor Green
Write-Host "  • Sandbox Path: $sandboxPath" -ForegroundColor White
Write-Host "  • Max File Size: $maxFileSize MB" -ForegroundColor White
Write-Host "  • Max Total Size: $maxTotalSize MB" -ForegroundColor White
Write-Host "  • Cleanup After: $cleanupDays days" -ForegroundColor White
if (-not [string]::IsNullOrWhiteSpace($vtApiKey)) {
    Write-Host "  • VirusTotal: ENABLED" -ForegroundColor Green
} else {
    Write-Host "  • VirusTotal: DISABLED" -ForegroundColor Gray
}
if ($enableNSA -eq "" -or $enableNSA -eq "y" -or $enableNSA -eq "Y") {
    Write-Host "  • NSA Collaboration: ENABLED" -ForegroundColor Green
} else {
    Write-Host "  • NSA Collaboration: DISABLED" -ForegroundColor Gray
}
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host "  1. Build the project: cargo build --release" -ForegroundColor White
Write-Host "  2. Start Phoenix: cargo run --bin pagi-sola-web" -ForegroundColor White
Write-Host "  3. Test sandbox: scripts\test\test-sandbox.ps1" -ForegroundColor White
Write-Host ""
Write-Host "API Endpoints:" -ForegroundColor Yellow
Write-Host "  • GET  /api/sandbox/status" -ForegroundColor White
Write-Host "  • POST /api/sandbox/session/create" -ForegroundColor White
Write-Host "  • POST /api/sandbox/upload" -ForegroundColor White
Write-Host "  • POST /api/sandbox/analyze" -ForegroundColor White
Write-Host "  • POST /api/sandbox/scan/quick" -ForegroundColor White
Write-Host "  • GET  /api/sandbox/playbooks" -ForegroundColor White
Write-Host ""
