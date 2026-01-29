# ===================================================================
# Sola AGI - Environment Setup Script
# ===================================================================
# This script creates a .env file with recommended defaults
# Run: .\setup-env.ps1
# ===================================================================

$envContent = @"
# ===================================================================
# Consumer-Ready AGI Configuration Template (Sola AGI)
# ===================================================================
# Copy this file to .env and customize for your deployment
# All values below are defaults used throughout the codebase
# ===================================================================

# ===================================================================
# AGI Identity & Branding
# ===================================================================
PHOENIX_NAME=Sola
# The name of your AGI assistant (used in UI, logs, prompts, voice)
# Examples: "Sola", "Atlas", "Nova", "Echo", "Lumina"

USER_NAME=User
# Default display name for the user (if not provided elsewhere)
# Examples: "User", "Alex", "Partner", "Friend"

APP_TITLE=Sola AGI
# Window title and tray tooltip (Tauri)

APP_ICON_PATH=./assets/icon.png
# Relative path to 1024x1024 PNG icon (used by Tauri for tray/window)

# ===================================================================
# Core Backend Configuration
# ===================================================================
PHOENIX_WEB_BIND=127.0.0.1:8888
# Backend HTTP/WebSocket bind address
# Use 0.0.0.0:8888 for LAN access (careful with security)
# Use 127.0.0.1:8888 for local-only (recommended default)

# ===================================================================
# LLM Configuration (Required)
# ===================================================================
LLM_PROVIDER=openrouter
# Options: openrouter (default), ollama

# OpenRouter (recommended for most users)
OPENROUTER_API_KEY=
# Get your key at https://openrouter.ai/keys
# REQUIRED for LLM functionality - ADD YOUR KEY HERE!

DEFAULT_LLM_MODEL=anthropic/claude-3.5-sonnet
# Recommended models (best quality to fastest/cheapest):
# - anthropic/claude-3.5-sonnet (best reasoning)
# - openai/gpt-4o-mini (fast and cheap)
# - deepseek/deepseek-v3.2 (very good and inexpensive)
# - meta-llama/llama-3.1-405b-instruct (largest open model on OpenRouter)

FALLBACK_LLM_MODEL=openai/gpt-4o-mini
# Fallback model if default fails

# Ollama (local GPU/CPU - alternative to OpenRouter)
# OLLAMA_BASE_URL=http://127.0.0.1:11434
# OLLAMA_MODEL=llama3.1:8b

TEMPERATURE=0.7
# Creativity vs determinism (0.0 = deterministic, 1.0 = creative)

MAX_TOKENS=8192
# Max tokens per response (adjust based on model context window)

# ===================================================================
# Memory & Vector Search
# ===================================================================
VECTOR_KB_ENABLED=true
# Enable semantic search/embeddings (highly recommended)

VECTOR_DB_PATH=./data/vector_db
# Local path for vector database storage

EMBEDDING_MODEL=all-MiniLM-L6-v2
# Default small/fast model
# Alternatives (larger/better):
# - all-mpnet-base-v2 (better quality, ~4x slower)
# - BAAI/bge-small-en-v1.5 (good multilingual)

VECTOR_SEARCH_TOP_K=5
# Number of similar memories returned by default

VECTOR_DISTANCE_METRIC=cosine
# Options: cosine (default), euclidean, dot

# ===================================================================
# Browser Control (CDP)
# ===================================================================
BROWSER_TYPE=chrome
# Options: chrome, edge, firefox (chrome/edge fully supported via CDP)

BROWSER_DEBUG_PORT=9222
# Port for browser remote debugging protocol

# ===================================================================
# Voice & Audio (TTS/STT)
# ===================================================================
VOICE_ENABLED=false
# Enable voice input/output by default (can toggle in UI/chat)

TTS_ENGINE=coqui
# Options: coqui (offline), elevenlabs (cloud, high quality)

# Coqui TTS (offline)
COQUI_MODEL_PATH=./models/coqui/tts_model.pth
# Path to Coqui TTS model (auto-downloaded if missing)

# ElevenLabs TTS (cloud)
ELEVENLABS_API_KEY=
ELEVENLABS_VOICE_ID=

STT_ENGINE=whisper
# Options: whisper (accurate, offline), vosk (fast, offline)

WHISPER_MODEL_PATH=./models/whisper/base.en
# Path to Whisper model (tiny/base/small/medium/large)

# ===================================================================
# Security & Access Control
# ===================================================================
SYSTEM_ACCESS_TIER1_ENABLED=true
# Tier 1: Safe operations (read files, list processes)

SYSTEM_ACCESS_TIER2_ENABLED=false
# Tier 2: Privileged operations (write files, execute commands)
# Requires per-connection consent via WebSocket

# ===================================================================
# Proactive Communication
# ===================================================================
PROACTIVE_ENABLED=false
# Enable Sola to initiate messages autonomously

PROACTIVE_INTERVAL_SECS=60
# How often the scheduler checks for triggers

PROACTIVE_SILENCE_MINUTES=10
# Silence threshold to trigger check-in

PROACTIVE_MIN_INTERVAL_MINUTES=10
# Minimum time between proactive messages (rate limit)

# ===================================================================
# Optional Features (toggle to enable)
# ===================================================================
AUDIO_INTELLIGENCE_ENABLED=false
DESKTOP_CAPTURE_ENABLED=false
WIFI_ANALYSIS_ENABLED=false
BLUETOOTH_SNIFFER_ENABLED=false
HOME_AUTOMATION_ENABLED=false
OUTLOOK_COM_ENABLED=false
# Windows only: Outlook COM integration

# ===================================================================
# GitHub Integration (Optional)
# ===================================================================
GITHUB_PAT=
# GitHub Personal Access Token for repository management
# Create at: https://github.com/settings/tokens
# Required scopes: repo, admin:org, workflow

GITHUB_USERNAME=
# Your GitHub username

# ===================================================================
# Google OAuth Integration (Optional)
# ===================================================================
GOOGLE_OAUTH_CLIENT_ID=
# Google OAuth Client ID for Gmail, Drive, Calendar access
# Get from: https://console.cloud.google.com/apis/credentials

GOOGLE_OAUTH_CLIENT_SECRET=
# Google OAuth Client Secret

GOOGLE_OAUTH_REDIRECT_URL=http://127.0.0.1:8888/api/google/oauth2/callback
# OAuth redirect URL (must match Google Console settings)

# ===================================================================
# Logging & Debug
# ===================================================================
RUST_LOG=info
# Options: error, warn, info, debug, trace

# ===================================================================
# Advanced / Custom Paths
# ===================================================================
PHOENIX_DOTENV_PATH=.env
# Override .env file location

SKILLS_FOLDER=./skills
# Path to custom skills library

ECOSYSTEM_REPOS=./ecosystem_repos
# Path for ecosystem manager repos

AGENTS_FOLDER=./agents
# Path for spawned agent repos (if local)

# ===================================================================
# Frontend Configuration
# ===================================================================
VITE_PORT=3000
# Frontend development server port

VITE_PHOENIX_API_URL=http://localhost:8888
# Backend API URL for frontend to connect to

# ===================================================================
# Developer / Debug (optional)
# ===================================================================
DEV_MODE=false
# Enable developer features (debug logs, verbose output)

ANALYTICS_ENABLED=false
# Opt-in anonymous usage stats (helps improve Sola)

# ===================================================================
# Relationship & Personality (Advanced)
# ===================================================================
RELATIONSHIP_TEMPLATE=Companion
# Options: IntimatePartnership, Professional, Companion, Friend

PARTNER_MODE_ENABLED=false
# Enable relationship-aware responses

PARTNER_AFFECTION_LEVEL=0.5
# Affection level (0.0 to 1.0)

# ===================================================================
# Master Orchestration (Multi-Instance)
# ===================================================================
ORCH_MASTER_MODE=false
# Enable cross-ORCH coordination (advanced)

# ===================================================================
# Telemetry (Optional)
# ===================================================================
TELEMETRY_ENABLED=false
# Enable telemetry collection for system optimization

TELEMETRY_COLLECTOR_URL=http://127.0.0.1:7242/ingest
# Telemetry collector URL

# ===================================================================
# QUICK START INSTRUCTIONS
# ===================================================================
# 1. Add your OPENROUTER_API_KEY above (get it at https://openrouter.ai/keys)
# 2. Save this file
# 3. Start backend: cd phoenix-web && cargo run --release
# 4. Start frontend: cd frontend_desktop && npm run dev
# 5. Open http://localhost:3000
# ===================================================================
"@

# Check if .env already exists
if (Test-Path ".env") {
    Write-Host "WARNING: .env file already exists!" -ForegroundColor Yellow
    $response = Read-Host "Do you want to overwrite it? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-Host "Setup cancelled. Existing .env file preserved." -ForegroundColor Red
        exit 0
    }
    Write-Host "Backing up existing .env to .env.backup..." -ForegroundColor Cyan
    Copy-Item ".env" ".env.backup" -Force
}

# Create .env file
$envContent | Out-File -FilePath ".env" -Encoding UTF8 -NoNewline

Write-Host ""
Write-Host "SUCCESS: .env file created successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "NEXT STEPS:" -ForegroundColor Cyan
Write-Host "1. Get your OpenRouter API key from: https://openrouter.ai/keys" -ForegroundColor White
Write-Host "2. Edit .env and add your OPENROUTER_API_KEY" -ForegroundColor White
Write-Host "3. Start the backend:" -ForegroundColor White
Write-Host "   cd phoenix-web" -ForegroundColor Gray
Write-Host "   cargo run --release" -ForegroundColor Gray
Write-Host "4. Start the frontend (in new terminal):" -ForegroundColor White
Write-Host "   cd frontend_desktop" -ForegroundColor Gray
Write-Host "   npm run dev" -ForegroundColor Gray
Write-Host "5. Open http://localhost:3000" -ForegroundColor White
Write-Host ""

# Offer to open .env in editor
$openEditor = Read-Host "Would you like to open .env in your default editor now? (Y/n)"
if ($openEditor -ne "n" -and $openEditor -ne "N") {
    Start-Process ".env"
}

Write-Host ""
Write-Host "For more help, see OPENROUTER_SETUP_GUIDE.md" -ForegroundColor Cyan
