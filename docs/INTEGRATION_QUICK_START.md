# Integration Quick Start Guide

## Current State Summary

### ✅ What Works
- **HTTP API**: Frontend ↔ Backend via REST (`/api/speak`, `/api/command`)
- **Memory Systems**: Backend has full memory API (Neural Cortex, Vital Vaults, Vector KB)
- **Core Services**: LLM, System Access, Home Automation all functional

### ⚠️ What's Missing
- **WebSocket**: No real-time communication
- **Memory UI**: Frontend doesn't use memory endpoints
- **Streaming**: No progressive response rendering
- **Desktop**: No Tauri/Electron packaging

---

## Quick Implementation Checklist

### Phase 1: WebSocket (2-3 days)

**Backend** (`phoenix-web`):
```bash
# 1. Add dependency
cd phoenix-web
# Add to Cargo.toml: actix-web-actors = "4"

# 2. Create websocket.rs
# (See full code in ORCHESTRATOR_INTEGRATION_REPORT.md)

# 3. Add route in main.rs
.service(web::resource("/ws").route(web::get().to(websocket_handler)))
```

**Frontend** (`frontend_desktop`):
```bash
# 1. Create services/websocketService.ts
# (See full code in ORCHESTRATOR_INTEGRATION_REPORT.md)

# 2. Integrate in App.tsx
import { WebSocketService } from './services/websocketService';
const ws = new WebSocketService('ws://localhost:8888/ws');
```

### Phase 2: Memory Integration (1-2 days)

**Frontend**:
```bash
# 1. Create services/memoryService.ts
# (See full code in ORCHESTRATOR_INTEGRATION_REPORT.md)

# 2. Create components/MemoryBrowser.tsx
# (See full code in ORCHESTRATOR_INTEGRATION_REPORT.md)

# 3. Add to App.tsx
import { MemoryBrowser } from './components/MemoryBrowser';
```

### Phase 3: Streaming (1 day)

**Backend**:
```rust
// Add SSE endpoint
.service(web::resource("/api/speak/stream")
    .route(web::post().to(api_speak_stream)))
```

**Frontend**:
```typescript
// Use EventSource or WebSocket streaming
const eventSource = new EventSource('/api/speak/stream');
```

### Phase 4: Desktop (3-5 days)

**Tauri Setup**:
```bash
cd frontend_desktop
npm install @tauri-apps/api
npx tauri init
```

---

## Key Files to Modify

### Backend
- `phoenix-web/Cargo.toml` - Add `actix-web-actors`
- `phoenix-web/src/main.rs` - Add WebSocket route
- `phoenix-web/src/websocket.rs` - New file (WebSocket handler)

### Frontend
- `frontend_desktop/services/websocketService.ts` - New file
- `frontend_desktop/services/memoryService.ts` - New file
- `frontend_desktop/components/MemoryBrowser.tsx` - New file
- `frontend_desktop/App.tsx` - Integrate WebSocket and memory

---

## API Endpoints Reference

### Core
- `POST /api/speak` - Chat message
- `POST /api/command` - Execute command
- `GET /api/status` - System status

### Memory
- `POST /api/memory/store` - Store memory
- `GET /api/memory/get/{key}` - Get memory
- `GET /api/memory/search?q={query}` - Search memories
- `POST /api/memory/vector/store` - Store vector memory
- `GET /api/memory/vector/search?q={query}&k={k}` - Semantic search

### System
- `POST /api/system/exec` - Execute command
- `POST /api/system/read-file` - Read file
- `POST /api/system/write-file` - Write file

### WebSocket (to be added)
- `ws://localhost:8888/ws` - WebSocket connection

---

## Testing

```bash
# Backend
cargo test -p phoenix-web

# Frontend
cd frontend_desktop
npm test

# Integration
# Start backend: cargo run -p phoenix-web
# Start frontend: npm run dev
# Test WebSocket: wscat -c ws://localhost:8888/ws
```

---

## Next Steps

1. **Start with WebSocket** - Enables real-time features
2. **Add Memory Integration** - Leverages existing backend capabilities
3. **Implement Streaming** - Better UX for long responses
4. **Desktop Packaging** - Native app experience

See `ORCHESTRATOR_INTEGRATION_REPORT.md` for full details.
