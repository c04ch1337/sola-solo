# 4. “Phase 3 — Streaming Responses” Prompt

Ready when you are.

---

```text
Implement Phase 3: Token-by-token streaming responses in frontend_desktop.

Requirements:
- Backend already supports streaming via WS (speak_response_chunk or similar event)
- Frontend should show words appearing gradually in chat bubble
- Preserve existing auto-memory (EPM store) and vector pre-fetch
- Keep HTTP fallback working (non-streaming)
- Use React state to append chunks smoothly
- Add subtle typing indicator during stream
- Handle stream end / error gracefully

First:
1. Confirm backend WS already sends chunked speak responses (check websocket.rs or logs)
2. If yes → generate:
   - Update to websocketService.ts (handle speak_response_chunk)
   - Modify chat message rendering in App.tsx (streaming mode)
   - Any new state/ref needed
3. Show before/after diff for chat handler
4. One test: send long message → words should appear gradually

Output only code + integration + test.
```

---

## Status

Phase 3 streaming is now implemented end-to-end:

- Backend now emits `speak_response_chunk` events: `{type:"speak_response_chunk", chunk, done, memory_commit}`.
- Backend still emits a legacy `speak_response` after streaming completes for compatibility.
- Frontend consumes `speak_response_chunk`, appends tokens into a pending assistant message, and shows subtle typing dots while `isStreaming`.
- Frontend ignores the legacy `speak_response` when chunks were already received (prevents duplicate assistant messages).
