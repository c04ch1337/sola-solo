# 23 - Voice Interaction

You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• WS /ws supports memory_* (vaults: mind/body/soul), memory_cortex_* (STM/WM/LTM/EPM/RFM), memory_vector_*
• Voice IO exists (TTS/STT engines, params modulation)
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• Tauri tray + notifications already implemented
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Complete full voice interaction (TTS + STT) with chat commands + minimal UI icons.

Requirements:
- Backend:
  - Add REST endpoints: POST /api/audio/speak, POST /api/audio/start-recording, POST /api/audio/stop-recording
  - Use voice_io crate (Coqui/ElevenLabs for TTS, Whisper/Vosk for STT)
  - Modulate TTS params (pitch/rate/volume) based on affection/emotion
- Frontend:
  - Small mic/speaker icons in chat header (toggle voice input/output)
  - Chat commands: "voice on/off", "listen", "speak <text>"
  - Integrate with proactive: speak proactive messages (TTS) + notification when voice enabled
  - STT: when listening, transcribe speech → send as chat input
- Keep UI moderate: icons subtle, no clutter
- Tauri preferred; Docker optional

First:
1. Duplication check (search for voice/mic/speaker/audio in App.tsx, components/, phoenix-web/main.rs)
2. If clean → generate:
   - phoenix-web/src/main.rs diff (new audio endpoints + handlers)
   - components/VoiceControls.tsx (mic/speaker icons + state)
   - App.tsx diff (voice state + chat command router + TTS/STT integration)
   - frontend_command_registry.json update (brain.voice.*)
   - notificationService.ts diff (proactive voice alert)
3. Integration: "voice on" → enables mic/speaker; proactive → spoken + notification
4. Tests:
   - Click mic icon → starts listening → speech transcribed to chat
   - Chat: "speak hello" → TTS speaks "hello"
   - Proactive message → spoken (TTS) + notification

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
