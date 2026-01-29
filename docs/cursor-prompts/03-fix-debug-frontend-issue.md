# 3. “Fix / Debug Frontend Issue” Prompt

---

```text
Debug and fix this frontend issue in frontend_desktop.

Problem: [paste error message / describe behavior]

Context:
• Using websocketService.ts + memoryService.ts
• App.tsx is main entry
• Running on http://localhost:3000
• Backend WS at ws://localhost:8888/ws

Steps you must follow:
1. Ask yourself: is this likely a ref issue, race condition, missing import, serialization mismatch, or WS event not handled?
2. Grep/search codebase for relevant terms (error keywords, component names, WS event types)
3. Propose the smallest fix:
   - File + line to change
   - Exact code diff or replacement
4. If needed, add console.log() or error boundary for easier repro
5. Give one-line test to verify fix

Output format:
• Suspected cause (1 sentence)
• File + diff / replacement code
• Test command / action
• If unsure → list 2–3 things I should check in console/network tab
```
