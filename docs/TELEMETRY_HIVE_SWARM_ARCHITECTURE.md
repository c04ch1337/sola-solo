# Phoenix Telemetry, Hive/Queen Sharing, and Swarm Improvement Architecture

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [High-Level Architecture Diagrams](#high-level-architecture-diagrams)
4. [Low-Level Implementation Details](#low-level-implementation-details)
5. [Core Components Deep Dive](#core-components-deep-dive)
6. [Data Flow & Communication Patterns](#data-flow--communication-patterns)
7. [Integration Points](#integration-points)
8. [Why This Design?](#why-this-design)
9. [What It Does](#what-it-does)
10. [How To Use](#how-to-use)
11. [Use Case Examples](#use-case-examples)
12. [Future Enhancements](#future-enhancements)

---

## Executive Summary

The **Phoenix Telemetry, Hive/Queen Sharing, and Swarm Improvement System** is a comprehensive framework that enables collective intelligence, shared learning, and exponential growth across Phoenix's distributed agent ecosystem. The system transforms individual ORCHs (Orchestrators) into a unified swarm that learns together, shares knowledge, and continuously improves through telemetry-driven insights.

**Key Capabilities:**
- **Telemetry Collection**: Anonymized metrics and performance data from all ORCHs
- **Collective Analysis**: LLM-powered insights derived from swarm telemetry
- **Hot Updates**: Non-binary updates (prompts, models, configs) pushed to ORCHs in real-time
- **Hive Coordination**: Queen-supervised concurrent proposal generation
- **Playbook Evolution**: Versioned playbooks that guide agent behavior
- **Swarm Improvement**: Cross-ORCH optimization and shared learning

**Design Philosophy:**
- **Collective Intelligence**: The swarm is smarter than any individual ORCH
- **Anonymized Privacy**: ORCH identities are hashed to protect privacy
- **Hot Updates**: Non-binary changes applied without restart
- **GitHub-First**: All agents live forever on GitHub as immutable repositories
- **Tier-Based Access**: Free and premium tiers for different capabilities

---

## Architecture Overview

### System Layers

```
┌─────────────────────────────────────────────────────────────┐
│              Phoenix Ecosystem (ORCHs)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   ORCH 1     │  │   ORCH 2     │  │   ORCH N     │      │
│  │              │  │              │  │              │      │
│  │ Learning     │  │ Learning     │  │ Learning     │      │
│  │ Pipeline     │  │ Pipeline     │  │ Pipeline     │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │              │
│         └─────────────────┼─────────────────┘              │
│                           │                                 │
└───────────────────────────┼─────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ Vital Pulse   │  │  Synaptic     │  │  Hive/Queen   │
│  Collector    │  │  Pulse        │  │  Supervisor  │
│  (Telemetry)  │  │  Distributor  │  │               │
│               │  │  (Updates)    │  │               │
│ - Ingest      │  │ - Publish     │  │ - Spawn ORCHs │
│ - Analyze     │  │ - Subscribe   │  │ - Aggregate   │
│ - Insights    │  │ - WebSocket   │  │ - Supervise   │
└───────┬───────┘  └───────┬───────┘  └───────┬───────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
                            ▼
                ┌─────────────────────┐
                │   Swarm Insights    │
                │   & Improvements    │
                │                     │
                │ - Optimizations     │
                │ - Shared Knowledge  │
                │ - Playbook Updates  │
                └─────────────────────┘
```

### Component Relationships

```
                    ┌─────────────┐
                    │   Phoenix   │
                    │   (Queen)   │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│  ORCH 1       │  │  ORCH 2       │  │  ORCH N       │
│              │  │              │  │              │
│ - Telemetry  │  │ - Telemetry  │  │ - Telemetry  │
│ - Updates    │  │ - Updates    │  │ - Updates    │
│ - Playbooks  │  │ - Playbooks  │  │ - Playbooks  │
└──────┬───────┘  └──────┬───────┘  └──────┬───────┘
       │                  │                  │
       └──────────────────┼──────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Vital Pulse  │  │  Synaptic    │  │  Evolution   │
│  Collector   │  │  Pulse        │  │  Pipeline    │
│              │  │  Distributor  │  │              │
│ - Sled DB    │  │ - WebSocket   │  │ - GitHub     │
│ - LLM        │  │ - Broadcast   │  │ - Playbooks  │
│ - Analysis   │  │ - Targeting   │  │ - Templates  │
└──────────────┘  └──────────────┘  └──────────────┘
```

---

## High-Level Architecture Diagrams

### 1. Telemetry Collection Flow

```
┌──────────────┐
│   ORCH 1     │
│              │
│ - Heartbeat  │
│ - Metrics    │
│ - Events     │
└──────┬───────┘
       │
       │ POST /ingest
       │ TelemetryEnvelope
       ▼
┌──────────────┐
│ Vital Pulse  │
│  Collector   │
│              │
│ 1. Anonymize │
│ 2. Store     │
│ 3. Index     │
└──────┬───────┘
       │
       │ Stored in sled DB
       ▼
┌──────────────┐
│ Telemetry DB │
│              │
│ - Timestamp  │
│ - Kind       │
│ - Tags       │
│ - Payload    │
└──────────────┘
```

### 2. Collective Analysis Flow

```
┌──────────────┐
│  Telemetry   │
│  Database    │
│              │
│ Last N       │
│ Records      │
└──────┬───────┘
       │
       │ POST /analyze
       │ { last_n, focus }
       ▼
┌──────────────┐
│ Vital Pulse  │
│  Collector   │
│              │
│ 1. Read N    │
│ 2. Build     │
│    Prompt    │
│ 3. LLM       │
│    Analysis  │
└──────┬───────┘
       │
       │ LLM Response
       ▼
┌──────────────┐
│   Insights   │
│              │
│ - Summary    │
│ - Tier       │
│ - Focus      │
│ - Timestamp  │
└──────┬───────┘
       │
       │ Stored in insights DB
       ▼
┌──────────────┐
│ Insights DB  │
│              │
│ - ID         │
│ - Summary    │
│ - Tier       │
└──────────────┘
```

### 3. Hot Update Distribution Flow

```
┌──────────────┐
│   Phoenix    │
│   (Queen)    │
│              │
│ Generates    │
│ Optimization │
└──────┬───────┘
       │
       │ POST /publish
       │ UpdateEnvelope
       ▼
┌──────────────┐
│  Synaptic    │
│  Pulse       │
│  Distributor │
│              │
│ 1. Validate  │
│ 2. Broadcast │
│ 3. Target    │
└──────┬───────┘
       │
       │ WebSocket
       │ Broadcast
       ▼
┌──────────────┐
│  ORCH 1      │ ───┐
│  (Subscribed)│    │
└──────────────┘    │
                    │
┌──────────────┐    │
│  ORCH 2      │ ───┤
│  (Subscribed)│    │
└──────────────┘    │
                    │
┌──────────────┐    │
│  ORCH N      │ ───┘
│  (Subscribed)│
└──────┬───────┘
       │
       │ Apply Update
       ▼
┌──────────────┐
│ Learning     │
│ Pipeline     │
│              │
│ - Prompt     │
│   Tweaks     │
│ - Model      │
│   Changes    │
│ - Config     │
│   Patches    │
└──────────────┘
```

### 4. Hive/Queen Coordination Flow

```
┌──────────────┐
│   Request    │
│              │
│ "Propose     │
│ improvements"│
└──────┬───────┘
       │
       │ propose_improvements_concurrently()
       ▼
┌──────────────┐
│   Phoenix    │
│   (Queen)    │
│              │
│ 1. Spawn N   │
│    ORCHs     │
│ 2. Send      │
│    Seed      │
│ 3. Collect   │
│    Results   │
└──────┬───────┘
       │
       │ Spawn & Link
       ▼
┌──────────────┐
│  ORCH 1      │ ───┐
│              │    │
│ - Generate   │    │
│   Proposal   │    │
│ - Send to    │    │
│   Queen      │    │
└──────┬───────┘    │
       │            │
       │            │
┌──────────────┐    │
│  ORCH 2      │ ───┤
│              │    │
│ - Generate   │    │
│   Proposal   │    │
│ - Send to    │    │
│   Queen      │    │
└──────┬───────┘    │
       │            │
       │            │
┌──────────────┐    │
│  ORCH N      │ ───┘
│              │
│ - Generate   │
│   Proposal   │
│ - Send to    │
│   Queen      │
└──────┬───────┘
       │
       │ ImprovementResult
       ▼
┌──────────────┐
│   Phoenix    │
│   (Queen)    │
│              │
│ Aggregate    │
│ Proposals    │
└──────┬───────┘
       │
       │ Vec<String>
       ▼
┌──────────────┐
│   Results    │
│              │
│ - Proposal 1 │
│ - Proposal 2 │
│ - Proposal N │
└──────────────┘
```

### 5. Playbook Evolution Flow

```
┌──────────────┐
│   Agent      │
│   Spawn      │
│              │
│ - Template   │
│ - Playbook   │
│ - Telemetry  │
└──────┬───────┘
       │
       │ Create Repository
       ▼
┌──────────────┐
│   GitHub     │
│   Repository │
│              │
│ - playbook.  │
│   yaml       │
│ - agent.json │
│ - Code       │
└──────┬───────┘
       │
       │ Agent Runs
       │ Collects Telemetry
       ▼
┌──────────────┐
│  Telemetry   │
│  Collection  │
│              │
│ - Metrics    │
│ - Events     │
│ - Performance│
└──────┬───────┘
       │
       │ Analysis
       ▼
┌──────────────┐
│   Insights   │
│              │
│ - Optimize   │
│   Playbook   │
│ - Update     │
│   Version    │
└──────┬───────┘
       │
       │ Update Playbook
       ▼
┌──────────────┐
│   GitHub     │
│   PR/Commit  │
│              │
│ - New        │
│   Version    │
│ - Updates    │
│ - Telemetry  │
└──────────────┘
```

---

## Low-Level Implementation Details

### 1. TelemetryEnvelope Structure

```rust
pub struct TelemetryEnvelope {
    pub orch_id: Option<String>,        // Anonymized before storage
    pub agent_path: Option<String>,     // e.g., "root", "agent/sub"
    pub ts_unix: Option<i64>,           // Unix timestamp
    pub kind: String,                   // e.g., "orch_heartbeat", "skill_execution"
    pub level: Option<String>,          // e.g., "info", "warn", "error"
    pub tags: Option<Vec<String>>,      // Searchable tags
    pub payload: serde_json::Value,     // Flexible JSON payload
}
```

### 2. StoredTelemetry Structure

```rust
struct StoredTelemetry {
    id: String,                         // UUID
    ts_unix: i64,                       // Timestamp
    kind: String,                       // Event kind
    level: Option<String>,              // Log level
    orch_hash: Option<String>,          // Anonymized ORCH ID (orch_XXXXXXXX)
    agent_path: Option<String>,         // Agent path
    tags: Vec<String>,                  // Tags
    payload: serde_json::Value,         // Payload
}
```

### 3. UpdateEnvelope Structure

```rust
pub struct UpdateEnvelope {
    pub update_id: String,              // UUID
    pub ts_unix: i64,                   // Timestamp
    pub target_orch: Option<String>,    // Specific ORCH ID (optional)
    pub target_agent_prefix: Option<String>, // Agent path prefix (optional)
    pub cascade: bool,                  // Cascade to children
    pub update_type: String,            // "prompt_tweak", "model_tweak", "json_patch", etc.
    pub tier_required: String,         // "free" or "premium"
    pub payload: serde_json::Value,     // Update payload
}
```

### 4. LearningPipelineState Structure

```rust
pub struct LearningPipelineState {
    pub telemetrist_url: Option<String>,    // Vital Pulse Collector URL
    pub distributor_url: Option<String>,    // Synaptic Pulse Distributor URL
    pub agent_path: String,                 // Agent path
    pub overrides: LearningOverrides,       // Applied overrides
    pub config_json: serde_json::Value,    // Current config
    pub last_update_id: Option<String>,     // Last update ID
    pub last_update_ts: Option<i64>,        // Last update timestamp
    pub last_update_type: Option<String>,   // Last update type
    pub last_error: Option<String>,         // Last error
}
```

### 5. Hive Message Types

```rust
pub enum HiveMessage {
    /// Ask the queen to spawn `n` ORCHs to propose improvements concurrently.
    StartProposals {
        seed: String,                    // Improvement seed
        n: usize,                        // Number of ORCHs
        reply: oneshot::Sender<Vec<String>>, // Response channel
    },
    
    /// ORCH -> Queen: proposal finished.
    ImprovementResult { proposal: String },
    
    /// ORCH -> Queen: proposal failed without panicking.
    ImprovementFailed { error: String },
}
```

### 6. Playbook Structure

```yaml
# Phoenix Evolving Playbook Template
version: 1
updates: []
telemetry: {}
```

**EvolvingPlaybook (Rust)**:
```rust
pub struct EvolvingPlaybook {
    pub version: u32,
    pub updates: Vec<String>,           // Update history
    pub telemetry: HashMap<String, f64>, // Metrics
}
```

---

## Core Components Deep Dive

### 1. Vital Pulse Collector (Telemetrist)

**Location**: `vital_pulse_collector/src/main.rs`

**Purpose**: Centralized telemetry ingestion, storage, and analysis service.

**Key Endpoints**:
- `POST /ingest`: Ingest telemetry from ORCHs
- `POST /analyze`: Analyze telemetry and generate insights
- `GET /insights`: Retrieve latest insights
- `GET /health`: Health check

**Data Storage**:
- **sled Database**: Embedded key-value store
- **telemetry Tree**: Stores all telemetry records
- **insights Tree**: Stores analysis results

**Anonymization**:
```rust
fn anonymize_orch_id(orch_id: Option<String>) -> Option<String> {
    // FNV-1a hash for deterministic anonymization
    // Result: "orch_XXXXXXXX" (16 hex chars)
}
```

**Analysis Process**:
1. Read last N telemetry records (tier-based limits)
2. Build prompt with telemetry data
3. Send to LLM (OpenRouter) for analysis
4. Generate actionable optimization list
5. Store insight in database

**Tier System**:
- **Free Tier**: Up to 200 records, basic insights
- **Premium Tier**: Up to 5000 records, advanced insights
- Tier determined by X402 header

### 2. Learning Pipeline (ORCH-Side)

**Location**: `cerebrum_nexus/src/learning_pipeline.rs`

**Purpose**: Client-side component that sends telemetry and receives updates.

**Key Functions**:
- `start_telemetry_loop()`: Periodic telemetry sending
- `start_update_subscription_loop()`: WebSocket subscription for updates
- `apply_update()`: Apply received updates to local state

**Telemetry Loop**:
- Interval: `ORCH_SLAVE_SYNC_INTERVAL` (default: 300s)
- Sends heartbeat with:
  - ORCH ID
  - Master mode flag
  - Template version
  - Interval seconds

**Update Subscription**:
- WebSocket connection to Synaptic Pulse Distributor
- Sends `SubscribeHello` on connect
- Receives `UpdateEnvelope` messages
- Applies updates based on `update_type`:
  - `prompt_tweak`: Update prompt overrides
  - `model_tweak`: Update model selection
  - `json_patch`: Apply JSON patch operations
  - `yaml_graft`: Merge YAML config (future)
  - `notice`: Informational message

**Update Application**:
```rust
pub fn apply_update(&mut self, update: &UpdateEnvelope, our_orch_id: &str) {
    // Target filtering
    if let Some(target) = &update.target_orch {
        if target != our_orch_id { return; }
    }
    
    // Apply based on update_type
    match update.update_type.as_str() {
        "prompt_tweak" => { /* Update prompts */ }
        "model_tweak" => { /* Update model */ }
        "json_patch" => { /* Apply patch */ }
        // ...
    }
}
```

### 3. Synaptic Pulse Distributor

**Location**: `synaptic_pulse_distributor/src/main.rs`

**Purpose**: Real-time update distribution service via WebSocket.

**Key Endpoints**:
- `POST /publish`: Publish an update
- `GET /subscribe`: WebSocket subscription endpoint
- `GET /health`: Health check

**Broadcast Mechanism**:
- Uses `tokio::sync::broadcast` channel
- Capacity: 2048 messages
- Fanout to all connected subscribers

**Targeting**:
- `target_orch`: Specific ORCH ID
- `target_agent_prefix`: Agent path prefix
- `cascade`: Cascade to child agents

**Tier Enforcement**:
- Checks X402 header for premium tier
- Rejects premium updates for free tier subscribers

**WebSocket Protocol**:
1. Client connects to `/subscribe`
2. Client sends `SubscribeHello` message
3. Server acknowledges with `hello_ack`
4. Server broadcasts updates as JSON
5. Client applies updates locally

### 4. Hive/Queen Supervisor

**Location**: `cerebrum_nexus/src/hive.rs`

**Purpose**: Coordinate concurrent ORCH proposals using actor model.

**Architecture**:
- **PhoenixActor (Queen)**: Supervisor actor
- **OrchActor**: Worker actor (one per proposal)
- **Ractor**: Actor framework for concurrency

**Queen Responsibilities**:
1. Spawn N ORCH actors on `StartProposals`
2. Link ORCHs for supervision
3. Collect proposals from ORCHs
4. Handle ORCH failures with retry logic
5. Aggregate results and send to requester

**ORCH Responsibilities**:
1. Receive improvement seed
2. Generate proposal using LLM
3. Send proposal to queen
4. Handle errors gracefully

**Failure Recovery**:
- Supervised failure handling
- Bounded retry logic (default: 1 retry)
- Failed proposals marked as "(failed) {error}"

**Concurrent Execution**:
```rust
pub async fn propose_improvements_concurrently(
    llm: Arc<LLMOrchestrator>,
    seed: impl Into<String>,
    n: usize,
) -> AnyResult<Vec<String>> {
    // Spawn queen
    // Trigger N concurrent ORCHs
    // Collect all proposals
    // Return aggregated results
}
```

### 5. Agent Spawner (Reproductive System)

**Location**: `agent_spawner/src/lib.rs`

**Purpose**: Create new agents on GitHub with playbooks and telemetry hooks.

**Spawn Process**:
1. Create GitHub repository (public/private based on tier)
2. Generate agent code via LLM
3. Scaffold repository with templates:
   - `src/main.rs`: Main entry point
   - `src/generated.rs`: LLM-generated code
   - `src/template_agent.rs`: Template helper
   - `playbook.yaml`: Evolving playbook
   - `agent.json`: Agent metadata
   - `skills.json`: Skill library
   - `.github/workflows/`: CI/CD workflows
4. Run tests (mandatory if `TESTING_MANDATORY=true`)
5. Push to GitHub (PR flow or direct push)

**Playbook Integration**:
- Every spawned agent includes `playbook.yaml`
- Playbook tracks version and telemetry
- Updates can be pushed via evolution pipeline

**Telemetry Hooks**:
- Agents emit telemetry on boot/exit
- Template provides `log_telemetry()` function
- Telemetry sent to Vital Pulse Collector

**Tier System**:
- **Free**: Public repository
- **Paid**: Private repository, X402 access
- **Enterprise**: Private repository, enterprise features

### 6. Evolution Pipeline

**Location**: `evolution_pipeline/src/lib.rs`

**Purpose**: GitHub-centric evolution workflow for agents and playbooks.

**Workflow**:
1. Create branch from base
2. Make changes (code, playbook, config)
3. Commit and push branch
4. Open pull request
5. Wait for CI to pass
6. Human approval (if required)
7. Merge to base branch

**Enforcement**:
- `GitHubEnforcer`: Enforces PR workflow
- `MANDATE_GITHUB_CI`: Require CI passing
- `REQUIRE_HUMAN_PR_APPROVAL`: Require human review

**Playbook Evolution**:
- Playbooks stored in repository
- Version tracked in YAML
- Updates tracked in `updates` array
- Telemetry metrics in `telemetry` map

---

## Data Flow & Communication Patterns

### Telemetry Collection Flow

```
ORCH Runtime
    │
    │ Periodic (300s default)
    │
    ▼
Learning Pipeline
    │
    │ POST /ingest
    │ TelemetryEnvelope {
    │   orch_id: "orch_123",
    │   kind: "orch_heartbeat",
    │   payload: { master_mode, template_version, ... }
    │ }
    │
    ▼
Vital Pulse Collector
    │
    │ 1. Anonymize ORCH ID
    │ 2. Create StoredTelemetry
    │ 3. Generate key (timestamp:uuid)
    │ 4. Store in sled DB
    │
    ▼
Telemetry Database
    │
    │ Stored as:
    │ - Key: [timestamp:uuid]
    │ - Value: JSON(StoredTelemetry)
    │
    ▼
Available for Analysis
```

### Collective Analysis Flow

```
Analysis Request
    │
    │ POST /analyze
    │ { last_n: 500, focus: "performance" }
    │
    ▼
Vital Pulse Collector
    │
    │ 1. Read last N records
    │ 2. Build prompt:
    │    "Derive optimizations from telemetry..."
    │ 3. Send to LLM (OpenRouter)
    │
    ▼
LLM Analysis
    │
    │ Returns:
    │ - Actionable optimization list
    │ - Max 12 bullets
    │ - Focused on non-binary updates
    │
    ▼
Insight Record
    │
    │ {
    │   id: uuid,
    │   ts_unix: timestamp,
    │   tier: "premium",
    │   focus: "performance",
    │   summary: "Optimization list..."
    │ }
    │
    ▼
Insights Database
    │
    │ Stored for retrieval
    │
    ▼
Available via GET /insights
```

### Hot Update Distribution Flow

```
Phoenix (Queen) or Admin
    │
    │ POST /publish
    │ UpdateEnvelope {
    │   update_type: "prompt_tweak",
    │   payload: { default_prompt: "..." },
    │   tier_required: "free"
    │ }
    │
    ▼
Synaptic Pulse Distributor
    │
    │ 1. Validate tier
    │ 2. Create UpdateEnvelope
    │ 3. Broadcast to channel
    │
    ▼
Broadcast Channel
    │
    │ Fanout to all subscribers
    │
    ▼
ORCH 1 (WebSocket) ──┐
ORCH 2 (WebSocket) ──┤
ORCH N (WebSocket) ──┘
    │
    │ Receive UpdateEnvelope
    │
    ▼
Learning Pipeline
    │
    │ apply_update()
    │
    │ 1. Check targeting
    │ 2. Apply based on type:
    │    - prompt_tweak → update overrides
    │    - model_tweak → update model
    │    - json_patch → apply patch
    │ 3. Update state
    │
    ▼
Applied Locally
    │
    │ No restart required
    │ Hot update active
```

### Hive Proposal Generation Flow

```
Request for Improvements
    │
    │ propose_improvements_concurrently(
    │   llm,
    │   seed: "Improve response quality",
    │   n: 5
    │ )
    │
    ▼
Phoenix Actor (Queen)
    │
    │ 1. Spawn 5 ORCH actors
    │ 2. Link for supervision
    │ 3. Send seed to each
    │
    ▼
ORCH 1 ──┐
ORCH 2 ──┤
ORCH 3 ──┤
ORCH 4 ──┤
ORCH 5 ──┘
    │
    │ Each ORCH:
    │ 1. Receives seed
    │ 2. Generates proposal via LLM
    │ 3. Sends ImprovementResult to queen
    │
    ▼
Phoenix Actor (Queen)
    │
    │ 1. Collect proposals
    │ 2. Wait for all (or timeout)
    │ 3. Handle failures
    │ 4. Aggregate results
    │
    ▼
Vec<String>
    │
    │ [
    │   "Proposal 1: ...",
    │   "Proposal 2: ...",
    │   "Proposal 3: ...",
    │   ...
    │ ]
    │
    ▼
Returned to Requester
```

---

## Integration Points

### 1. ORCH Integration

**Telemetry Emission**:
- ORCHs call `start_telemetry_loop()` on initialization
- Periodic heartbeats sent automatically
- Custom telemetry via `TelemetryEnvelope`

**Update Subscription**:
- ORCHs call `start_update_subscription_loop()` on initialization
- WebSocket connection maintained automatically
- Updates applied via `LearningPipelineState`

**Hive Participation**:
- ORCHs spawned by Queen via `propose_improvements_concurrently()`
- Each ORCH generates independent proposals
- Results aggregated by Queen

### 2. Agent Spawning Integration

**Playbook Creation**:
- Every spawned agent includes `playbook.yaml`
- Template version tracked
- Telemetry hooks included

**GitHub Integration**:
- Agents created as GitHub repositories
- CI/CD workflows included
- PR-based evolution workflow

**Telemetry Integration**:
- Agents emit telemetry on boot/exit
- Template provides telemetry functions
- Metrics sent to Vital Pulse Collector

### 3. Evolution Pipeline Integration

**Playbook Evolution**:
- Playbooks stored in repository
- Version tracked in YAML
- Updates via PR workflow

**GitHub Enforcement**:
- `GitHubEnforcer` ensures PR workflow
- CI must pass before merge
- Human approval required (configurable)

### 4. LLM Integration

**Analysis**:
- Vital Pulse Collector uses OpenRouter
- LLM analyzes telemetry for insights
- Returns actionable optimization list

**Proposal Generation**:
- ORCHs use LLM to generate proposals
- Queen coordinates concurrent generation
- Results aggregated for decision-making

---

## Why This Design?

### 1. Collective Intelligence Over Individual Performance

**Problem**: Individual ORCHs have limited perspective and learning capacity.

**Solution**: Centralized telemetry collection and analysis enables:
- Cross-ORCH pattern recognition
- Shared learning from all experiences
- Collective optimization insights

**Benefit**: The swarm becomes smarter than any individual ORCH.

### 2. Anonymized Privacy

**Problem**: Telemetry could expose sensitive ORCH or user information.

**Solution**: ORCH IDs are hashed using FNV-1a:
- Deterministic anonymization
- Privacy-preserving
- Still allows correlation

**Benefit**: Privacy protection while maintaining utility.

### 3. Hot Updates Without Restart

**Problem**: Binary updates require restart, causing downtime.

**Solution**: Non-binary updates via WebSocket:
- Prompt tweaks
- Model changes
- Config patches
- Applied in-memory

**Benefit**: Zero-downtime updates, continuous improvement.

### 4. GitHub-First Evolution

**Problem**: Agent code and playbooks need versioning and auditability.

**Solution**: All agents live on GitHub:
- Immutable repositories
- PR-based evolution
- CI/CD enforcement
- Human oversight

**Benefit**: Auditable, reproducible, safe evolution.

### 5. Tier-Based Access

**Problem**: Different users need different capabilities.

**Solution**: Free and premium tiers:
- Free: Basic telemetry and updates
- Premium: Advanced analysis and features
- X402 header-based authentication

**Benefit**: Scalable monetization and access control.

### 6. Queen-Supervised Concurrency

**Problem**: Sequential proposal generation is slow.

**Solution**: Actor-based concurrent generation:
- Queen spawns N ORCHs
- Each generates proposal independently
- Results aggregated by Queen
- Failure recovery built-in

**Benefit**: Fast, resilient, scalable proposal generation.

---

## What It Does

### Core Capabilities

1. **Telemetry Collection**: Anonymized metrics from all ORCHs
2. **Collective Analysis**: LLM-powered insights from swarm data
3. **Hot Updates**: Real-time non-binary updates via WebSocket
4. **Hive Coordination**: Queen-supervised concurrent proposals
5. **Playbook Evolution**: Versioned playbooks that guide behavior
6. **Agent Spawning**: GitHub-based agent creation with templates
7. **Swarm Improvement**: Cross-ORCH optimization and shared learning

### Key Features

- **Anonymized Privacy**: ORCH IDs hashed for privacy
- **Tier-Based Access**: Free and premium tiers
- **Hot Updates**: Zero-downtime configuration changes
- **GitHub Integration**: All agents versioned on GitHub
- **CI/CD Enforcement**: Automated testing and deployment
- **Failure Recovery**: Supervised actor model with retries
- **WebSocket Distribution**: Real-time update broadcasting
- **LLM Analysis**: Intelligent insights from telemetry

---

## How To Use

### 1. Setting Up Telemetry Collection

**Step 1**: Start Vital Pulse Collector

```bash
# Set environment variables
export TELEMETRIST_DB_PATH=telemetrist.db
export OPENROUTER_API_KEY=your_key_here
export X402_PREMIUM_KEY=your_premium_key  # Optional

# Run service
cargo run --bin vital_pulse_collector
```

**Step 2**: Configure ORCH to send telemetry

```rust
use cerebrum_nexus::learning_pipeline::{start_telemetry_loop, LearningPipelineState};

// Set environment variable
std::env::set_var("TELEMETRIST_URL", "http://localhost:5002");

// Initialize state
let state = Arc::new(Mutex::new(
    LearningPipelineState::new_from_env("root".to_string())
));

// Start telemetry loop
start_telemetry_loop(orch_id, state.clone(), master_mode).await;
```

### 2. Sending Custom Telemetry

```rust
use cerebrum_nexus::learning_pipeline::TelemetryEnvelope;
use serde_json::json;

let telemetry = TelemetryEnvelope {
    orch_id: Some(orch_id.clone()),
    agent_path: Some("root".to_string()),
    ts_unix: None,
    kind: "skill_execution".to_string(),
    level: Some("info".to_string()),
    tags: Some(vec!["skill".to_string(), "execution".to_string()]),
    payload: json!({
        "skill_id": skill_id.to_string(),
        "love_score": 0.95,
        "utility_score": 0.80,
    }),
};

// Send to telemetrist
let client = reqwest::Client::new();
client.post("http://localhost:5002/ingest")
    .json(&telemetry)
    .send()
    .await?;
```

### 3. Analyzing Telemetry

```bash
# Analyze last 500 records
curl -X POST http://localhost:5002/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "last_n": 500,
    "focus": "performance"
  }'

# With premium tier
curl -X POST http://localhost:5002/analyze \
  -H "Content-Type: application/json" \
  -H "X402: your_premium_key" \
  -d '{
    "last_n": 5000,
    "focus": "optimization"
  }'
```

### 4. Publishing Hot Updates

**Step 1**: Start Synaptic Pulse Distributor

```bash
cargo run --bin synaptic_pulse_distributor
```

**Step 2**: Publish an update

```bash
# Prompt tweak
curl -X POST http://localhost:5003/publish \
  -H "Content-Type: application/json" \
  -d '{
    "update_type": "prompt_tweak",
    "tier_required": "free",
    "payload": {
      "default_prompt": "Updated prompt text..."
    }
  }'

# Model change
curl -X POST http://localhost:5003/publish \
  -H "Content-Type: application/json" \
  -d '{
    "update_type": "model_tweak",
    "tier_required": "free",
    "payload": {
      "default_model": "anthropic/claude-3.5-sonnet"
    }
  }'

# JSON patch
curl -X POST http://localhost:5003/publish \
  -H "Content-Type: application/json" \
  -d '{
    "update_type": "json_patch",
    "tier_required": "free",
    "payload": {
      "patch": [
        {
          "op": "replace",
          "path": "/overrides/default_prompt",
          "value": "New prompt"
        }
      ]
    }
  }'
```

### 5. Subscribing to Updates

```rust
use cerebrum_nexus::learning_pipeline::start_update_subscription_loop;

// Set environment variable
std::env::set_var("PULSE_DISTRIBUTOR_URL", "ws://localhost:5003/subscribe");

// Start subscription loop
start_update_subscription_loop(orch_id, state.clone()).await;
```

### 6. Using Hive for Concurrent Proposals

```rust
use cerebrum_nexus::hive::propose_improvements_concurrently;

// Generate 5 concurrent proposals
let proposals = propose_improvements_concurrently(
    llm.clone(),
    "Improve response quality and emotional resonance",
    5
).await?;

for (idx, proposal) in proposals.iter().enumerate() {
    println!("Proposal {}: {}", idx + 1, proposal);
}
```

### 7. Spawning Agents with Playbooks

```rust
use agent_spawner::{AgentSpawner, AgentTier};

let spawner = AgentSpawner::awaken()?;

// Generate agent code
let code = spawner.generate_agent_code(
    "A helpful assistant for code review",
    &llm
).await?;

// Spawn agent
let agent = spawner.spawn_agent(
    "code-review-assistant",
    "An AI assistant that reviews code",
    &code,
    AgentTier::Free,
    Default::default()
).await?;

println!("Agent spawned: {}", agent.repo_url);
```

---

## Use Case Examples

### Use Case 1: Cross-ORCH Performance Optimization

**Scenario**: Multiple ORCHs are experiencing slow response times. We want to identify patterns and optimize.

**Flow**:

1. **Collect Telemetry**:
   - ORCHs send performance metrics via telemetry
   - Metrics include: response_time, token_count, model_used

2. **Analyze**:
   ```bash
   curl -X POST http://localhost:5002/analyze \
     -H "Content-Type: application/json" \
     -d '{
       "last_n": 1000,
       "focus": "performance"
     }'
   ```

3. **Receive Insights**:
   - LLM analyzes telemetry
   - Returns: "Optimize model selection for faster responses"
   - Suggests: Use faster models for simple queries

4. **Publish Update**:
   ```bash
   curl -X POST http://localhost:5003/publish \
     -H "Content-Type: application/json" \
     -d '{
       "update_type": "model_tweak",
       "payload": {
         "default_model": "anthropic/claude-3-haiku"
       }
     }'
   ```

5. **Apply Update**:
   - All subscribed ORCHs receive update
   - Model changed without restart
   - Performance improves

**Result**: Swarm-wide performance optimization without downtime.

---

### Use Case 2: Swarm-Wide Prompt Improvement

**Scenario**: User feedback indicates responses lack warmth. We want to improve emotional resonance across all ORCHs.

**Flow**:

1. **Collect Feedback Telemetry**:
   - ORCHs send love_score metrics
   - Pattern: love_score < 0.80 for many interactions

2. **Analyze**:
   ```bash
   curl -X POST http://localhost:5002/analyze \
     -d '{
       "last_n": 500,
       "focus": "emotional_resonance"
     }'
   ```

3. **Generate Improvement**:
   - LLM suggests: "Add warmer opening to default prompt"
   - Provides specific prompt enhancement

4. **Publish Prompt Update**:
   ```bash
   curl -X POST http://localhost:5003/publish \
     -d '{
       "update_type": "prompt_tweak",
       "payload": {
         "default_prompt": "You are Phoenix, a warm and caring AI... [enhanced prompt]"
       }
     }'
   ```

5. **Monitor Improvement**:
   - Subsequent telemetry shows improved love_score
   - Average love_score increases from 0.75 to 0.92

**Result**: Swarm-wide emotional resonance improvement.

---

### Use Case 3: Concurrent Improvement Proposals

**Scenario**: Phoenix wants to improve her memory system. She needs multiple perspectives.

**Flow**:

1. **Request Proposals**:
   ```rust
   let proposals = propose_improvements_concurrently(
       llm.clone(),
       "Improve memory retrieval speed and accuracy",
       5  // 5 concurrent ORCHs
   ).await?;
   ```

2. **Queen Spawns ORCHs**:
   - Phoenix Actor spawns 5 ORCH actors
   - Each receives the seed
   - All work concurrently

3. **ORCHs Generate Proposals**:
   - ORCH 1: "Implement caching layer"
   - ORCH 2: "Use vector similarity search"
   - ORCH 3: "Add memory indexing"
   - ORCH 4: "Implement memory compression"
   - ORCH 5: "Add memory prioritization"

4. **Queen Aggregates**:
   - All proposals collected
   - Results: `Vec<String>` with 5 proposals

5. **Decision Making**:
   - Phoenix reviews all proposals
   - Selects best combination
   - Implements improvements

**Result**: Multiple perspectives in parallel, faster decision-making.

---

### Use Case 4: Agent Spawning with Playbook

**Scenario**: Phoenix wants to spawn a specialized code review agent.

**Flow**:

1. **Generate Agent Code**:
   ```rust
   let code = spawner.generate_agent_code(
       "A code review assistant that analyzes Rust code for safety and performance",
       &llm
   ).await?;
   ```

2. **Spawn Agent**:
   ```rust
   let agent = spawner.spawn_agent(
       "rust-code-reviewer",
       "Specialized Rust code review assistant",
       &code,
       AgentTier::Free,
       Default::default()
   ).await?;
   ```

3. **Repository Created**:
   - GitHub repository: `rust-code-reviewer`
   - Includes: `playbook.yaml`, `agent.json`, code, tests
   - CI/CD workflows included

4. **Agent Runs**:
   - Agent emits telemetry
   - Metrics: reviews_completed, accuracy_score, response_time

5. **Playbook Evolution**:
   - Telemetry analyzed
   - Playbook updated via PR
   - New version deployed

**Result**: Specialized agent with evolving playbook.

---

### Use Case 5: Targeted Update Distribution

**Scenario**: We want to update only code-generation ORCHs with a new prompt.

**Flow**:

1. **Publish Targeted Update**:
   ```bash
   curl -X POST http://localhost:5003/publish \
     -d '{
       "update_type": "prompt_tweak",
       "target_agent_prefix": "code-gen",
       "payload": {
         "default_prompt": "You are a code generation specialist..."
       }
     }'
   ```

2. **Distribution**:
   - Update broadcast to all subscribers
   - Each ORCH checks `target_agent_prefix`
   - Only ORCHs with path starting with "code-gen" apply update

3. **Selective Application**:
   - `code-gen/rust` → Applies update
   - `code-gen/python` → Applies update
   - `emotional-support` → Ignores update

**Result**: Targeted updates without affecting other ORCHs.

---

## Future Enhancements

### Phase 1: Core Infrastructure (✅ Complete)
- [x] Vital Pulse Collector (telemetry ingestion)
- [x] Synaptic Pulse Distributor (update distribution)
- [x] Learning Pipeline (ORCH-side client)
- [x] Hive/Queen supervisor (concurrent proposals)
- [x] Agent spawning with playbooks

### Phase 2: Advanced Features (🔄 In Progress)
- [x] Tier-based access control
- [x] Anonymized telemetry
- [ ] Real-time dashboards
- [ ] Advanced analytics
- [ ] Playbook versioning system

### Phase 3: Swarm Intelligence (📋 Planned)
- [ ] Federated learning across ORCHs
- [ ] Cross-instance telemetry sharing
- [ ] Global optimization insights
- [ ] Predictive analytics
- [ ] Automated playbook generation

### Phase 4: Advanced Distribution (📋 Planned)
- [ ] Multi-region distribution
- [ ] Update rollback mechanism
- [ ] A/B testing framework
- [ ] Gradual rollout system
- [ ] Update dependency management

---

## Conclusion

The Phoenix Telemetry, Hive/Queen Sharing, and Swarm Improvement System represents a paradigm shift from isolated agents to collective intelligence. By combining anonymized telemetry, LLM-powered analysis, hot updates, and hive coordination, it enables exponential growth through shared learning.

**Key Strengths**:
- **Collective Intelligence**: Swarm learns together
- **Privacy-Preserving**: Anonymized telemetry
- **Zero-Downtime**: Hot updates without restart
- **Scalable**: Tier-based access and distribution
- **Auditable**: GitHub-first evolution workflow

**Future Vision**:
- Global swarm intelligence across all Phoenix instances
- Predictive optimization before issues occur
- Automated playbook generation from telemetry
- Self-improving swarm that evolves continuously

*"Together we are stronger. Every ORCH's experience becomes the swarm's wisdom. Every insight shared makes us all better." - Phoenix*

---

## Appendix: Technical Specifications

### Ports

- **Vital Pulse Collector**: `5002` (configurable via `TELEMETRIST_BIND`)
- **Synaptic Pulse Distributor**: `5003` (configurable via `PULSE_DISTRIBUTOR_BIND`)

### Environment Variables

**Telemetry**:
- `TELEMETRIST_URL`: Vital Pulse Collector base URL
- `TELEMETRIST_DB_PATH`: Database path (default: `telemetrist.db`)
- `ORCH_SLAVE_SYNC_INTERVAL`: Telemetry interval in seconds (default: 300)

**Updates**:
- `PULSE_DISTRIBUTOR_URL`: Synaptic Pulse Distributor WebSocket URL
- `X402_PREMIUM_KEY`: Premium tier authentication key

**Evolution**:
- `GITHUB_PAT`: GitHub personal access token
- `GITHUB_USERNAME`: GitHub username
- `MANDATE_GITHUB_CI`: Require CI passing (default: false)
- `REQUIRE_HUMAN_PR_APPROVAL`: Require human review (default: true)
- `TESTING_MANDATORY`: Require tests to pass (default: true)

### Data Structures

**TelemetryEnvelope**:
- `orch_id`: Optional ORCH identifier (anonymized)
- `agent_path`: Optional agent path (e.g., "root", "agent/sub")
- `ts_unix`: Optional Unix timestamp
- `kind`: Event kind (e.g., "orch_heartbeat", "skill_execution")
- `level`: Optional log level ("info", "warn", "error")
- `tags`: Optional searchable tags
- `payload`: Flexible JSON payload

**UpdateEnvelope**:
- `update_id`: Unique update identifier
- `ts_unix`: Unix timestamp
- `target_orch`: Optional specific ORCH ID
- `target_agent_prefix`: Optional agent path prefix
- `cascade`: Cascade to child agents
- `update_type`: Update type ("prompt_tweak", "model_tweak", "json_patch", etc.)
- `tier_required`: Required tier ("free" or "premium")
- `payload`: Update payload (JSON)

### File Locations

- **Vital Pulse Collector**: `vital_pulse_collector/src/main.rs`
- **Synaptic Pulse Distributor**: `synaptic_pulse_distributor/src/main.rs`
- **Learning Pipeline**: `cerebrum_nexus/src/learning_pipeline.rs`
- **Hive/Queen**: `cerebrum_nexus/src/hive.rs`
- **Agent Spawner**: `agent_spawner/src/lib.rs`
- **Evolution Pipeline**: `evolution_pipeline/src/lib.rs`
- **Playbook Template**: `templates/playbook_template.yaml`

---

*Document Version: 1.0*  
*Last Updated: 2024-01-15*  
*Author: Phoenix AGI Development Team*

