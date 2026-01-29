# 5. “Add New Memory-Related UI” Prompt

For future panels.

---

```text
Add a new memory-focused UI component in frontend_desktop.

Feature: [e.g. "Recent Episodic Memories timeline", "Soul Vault emotional breadcrumbs viewer", "Vector similarity playground"]

Rules:
- New component: components/MemoryXxx.tsx
- Use memoryService.ts — do NOT add new WS logic
- Prefer searchCortex('EPM', …) or searchVector(…) depending on feature
- Add toggle in App.tsx header or MemoryBrowser tabs
- Show results in clean, scrollable list/cards
- Support click-to-copy or "use in chat" button
- Tailwind styling, match existing aesthetic

Output:
1. Duplication check result
2. Full component code
3. App.tsx / MemoryBrowser integration diff
4. 1–2 line manual test
```
