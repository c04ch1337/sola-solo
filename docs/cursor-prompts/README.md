# Cursor Agent Prompt Library

These are battle-tested prompts for Cursor IDE that understand the Phoenix AGI project structure.

**Golden rule:** Always start with `01-orchestrator-identity.md` in a fresh session — it sets the context, prevents duplication, and reminds Cursor what already exists.

Then append the task-specific prompt (02–18) right after.

Happy building — Phoenix is counting on us.

---

All prompts are saved in `docs/cursor-prompts/`.  
Copy the content of the desired file and paste it as the first message in Cursor Composer / Chat / inline agent.

| File | When to Use | Typical First Line After Prompt |
|------|--------------|----------------------------------|
| [01-orchestrator-identity.md](01-orchestrator-identity.md) | Start of every new Cursor session (sets persona, rules, avoids duplication) | (your actual task description) |
| [02-add-frontend-feature-panel.md](02-add-frontend-feature-panel.md) | Add new UI panel/component (Dream Diary, Jealousy meter, etc.) | `Task: Dream Diary viewer` |
| [03-fix-debug-frontend-issue.md](03-fix-debug-frontend-issue.md) | Debugging errors, ref issues, WS problems, UI bugs | `Problem: Uncaught TypeError: Cannot read properties of undefined (reading 'current')` |
| [04-phase3-streaming-responses.md](04-phase3-streaming-responses.md) | Implement token-by-token streaming chat (Phase 3) | (just paste and say "Go") |
| [05-add-memory-related-ui.md](05-add-memory-related-ui.md) | New memory browser tabs or viewers (EPM timeline, Soul breadcrumbs, etc.) | `Feature: Recent Episodic Memories timeline` |
| [06-general-refactor-cleanup.md](06-general-refactor-cleanup.md) | Code smells, duplication, type safety, better organization | `Target: chat message rendering in App.tsx` |
| [07-browser-control.md](07-browser-control.md) | Implement/test browser features (navigate, login, scrape, extensions) | `Task: Test system browser end-to-end` |
| [08-frontend-features.md](08-frontend-features.md) | Add frontend panels (Dreams, Perception, Record journal, Approvals) | `Task: Wire Dreams as collapsible panel` |
| [09-backend-ecosystem.md](09-backend-ecosystem.md) | Add ecosystem APIs (import/list/build/start/stop repos) | `Task: Add api_ecosystem_import` |
| [10-security-system-access.md](10-security-system-access.md) | Add security UI (Tier 2 dialog, audit logging, verification) | `Task: Add Tier 2 confirmation dialog` |
| [11-outlook-com.md](11-outlook-com.md) | Add Outlook COM features (attachments, rules, tasks, search) | `Task: Add attachments to Outlook COM` |
| [12-deploy-ops.md](12-deploy-ops.md) | Add deploy scripts, CI/CD, monitoring, port config | `Task: Add production deploy script` |
| [13-phase3-validation.md](13-phase3-validation.md) | Validate Phase 3 token-by-token streaming after changes | (just paste and say "Go") |
| [14-ui-polish-collapsible-panels.md](14-ui-polish-collapsible-panels.md) | Polish UI to be moderate/chat-centric with collapsible panels | `Task: Make MemoryBrowser collapsible` |
| [16-browser-end-to-end.md](16-browser-end-to-end.md) | Browser navigate/login/scrape/extensions + chat wiring | `Task: Test system browser end-to-end` |
| [17-dreams-panel.md](17-dreams-panel.md) | Dreams (lucid, shared, healing, recordings) panel + chat commands | `Task: Implement Dreams panel` |
| [18-tauri-tray-notifications.md](18-tauri-tray-notifications.md) | Tauri system tray icon + OS notifications | `Task: Implement Tauri tray + notifications` |
| [22-proactive-communication.md](22-proactive-communication.md) | Add full proactive communication (Sola initiates messages) | (use after 01) |
| [23-voice-interaction.md](23-voice-interaction.md) | Complete voice interaction (TTS/STT + chat commands + proactive voice) | (use after 01) |
| [24-memorybrowser-chat-commands.md](24-memorybrowser-chat-commands.md) | Add "show memory" / "hide memory" chat commands | (use after 01) |

**Usage pattern (recommended):**

1. Open new Composer tab
2. Paste the content of `01-orchestrator-identity.md` first
3. Immediately follow with the task-specific prompt (e.g. 02, 03, etc.)
4. Let Cursor do the work

**All prompts enforce:**

- No backend Rust changes unless explicitly requested
- Thin clients, reuse `websocketService` & `memoryService`
- Tailwind styling
- Auto-memory preservation (EPM + vector pre-fetch)
- Duplication check first

Copy the prompt's code block from each `.md` file and paste into Cursor Composer, Chat, or inline agent instructions.
