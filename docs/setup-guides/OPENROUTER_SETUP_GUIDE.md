# OpenRouter Connection Setup Guide

## Issue
The frontend shows **"OpenRouter not connected"** because:
1. No `.env` file exists in the project root
2. The `OPENROUTER_API_KEY` environment variable is not configured
3. The backend requires this API key to function

## Solution

### Step 1: Get an OpenRouter API Key

1. Visit https://openrouter.ai/keys
2. Sign up or log in to your account
3. Click **"Create Key"**
4. Copy your new API key (it will look like: `sk-or-v1-...`)

### Step 2: Create a `.env` File

You have two options:

#### Option A: Use the Setup Script (Recommended)

Run the PowerShell setup script in the project root:

```powershell
cd C:\Users\JAMEYMILNER\AppData\Local\pagi-twin-desktop
.\setup-env.ps1
```

This will create a `.env` file with all recommended settings and open it in your default editor.

#### Option B: Create Manually

Create a file named `.env` in the project root directory with at minimum:

```bash
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

# Memory
VECTOR_KB_ENABLED=true
VECTOR_DB_PATH=./data/vector_db
```

**Important**: Replace `sk-or-v1-YOUR-KEY-HERE` with your actual OpenRouter API key!

For a complete configuration with all available options, run `.\setup-env.ps1` or see the comprehensive template in the script.

### Step 3: Restart the Backend

After creating the `.env` file, restart the Phoenix backend:

#### Option A: Using PowerShell

```powershell
# Navigate to phoenix-web directory
cd C:\Users\JAMEYMILNER\AppData\Local\pagi-twin-desktop\phoenix-web

# Run the backend
cargo run --release
```

#### Option B: Using the provided launcher

```powershell
cd C:\Users\JAMEYMILNER\AppData\Local\pagi-twin-desktop
.\launcher.cmd
```

### Step 4: Verify Connection

1. Wait for the backend to start (you should see "LLM Provider: OpenRouter (500+ models available)")
2. Open the frontend at http://localhost:3000
3. The status should now show **"OpenRouter CONNECTED"**

## Troubleshooting

### Backend fails to start with "OPENROUTER_API_KEY not found"
- Make sure the `.env` file is in the project root directory
- Verify the file is named exactly `.env` (not `.env.txt` or similar)
- Check that your API key is correctly pasted without extra spaces

### Backend starts but frontend shows "OpenRouter not connected"
- Check if the backend is running on port 8888
- Open http://localhost:8888/health in your browser
- You should see: `{"status":"ok"}`

### "Invalid API key" errors
- Verify your OpenRouter API key at https://openrouter.ai/keys
- Make sure you copied the complete key including the `sk-or-v1-` prefix
- Regenerate a new key if needed

## Alternative: Use Environment Variables Directly

If you prefer not to use a `.env` file, you can set the environment variable directly in PowerShell:

```powershell
$env:OPENROUTER_API_KEY = "sk-or-v1-YOUR-KEY-HERE"
cd phoenix-web
cargo run --release
```

Note: This only sets the variable for the current PowerShell session.

## Next Steps

Once OpenRouter is connected, you can:
- Chat with Phoenix AGI through the frontend
- Execute commands using the `/api/command` endpoint
- Use voice features (if configured)
- Manage your projects and workflows

## Need Help?

- OpenRouter Documentation: https://openrouter.ai/docs
- OpenRouter Models List: https://openrouter.ai/models
- OpenRouter Pricing: https://openrouter.ai/pricing

## Additional Configuration

For additional features like GitHub integration, Google OAuth, or browser automation, add these to your `.env`:

```bash
# GitHub Integration (optional)
GITHUB_PAT=your_github_personal_access_token

# Google OAuth (optional)
GOOGLE_OAUTH_CLIENT_ID=your_client_id
GOOGLE_OAUTH_CLIENT_SECRET=your_client_secret

# Browser Automation (optional)
BROWSER_TYPE=chrome
BROWSER_DEBUG_PORT=9222
```

See the full environment variable documentation in `README.md` for all available options.
