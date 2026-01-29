# Quick Start: Create Your .env File

## What You Need

Your OpenRouter API key from https://openrouter.ai/keys

## Option 1: Use the Setup Script (Recommended)

```powershell
.\setup-env.ps1
```

This will:
- Create a comprehensive `.env` file with all options
- Prompt you to open it in your editor
- Show you next steps

Then just edit the file and add your OpenRouter API key.

## Option 2: Create Minimal .env Manually

Open PowerShell in this directory and run:

```powershell
@"
# REQUIRED
OPENROUTER_API_KEY=sk-or-v1-YOUR-KEY-HERE

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
"@ | Out-File -FilePath .env -Encoding UTF8
```

## Edit Your API Key

1. Open the newly created `.env` file in a text editor
2. Replace `sk-or-v1-YOUR-KEY-HERE` with your actual OpenRouter API key
3. Save the file

## Start the Backend

```powershell
cd phoenix-web
cargo run --release
```

You should see: **"LLM Provider: OpenRouter (500+ models available)"**

## Start the Frontend

In a new terminal:

```powershell
cd frontend_desktop
npm run dev
```

Then open http://localhost:3000

The status should now show **"OpenRouter CONNECTED"**
