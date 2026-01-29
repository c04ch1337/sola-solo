# Sola AGI - Quick Start

## 🚀 Three Steps to Launch

### 1️⃣ Create .env File

**Option A - Quick (1 minute):**
```powershell
.\quick-setup.ps1
```

**Option B - Full (recommended):**
```powershell
.\setup-env.ps1
```

Then add your OpenRouter API key from https://openrouter.ai/keys

### 2️⃣ Start Backend

```powershell
cd phoenix-web
cargo run --release
```

Wait for: `"LLM Provider: OpenRouter (500+ models available)"`

### 3️⃣ Start Frontend

New terminal:
```powershell
cd frontend_desktop
npm run dev
```

Open: http://localhost:3000

---

## ✅ Success Indicators

- Backend: Shows "LLM Provider: OpenRouter"
- Frontend: Shows "OpenRouter CONNECTED"
- Can send messages and get responses

---

## 🔧 Minimal .env Example

```bash
OPENROUTER_API_KEY=sk-or-v1-your-key-here
LLM_PROVIDER=openrouter
DEFAULT_LLM_MODEL=anthropic/claude-3.5-sonnet
FALLBACK_LLM_MODEL=openai/gpt-4o-mini
PHOENIX_WEB_BIND=127.0.0.1:8888
PHOENIX_NAME=Sola
USER_NAME=User
VECTOR_KB_ENABLED=true
VECTOR_DB_PATH=./data/vector_db
```

---

## 🆘 Troubleshooting

**Backend won't start:**
- Check .env exists in project root
- Verify OPENROUTER_API_KEY is set
- Ensure no typos in the key

**Frontend shows "not connected":**
- Is backend running on port 8888?
- Test: http://localhost:8888/health
- Should return: `{"status":"ok"}`

**Compilation errors:**
- Run: `cargo clean`
- Then: `cargo build --release`

---

## 📚 More Help

- Full guide: `ENV_SETUP_README.md`
- OpenRouter setup: `OPENROUTER_SETUP_GUIDE.md`
- Script details: `SETUP_FIXED.md`

---

**That's it! You're ready to use Sola AGI.** 🎉
