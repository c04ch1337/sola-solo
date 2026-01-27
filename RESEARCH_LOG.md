# RESEARCH LOG — The Relational Autopilot (Phases 1–15)

Related:
- Training curriculum: [`TRAINING_CURRICULUM.md`](TRAINING_CURRICULUM.md)
- Project overview: [`README.md`](README.md)

This document archives the completed **Safe-Space Counselor** build arc (Phases 1–15): a local-first, closed-loop, bio-digital system that turns **environmental friction + episodic emotion logs + semantic context** into actionable relational interventions.

---

## Executive Summary

The Safe-Space Counselor is a **cybernetic feedback loop** designed to reduce relational harm during physiological or cognitive “flooding” states.

**Loop:**

1. **Perception** — capture *episodic* emotional state (grief-stage + intensity + energy + context tags) plus *techno-somatic* proxy signals (CPU load / temperature).
2. **Cognition** — synthesize across time windows (narrative summaries, correlations, hotspots) and incorporate *semantic life context* (Scratchpad).
3. **Intervention** — enforce “regulatory brakes”, provide grounding exercises, and recommend cooling behaviors to reduce arousal and restore the Window of Tolerance.
4. **Ubiquity** — mobile PWA provides offline-first capture and sync; pairing UX eliminates manual LAN IP transcription.

**Outcome:** A vertically-integrated “Safe-Space Counselor” that does not merely log, but actively **governs** the user’s capacity to communicate safely.

---

## Architecture Snapshot (as of Phase 15)

| Layer | Component | Tech | Primary Function |
|---|---|---|---|
| Persistence | **Soul Vault** | Rust (local storage, encrypted-ready) | Durable emotional + relational records |
| API Gateway | **Phoenix Web** | Rust + Actix-web | `/api/*` surface for counselor, memory, analytics |
| Perception | `env_sensor` + event payloads | Rust | CPU load + best-effort thermal context for each event |
| Cognition | Narrative + correlations + hotspots | Rust | Windowed summaries + tag correlations + contextual hotspots |
| Intervention | Regulatory Brake + grounding | Rust backend + Desktop UI | Enforced pause + tailored calming exercise |
| Desktop Hub | Orchestrator UI | React/Vite (desktop frontend) | High-resolution entry, dashboards, export, resonance simulation |
| Mobile Bridge | L9 Mobile PWA | React/Vite (mobile) | Offline-first queue + background sync to counselor endpoint |
| Pairing | QR in Rust logs | Rust | Zero-manual-entry pairing of mobile→backend base URL |

### Minimal data model

**Episodic event (“GriefEvent”)**

- `timestamp_ms`
- `stage` (Denial / Anger / Bargaining / Depression / Acceptance)
- `intensity` (0–100)
- `energy_level` (0–100)
- `context_tags` (e.g., Work / Partner / Health)
- optional `text`
- **techno-somatic**: `system_load` (0–100), optional `temperature_c`

**Semantic context (“Scratchpad”)**

- persistent narrative note used to explain / contextualize event windows

---

## Phase Timeline (1–15)

This timeline is written as a functional narrative (not a git-history reconstruction) so it remains readable even after refactors.

### Phases 1–5: Core logging + NVC engine + readiness interlock

**Goal:** Capture high-resolution emotional data and enable immediate “repair language” generation.

- **Core check-in logging** (stage + intensity + energy + context tags)
- **NVC Script Engine**: Observation → Feeling → Need → Request, stored for longitudinal learning
- **HALT Readiness** pre-flight: Hungry / Angry / Lonely / Tired checks before significant communication

### Phases 6–10: Resonance simulation + export + regulatory brake

**Goal:** Move from writing to *governance*.

- **Resonance simulator**: dry-run “how this might land” using partner persona + tone heuristics
- **Markdown export**: weekly reports for reflection + pattern detection
- **High-resolution dashboards**: stage distributions, intensity/energy trends, tag filters
- **Regulatory Brake**: when risk is elevated, enforce a time-bound pause and offer grounding

### Phases 11–15: Techno-somatic sensing + semantic memory + unified correlation + predictive cooling + QR pairing

**Goal:** Close the loop: perception → cognition → intervention → ubiquity.

- **Techno-somatic sensing**: attach CPU/thermal proxy signals to events
- **Semantic memory bridge**: integrate persistent Scratchpad context into narratives
- **Unified correlation**: identify tag-level triggers and contextual hotspots
- **Predictive cooling**: best-effort cooling recommendations based on system stress
- **QR pairing**: emit a QR code in Rust logs to eliminate manual IP entry for mobile pairing

---

## Theoretical Frameworks (codified)

### Non-Violent Communication (NVC — Marshall Rosenberg)

The system’s script engine encodes NVC as a **structured transformation** from reactive content into a repair-capable message:

1. **Observation** (non-evaluative)
2. **Feeling** (named affect)
3. **Need** (universal value / unmet need)
4. **Request** (concrete, doable, present-tense)

### Gottman Method: Flooding + repair impossibility under high arousal

The system treats “flooding” as the state where repair becomes implausible. It approximates flooding using:

- **self-reported intensity / energy**
- **contextual triggers**
- **system-stress proxy** (CPU/thermal) as a coarse arousal indicator

### Window of Tolerance

Operationalized as:

- **regulated**: low-to-moderate intensity + adequate energy
- **hyper-aroused**: high intensity + low energy and/or high system stress
- **hypo-aroused**: low energy + disengagement cues (future expansion)

### HALT (Hungry / Angry / Lonely / Tired)

Used as a checksum before “Proceed to Copy / Send”. The goal is not moral policing—it's *capacity gating*.

### Cybernetic feedback loops

The architecture implements **closed-loop control**:

- **Sensor**: events + environment
- **Controller**: analytics + rules
- **Actuator**: brakes + grounding + cooling recommendations
- **Plant**: user + device + relationship environment

---

## Key API Surface (core endpoints)

These endpoints form the “mechanical interface” for the Safe-Space Counselor.

### Counselor (`/api/counselor/*`)

| Method | Endpoint | Purpose |
|---|---|---|
| POST | `/api/counselor/events` | Persist grief-stage event + intensity/energy/tags + system stress |
| POST | `/api/counselor/scripts` | Persist NVC scripts for longitudinal analysis |
| GET | `/api/counselor/grief-stats` | Aggregate events for dashboards (counts + per-day averages) |
| GET | `/api/counselor/narrative` | Windowed supportive synthesis with Scratchpad context |
| POST | `/api/counselor/resonate` | Persona-based “how it might land” simulation |
| POST | `/api/counselor/readiness` | HALT-based readiness assessment |
| GET | `/api/counselor/export` | Markdown export (events + scripts + readiness) |
| GET | `/api/counselor/analytics/correlations` | Per-tag correlations + hotspots |
| GET | `/api/counselor/intervention` | Grounding exercise tuned to risk score |
| GET | `/api/counselor/cooling-recommendations` | Best-effort cooling suggestions |

### Memory (`/api/memory/*`)

| Method | Endpoint | Purpose |
|---|---|---|
| POST | `/api/memory/store` | Store key-value semantic memory |
| GET | `/api/memory/get/{key}` | Retrieve a key-value memory |
| GET | `/api/memory/search` | Prefix search |
| DELETE | `/api/memory/delete/{key}` | Delete key-value memory |
| POST | `/api/memory/vector/store` | Store vector/embedding memory |
| GET | `/api/memory/vector/search` | Semantic search |
| GET | `/api/memory/vector/all` | List vector entries |

### Analytics (`/api/analytics/*`)

| Method | Endpoint | Purpose |
|---|---|---|
| POST | `/api/analytics/track` | Event tracking (opt-in telemetry / UX analytics) |

---

## Feature Matrix (Phases 1–15)

| Capability | Description | Primary Surface |
|---|---|---|
| Episodic L9 logging | Stage + intensity + energy + tags + text | Desktop UI + Mobile PWA → `POST /api/counselor/events` |
| Techno-somatic context | CPU load / best-effort thermal per event | Backend `env_sensor` → stored with event |
| Semantic scratchpad | Persistent life-narrative context used in summaries | Memory API + counselor narrative synthesis |
| Narrative synthesis | 3-sentence supportive summary over rolling window | `GET /api/counselor/narrative` |
| Correlations + hotspots | Tag correlations + context note hotspots | `GET /api/counselor/analytics/correlations` |
| NVC scripts | OFNR generator + storage | `POST /api/counselor/scripts` |
| Readiness interlock | HALT pre-flight | `POST /api/counselor/readiness` |
| Regulatory Brake | Enforced pause + grounding exercise | UI + `GET /api/counselor/intervention` |
| Export | Markdown report for reflection | `GET /api/counselor/export` |
| Offline-first mobile sync | Queue while offline + retry/backoff | Mobile PWA sync store |
| Zero-entry pairing | QR code in logs to avoid manual IP typing | Rust logs / pairing routine |

---

## Bare Metal Setup (re-deploy checklist)

This is a pragmatic “get it running again” checklist.

### 1) Backend

- Install Rust toolchain (stable)
- Configure env vars (see `.env.example`)
- Start backend (Actix-web) on port `8888`

### 2) Desktop frontend

- Install Node.js LTS
- `cd frontend_desktop`
- `npm install`
- `npm run dev`

### 3) Mobile PWA

- `cd frontend_mobile`
- `npm install`
- `npm run dev` (or build + serve)
- Pair to backend base URL (via QR or manual override)

### 4) Verify loop integrity

- Log an event (desktop or mobile)
- Confirm it persists via dashboard / narrative
- Trigger intervention via risk score path
- Export report and verify Markdown renders

---

## Notes / Next Research Modules (queued)

1. **Relational Ghost** — roleplay recipient response to NVC scripts (pre-send empathy simulator)
2. **Biometric integration** — HRV / wearable Bluetooth inputs to replace best-effort CPU proxies
3. **Project Zenit** — single-binary distribution and one-click deployment (e.g., Tauri)

