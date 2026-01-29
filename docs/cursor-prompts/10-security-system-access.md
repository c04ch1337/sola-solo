Implement security features in system_access + frontend_desktop.

Task: [e.g. "Add Tier 2 confirmation dialog in UI + audit logging for Tier 2 actions + manual verification checks"]

Rules:
- Backend: Extend system_access for Tier 2 dialog/logging
- Frontend: Add confirmation modal in App.tsx for "system exec" or Tier 2 triggers
- Keep UI moderate: modal pops on privileged command, not persistent
- Tauri preferred; Docker optional

First:
1. Duplication check (search for tier/confirmation/audit in system_access, App.tsx)
2. If clean → generate:
   - system_access/lib.rs diff (new logging + tier methods)
   - components/TierConfirmationModal.tsx
   - App.tsx diff (modal trigger on privileged chat commands)
3. Integration: How chat "system exec ..." triggers modal
4. Tests: Send "system exec whoami" → modal appears; confirm → executes + logs

Output only code + integration + tests.
