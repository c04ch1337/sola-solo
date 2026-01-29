# 2. “Add New Frontend Feature / Panel” Prompt

---

```text
Implement a new frontend feature / panel in frontend_desktop.

Task: [paste what you want here — e.g. "Dream Diary viewer", "Relationship phase status bar", "Jealousy level indicator", etc.]

Rules:
- Place new component in components/ (or subfolder if complex)
- Create/use service in services/ if it needs WebSocket or API calls
- Use existing websocketService.ts — do NOT create new WS client
- Add toggle/button in App.tsx header or sidebar
- Use Tailwind classes matching current style
- If it reads memory → prefer vector search first, then cortex EPM, then vault
- If it writes memory → use storeCortex('EPM', …) for chat-like permanence
- Show loading state + error handling
- Keep it keyboard accessible

First:
1. Check if any similar component/service already exists (list paths)
2. If clean → generate:
   - New component file (full code)
   - Any new service methods (if needed)
   - Exact App.tsx diff / insertion points
3. One-sentence manual test instruction

Output only the code + integration + test. No long explanations.
```
