# Sola AGI - Environment Configuration Guide

## 🚀 Quick Start

### 1. Run the Setup Script

```powershell
.\setup-env.ps1
```

This creates a `.env` file with all configuration options documented.

### 2. Add Your OpenRouter API Key

1. Get your key from https://openrouter.ai/keys
2. Open `.env` in your text editor
3. Find the line: `OPENROUTER_API_KEY=`
4. Add your key: `OPENROUTER_API_KEY=sk-or-v1-your-actual-key`
5. Save the file

### 3. Start the Backend

```powershell
cd phoenix-web
cargo run --release
```

Wait for: **"LLM Provider: OpenRouter (500+ models available)"**

### 4. Start the Frontend

In a new terminal:

```powershell
cd frontend_desktop
npm run dev
```

### 5. Open Sola AGI

Navigate to http://localhost:3000

You should see **"OpenRouter CONNECTED"** ✅

---

## 📋 Configuration Categories

### Required Settings

| Variable | Description | Get It From |
|----------|-------------|-------------|
| `OPENROUTER_API_KEY` | Your OpenRouter API key | https://openrouter.ai/keys |

### Core Settings

| Variable | Default | Description |
|----------|---------|-------------|
| `PHOENIX_NAME` | `Sola` | Your AGI's name |
| `USER_NAME` | `User` | Your display name |
| `PHOENIX_WEB_BIND` | `127.0.0.1:8888` | Backend address |
| `LLM_PROVIDER` | `openrouter` | LLM provider choice |
| `DEFAULT_LLM_MODEL` | `anthropic/claude-3.5-sonnet` | Primary model |
| `FALLBACK_LLM_MODEL` | `openai/gpt-4o-mini` | Backup model |

### Recommended Models

**Best Quality (Most Expensive)**
- `anthropic/claude-3.5-sonnet` - Best reasoning and coding
- `openai/gpt-4-turbo` - Very capable, good for complex tasks

**Balanced (Good Quality, Affordable)**
- `openai/gpt-4o-mini` - Fast, cheap, very capable
- `deepseek/deepseek-v3.2` - Excellent quality, very inexpensive
- `anthropic/claude-3-haiku` - Fast responses, good quality

**Budget (Cheapest)**
- `meta-llama/llama-3.1-8b-instruct:free` - Free tier
- `google/gemini-flash-1.5` - Fast and cheap

### Optional Features

Enable these by setting to `true` in your `.env`:

| Feature | Variable | Description |
|---------|----------|-------------|
| Voice I/O | `VOICE_ENABLED=true` | Text-to-speech and speech-to-text |
| Proactive Mode | `PROACTIVE_ENABLED=true` | Sola initiates conversations |
| Desktop Capture | `DESKTOP_CAPTURE_ENABLED=true` | Screen analysis |
| Outlook Integration | `OUTLOOK_COM_ENABLED=true` | Windows Outlook COM (Windows only) |
| Home Automation | `HOME_AUTOMATION_ENABLED=true` | Smart home control |

### GitHub Integration

For repository management features:

```bash
GITHUB_PAT=your_github_personal_access_token
GITHUB_USERNAME=your_username
```

Create a token at: https://github.com/settings/tokens

Required scopes: `repo`, `admin:org`, `workflow`

### Google Integration

For Gmail, Drive, and Calendar access:

```bash
GOOGLE_OAUTH_CLIENT_ID=your_client_id
GOOGLE_OAUTH_CLIENT_SECRET=your_client_secret
GOOGLE_OAUTH_REDIRECT_URL=http://127.0.0.1:8888/api/google/oauth2/callback
```

Get credentials from: https://console.cloud.google.com/apis/credentials

### Voice Configuration

#### Text-to-Speech (TTS)

**Coqui TTS (Offline, Free)**
```bash
TTS_ENGINE=coqui
COQUI_MODEL_PATH=./models/coqui/tts_model.pth
```

**ElevenLabs (Cloud, High Quality)**
```bash
TTS_ENGINE=elevenlabs
ELEVENLABS_API_KEY=your_key
ELEVENLABS_VOICE_ID=your_voice_id
```

#### Speech-to-Text (STT)

**Whisper (Accurate, Offline)**
```bash
STT_ENGINE=whisper
WHISPER_MODEL_PATH=./models/whisper/base.en
```

Model sizes: `tiny`, `base`, `small`, `medium`, `large`

---

## 🔧 Advanced Configuration

### Using Ollama (Local GPU)

Instead of OpenRouter, run models locally:

```bash
LLM_PROVIDER=ollama
OLLAMA_BASE_URL=http://127.0.0.1:11434
OLLAMA_MODEL=llama3.1:8b
```

Requirements:
- Ollama installed: https://ollama.ai/download
- Run: `ollama pull llama3.1:8b`
- Start: `ollama serve`

### Memory & Vector Search

```bash
VECTOR_KB_ENABLED=true
VECTOR_DB_PATH=./data/vector_db
EMBEDDING_MODEL=all-MiniLM-L6-v2
VECTOR_SEARCH_TOP_K=5
```

Better quality (slower):
```bash
EMBEDDING_MODEL=all-mpnet-base-v2
```

### Security & Access Control

```bash
SYSTEM_ACCESS_TIER1_ENABLED=true   # Read files, list processes
SYSTEM_ACCESS_TIER2_ENABLED=false  # Write files, execute commands
```

**Warning**: Tier 2 requires per-connection consent and allows system modifications.

### Proactive Communication

```bash
PROACTIVE_ENABLED=true
PROACTIVE_INTERVAL_SECS=60              # Check every 60 seconds
PROACTIVE_SILENCE_MINUTES=10            # Message after 10 min silence
PROACTIVE_MIN_INTERVAL_MINUTES=10       # Min 10 min between messages
```

---

## 🐛 Troubleshooting

### "OPENROUTER_API_KEY not found"

1. Make sure `.env` exists in project root
2. Check the file is named exactly `.env` (not `.env.txt`)
3. Verify your API key has no extra spaces
4. Restart the backend

### Backend won't start

```powershell
# Clean build
cd phoenix-web
cargo clean
cargo build --release
cargo run --release
```

### Frontend shows "OpenRouter not connected"

1. Verify backend is running on port 8888
2. Test: http://localhost:8888/health
3. Should return: `{"status":"ok"}`
4. Check browser console for errors

### "Invalid API key" errors

1. Verify key at https://openrouter.ai/keys
2. Make sure you copied the full key including `sk-or-v1-` prefix
3. Check for typos
4. Generate a new key if needed

### Port already in use

Change the port in `.env`:
```bash
PHOENIX_WEB_BIND=127.0.0.1:8889  # Use different port
```

Then update frontend:
```bash
VITE_PHOENIX_API_URL=http://localhost:8889
```

---

## 📖 Additional Resources

- **OpenRouter Documentation**: https://openrouter.ai/docs
- **Available Models**: https://openrouter.ai/models
- **Pricing**: https://openrouter.ai/pricing
- **Main README**: `README.md`
- **Quick Setup**: `CREATE_ENV_FILE.md`
- **Detailed Guide**: `OPENROUTER_SETUP_GUIDE.md`

---

## 💡 Tips

1. **Start with defaults** - The setup script provides good defaults
2. **Use fallback models** - Set a cheaper fallback for reliability
3. **Monitor costs** - Check OpenRouter dashboard regularly
4. **Test locally first** - Use Ollama for development if you have a GPU
5. **Enable features gradually** - Start simple, add features as needed

---

## 🆘 Getting Help

If you encounter issues:

1. Check the troubleshooting section above
2. Review the logs in the backend terminal
3. Check the browser console for frontend errors
4. Ensure all required dependencies are installed
5. Verify your `.env` file syntax (no smart quotes, proper encoding)

---

## ✅ Verification Checklist

- [ ] `.env` file created
- [ ] `OPENROUTER_API_KEY` added
- [ ] Backend starts without errors
- [ ] Backend shows "LLM Provider: OpenRouter"
- [ ] Frontend starts on port 3000
- [ ] http://localhost:3000 opens
- [ ] Status shows "OpenRouter CONNECTED"
- [ ] Can send a test message
- [ ] Receive a response from Sola

If all items are checked, you're ready to go! 🎉
