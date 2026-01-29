# Browser Control End-to-End Testing Guide

## Current Status

✅ **Backend**: Running (`pagi-sola-web`, PID 123270, port 8888)  
✅ **Frontend**: Running (Vite dev server, port 3000)  
✅ **Browser Commands**: Implemented in `system_access/src/lib.rs`  
✅ **WebSocket Handler**: Routes "system browser *" commands in `phoenix-web/src/main.rs`

## Architecture Overview

```
User types in chat → Frontend (App.tsx)
  ↓
  sendSpeak(ws, user_input) → WebSocket message: {"type":"speak","user_input":"system browser help"}
  ↓
  Backend websocket.rs → handle_speak → LLM processes input
  ↓
  LLM recognizes "system browser" → handle_browser_command() in main.rs
  ↓
  system_access::SystemAccess methods (browser_navigate, browser_login, browser_scrape)
  ↓
  Response streamed back via speak_response_chunk + speak_response
  ↓
  Frontend displays result in chat
```

## Test Plan

### Test 1: Browser Help (via Frontend Chat)

**Steps:**
1. Open browser: http://localhost:3000
2. In the chat input, type: `system browser help`
3. Press Enter

**Expected Result:**
- Chat shows browser commands help text with available commands:
  - sessions
  - launch <chrome|edge> [port=9222]
  - connect <chrome|edge> [port=9222]
  - tabs [port=9222]
  - cookies <chrome|edge> [port=9222]
  - set-cookie <chrome|edge> name=... value=... [domain=...] [port=9222]
  - extensions <chrome|edge>
  - js <code> [port=9222]
  - navigate <url> [port=9222]
  - login <url> | username=... | password=... [port=9222]
  - scrape [url] | selector=... [port=9222]

### Test 2: Find Browser Sessions

**Steps:**
1. In chat, type: `system browser sessions`
2. Press Enter

**Expected Result:**
- Lists any running Chrome/Edge instances with debugging enabled
- Shows port, PID, and browser type
- Or shows "No browser sessions found" if none are running

### Test 3: Launch Browser with Debugging

**Steps:**
1. In chat, type: `system browser launch chrome`
2. Press Enter
3. Wait for browser window to open

**Expected Result:**
- Chrome launches with debugging enabled on port 9222
- Chat shows: "Launched chrome on port 9222 (PID: xxxxx)"
- Browser window appears

### Test 4: Navigate to URL

**Prerequisites:** Browser launched (Test 3) or existing session on port 9222

**Steps:**
1. In chat, type: `system browser navigate | url=https://news.ycombinator.com`
2. Press Enter

**Expected Result:**
- Browser navigates to Hacker News
- Chat shows: "Navigated to https://news.ycombinator.com"

### Test 5: Scrape Content

**Prerequisites:** Browser with page loaded (Test 4)

**Steps:**
1. In chat, type: `system browser scrape | selector=.titleline`
2. Press Enter

**Expected Result:**
- Chat displays extracted text from all `.titleline` elements on the page
- Shows article titles from Hacker News front page

### Test 6: Login Flow (Example Site)

**Warning:** Only use on test sites or sites you have permission to automate

**Steps:**
1. In chat, type: `system browser login https://example.com/login | username=testuser | password=testpass`
2. Press Enter

**Expected Result:**
- Browser navigates to login page
- Automatically fills username and password fields
- Submits the form
- Chat shows: "Login submitted successfully." or error if fields not found

## Manual WebSocket Testing (Alternative)

If frontend testing fails, test directly via wscat:

```bash
# Test 1: Browser help
echo '{"type":"speak","user_input":"system browser help"}' | wscat -c ws://localhost:8888/ws &
sleep 3

# Test 2: Browser sessions  
echo '{"type":"speak","user_input":"system browser sessions"}' | wscat -c ws://localhost:8888/ws &
sleep 3

# Test 3: Launch browser
echo '{"type":"speak","user_input":"system browser launch chrome"}' | wscat -c ws://localhost:8888/ws &
sleep 5

# Test 4: Navigate
echo '{"type":"speak","user_input":"system browser navigate | url=https://example.com"}' | wscat -c ws://localhost:8888/ws &
sleep 3
```

## Browser Command Syntax Reference

All commands start with `system browser` followed by the subcommand:

### Sessions
```
system browser sessions
```

### Launch
```
system browser launch <chrome|edge> [port=9222]
```

### Navigate
```
system browser navigate | url=<url> [port=9222]
```

### Login
```
system browser login <url> | username=<user> | password=<pass> [port=9222]
```

### Scrape
```
system browser scrape [url] | selector=<css-selector> [port=9222]
```

### Execute JavaScript
```
system browser js | code=<javascript> [port=9222]
```

### Get Tabs
```
system browser tabs [port=9222]
```

### Get Cookies
```
system browser cookies <chrome|edge> [port=9222]
```

### Set Cookie
```
system browser set-cookie <chrome|edge> name=<name> value=<value> [domain=<domain>] [port=9222]
```

### List Extensions
```
system browser extensions <chrome|edge>
```

## Troubleshooting

### Browser Won't Launch
- Check if Chrome/Edge is installed
- Ensure no other instance is running on port 9222
- Try specifying a different port: `system browser launch chrome port=9223`

### "Security gate check failed"
- Backend has Tier 0-2 security system
- Browser operations require appropriate tier access
- Check environment variable: `TIER_2_OVERRIDE=true` in backend

### No Response in Chat
- Check DevTools → Network → WS to see WebSocket messages
- Verify WebSocket connection is active (green status indicator)
- Check backend logs for errors: `journalctl -u pagi-sola-web -f`

### Browser Commands Don't Work
- Verify backend is running: `ps aux | grep pagi-sola-web`
- Check health: `curl http://localhost:8888/health`
- Restart backend if needed: `cd phoenix-web && cargo run`

## Security Notes

- Browser control requires Tier 1 or higher access
- Login credentials are passed through secure WebSocket connection
- Credentials are not logged by default
- Use caution with automation on production sites
- Respect robots.txt and site terms of service

## Next Steps

After successful testing:
1. ✅ Browser control works end-to-end via chat
2. Add collapsible "Browser" panel in frontend (optional)
3. Add browser command shortcuts/presets
4. Integrate with Dream Diary for web research
5. Add browser automation workflows

## Files Modified/Created

- ✅ `system_access/src/lib.rs` - Browser methods (already exists)
- ✅ `phoenix-web/src/main.rs` - handle_browser_command (already exists)
- ✅ `phoenix-web/src/websocket.rs` - WebSocket routing (already exists)
- 📝 `docs/BROWSER_CONTROL_TESTING.md` - This test guide
- ⏳ `frontend_desktop/components/BrowserPanel.tsx` - Optional collapsible panel (future)
- ⏳ `frontend_command_registry.json` - Browser command shortcuts (future)
