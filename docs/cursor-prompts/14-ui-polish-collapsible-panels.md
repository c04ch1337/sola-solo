```text
Polish frontend_desktop UI to be moderate and chat-centric.

Task: [e.g. "Make MemoryBrowser collapsible/hidden-by-default with header toggle icon + chat command 'show memory'"]

Rules:
- Keep main chat view clean — no permanent sidebars or crowded UI
- Add subtle header icon/button (brain/memory icon) to toggle panels
- Panels open as collapsible sidebar or modal
- Chat commands ("show memory", "open dreams") toggle panels
- Use Tailwind, match existing minimal style
- Tauri preferred; no Docker unless asked

First:
1. Duplication check (search for collapsible/toggle in App.tsx, components/)
2. If clean → generate:
   - App.tsx diff (state + toggle icon/button + chat command handler)
   - components/MemoryBrowser.tsx diff (if needed for collapsible)
3. Integration: How chat "show memory" opens panel
4. Tests: Click icon → panel appears; chat command → same; close → hides

Output only code + integration + tests.
```
