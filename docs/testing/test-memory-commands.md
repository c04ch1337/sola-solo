# Test: Memory Browser Chat Commands

## Quick Test Guide

### Prerequisites
- Frontend running on http://localhost:3000
- Backend running on http://localhost:8888
- WebSocket connected

### Test Steps

#### Test 1: Show Memory Command
1. Open chat in frontend
2. Type: `show memory`
3. Press Enter

**Expected Result**:
- ✅ MemoryBrowser panel opens/appears
- ✅ Chat shows confirmation: "Memory browser opened. You can now browse your memories."

#### Test 2: Hide Memory Command
1. With MemoryBrowser visible
2. Type: `hide memory`
3. Press Enter

**Expected Result**:
- ✅ MemoryBrowser panel closes/hides
- ✅ Chat shows confirmation: "Memory browser closed."

#### Test 3: Alternative Commands
Test these variations:
- `open memory` → should open panel
- `close memory` → should close panel
- `memory show` → should open panel
- `memory hide` → should close panel

#### Test 4: Header Icon Toggle
1. Click brain icon in header
2. Panel should toggle (open/close)
3. Click again to toggle back

**Expected Result**:
- ✅ Icon toggle works independently of chat commands
- ✅ Both methods control the same panel state

### Code Location

**File**: `frontend_desktop/App.tsx`

**Lines Added** (around line 693-701):
```typescript
// Memory browser commands
if (lower === 'show memory' || lower === 'open memory' || lower === 'memory show') {
  setShowMemoryBrowser(true);
  return { kind: 'handled', localAssistantMessage: 'Memory browser opened. You can now browse your memories.' };
}
if (lower === 'hide memory' || lower === 'close memory' || lower === 'memory hide') {
  setShowMemoryBrowser(false);
  return { kind: 'handled', localAssistantMessage: 'Memory browser closed.' };
}
```

### Visual Verification

When MemoryBrowser is open, you should see:
- Memory vault tabs (Mind, Body, Soul)
- Cortex tabs (STM, WM, LTM, EPM, RFM)
- Search functionality
- Recent memories listed

### Troubleshooting

**Panel doesn't open?**
- Check console for errors
- Verify `showMemoryBrowser` state in React DevTools
- Ensure MemoryBrowser component is imported

**Commands not recognized?**
- Check `parseChatCommand` function in App.tsx
- Verify command is being processed (add console.log)
- Check for typos in command string

**Panel opens but empty?**
- Check WebSocket connection
- Verify backend is running
- Check memory service initialization

### Success Criteria

✅ All 6 command variations work  
✅ Confirmation messages appear in chat  
✅ Panel state persists until toggled  
✅ Header icon toggle still works  
✅ No console errors  

---

**Test Date**: January 22, 2026  
**Status**: Ready to test  
**Feature**: Memory Browser Chat Commands (Phase 24)
