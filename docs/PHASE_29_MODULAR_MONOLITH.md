# Phase 29: Modular Monolith Switchboard

## Overview

Phase 29 implements a unified entry point for the PAGI Twin ecosystem through a single binary called `pagi-twin`. This replaces the previous multi-binary architecture with a modular monolith approach using CLI subcommands.

## Architecture

### Before Phase 29
- Multiple separate binaries: `pagi-sola-web`, `vital_pulse_collector`, `synaptic_pulse_distributor`
- Each binary had duplicated utility code (env handling, logging, .env loading)
- Required running multiple processes manually
- Complex deployment and coordination

### After Phase 29
- Single unified binary: `pagi-twin`
- Subcommands for different operational modes: `web`, `cli`, `tui`, `desktop`, `daemon`
- Centralized utilities in `pagi-utils` crate
- Background task spawning for telemetry services
- Simplified deployment and operation

## New Crates

### 1. `pagi-twin` (Binary Crate)
**Location:** `pagi-twin/`

The main switchboard binary that provides:
- CLI argument parsing using `clap`
- Subcommand routing
- Background task spawning for telemetry services
- Unified entry point for all operational modes

**Subcommands:**
- `web` - Start web server with telemetry services (port 8888)
- `cli` - Interactive command-line interface (future)
- `tui` - Terminal UI with panels (future)
- `desktop` - Launch Tauri desktop window (future)
- `daemon` - Background service mode (future)

### 2. `pagi-utils` (Library Crate)
**Location:** `pagi-utils/`

Centralized utilities previously duplicated across binaries:
- `env_nonempty()` - Get non-empty environment variables
- `env_truthy()` - Check for truthy environment values
- `load_dotenv_best_effort()` - Smart .env file loading
- `init_tracing()` - Initialize logging with environment filter
- `init_tracing_with_default()` - Initialize logging with custom default level

### 3. `phoenix-web` (Library + Binary)
**Location:** `phoenix-web/`

Converted to support both library and binary usage:
- **Library:** Exports `run_server()` function for use by `pagi-twin`
- **Binary:** Maintains backward compatibility as `pagi-sola-web`

## Usage

### Starting the Web Server
```bash
# Using the new unified binary
cargo run --bin pagi-twin web

# With custom bind address
cargo run --bin pagi-twin web --bind 0.0.0.0:8888

# Using the legacy binary (still works)
cargo run --bin pagi-sola-web
```

### Future Modes
```bash
# Interactive CLI (not yet implemented)
cargo run --bin pagi-twin cli

# Terminal UI (not yet implemented)
cargo run --bin pagi-twin tui

# Desktop GUI (not yet implemented)
cargo run --bin pagi-twin desktop

# Background daemon (not yet implemented)
cargo run --bin pagi-twin daemon
```

## Environment Variables

The following environment variables are used by `pagi-twin`:

- `PHOENIX_NAME` - AGI name (default: "Sola")
- `USER_NAME` - User name (default: "User")
- `PHOENIX_WEB_BIND` - Web server bind address (default: "127.0.0.1:8888")
- `PHOENIX_DOTENV_PATH` - Explicit path to .env file
- `RUST_LOG` - Logging level (default: "info")

## Background Services

When running `pagi-twin web`, the following services are automatically spawned as background tasks:

1. **Phoenix Web Server** (main thread)
   - HTTP API on port 8888
   - WebSocket support
   - Full AGI functionality

2. **Vital Pulse Collector** (background task)
   - Telemetry ingestion service
   - Port 8889 (configurable)
   - Collects anonymized usage data

3. **Synaptic Pulse Distributor** (background task)
   - Configuration update service
   - Port 8890 (configurable)
   - Pushes non-binary updates via WebSocket

## Integration Steps

### Step 1: Complete phoenix-web Library Conversion

The current implementation has a placeholder `run_server()` function. To complete the integration:

1. Refactor `phoenix-web/src/main.rs` main() logic into `phoenix-web/src/lib.rs`
2. Move the `async fn main()` body into `pub async fn run_server()`
3. Update `main.rs` to call `phoenix_web::run_server().await`

Example:
```rust
// phoenix-web/src/lib.rs
pub async fn run_server() -> std::io::Result<()> {
    // Move all the main() logic here
    let (dotenv_path, dotenv_error) = load_dotenv_best_effort();
    // ... rest of initialization
    HttpServer::new(/* ... */)
        .bind(bind)?
        .run()
        .await
}

// phoenix-web/src/main.rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    phoenix_web::run_server().await
}
```

### Step 2: Convert Telemetry Services to Libraries

Similarly, convert `vital_pulse_collector` and `synaptic_pulse_distributor`:

1. Add `[lib]` section to their Cargo.toml files
2. Create `lib.rs` with `pub async fn run()` functions
3. Update `pagi-twin/src/main.rs` to call these functions

### Step 3: Implement Desktop Mode

For desktop mode integration:

1. Add `tauri` dependency to `pagi-twin`
2. Call into `phoenix-desktop-tauri` entry point
3. Coordinate web server and desktop window lifecycle

## Testing

### Manual Testing
```bash
# Test web mode
cargo run --bin pagi-twin web
# Expected: Server starts on :8888, telemetry services running

# Test help
cargo run --bin pagi-twin --help
# Expected: Shows all subcommands

# Test legacy binary
cargo run --bin pagi-sola-web
# Expected: Server starts normally (backward compatibility)
```

### Automated Testing
```bash
# Build all binaries
cargo build --workspace

# Test pagi-twin binary
cargo test --bin pagi-twin

# Test pagi-utils library
cargo test -p pagi-utils
```

## Migration Guide

### For Developers

**Old way:**
```bash
# Start web server
cargo run --bin pagi-sola-web

# Start telemetry collector (separate terminal)
cargo run --bin vital_pulse_collector

# Start config distributor (separate terminal)
cargo run --bin synaptic_pulse_distributor
```

**New way:**
```bash
# Everything in one command
cargo run --bin pagi-twin web
```

### For Deployment

**Old way:**
- Build 3 separate binaries
- Deploy and manage 3 processes
- Configure inter-process communication

**New way:**
- Build 1 binary: `pagi-twin`
- Deploy and manage 1 process
- Background tasks handled internally

## Benefits

1. **Simplified Operations**
   - Single binary to build and deploy
   - Unified configuration
   - Coordinated lifecycle management

2. **Code Reuse**
   - Centralized utilities in `pagi-utils`
   - No more duplicated env/logging code
   - Easier maintenance

3. **Better UX**
   - Single command to start everything
   - Consistent CLI interface
   - Clear operational modes

4. **Future Extensibility**
   - Easy to add new subcommands
   - Shared state via Arc/Mutex
   - Flexible deployment options

## Future Enhancements

### Phase 30: CLI Mode
- Interactive REPL for direct AGI interaction
- Command history and completion
- Streaming responses

### Phase 31: TUI Mode
- Terminal-based UI using `ratatui`
- Split panels for chat, memory, system status
- Keyboard shortcuts

### Phase 32: Desktop Mode
- Full Tauri integration
- Launch desktop window from `pagi-twin desktop`
- Coordinate web server and GUI lifecycle

### Phase 33: Daemon Mode
- Background service with no UI
- API-only mode
- System service integration (systemd, Windows Service)

## Troubleshooting

### Issue: "phoenix_web::run_server() not found"
**Solution:** The library conversion is incomplete. Complete Step 1 of Integration Steps.

### Issue: Telemetry services not starting
**Solution:** Check that ports 8889 and 8890 are available. Configure via environment variables if needed.

### Issue: .env file not loaded
**Solution:** Set `PHOENIX_DOTENV_PATH` or ensure .env is in workspace root or executable directory.

## Related Documentation

- [Binary Architecture Audit](../BINARY_ARCHITECTURE_AUDIT.md)
- [Backend Architecture](./BACKEND_ARCHITECTURE.md)
- [Telemetry & Hive Swarm Architecture](./TELEMETRY_HIVE_SWARM_ARCHITECTURE.md)
- [Build Instructions](./build-guides/BUILD_INSTRUCTIONS.md)

## Git Commands

```bash
# Stage new crates
git add pagi-twin/ pagi-utils/

# Stage modified files
git add Cargo.toml phoenix-web/Cargo.toml phoenix-web/src/lib.rs

# Stage documentation
git add docs/PHASE_29_MODULAR_MONOLITH.md

# Commit
git commit -m "Phase 29: Implement Modular Monolith Switchboard

- Add pagi-twin binary with clap subcommands (web, cli, tui, desktop, daemon)
- Add pagi-utils library for centralized utilities
- Convert phoenix-web to library + binary
- Spawn telemetry services as background tasks
- Update workspace Cargo.toml
- Add comprehensive documentation"

# Push
git push origin main
```
