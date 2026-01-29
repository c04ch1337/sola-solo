Implement frontend features in frontend_desktop/App.tsx.

Task: [e.g. "Wire Dreams (lucid, shared, healing, recordings) as collapsible panel + chat commands like 'lucid user'"]

Rules:
- Add to App.tsx as collapsible panels/modals (toggle via header icon or chat command)
- Keep UI moderate/clean: chat-view centric, no crowding — most via orchestrator/chat ("show dreams", "start journal")
- Use existing services for WS/API (e.g. memoryService for recordings)
- Tauri preferred; Docker optional
- Update frontend_command_registry.json with new brain.dreams.*, brain.perception.*, etc.

First:
1. Duplication check (search for dreams/perception/record/approvals in App.tsx, components/)
2. If clean → generate:
   - New components (e.g. DreamsPanel.tsx, PerceptionPanel.tsx)
   - App.tsx diff (state/toggles + chat command router)
   - frontend_command_registry.json update
3. Integration: How to trigger via chat or button
4. Tests: Send chat command "show dreams" → panel opens; "record journal" → starts 120s capture

Output only code + integration + tests.
