# 🚀 Quick Start - Development Mode

## What to Test

**Priority 1: New Help System** ⭐
The comprehensive help command was just added. Test all these:
- `help` - General help
- `help voice` - Voice commands
- `help browser` - Browser automation
- `help dreams` - Dreams panel
- `help memory` - Memory system
- `help ecosystem` - Ecosystem management
- `help agents` - Agent spawning
- `help proactive` - Proactive communication

**Priority 2: Core Features**
- Theme toggle: `theme dark` / `theme light`
- Voice: `voice on` / `voice off` / `speak Hello`
- Status: `status all`
- Panels: `show memory` / `show dreams` / `show browser`

---

## Start Development Servers

### Option A: Web Mode (Fastest for Testing Help System)

**Terminal 1 - Backend:**
```bash
cd phoenix-web
cargo run
```
Wait for: `Server running on http://localhost:8888`

**Terminal 2 - Frontend:**
```bash
cd frontend_desktop
npm run dev
```
Wait for: `Local: http://localhost:3000` (or similar)

Then open: **http://localhost:3000** in your browser

---

### Option B: Tauri Mode (Full Native Experience)

**Terminal 1 - Backend:**
```bash
cd phoenix-web
cargo run
```

**Terminal 2 - Tauri:**
```bash
cd phoenix-desktop-tauri
npm run dev
```
Native window will open with tray icon.

---

## Quick Test Script

Once both servers are running:

1. **Type in chat**: `help`
   - ✅ Should show full command reference

2. **Type**: `help voice`
   - ✅ Should show voice-specific guide

3. **Type**: `theme dark`
   - ✅ UI should switch to dark mode

4. **Type**: `status all`
   - ✅ Should show system overview

5. **Type**: `voice on`
   - ✅ Speaker icon should show enabled

6. **Refresh page**
   - ✅ Theme should persist
   - ✅ No onboarding (already seen)

7. **Clear localStorage** (console): `localStorage.clear()`
   - **Refresh**
   - ✅ Onboarding message should appear

---

## What's New in This Build

✅ **Comprehensive Help System** (just added!)
- Self-documenting command reference
- Topic-specific help for all features
- Examples and tips included
- Markdown-formatted for readability

✅ **Theme Support** (already working)
- Dark/light mode toggle
- Persistent across sessions

✅ **Onboarding** (already working)
- Welcome message on first launch
- Feature highlights

✅ **All Phase 25 Features**
- Voice I/O integration
- Proactive communication
- Dreams panel
- Browser control
- Memory system
- Ecosystem management
- Agent spawning

---

## If You See Errors

**Backend connection failed:**
- Ensure backend is running on :8888
- Check backend logs for errors

**Frontend build errors:**
- Run `npm install` in `frontend_desktop/`
- Check for missing dependencies

**Voice not working:**
- Requires backend .env configuration:
  ```
  TTS_ENGINE=coqui
  COQUI_MODEL_PATH=./models/coqui/tts_model.pth
  ```

**Browser control not working:**
- Launch Chrome with: `chrome.exe --remote-debugging-port=9222`
- Type: `use chrome for browsing`
- Type: `system grant`

---

## After Testing

When ready to build release:
```bash
cd phoenix-desktop-tauri
npm run build
```

But test thoroughly first! 🧪

---

**Next Steps:**
1. Start the servers (see above)
2. Test the help system thoroughly
3. Test other features
4. Report any issues
5. Then we'll build the release! 🚀
