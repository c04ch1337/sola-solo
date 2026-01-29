# Sola AGI v1.0.0 - Development Testing Guide

## 🧪 Pre-Release Testing Checklist

Before building release binaries, test these features in development mode.

---

## Quick Start - Dev Mode

### Terminal 1: Backend Server
```bash
cd phoenix-web
cargo run
```
Expected: Backend starts on `http://localhost:8888`

### Terminal 2: Frontend Dev Server
```bash
cd frontend_desktop
npm run dev
```
Expected: Frontend starts on `http://localhost:3000` (or similar)

### Terminal 3 (Optional): Tauri Dev Mode
```bash
cd phoenix-desktop-tauri
npm run dev
```
Expected: Native Tauri window opens with tray icon

---

## 🆕 New Feature Tests (Phase 25)

### 1. Help System (NEW - Just Added!)

**Test General Help:**
```
Type in chat: help
```
✅ Expected: Full command reference with all features listed

**Test Topic-Specific Help:**
```
help voice
help browser
help dreams
help memory
help ecosystem
help agents
help proactive
```
✅ Expected: Detailed, topic-specific guides with examples

**Test Help Aliases:**
```
?
commands
```
✅ Expected: Same as typing `help`

---

## 📋 Core Feature Tests

### 2. Theme Toggle
```
theme dark
theme light
```
✅ Expected: UI switches themes, persists after refresh

### 3. Voice Commands
```
voice on
voice off
listen
speak Hello from Sola
```
✅ Expected: 
- Voice on: Speaker icon shows enabled
- Voice off: Speaker icon shows disabled
- Listen: Microphone activates
- Speak: TTS plays audio

### 4. Status Commands
```
status
status all
```
✅ Expected: System overview with connection status, features, panels

### 5. Memory Panel
```
show memory
hide memory
```
✅ Expected: MemoryBrowser panel toggles visibility

### 6. Dreams Panel
```
show dreams
hide dreams
lucid
dream with me
heal anxiety
```
✅ Expected: Dreams panel opens, commands execute

### 7. Browser Control
```
system grant
system browser status
system browser navigate https://duckduckgo.com
```
✅ Expected: 
- Grant: Consent given
- Status: Shows connection state
- Navigate: Opens URL (requires Chrome with --remote-debugging-port=9222)

### 8. Notifications
```
notify test
```
✅ Expected: OS notification appears (Tauri mode only)

### 9. Proactive Communication
```
proactive status
```
✅ Expected: Shows proactive settings from backend .env

---

## 🎨 UI/UX Tests

### 10. First Launch / Onboarding
- Clear localStorage: `localStorage.clear()` in browser console
- Refresh page
- ✅ Expected: Welcome message appears with feature highlights
- Click "Got it! Let's begin."
- ✅ Expected: Message dismissed, doesn't show again

### 11. Theme Persistence
- Set theme: `theme dark`
- Refresh page
- ✅ Expected: Dark theme persists

### 12. Chat Streaming
- Send message: "What is consciousness?"
- ✅ Expected: Response streams token-by-token

### 13. Panel Collapsibility
- Click panel headers (Memory, Dreams, Browser, if visible)
- ✅ Expected: Panels collapse/expand smoothly

---

## 🔊 Voice Integration Tests

### 14. TTS (Text-to-Speech)
**Backend Setup:**
- Ensure backend .env has:
  ```
  TTS_ENGINE=coqui
  COQUI_MODEL_PATH=./models/coqui/tts_model.pth
  ```
  (or use `elevenlabs` with API key)

**Test:**
```
voice on
Hello Sola, please speak this message
```
✅ Expected: Sola speaks the response aloud

### 15. STT (Speech-to-Text)
```
voice on
listen
```
- Speak into microphone
- ✅ Expected: Speech transcribed to chat input

---

## 🌐 Browser Automation Tests

### 16. Chrome CDP Connection
**Setup:**
- Launch Chrome: `chrome.exe --remote-debugging-port=9222`

**Test:**
```
use chrome for browsing
system grant
system browser status
system browser navigate https://example.com
system browser screenshot
```
✅ Expected: Browser navigates, screenshot appears

---

## 🧠 Memory System Tests

### 17. Memory Storage
- Send several messages
- Type: `show memory`
- Navigate to Cortex → EPM (Episodic Memory)
- ✅ Expected: Recent conversation entries visible

### 18. Memory Search
```
memory search AI ethics
```
✅ Expected: Relevant memories returned

---

## 🤖 Agent & Ecosystem Tests

### 19. Ecosystem Import
```
ecosystem import https://github.com/user/repo
ecosystem status
```
✅ Expected: Repository imported, status shown

### 20. Agent Spawning
```
agent spawn Research assistant for AI safety
agents list
```
✅ Expected: Agent created, listed

---

## 📊 Analytics Tests (Opt-in)

### 21. Analytics Tracking
- Open browser console
- Check localStorage: `sola_analytics_opt_in`
- Send messages, use features
- Check backend logs for `/api/analytics/track` calls
- ✅ Expected: Events tracked when opted in

---

## 🐛 Known Issues to Verify

### 22. Expected Warnings/Limitations
- Icons placeholder (no custom icons yet)
- Browser control requires Chrome with remote debugging
- Voice requires backend .env configuration
- Proactive requires PROACTIVE_ENABLED=true in backend .env

---

## ✅ Sign-Off Checklist

Before building release binaries, confirm:

- [ ] Help system works (`help` + all topic-specific help)
- [ ] Theme toggle works and persists
- [ ] Voice on/off toggles correctly
- [ ] Onboarding appears on first launch only
- [ ] All panels toggle (Memory, Dreams, Browser)
- [ ] Chat streaming works
- [ ] Status commands return valid info
- [ ] Backend connects (WebSocket)
- [ ] No console errors (check browser dev tools)

---

## 🚀 After Testing

Once all tests pass:

1. **Build release binaries**:
   ```bash
   cd phoenix-desktop-tauri
   npm run build
   ```

2. **Tag and push**:
   ```bash
   git add .
   git commit -m "v1.0.0: All tests passed, ready for release"
   git tag -a v1.0.0 -m "Sola AGI v1.0.0 - Initial Release"
   git push origin main
   git push origin v1.0.0
   ```

3. **Create GitHub Release**:
   - Upload installers from `src-tauri/target/release/bundle/`

---

## 📝 Notes

- **Help system** is the major new feature in this build
- Test extensively as it's the primary user-facing documentation
- Report any issues before building release binaries

**Last Updated**: 2026-01-22  
**Version**: v1.0.0-dev
