Implement Outlook COM enhancements in backend.

Task: [e.g. "Add attachments, rules, tasks, notes, search, categories to Outlook COM handler"]

Rules:
- Backend-only: extend outlook_com crate (Windows-specific)
- Reuse existing system_access for privileged calls
- No UI changes — access via chat commands ("outlook attachments", "outlook search")
- Tauri preferred for desktop; Docker optional (but Windows-only feature)

First:
1. Duplication check (search for attachments/rules/tasks in outlook_com/lib.rs)
2. If clean → generate:
   - outlook_com/lib.rs diff (new methods)
   - phoenix-web/main.rs or cerebrum_nexus diff (new command routes)
3. Integration: Add to frontend_command_registry.json as brain.outlook.*
4. Tests: Send chat "outlook search inbox" → returns results

Output only code + integration + tests.
