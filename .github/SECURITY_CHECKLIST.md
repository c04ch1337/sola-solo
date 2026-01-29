# Security Checklist for GitHub Sync

## ✅ Pre-Push Verification

Before pushing to GitHub, verify:

1. **No `.env` files**
   ```bash
   git status | grep "\.env"
   ```
   Should return nothing (or only `.env.example`)

2. **No database files**
   ```bash
   git status | grep "\.db"
   ```
   Should return nothing

3. **No hardcoded API keys**
   ```bash
   git diff | grep -E "sk-or-v1-[a-zA-Z0-9]{20,}|ghp_[a-zA-Z0-9]{20,}"
   ```
   Should return nothing (placeholders like "your-key-here" are OK)

4. **No secrets in code**
   ```bash
   git diff | grep -iE "password|secret|token" | grep -v "your.*here" | grep -v "placeholder"
   ```
   Review any matches carefully

## 🔒 Protected Files

These files are **NEVER** committed:
- `.env` (contains actual API keys)
- `*.db` (database files with sensitive data)
- `*.key`, `*.pem` (certificates and keys)
- `secrets/` (any secrets directory)
- `credentials.json` (API credentials)

## 📝 Safe to Commit

These files are **SAFE** to commit:
- `.env.example` (contains placeholders only)
- `*.md` (documentation)
- Source code (`.rs`, `.ts`, `.tsx`, `.js`)
- Configuration files (`.toml`, `.json` without secrets)

## 🚨 If You See This, STOP

- `.env` in `git status` output
- Real API keys (not placeholders) in `git diff`
- Database files in `git status`
- Any file with "secret" or "password" in the name

## ✅ Current Status

Last verified: All sensitive files are properly ignored.
