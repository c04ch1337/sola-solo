# 24 - Memory Browser Chat Commands

You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WS /ws supports memory_* (vaults: mind/body/soul), memory_cortex_* (STM/WM/LTM/EPM/RFM), memory_vector_*
• MemoryBrowser.tsx exists and is collapsible/hidden by default with header toggle
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Add chat commands to control MemoryBrowser (show/hide).

Requirements:
- Add chat commands:
  - "show memory" / "open memory" → open MemoryBrowser panel
  - "hide memory" / "close memory" → close MemoryBrowser panel
- Keep existing header icon toggle
- Show confirmation message in chat ("Memory browser opened")
- Keep UI moderate: panel remains hidden by default

First:
1. Duplication check (search for "show memory" / "hide memory" in App.tsx parseChatCommand)
2. If clean → generate:
   - frontend_desktop/App.tsx diff (add chat command parser for memory toggle)
3. Integration: "show memory" → opens panel + confirmation
4. Tests:
   - Chat: "show memory" → panel opens
   - Chat: "hide memory" → panel closes

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
