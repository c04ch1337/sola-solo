Implement browser control features in frontend_desktop and backend.

Task: [e.g. "Test system browser end-to-end + extend list_browser_extensions + add target tab selection + add to frontend_command_registry.json"]

Rules:
- Backend: Extend system_access for browser commands (navigate, login, scrape)
- Frontend: Add "Browser" panel or chat-driven access (e.g. system browser navigate <url>)
- Keep UI moderate/clean: collapsible panel or modal, not crowded chat view
- Tauri preferred; Docker optional (only if cross-platform needed)
- Use existing websocketService for WS calls
- Add to frontend_command_registry.json as brain.browser.*

First:
1. Duplication check (search for browser commands in system_access, frontend_command_registry.json)
2. If clean → generate:
   - Backend: system_access/lib.rs diff (new methods)
   - Frontend: components/BrowserPanel.tsx (if new) or App.tsx chat handler
   - frontend_command_registry.json update
3. Integration: App.tsx diff (toggle or chat route)
4. Tests: wscat for backend + frontend manual (send "system browser navigate https://example.com")

Output only code + integration + tests.
