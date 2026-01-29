# Security Checklist

## ✅ Confirmed: No Sensitive Files in Git

### Environment Variables
- ✅ `.env` file is **NOT** tracked (ignored by `.gitignore`)
- ✅ `.env.local` files are ignored
- ✅ Only `.env.example` is tracked (contains placeholders only)

### API Keys and Tokens
- ✅ No hardcoded API keys found in source code
- ✅ All API keys are read from environment variables
- ✅ OpenRouter API key: `OPENROUTER_API_KEY` (from `.env`)
- ✅ GitHub PAT: `GITHUB_PAT` (from `.env`)
- ✅ All API key references in code use `env_nonempty()` or `std::env::var()`

### Database Files
- ✅ `body_vault.db/` - **IGNORED** (may contain sensitive memory data)
- ✅ `eternal_memory.db/` - **IGNORED** (may contain sensitive memory data)
- ✅ `mind_vault.db/` - **IGNORED** (may contain sensitive memory data)
- ✅ `soul_kb.db/` - **IGNORED** (may contain sensitive memory data)
- ✅ All `*.db`, `*.sqlite`, `*.sqlite3` files are ignored

### Build Artifacts
- ✅ `target/` directory is ignored (Rust build artifacts)
- ✅ `node_modules/` is ignored (frontend dependencies)
- ✅ `dist/` and build outputs are ignored

### Other Sensitive Files
- ✅ `*.key`, `*.pem`, `*.p12`, `*.crt` files are ignored
- ✅ `secrets/` directory is ignored
- ✅ `credentials.json` and `config.json` are ignored
- ✅ Audio/video recordings are ignored (may contain sensitive content)
- ✅ Screenshots and captures are ignored

## 🔒 Security Best Practices

### For Developers

1. **Never commit `.env` files**
   - Always use `.env.example` as a template
   - Add actual values to `.env` (which is gitignored)

2. **Never hardcode API keys**
   - Always use environment variables
   - Use `env_nonempty()` or `std::env::var()` in Rust
   - Use `process.env` in TypeScript/JavaScript

3. **Review before committing**
   ```bash
   git status
   git diff
   ```
   - Check for any `.env`, `.key`, or database files
   - Verify no API keys are in the diff

4. **Use `.env.example` for documentation**
   - Include all required variables
   - Use placeholder values like `your-key-here`
   - Document where to get real values

### Environment Variables to Set

Create a `.env` file (NOT committed) with:

```bash
# LLM Provider (Required for OpenRouter)
OPENROUTER_API_KEY=sk-or-v1-your-actual-key-here

# GitHub Integration (Required for agent spawning)
GITHUB_PAT=ghp_your-actual-token-here

# Optional: Ollama (if using local LLM)
OLLAMA_BASE_URL=http://192.168.1.100:11434
OLLAMA_MODEL=llama3

# Phoenix Configuration
PHOENIX_NAME=Phoenix
PHOENIX_CUSTOM_NAME=Sola
# ... (see .env.example for full list)
```

### If You Accidentally Commit Sensitive Data

1. **Immediately rotate/revoke the exposed credentials**
   - OpenRouter: https://openrouter.ai/keys
   - GitHub: https://github.com/settings/tokens

2. **Remove from git history** (if needed):
   ```bash
   git filter-branch --force --index-filter \
     "git rm --cached --ignore-unmatch .env" \
     --prune-empty --tag-name-filter cat -- --all
   ```

3. **Force push** (coordinate with team):
   ```bash
   git push origin --force --all
   ```

## 🔍 Verification Commands

### Check for sensitive files in git:
```bash
git ls-files | grep -E "\.env$|\.key$|\.db$|secret|password"
```

### Check for hardcoded API keys:
```bash
git grep -i "sk-or-v1-[a-zA-Z0-9]" -- "*.rs" "*.ts" "*.tsx"
git grep -i "ghp_[a-zA-Z0-9]" -- "*.rs" "*.ts" "*.tsx"
```

### Verify .gitignore is working:
```bash
git check-ignore -v .env body_vault.db
```

## 📋 Pre-Commit Checklist

Before committing, verify:
- [ ] No `.env` files in `git status`
- [ ] No database files (`.db`, `.sqlite`) in `git status`
- [ ] No hardcoded API keys in code
- [ ] All sensitive files are in `.gitignore`
- [ ] `.env.example` only contains placeholders

## 🚨 Red Flags

**STOP and review if you see:**
- `.env` file in `git status`
- Database files in `git status`
- API keys that look real (not placeholders) in code
- `sk-or-v1-` followed by actual characters (not "your-key-here")
- `ghp_` followed by actual characters (not "your_token")

## 📝 Notes

- The `.gitignore` file is comprehensive and covers all common sensitive file patterns
- Database files are ignored because they may contain sensitive memory data
- Build artifacts are ignored to keep the repo clean
- Frontend `.env` files are also ignored (see `frontend_desktop/.gitignore`)
