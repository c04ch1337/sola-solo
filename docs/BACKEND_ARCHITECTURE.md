# Phoenix Backend - Comprehensive Architecture & Implementation Documentation

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [High-Level Architecture Diagrams](#high-level-architecture-diagrams)
4. [Low-Level Implementation Details](#low-level-implementation-details)
5. [Core Components Deep Dive](#core-components-deep-dive)
6. [API Endpoints Reference](#api-endpoints-reference)
7. [Command Routing System](#command-routing-system)
8. [Module Reference Table](#module-reference-table)
9. [Why This Design?](#why-this-design)
10. [What It Does](#what-it-does)
11. [How To Use](#how-to-use)
12. [Use Case Examples](#use-case-examples)
13. [Future Enhancements](#future-enhancements)

---

## Executive Summary

The **Phoenix Backend** is a sophisticated, multi-layered web service built with Rust and Actix-web that serves as the central orchestration hub for Phoenix AGI. It provides a unified HTTP API, intelligent command routing, comprehensive memory management, system access control, ecosystem orchestration, and seamless integration with external services.

**Key Capabilities:**
- **Unified API Gateway**: Single entry point for all frontend requests
- **Intelligent Command Routing**: Routes commands to appropriate subsystems
- **Memory Management**: Multi-layered memory system (episodic, semantic, vector)
- **System Access Control**: Full system access with security controls
- **Ecosystem Orchestration**: Import and manage external GitHub repositories
- **Google Integration**: OAuth-based Google services integration
- **LLM Orchestration**: Context-aware LLM interactions
- **Relationship Dynamics**: Emotional intelligence and relationship management

**Design Philosophy:**
- **Single Responsibility**: Each subsystem handles one concern
- **Loose Coupling**: Subsystems communicate through well-defined interfaces
- **High Performance**: Async/await throughout, minimal blocking
- **Type Safety**: Full Rust type system for compile-time guarantees
- **Extensibility**: Easy to add new subsystems and endpoints

---

## Architecture Overview

### System Layers

```
┌─────────────────────────────────────────────────────────────┐
│              Frontend (React/Vite)                          │
│  - Port 3000 (Dev)                                          │
│  - HTTP Requests to /api/*                                  │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   │ HTTP/HTTPS
                   │ /api/*, /health
                   │
┌──────────────────▼──────────────────────────────────────────┐
│              Phoenix Web Backend (Actix-web)                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  HTTP Server (Port 8888)                            │   │
│  │  - Request Routing                                  │   │
│  │  - Command Processing                               │   │
│  │  - Response Generation                              │   │
│  └───────────────┬─────────────────────────────────────┘   │
│                  │                                          │
│  ┌───────────────▼─────────────────────────────────────┐   │
│  │  AppState (Shared State)                            │   │
│  │  - VitalOrganVaults (Memory)                        │   │
│  │  - NeuralCortexStrata (Memory Layers)               │   │
│  │  - ContextEngine (Context Builder)                 │   │
│  │  - LLMOrchestrator (LLM Access)                    │   │
│  │  - SystemAccessManager (System Access)              │   │
│  │  - EcosystemManager (Repo Management)               │   │
│  │  - GoogleManager (Google Services)                  │   │
│  │  - PhoenixIdentityManager (Identity)                │   │
│  │  - Partnership (Relationship State)                 │   │
│  │  - VectorKB (Vector Memory)                         │   │
│  └─────────────────────────────────────────────────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   │ Internal Calls
                   │
┌──────────────────▼──────────────────────────────────────────┐
│              Core Services                                    │
│  - Memory Systems (Vaults, Cortex, Vector KB)              │
│  - LLM Services (OpenRouter Integration)                    │
│  - System Services (File, Process, Network)                 │
│  - External Services (Google, GitHub, Ecosystem)           │
└─────────────────────────────────────────────────────────────┘
```

### Component Hierarchy

```
phoenix-web (Main Server)
    │
    ├── HTTP Server (Actix-web)
    │   ├── Request Handlers
    │   ├── Middleware (CORS, Logger)
    │   └── Static File Serving
    │
    ├── AppState (Shared State)
    │   ├── VitalOrganVaults
    │   ├── NeuralCortexStrata
    │   ├── ContextEngine
    │   ├── LLMOrchestrator
    │   ├── SystemAccessManager
    │   ├── EcosystemManager
    │   ├── GoogleManager
    │   ├── PhoenixIdentityManager
    │   ├── Partnership
    │   └── VectorKB
    │
    ├── Command Router
    │   ├── Command Parser
    │   ├── Route Dispatcher
    │   └── Response Builder
    │
    └── API Endpoints
        ├── Core Endpoints (/api/command, /api/speak)
        ├── Memory Endpoints (/api/memory/*)
        ├── System Endpoints (/api/system/*)
        ├── Ecosystem Endpoints (/api/ecosystem/*)
        ├── Google Endpoints (/api/google/*)
        └── Utility Endpoints (/health, /api/status)
```

---

## High-Level Architecture Diagrams

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Client Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Frontend   │  │   CLI Tools  │  │  External   │          │
│  │   (React)    │  │              │  │   Services  │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
└─────────┼──────────────────┼──────────────────┼─────────────────┘
          │                  │                  │
          │ HTTP/HTTPS       │ HTTP/HTTPS       │ HTTP/HTTPS
          │                  │                  │
┌─────────▼──────────────────▼──────────────────▼─────────────────┐
│                    Phoenix Web Backend                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              HTTP API Gateway (Port 8888)                │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │   │
│  │  │   Request    │  │   Command    │  │   Response   │  │   │
│  │  │   Router     │→ │   Router     │→ │   Builder    │  │   │
│  │  └──────────────┘  └──────┬───────┘  └──────────────┘  │   │
│  └────────────────────────────┼────────────────────────────┘   │
│                               │                                  │
│  ┌────────────────────────────▼────────────────────────────┐   │
│  │                    AppState Manager                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │   │
│  │  │   Memory     │  │   LLM        │  │   System    │  │   │
│  │  │   Systems    │  │   Services   │  │   Access    │  │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │   │
│  │  │  Ecosystem   │  │   Google    │  │  Identity   │  │   │
│  │  │  Manager     │  │   Manager   │  │   Manager    │  │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             │ Internal Calls
                             │
┌────────────────────────────▼─────────────────────────────────────┐
│                      Core Services Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Memory     │  │   LLM        │  │   System     │          │
│  │   Vaults     │  │   OpenRouter │  │   Access     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Vector KB  │  │   GitHub     │  │   Google     │          │
│  │              │  │   API        │  │   API        │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

### Request Flow

```
User Request
    │
    ▼
┌─────────────────┐
│  HTTP Server    │  (Actix-web, Port 8888)
│  - CORS Check   │
│  - Auth Check   │
│  - Route Match  │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  Request Handler│  (api_command, api_memory_store, etc.)
│  - Parse Body    │
│  - Validate      │
│  - Extract Data  │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  Command Router │  (command_to_response_json)
│  - Parse Command │
│  - Route to      │
│    Subsystem     │
└────────┬─────────┘
         │
         ├──→ Memory System
         ├──→ LLM Orchestrator
         ├──→ System Access
         ├──→ Ecosystem Manager
         ├──→ Google Manager
         └──→ Default (LLM)
         │
         ▼
┌─────────────────┐
│  Subsystem      │  (VitalOrganVaults, LLMOrchestrator, etc.)
│  - Process       │
│  - Execute       │
│  - Return Result │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  Response       │  (JSON Response)
│  - Format       │
│  - Serialize    │
│  - Return       │
└─────────────────┘
```

### Command Routing Flow

```
Command Input
    │
    ▼
┌─────────────────┐
│  Normalize      │  (trim, normalize line endings)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Parse Prefix   │  (lowercase, check prefix)
└────────┬────────┘
         │
         ├──→ "google " → GoogleManager
         ├──→ "ecosystem " → EcosystemManager
         ├──→ "system " → SystemAccessManager
         ├──→ "code " → CodeAnalysis
         ├──→ "exec " → UnrestrictedExecution
         ├──→ "help" → Built-in Help
         ├──→ "status" → Built-in Status
         └──→ Default → LLM Orchestrator
         │
         ▼
┌─────────────────┐
│  Execute        │  (subsystem-specific logic)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Build Response │  (JSON format)
└────────┬────────┘
         │
         ▼
    JSON Response
```

### Memory System Architecture

```
Memory Request
    │
    ▼
┌─────────────────┐
│  Memory Router  │  (episodic, semantic, vector)
└────────┬────────┘
         │
         ├──→ Episodic → VitalOrganVaults
         │                └──→ Soul Vault (key-value)
         │
         ├──→ Semantic → NeuralCortexStrata
         │                └──→ EPM Layer (episodic)
         │
         └──→ Vector → VectorKB
                      └──→ Embeddings (semantic search)
         │
         ▼
┌─────────────────┐
│  Context Engine │  (build_memory_context)
│  - Retrieve     │
│  - Format       │
│  - Combine      │
└────────┬────────┘
         │
         ▼
    Context String
```

---

## Low-Level Implementation Details

### AppState Structure

```rust
#[derive(Clone)]
struct AppState {
    // Memory Systems
    vaults: Arc<VitalOrganVaults>,              // Key-value memory storage
    neural_cortex: Arc<NeuralCortexStrata>,      // Layered memory system
    context_engine: Arc<ContextEngine>,          // Context builder
    vector_kb: Option<Arc<vector_kb::VectorKB>>, // Vector embeddings
    
    // Core Services
    llm: Option<Arc<LLMOrchestrator>>,           // LLM access
    system: Arc<SystemAccessManager>,            // System access
    phoenix_identity: Arc<PhoenixIdentityManager>, // Identity management
    
    // Relationship & Integration
    relationship: Arc<Mutex<Partnership>>,       // Relationship state
    google: Option<GoogleManager>,               // Google services
    ecosystem: Arc<EcosystemManager>,            // Repository management
    
    // Metadata
    version: String,                              // Version string
}
```

**Key Design Decisions:**
- **Arc (Atomically Reference Counted)**: Shared ownership across async tasks
- **Option<T>**: Optional services (LLM, VectorKB, Google) can be disabled
- **Mutex<Partnership>**: Thread-safe relationship state
- **Clone**: AppState is cloned for each request handler

### Request Handler Pattern

```rust
async fn api_memory_store(
    state: web::Data<AppState>,
    body: web::Json<MemoryStoreRequest>,
) -> Result<HttpResponse, ApiError> {
    // 1. Validate input
    let key = body.key.trim();
    if key.is_empty() {
        return Err(ApiError::bad_request("Empty key."));
    }
    
    // 2. Execute operation
    state
        .vaults
        .store_soul(key, &body.value)
        .map_err(|e| ApiError::internal(format!("Failed to store memory: {e}")))?;
    
    // 3. Return response
    Ok(HttpResponse::Ok().json(StatusOkResponse { status: "ok" }))
}
```

**Pattern:**
1. Extract and validate input
2. Execute operation on AppState
3. Handle errors with ApiError
4. Return HttpResponse with JSON

### Command Router Implementation

```rust
async fn command_to_response_json(state: &AppState, command: &str) -> serde_json::Value {
    let cmd = normalize_command(command);
    if cmd.is_empty() {
        return json!({"type": "error", "message": "Empty command."});
    }
    
    let lower = cmd.to_ascii_lowercase();
    
    // Route by prefix
    if lower.starts_with("google ") {
        return handle_google_command(state, &cmd).await;
    }
    
    if lower.starts_with("ecosystem ") {
        return handle_ecosystem_command(state, &cmd).await;
    }
    
    if lower.starts_with("system ") {
        return handle_system_command(state, &cmd).await;
    }
    
    // ... more routes ...
    
    // Default: route to LLM
    handle_llm_command(state, &cmd).await
}
```

**Routing Strategy:**
- Prefix-based routing (fast, simple)
- Case-insensitive matching
- Fallback to LLM for unknown commands
- Special handlers for built-in commands (help, status)

### Memory Context Building

```rust
async fn build_memory_context(
    state: &AppState,
    user_input: &str,
    emotion_hint: Option<&str>,
) -> String {
    let mut context = String::new();
    
    // 1. Relational memories from Soul Vault
    if let Some(memory) = state.vaults.recall_soul("dad:last_soft_memory") {
        context.push_str(&format!("Last soft memory: {}\n", memory));
    }
    
    // 2. Episodic memories from Neural Cortex
    let episodic = state.neural_cortex.recall_prefix("epm:dad:", 8);
    if !episodic.is_empty() {
        context.push_str("Recent interactions:\n");
        for (key, layer) in episodic {
            if let MemoryLayer::EPM(text) = layer {
                context.push_str(&format!("- {}\n", text));
            }
        }
    }
    
    // 3. Semantic search if needed
    if let Some(kb) = state.vector_kb.as_ref() {
        if let Ok(results) = kb.semantic_search(user_input, 3).await {
            if !results.is_empty() {
                context.push_str("Relevant memories:\n");
                for r in results {
                    context.push_str(&format!("- ({:.0}%) {}\n", r.score * 100.0, r.text));
                }
            }
        }
    }
    
    context
}
```

**Context Building Strategy:**
1. Retrieve relational memories (Soul Vault)
2. Retrieve episodic memories (Neural Cortex)
3. Perform semantic search if needed (Vector KB)
4. Format and combine into context string
5. Inject into LLM prompt

---

## Core Components Deep Dive

### 1. HTTP Server (Actix-web)

**Location**: `phoenix-web/src/main.rs`

**Responsibilities:**
- Start HTTP server on port 8888
- Configure CORS for frontend
- Register API endpoints
- Serve static files (production)
- Request/response handling

**Configuration:**
```rust
HttpServer::new(move || {
    let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://127.0.0.1:3000")
        .supports_credentials();
    
    App::new()
        .app_data(web::Data::new(state.clone()))
        .wrap(middleware::Logger::default())
        .wrap(cors)
        .service(web::scope("/api")
            .service(web::resource("/command").route(web::post().to(api_command)))
            // ... more endpoints
        )
})
.bind("127.0.0.1:8888")?
.run()
.await
```

### 2. Command Router

**Location**: `phoenix-web/src/main.rs::command_to_response_json`

**Responsibilities:**
- Parse and normalize commands
- Route to appropriate subsystem
- Handle built-in commands
- Default to LLM for unknown commands

**Routing Table:**
| Prefix | Handler | Description |
|--------|---------|-------------|
| `google ` | GoogleManager | Google services commands |
| `ecosystem ` | EcosystemManager | Repository management |
| `system ` | SystemAccessManager | System operations |
| `code ` | CodeAnalysis | Code analysis operations |
| `exec ` | UnrestrictedExecution | Shell command execution |
| `help` | Built-in | Help message |
| `status` | Built-in | System status |
| (default) | LLMOrchestrator | LLM conversation |

### 3. Memory Systems

#### VitalOrganVaults
- **Purpose**: Key-value memory storage
- **Location**: `vital_organ_vaults` crate
- **Operations**: `store_soul()`, `recall_soul()`, `forget_soul()`
- **Storage**: Sled database

#### NeuralCortexStrata
- **Purpose**: Layered memory system
- **Location**: `neural_cortex_strata` crate
- **Layers**: EPM (Episodic), Semantic, Long-term
- **Operations**: `recall_prefix()`, `store()`

#### VectorKB
- **Purpose**: Semantic search via embeddings
- **Location**: `vector_kb` crate
- **Operations**: `semantic_search()`, `add_memory()`, `all()`
- **Storage**: Vector database (Qdrant/Chroma)

### 4. LLM Orchestrator

**Location**: `llm_orchestrator` crate

**Responsibilities:**
- Manage LLM API connections (OpenRouter)
- Build context-aware prompts
- Handle LLM responses
- Manage conversation state

**Integration:**
- OpenRouter API for LLM access
- Context injection from memory systems
- Relationship-aware responses

### 5. System Access Manager

**Location**: `system_access` crate

**Responsibilities:**
- Execute shell commands
- Read/write files
- Process management
- Security controls

**Access Levels:**
- **Tier 1**: Restricted (workspace only)
- **Tier 2**: Unrestricted (full system access)

### 6. Ecosystem Manager

**Location**: `ecosystem_manager` crate

**Responsibilities:**
- Import GitHub repositories
- Detect build systems
- Build and run services
- Manage service lifecycle

**Supported Build Systems:**
- Cargo (Rust)
- npm/yarn (Node.js)
- pip (Python)
- Make
- Docker
- Maven/Gradle (Java)

### 7. Google Manager

**Location**: `phoenix-web/src/google.rs`

**Responsibilities:**
- OAuth 2.0 authentication
- Google API integration
- Gmail, Drive, Calendar access
- Token management

---

## API Endpoints Reference

### Core Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| GET | `/health` | Health check | None | `{"status": "ok"}` |
| GET | `/api/status` | System status | None | `{"status": "online", "llm_status": "online", "version": "...", "archetype": "..."}` |
| GET | `/api/name` | Phoenix name | None | `{"name": "Phoenix"}` |
| POST | `/api/command` | Execute command | `{"command": "..."}` | `{"type": "...", "message": "..."}` |
| POST | `/api/speak` | Direct LLM interaction | `{"user_input": "...", "dad_emotion_hint": "...", "mode": "..."}` | JSON string response |

### Memory Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| POST | `/api/memory/store` | Store memory | `{"key": "...", "value": "..."}` | `{"status": "ok"}` |
| GET | `/api/memory/get/{key}` | Get memory | Path param | `{"key": "...", "value": "..."}` |
| GET | `/api/memory/search` | Search memories | `?q=...&limit=...` | `{"items": [...], "count": N}` |
| DELETE | `/api/memory/delete/{key}` | Delete memory | Path param | `{"status": "ok"}` |
| POST | `/api/memory/vector/store` | Store vector memory | `{"text": "...", "metadata": {...}}` | `{"status": "ok", "id": "..."}` |
| GET | `/api/memory/vector/search` | Semantic search | `?q=...&k=...` | `{"results": [...], "count": N}` |
| GET | `/api/memory/vector/all` | List all vector memories | None | `{"entries": [...], "count": N}` |

### System Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| GET | `/api/system/status` | System access status | None | `{"full_access_granted": true, "self_modification_enabled": true}` |
| POST | `/api/system/exec` | Execute shell command | `{"command": "...", "cwd": "..."}` | `{"exit_code": 0, "stdout": "...", "stderr": "..."}` |
| POST | `/api/system/read-file` | Read file | `{"path": "..."}` | `{"path": "...", "content": "..."}` |
| POST | `/api/system/write-file` | Write file | `{"path": "...", "content": "..."}` | `{"status": "ok"}` |

### Ecosystem Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| POST | `/api/ecosystem/import` | Import repository | `{"owner": "...", "repo": "...", "branch": "..."}` | `RepoMetadata` |
| GET | `/api/ecosystem/list` | List repositories | None | `[RepoMetadata, ...]` |
| GET | `/api/ecosystem/{id}` | Get repository | Path param | `RepoMetadata` |
| POST | `/api/ecosystem/{id}/build` | Build repository | Path param | `{"status": "success", "output": "..."}` |
| POST | `/api/ecosystem/{id}/start` | Start service | Path param | `{"status": "started", "message": "..."}` |
| POST | `/api/ecosystem/{id}/stop` | Stop service | Path param | `{"status": "stopped", "message": "..."}` |
| DELETE | `/api/ecosystem/{id}` | Remove repository | Path param | `{"status": "ok"}` |

### Google Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| GET | `/api/google/auth/start` | Start OAuth flow | None | `{"auth_url": "..."}` |
| GET | `/api/google/oauth2/callback` | OAuth callback | Query params | HTML page |

### Utility Endpoints

| Method | Endpoint | Description | Request | Response |
|--------|----------|-------------|---------|----------|
| GET | `/api/command-registry` | Command registry | None | JSON command list |
| GET | `/api/evolution/status` | Evolution pipeline status | None | `{"github_configured": true, ...}` |

---

## Command Routing System

### Command Format

Commands are text strings that can be:
1. **Built-in commands**: `help`, `status`
2. **Prefixed commands**: `google ...`, `ecosystem ...`, `system ...`
3. **Natural language**: Anything else routes to LLM

### Routing Logic

```rust
async fn command_to_response_json(state: &AppState, command: &str) -> serde_json::Value {
    let cmd = normalize_command(command);
    let lower = cmd.to_ascii_lowercase();
    
    // Special commands
    if lower == "help" { return help_response(); }
    if lower == "status" { return status_response(state).await; }
    
    // Prefixed commands
    if lower.starts_with("google ") { return google_handler(state, &cmd).await; }
    if lower.starts_with("ecosystem ") { return ecosystem_handler(state, &cmd).await; }
    if lower.starts_with("system ") { return system_handler(state, &cmd).await; }
    if lower.starts_with("code ") { return code_handler(state, &cmd).await; }
    if lower.starts_with("exec ") { return exec_handler(state, &cmd).await; }
    
    // Default: LLM
    return llm_handler(state, &cmd).await;
}
```

### Command Examples

**Built-in:**
- `help` → Returns help message
- `status` → Returns system status

**Google:**
- `google auth start` → Start OAuth flow
- `google gmail list` → List emails

**Ecosystem:**
- `ecosystem my-repo build` → Build repository
- `ecosystem my-repo start` → Start service

**System:**
- `system exec ls -la` → Execute shell command
- `system read-file /path/to/file` → Read file

**LLM (default):**
- `What's the weather?` → Routes to LLM
- `Tell me a story` → Routes to LLM

---

## Module Reference Table

| Module | Description | Port/Protocol | Location | Dependencies |
|--------|-------------|---------------|-----------|--------------|
| **phoenix-web** | Main HTTP server | 8888/HTTP | `phoenix-web/src/main.rs` | Actix-web, all subsystems |
| **VitalOrganVaults** | Key-value memory | Internal | `vital_organ_vaults` | Sled database |
| **NeuralCortexStrata** | Layered memory | Internal | `neural_cortex_strata` | Memory layers |
| **ContextEngine** | Context builder | Internal | `context_engine` | Memory systems |
| **LLMOrchestrator** | LLM access | External (OpenRouter) | `llm_orchestrator` | HTTP client |
| **SystemAccessManager** | System operations | Internal | `system_access` | OS APIs |
| **EcosystemManager** | Repository management | Internal | `ecosystem_manager` | Git, process management |
| **GoogleManager** | Google services | External (Google APIs) | `phoenix-web/src/google.rs` | OAuth, HTTP client |
| **PhoenixIdentityManager** | Identity management | Internal | `phoenix_identity` | VitalOrganVaults |
| **Partnership** | Relationship state | Internal | `relationship_dynamics` | Memory systems |
| **VectorKB** | Vector embeddings | Internal | `vector_kb` | Vector database |
| **Vital Pulse Collector** | Telemetry service | 5002/HTTP | `vital_pulse_collector` | Actix-web, Sled |
| **Synaptic Pulse Distributor** | Config updates | 5003/WebSocket | `synaptic_pulse_distributor` | Actix-web, WebSocket |

### Port Summary

| Service | Port | Protocol | Env Var | Status |
|---------|------|----------|---------|--------|
| Phoenix Web UI | 8888 | HTTP | `PHOENIX_WEB_BIND` | ✅ Active |
| Vital Pulse Collector | 5002 | HTTP | `TELEMETRIST_BIND` | ✅ Optional |
| Synaptic Pulse Distributor | 5003 | WebSocket | `PULSE_DISTRIBUTOR_BIND` | ✅ Optional |
| Frontend Dev Server | 3000 | HTTP | `VITE_PORT` | ✅ Dev only |

---

## Why This Design?

### 1. Single Entry Point

**Why**: Centralized API gateway simplifies frontend integration and provides consistent error handling.

**Benefits:**
- Single CORS configuration
- Unified authentication
- Consistent response format
- Easy to add middleware

### 2. Command-Based Routing

**Why**: Text commands are natural for users and flexible for routing.

**Benefits:**
- Natural language interface
- Easy to extend with new commands
- Backward compatible
- LLM can understand commands

### 3. Shared AppState

**Why**: All subsystems need access to shared resources.

**Benefits:**
- Efficient resource sharing
- Consistent state across requests
- Easy to add new subsystems
- Type-safe access

### 4. Async/Await Throughout

**Why**: High concurrency and non-blocking I/O.

**Benefits:**
- Handle many concurrent requests
- Non-blocking database operations
- Efficient LLM API calls
- Better resource utilization

### 5. Optional Services

**Why**: Not all services are required for basic operation.

**Benefits:**
- Can run without LLM (for testing)
- Can disable Google integration
- Vector KB is optional
- Flexible deployment

### 6. Type Safety

**Why**: Rust's type system prevents many runtime errors.

**Benefits:**
- Compile-time error detection
- No null pointer exceptions
- Memory safety guarantees
- Better IDE support

---

## What It Does

### Core Functionality

1. **HTTP API Server**
   - Serves REST API endpoints
   - Handles CORS for frontend
   - Serves static files in production
   - Health check endpoint

2. **Command Processing**
   - Parses and routes commands
   - Executes subsystem operations
   - Returns formatted responses
   - Handles errors gracefully

3. **Memory Management**
   - Stores and retrieves memories
   - Semantic search capabilities
   - Context building for LLM
   - Multi-layered memory system

4. **LLM Integration**
   - Context-aware prompts
   - Relationship-aware responses
   - Emotion hint support
   - Conversation state management

5. **System Access**
   - Shell command execution
   - File read/write operations
   - Process management
   - Security controls

6. **Ecosystem Management**
   - GitHub repository import
   - Build system detection
   - Service lifecycle management
   - Command execution

7. **Google Integration**
   - OAuth 2.0 authentication
   - Gmail, Drive, Calendar access
   - Token management
   - API integration

---

## How To Use

### Starting the Server

```bash
# Development
cargo run --bin phoenix-web

# Production
cargo run --release --bin phoenix-web
```

**Environment Variables:**
```bash
PHOENIX_WEB_BIND=127.0.0.1:8888  # Server bind address
OPENROUTER_API_KEY=...            # LLM API key
VECTOR_KB_ENABLED=true            # Enable vector KB
GOOGLE_OAUTH_CLIENT_ID=...        # Google OAuth
GOOGLE_OAUTH_CLIENT_SECRET=...    # Google OAuth
```

### API Usage Examples

**Health Check:**
```bash
curl http://127.0.0.1:8888/health
# {"status":"ok"}
```

**Execute Command:**
```bash
curl -X POST http://127.0.0.1:8888/api/command \
  -H "Content-Type: application/json" \
  -d '{"command": "help"}'
```

**Store Memory:**
```bash
curl -X POST http://127.0.0.1:8888/api/memory/store \
  -H "Content-Type: application/json" \
  -d '{"key": "test", "value": "Hello, world!"}'
```

**Search Memories:**
```bash
curl "http://127.0.0.1:8888/api/memory/search?q=test&limit=10"
```

**Import Repository:**
```bash
curl -X POST http://127.0.0.1:8888/api/ecosystem/import \
  -H "Content-Type: application/json" \
  -d '{"owner": "octocat", "repo": "Hello-World", "branch": "main"}'
```

### Command Usage

**Built-in Commands:**
```
help    - Show help message
status  - Show system status
```

**Google Commands:**
```
google auth start        - Start OAuth flow
google gmail list        - List emails
google drive list        - List files
```

**Ecosystem Commands:**
```
ecosystem my-repo build  - Build repository
ecosystem my-repo start  - Start service
ecosystem my-repo stop   - Stop service
```

**System Commands:**
```
system exec ls -la       - Execute shell command
system read-file /path   - Read file
system write-file /path  - Write file
```

**LLM Commands (default):**
```
What's the weather?      - Routes to LLM
Tell me a story          - Routes to LLM
```

---

## Use Case Examples

### Use Case 1: Basic Chat Interaction

**Scenario**: User sends a message through the frontend.

**Flow:**
1. Frontend sends `POST /api/command` with `{"command": "Hello!"}`
2. Backend receives request in `api_command` handler
3. Command router normalizes and routes to LLM
4. Context engine builds memory context
5. LLM orchestrator generates response
6. Response stored in episodic memory
7. JSON response returned to frontend

**Code Path:**
```
api_command() 
  → command_to_response_json() 
    → build_memory_context() 
    → llm.speak() 
    → store_episodic_memory()
```

### Use Case 2: Memory Storage and Retrieval

**Scenario**: User stores a memory and later searches for it.

**Flow:**
1. Frontend sends `POST /api/memory/store` with key-value
2. Backend stores in VitalOrganVaults
3. Later, frontend sends `GET /api/memory/search?q=keyword`
4. Backend searches Soul Vault with prefix
5. Returns matching memories

**Code Path:**
```
api_memory_store() 
  → vaults.store_soul()

api_memory_search() 
  → vaults.recall_prefix()
```

### Use Case 3: Ecosystem Repository Import

**Scenario**: User imports a GitHub repository and builds it.

**Flow:**
1. Frontend sends `POST /api/ecosystem/import` with repo info
2. EcosystemManager clones repository
3. Detects build system (Cargo, npm, etc.)
4. Returns repository metadata
5. User sends `POST /api/ecosystem/{id}/build`
6. EcosystemManager executes build command
7. Returns build output

**Code Path:**
```
api_ecosystem_import() 
  → ecosystem.import_repo() 
    → git clone 
    → detect_build_system()

api_ecosystem_build() 
  → ecosystem.build_repo() 
    → execute build command
```

### Use Case 4: System File Operations

**Scenario**: User reads and writes files through DevTools.

**Flow:**
1. Frontend sends `POST /api/system/read-file` with path
2. SystemAccessManager reads file
3. Returns file content
4. User modifies content
5. Frontend sends `POST /api/system/write-file` with path and content
6. SystemAccessManager writes file
7. Returns success

**Code Path:**
```
api_system_read_file() 
  → system.read_file()

api_system_write_file() 
  → system.write_file()
```

### Use Case 5: Google OAuth Integration

**Scenario**: User connects Google account for Gmail access.

**Flow:**
1. Frontend sends `GET /api/google/auth/start`
2. GoogleManager generates OAuth URL
3. User redirected to Google OAuth
4. User authorizes
5. Google redirects to `/api/google/oauth2/callback`
6. GoogleManager exchanges code for token
7. Token stored for future API calls

**Code Path:**
```
api_google_auth_start() 
  → google.auth_start()

api_google_oauth2_callback() 
  → google.auth_callback() 
    → exchange_token() 
    → store_token()
```

### Use Case 6: Vector Memory Semantic Search

**Scenario**: User searches for similar memories using semantic search.

**Flow:**
1. Frontend sends `GET /api/memory/vector/search?q=query&k=5`
2. VectorKB performs semantic search
3. Returns top K similar memories with scores
4. Frontend displays results

**Code Path:**
```
api_memory_vector_search() 
  → vector_kb.semantic_search() 
    → embedding search 
    → similarity scoring
```

---

## Future Enhancements

### Phase 1: Performance

1. **Response Caching**
   - Cache frequent LLM responses
   - Cache memory search results
   - TTL-based invalidation

2. **Connection Pooling**
   - Database connection pools
   - HTTP client connection pools
   - LLM API connection reuse

3. **Async Batching**
   - Batch memory operations
   - Batch LLM requests
   - Parallel subsystem calls

### Phase 2: Features

1. **WebSocket Support**
   - Real-time command streaming
   - Live status updates
   - Push notifications

2. **Authentication & Authorization**
   - JWT token authentication
   - Role-based access control
   - API key management

3. **Rate Limiting**
   - Per-endpoint rate limits
   - Per-user rate limits
   - DDoS protection

### Phase 3: Scalability

1. **Horizontal Scaling**
   - Load balancing
   - Stateless design
   - Shared state management

2. **Database Optimization**
   - Query optimization
   - Indexing strategies
   - Connection pooling

3. **Monitoring & Observability**
   - Metrics collection
   - Distributed tracing
   - Performance monitoring

### Phase 4: Integration

1. **Additional Services**
   - Slack integration
   - Discord integration
   - Email notifications

2. **Plugin System**
   - Dynamic plugin loading
   - Plugin API
   - Third-party plugins

3. **Multi-Tenancy**
   - User isolation
   - Resource quotas
   - Billing integration

---

## Conclusion

The Phoenix Backend is a robust, extensible web service that provides a unified API for all Phoenix AGI capabilities. Its modular design, intelligent command routing, and comprehensive memory management make it a powerful foundation for building AI applications.

**Key Strengths:**
- ✅ Single entry point for all operations
- ✅ Intelligent command routing
- ✅ Comprehensive memory management
- ✅ Extensible architecture
- ✅ Type-safe implementation
- ✅ High performance (async/await)
- ✅ Optional service support

**Architecture Highlights:**
- Modular design with clear separation of concerns
- Shared state management via AppState
- Prefix-based command routing
- Context-aware LLM interactions
- Multi-layered memory system
- Comprehensive API coverage

The backend is production-ready and designed to scale with Phoenix AGI's growth.

