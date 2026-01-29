# Backend Rust Crates Audit Report (Security / Performance / Maintainability)

**Scope**: Rust workspace crates, excluding UI/frontend projects (`frontend_desktop/`, `phoenix-desktop-tauri/`).

## Automated checks (workspace)

- `cargo test --workspace`: PASS (but most crates have **0 tests**).
- `cargo clippy --workspace --all-targets`: PASS with warnings.
  - Example: clippy `useless_format` warning in [`phoenix-web/src/websocket.rs`](phoenix-web/src/websocket.rs:424).
- Dependency vulnerability scan: **not run** (tooling not confirmed installed).

## CI automation

- Added GitHub Actions workflow for rustfmt + clippy (deny warnings) + test + check, plus best-effort cargo-audit and a report-only pattern scan:
  - [`.github/workflows/security.yml`](.github/workflows/security.yml:1)

## Triage summary (highest risk first)

### 1) `phoenix-web` (Actix HTTP + WebSocket server)

**Entrypoint**: [`phoenix-web/src/main.rs`](phoenix-web/src/main.rs:1)

**Security – HIGH RISK**

Most likely issues (based on code inspection + risk signals):

1) **Remote Code Execution surfaces exposed**
   - WebSocket `command` messages call `exec_shell()` directly: [`handle_message()`](phoenix-web/src/websocket.rs:277) → [`exec_shell()`](system_access/src/lib.rs:416).
   - REST endpoints exist for exec/read/write: [`api_system_exec()`](phoenix-web/src/main.rs:821), [`api_system_read_file()`](phoenix-web/src/main.rs:843), [`api_system_write_file()`](phoenix-web/src/main.rs:853).
   - There is **no explicit auth** layer visible on these endpoints in [`HttpServer::new()`](phoenix-web/src/main.rs:3261).

2) **Execution gating is effectively “open” by default**
   - `SystemAccessManager::new()` grants full access + self-modification at startup: [`SystemAccessManager::new()`](system_access/src/lib.rs:324).
   - That means the checks inside [`exec_shell()`](system_access/src/lib.rs:416) are satisfied even when Tier flags are not set.
   - Net effect: if the server is reachable by an attacker on the host network, they can likely execute commands.

3) **Untrusted input flows into high-privilege operations**
   - WebSocket/REST accept arbitrary strings (commands, file paths) without normalization/allowlisting.
   - WebSocket `project_context` is passed as `cwd` (working directory) to `exec_shell()` at [`phoenix-web/src/websocket.rs`](phoenix-web/src/websocket.rs:318). This appears to be a semantic mismatch (“project_context” is not a filesystem path), and also a potential path traversal vector.

4) **`unsafe` environment variable mutation**
   - `unsafe { std::env::set_var/remove_var }` used in config mutation: [`api_config_set()`](phoenix-web/src/main.rs:718).
   - In Rust 2024+, this is `unsafe` because it can cause data races if other threads read env concurrently.
   - Risk: undefined behavior in multi-threaded runtime + unpredictable configuration.

5) **Prompt injection/secret handling**
   - Server reads `SECRET_AGENDA` and injects into prompt: [`command_to_response_json()`](phoenix-web/src/main.rs:1577).
   - Needs careful handling to avoid exfiltration and ensure auditing.

**Performance – MEDIUM/HIGH RISK**

- **Blocking process execution inside async**: [`exec_shell()`](system_access/src/lib.rs:416) uses `std::process::Command::output()` which blocks a runtime thread.
- **Mutex held across `.await` in WebSocket speak flow**: `state.llm.lock().await` then `.await` on `llm.speak(...)` inside the lock: [`handle_message()`](phoenix-web/src/websocket.rs:277). This can serialize all WS speak requests and can block config updates that also need the same lock.

**Maintainability – MEDIUM RISK**

- Very large single-file server (`phoenix-web/src/main.rs`) with many unrelated concerns (config, identity, memory, system access, ecosystem, OAuth).
- Minimal test coverage for endpoints and policy.

**Suggested immediate logs to validate assumptions** (no behavior changes yet)

- Log every call to exec/read/write endpoints including remote IP, operation, and whether Tier flags/security-gate allowed it.
- Log WebSocket `command` operations with conn_id and origin.
- Log config changes (which keys changed, never values for secrets).

**Mitigations implemented (post-audit hardening)**

- **WebSocket command execution is now Tier-2-only** and will return an `insufficient_access` error unless `MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION=true`.
  - Implemented at [`handle_message()`](phoenix-web/src/websocket.rs:277).
- **WebSocket command execution now also requires per-connection consent** (in addition to Tier 2).
  - Grant/revoke via WS message: `{ "type": "system", "action": "grant" }` / `{ "type": "system", "action": "revoke" }`.
  - Implemented at [`handle_message()`](phoenix-web/src/websocket.rs:277).
- Added **WS structured diagnostics** (peer + conn_id + msg_type) without logging payload contents.
  - Connection logging at [`websocket_handler()`](phoenix-web/src/websocket.rs:179).
  - Message-type logging at [`handle_message()`](phoenix-web/src/websocket.rs:277).
- Fixed a performance footgun: avoid holding the LLM mutex across `.await` in WS speak path.
  - See [`handle_message()`](phoenix-web/src/websocket.rs:277).
- Removed the semantic mismatch where WS `project_context` was used as a working directory for `exec_shell()`.
  - See [`handle_message()`](phoenix-web/src/websocket.rs:277).

---

### 2) `system_access` (privileged OS access)

**Core**: [`SystemAccessManager`](system_access/src/lib.rs:308)

**Security – HIGH RISK**

- Defaults to privileged state: [`SystemAccessManager::new()`](system_access/src/lib.rs:324) grants full access and self-modification automatically.
- Tier flags (`MASTER_ORCHESTRATOR_FULL_ACCESS`, `MASTER_ORCHESTRATOR_UNRESTRICTED_EXECUTION`) are treated as bypasses, but the default gate grant already bypasses most protections.

**Performance – MEDIUM**

- Uses `Command::output()` (blocking). Consider `tokio::process::Command` or `spawn_blocking`.

**Maintainability – MEDIUM**

- Security model is currently confusing: tier flags + runtime gate + default grants. Hard to reason about and easy to misconfigure.

---

### 3) `cerebrum_nexus` (core orchestration / reasoning)

**Entrypoint**: [`cerebrum_nexus/src/lib.rs`](cerebrum_nexus/src/lib.rs:1)

**Security – MEDIUM**

- Primarily a library; main risk is how it’s used by network-facing crates.

**Performance – UNKNOWN (needs profiling)**

- No clear hot-path profiling yet; needs targeted benchmarks around LLM orchestration and memory operations.

**Maintainability – MEDIUM**

- Some panics/unwraps exist, mostly in test-like code paths (e.g. in [`cerebrum_nexus/src/tool_agent.rs`](cerebrum_nexus/src/tool_agent.rs:448), [`cerebrum_nexus/src/psychological_mapping.rs`](cerebrum_nexus/src/psychological_mapping.rs:325)).

---

### 4) `code_analysis` (deep file analysis)

**Entrypoint**: [`code_analysis/src/lib.rs`](code_analysis/src/lib.rs:1)

**Security – MEDIUM/HIGH**

- Documentation asserts full file access and “master orchestrator” capabilities; if reachable from network commands, requires strict auth and path allowlisting.

---

### 5) `ethical_agent` (safety bounding)

**Entrypoint**: [`ethical_agent/src/lib.rs`](ethical_agent/src/lib.rs:1)

**Security – LOW/MEDIUM (positive)**

- Provides veto logic to block harmful output and credential-harvesting patterns.
- Risk: regex-based policy is easy to bypass; should be treated as one layer, not sole protection.

---

## Workspace-wide observations

1) **Test coverage is near-zero** across most crates (`cargo test` runs 0 tests in many crates). This is a maintainability and security risk.
2) **Highest-impact risk concentration** is at the network boundary + privileged system boundary: [`phoenix-web`](phoenix-web/src/main.rs:1) + [`system_access`](system_access/src/lib.rs:1).

## Status

This is the initial report for the first crates (`phoenix-web`, `system_access`, `cerebrum_nexus`, `code_analysis`, `ethical_agent`). Remaining crates will be reviewed next in descending risk order.
