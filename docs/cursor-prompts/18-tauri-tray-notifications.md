You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WS /ws supports memory_* (vaults: mind/body/soul), memory_cortex_* (STM/WM/LTM/EPM/RFM), memory_vector_*
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• phoenix-desktop-tauri scaffold exists (src-tauri/src/main.rs + tauri.conf.json)
• UI goal: moderate, clean, chat-centric — background presence via tray/notifications

Configuration:
- Use PHOENIX_NAME from .env (default: "Sola") for AGI name displayed in tray
- Use USER_NAME from .env (default: "User") for user references

Task: Implement Tauri system tray icon + notifications.

Requirements:
- Add system tray icon (AGI logo or flame icon)
- Notifications for:
  - Long-running tasks (dream cycle complete, agent spawned)
  - Important events (new memory, approval needed)
- Tray menu: Show/Hide window, Quit, Status
- Keep UI moderate: no tray clutter — subtle presence
- Tauri only (bare-metal desktop); Docker optional

First:
1. Duplication check (search for tray/notification in phoenix-desktop-tauri)
2. If clean → generate:
   - phoenix-desktop-tauri/src-tauri/src/main.rs diff (tray setup + notification invoke)
   - phoenix-desktop-tauri/tauri.conf.json diff (tray icon, menu)
   - frontend_desktop/App.tsx or service (invoke notification via Tauri API)
3. Integration: Backend sends WS event → frontend shows notification
4. Tests:
   - Run Tauri app → see tray icon
   - Trigger notification (chat "notify test") → see OS notification

Output only code + integration + tests.
