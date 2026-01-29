# Quick Setup - Minimal .env file for Sola AGI
# This creates a minimal .env file to get started quickly

Write-Host ""
Write-Host "Sola AGI - Quick Setup" -ForegroundColor Cyan
Write-Host "======================" -ForegroundColor Cyan
Write-Host ""

# Check if .env already exists
if (Test-Path ".env") {
    Write-Host "WARNING: .env file already exists!" -ForegroundColor Yellow
    $response = Read-Host "Overwrite it? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-Host "Cancelled. Use .\setup-env.ps1 for full configuration." -ForegroundColor Yellow
        exit 0
    }
}

# Prompt for OpenRouter API key
Write-Host "Get your API key from: https://openrouter.ai/keys" -ForegroundColor Cyan
Write-Host ""
$apiKey = Read-Host "Enter your OpenRouter API key (or press Enter to add later)"

if ([string]::IsNullOrWhiteSpace($apiKey)) {
    $apiKey = ""
    Write-Host ""
    Write-Host "NOTE: You must add your API key to .env before starting!" -ForegroundColor Yellow
    Write-Host ""
}

# Create minimal .env
$minimalEnv = @"
# ===================================================================
# Sola AGI - Minimal Configuration
# ===================================================================

# REQUIRED: OpenRouter API Key
OPENROUTER_API_KEY=$apiKey

# LLM Configuration
LLM_PROVIDER=openrouter
DEFAULT_LLM_MODEL=anthropic/claude-3.5-sonnet
FALLBACK_LLM_MODEL=openai/gpt-4o-mini

# Backend Configuration
PHOENIX_WEB_BIND=127.0.0.1:8888

# AGI Identity
PHOENIX_NAME=Sola
USER_NAME=User

# Memory Configuration
VECTOR_KB_ENABLED=true
VECTOR_DB_PATH=./data/vector_db

# ===================================================================
# For full configuration options, run: .\setup-env.ps1
# ===================================================================
"@

$minimalEnv | Out-File -FilePath ".env" -Encoding UTF8 -NoNewline

Write-Host "SUCCESS: Minimal .env file created!" -ForegroundColor Green
Write-Host ""
Write-Host "NEXT STEPS:" -ForegroundColor Cyan
Write-Host ""

if ([string]::IsNullOrWhiteSpace($apiKey)) {
    Write-Host "1. Edit .env and add your OPENROUTER_API_KEY" -ForegroundColor Yellow
    Write-Host "2. Start backend: cd phoenix-web; cargo run --release" -ForegroundColor White
} else {
    Write-Host "1. Start backend: cd phoenix-web; cargo run --release" -ForegroundColor White
}

Write-Host "2. Start frontend: cd frontend_desktop; npm run dev" -ForegroundColor White
Write-Host "3. Open http://localhost:3000" -ForegroundColor White
Write-Host ""
Write-Host "For advanced configuration, run: .\setup-env.ps1" -ForegroundColor Cyan
Write-Host ""
