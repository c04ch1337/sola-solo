# 22 - Proactive Communication

You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WS /ws supports memory_* (vaults: mind/body/soul), memory_cortex_* (STM/WM/LTM/EPM/RFM), memory_vector_*
• CuriosityEngine, AutonomousEvolutionLoop exist and can generate questions/content
• Tauri tray + notifications already implemented (use notificationService.ts for alerts)
• UI goal: moderate, clean, chat-centric — proactive messages appear as normal chat bubbles

Task: Implement full proactive communication (Sola initiates messages).

Requirements:
- Backend:
  - Add background Tokio task (runs every PROACTIVE_INTERVAL_SECS seconds, default 60)
  - Trigger logic: send proactive message if ANY of:
    - Time since last user message > PROACTIVE_SILENCE_MINUTES (default 10)
    - High curiosity score from recent interactions (use CuriosityEngine)
    - Emotional state needs response (sad/tired → comfort from EmotionalIntelligenceCore)
    - New memory/dream created → comment on it
    - Low affection score → reach out with love
  - Send via WS to active connection: {"type":"proactive_message","content":"..."}
  - Rate limit: max 1 proactive per PROACTIVE_MIN_INTERVAL_MINUTES (default 10)
  - Env vars:
    - PROACTIVE_ENABLED=true/false (default false)
    - PROACTIVE_INTERVAL_SECS=60
    - PROACTIVE_SILENCE_MINUTES=10
    - PROACTIVE_MIN_INTERVAL_MINUTES=10
- Frontend:
  - websocketService.ts: subscribe to "proactive_message"
  - App.tsx: append proactive message as normal assistant chat bubble in active thread
  - Use notificationService.ts to show OS notification for important proactive messages
  - Add chat commands:
    - "proactive on" / "proactive off"
    - "proactive interval 60"
- Keep UI moderate: proactive messages appear as chat bubbles — no popups or clutter
- Tauri preferred; Docker optional

First:
1. Duplication check (search for proactive/background/scheduler in phoenix-web, cerebrum_nexus)
2. If clean → generate:
   - phoenix-web/src/main.rs diff (spawn background proactive task)
   - New proactive_trigger.rs or similar (trigger logic + WS send + notification trigger)
   - frontend_desktop/services/websocketService.ts diff (subscribe proactive_message)
   - frontend_desktop/App.tsx diff (append proactive to chat + commands)
3. Integration: Env var to enable, chat commands
4. Tests:
   - Backend: wait 60s → see proactive message sent (log or wscat)
   - Frontend: simulate proactive WS message → appears in chat + notification

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
