# 6. “General Refactor / Cleanup” Prompt

---

```text
Refactor and clean up this part of frontend_desktop.

Target: [e.g. "chat message rendering in App.tsx", "memoryService event handling", "MemoryBrowser.tsx tabs"]

Goals:
- Remove duplication
- Improve type safety (add interfaces/types)
- Fix any ref/callback closure issues
- Better error handling / loading states
- Consistent Tailwind classes
- Smaller functions, better naming

First:
1. Scan target file(s) for common issues (useCursor or grep)
2. Propose refactored code (full file or focused sections)
3. Show before/after side-by-side if possible
4. Keep behavior identical — only improve structure/readability

Output only cleaned code + brief change summary.
```
