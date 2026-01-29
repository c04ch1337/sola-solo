You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WS /ws supports memory_* (vaults: mind/body/soul), memory_cortex_* (STM/WM/LTM/EPM/RFM), memory_vector_*
• Dreams (lucid, shared, healing, recordings) exist in cerebrum_nexus (lucid_command, healing_command, dream_recordings_view)
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Configuration:
- Use PHOENIX_NAME from .env (default: "Sola") for AGI name
- Use USER_NAME from .env (default: "User") for user references

Task: Implement Dreams panel + chat commands.

Requirements:
- Backend: Ensure cerebrum_nexus dream commands (lucid [topic], heal [emotion], dream with [subject], list dreams) are callable via chat
- Frontend:
  - Collapsible "Dreams" panel (hidden by default)
  - Show recent dream recordings, lucid/shared/healing options
  - Chat commands: "lucid [topic]", "heal tired", "show dreams", "list dreams" → open panel or show inline
- Keep UI moderate: panel opens on command or small header icon; chat primary
- Tauri preferred; Docker optional

First:
1. Duplication check (search for dreams in App.tsx, components/, cerebrum_nexus)
2. If clean → generate:
   - components/DreamsPanel.tsx (list recordings, command buttons)
   - App.tsx diff (toggle state + chat command router)
   - frontend_command_registry.json update (brain.dreams.*)
3. Integration: "show dreams" → opens panel; "lucid [topic]" → triggers backend + shows result
4. Tests:
   - Chat: "show dreams" → panel appears with recordings
   - Chat: "lucid work" → backend executes, result in chat or panel

Output only code + integration + tests.
