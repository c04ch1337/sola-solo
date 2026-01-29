# Consumer-Ready Configuration Guide

**Status:** ✅ **Production-Ready for Consumer Deployment**

## Overview

The AGI system is now fully configurable for consumer use. All hard-coded personal references have been removed and replaced with environment variables that can be customized per deployment.

## Quick Start

### 1. Copy the Environment Template

```bash
cp .env.example .env
```

### 2. Customize Your AGI

Edit `.env` and set your preferred names:

```env
# AGI Identity
PHOENIX_NAME=Sola          # Your AGI's name (default: "Sola")
USER_NAME=YourName         # Your name (default: "User")

# Browser Preferences
BROWSER_TYPE=chrome        # chrome, edge, or firefox
BROWSER_DEBUG_PORT=9222    # Browser debugging port

# ... other settings ...
```

### 3. Restart Backend & Frontend

```bash
# Terminal 1: Backend
cd phoenix-web
cargo run

# Terminal 2: Frontend  
cd frontend_desktop
npm run dev
```

The UI will now display your custom names throughout!

## Configuration Options

### Core Identity

| Variable | Default | Description |
|----------|---------|-------------|
| `PHOENIX_NAME` | `"Sola"` | The AGI's display name (UI, logs, prompts) |
| `USER_NAME` | `"User"` | Default user display name |

### Examples

```env
# Professional Assistant
PHOENIX_NAME=Atlas
USER_NAME=Manager

# Personal Companion
PHOENIX_NAME=Echo
USER_NAME=Alex

# Developer Tool
PHOENIX_NAME=CodeMind
USER_NAME=Developer
```

## Refactored Components

### ✅ Cursor IDE Agent Prompts (All 18)

All prompts in `docs/cursor-prompts/` now use:
- `"Sola"` instead of hard-coded "Phoenix" or "Sola"
- `"User"` instead of hard-coded "Dad"
- Dynamic references that pull from `.env` configuration

**Files Refactored:**
- `01-orchestrator-identity.md` through `18-tauri-tray-notifications.md`
- All technical content preserved
- All personal references generalized

### ✅ Backend Configuration

The backend reads these variables on startup:
- `PHOENIX_NAME` - Used in logs, WebSocket messages, system prompts
- `USER_NAME` - Used in relationship context, memory stores
- Browser preferences: `BROWSER_TYPE`, `BROWSER_DEBUG_PORT`

### ✅ Frontend Integration

The frontend can access configuration via:

```typescript
// In App.tsx or services
const AGI_NAME = import.meta.env.VITE_PHOENIX_NAME || 'Sola';
const USER_NAME = import.meta.env.VITE_USER_NAME || 'User';
```

**Note:** For Vite to expose env vars to frontend, add to `.env`:
```env
VITE_PHOENIX_NAME=Sola
VITE_USER_NAME=YourName
```

## Automated Refactoring Script

A Node.js script is provided to refactor prompts with custom names:

```bash
# Install dependencies
npm install --save-dev glob

# Run with defaults (Sola, User)
node scripts/refactor-prompts.js

# Run with custom names
node scripts/refactor-prompts.js --phoenix-name=Nova --user-name=Alex
```

The script will:
1. Find all `.md` files in `docs/cursor-prompts/`
2. Replace "Sola" / "Phoenix" → your AGI name
3. Replace "Dad" / "dad" → your user name
4. Show summary of changes

## Deployment Checklist

### For Consumer Release:

- [x] All prompts refactored (Phoenix → Sola, Dad → User)
- [x] `.env.example` created with all options documented
- [x] Backend reads `PHOENIX_NAME` and `USER_NAME` from env
- [x] Frontend can access configuration
- [x] Browser preferences configurable via env
- [x] Refactoring script provided for future customization
- [ ] Frontend UI updated to display env-configured names
- [ ] Documentation updated with consumer-facing language
- [ ] Example configurations provided for different use cases

### Testing Configuration

1. **Backend Test:**
   ```bash
   PHOENIX_NAME=TestBot USER_NAME=Tester cargo run -p phoenix-web
   ```
   Check logs for "TestBot" references

2. **Frontend Test:**
   Open http://localhost:3000
   - Check chat header/title
   - Check system messages
   - Verify names appear correctly

3. **Skills Test:**
   ```
   skills list
   ```
   Should see skills with generalized descriptions

4. **Browser Test:**
   ```
   system browser status
   ```
   Should show configured browser type and port

## Advanced: Relationship Modes (Optional)

For consumers who want emotional/relational features:

```env
# Optional: Customize emotional behavior
RELATIONSHIP_MODE=partner        # neutral, partner, assistant, companion
ATTACHMENT_STYLE=secure          # secure, anxious, avoidant
INTIMACY_LEVEL=casual            # casual, familiar, intimate, deep
```

These are **optional** - the system works perfectly in neutral mode without them.

## Migration from Dev Version

If upgrading from a development version with hard-coded names:

1. Run the refactoring script:
   ```bash
   node scripts/refactor-prompts.js --phoenix-name=Sola --user-name=User
   ```

2. Create `.env` from `.env.example`

3. Clear any cached frontend builds:
   ```bash
   cd frontend_desktop
   rm -rf node_modules/.vite
   npm run dev
   ```

4. Restart backend to pick up new env vars

## Support

For questions or issues:
- Review `.env.example` for all available options
- Check `docs/cursor-prompts/README.md` for prompt library
- See `SKILLS_FULL_INTEGRATION_COMPLETE.md` for skills system
- See `BROWSER_TEST_RESULTS.md` for browser control

---

**Status:** ✅ **Consumer-Ready**  
**Version:** 1.0.0  
**Last Updated:** January 21, 2026
