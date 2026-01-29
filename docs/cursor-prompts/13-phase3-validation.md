```text
Validate Phase 3 token-by-token streaming after any changes.

Backend (phoenix-web/src/websocket.rs):
- Confirm speak handler emits speak_response_chunk with done:false, final done:true
- Legacy speak_response sent after stream ends

Frontend (frontend_desktop):
- Pending message created on send
- Chunks appended live
- Typing dots while isStreaming
- Finalize on done:true + trigger EPM store
- 10s fallback removes empty bubble if no chunks

Output:
- wscat test sequence (send speak → see chunks → done)
- Frontend test steps (send long message → watch typing → check no duplicates)
- MemoryBrowser check: new EPM entry after reply
- Any fixes if streaming broken
```
