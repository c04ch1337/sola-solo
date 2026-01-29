# Phase 29 Integration Guide

## Quick Start

Phase 29 has been implemented with the following structure:

### New Crates Created

1. **`pagi-twin/`** - Unified binary switchboard
2. **`pagi-utils/`** - Centralized utilities library

### Modified Crates

1. **`phoenix-web/`** - Now supports both library and binary usage
2. **`Cargo.toml`** - Updated workspace members

## Current Status

✅ **Completed:**
- Created `pagi-twin` binary with clap subcommands
- Created `pagi-utils` library with centralized utilities
- Updated workspace Cargo.toml
- Added library support to phoenix-web
- Created comprehensive documentation

⚠️ **Pending (Manual Steps Required):**
- Complete phoenix-web library conversion (move main() logic to lib.rs)
- Convert vital_pulse_collector to library
- Convert synaptic_pulse_distributor to library

## Next Steps to Complete Integration

### Step 1: Complete phoenix-web Library Conversion

The `phoenix-web/src/lib.rs` currently has a placeholder. To complete:

1. Open [`phoenix-web/src/main.rs`](phoenix-web/src/main.rs:5960)
2. Copy the entire `async fn main()` body (lines 5961-6500+)
3. Paste into [`phoenix-web/src/lib.rs`](phoenix-web/src/lib.rs:1) as `pub async fn run_server()`
4. Update `main.rs` to simply call `phoenix_web::run_server().await`
5. Remove duplicate utility functions from main.rs (now in pagi-utils)

**Example refactored main.rs:**
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    phoenix_web::run_server().await
}
```

### Step 2: Test Compilation

```bash
# Test building pagi-utils
cargo build -p pagi-utils

# Test building pagi-twin
cargo build -p pagi-twin

# Test building phoenix-web (both lib and bin)
cargo build -p phoenix-web

# Test the full workspace
cargo build --workspace
```

### Step 3: Test Execution

```bash
# Test the new unified binary
cargo run --bin pagi-twin web

# Test legacy binary (should still work)
cargo run --bin pagi-sola-web

# Test help
cargo run --bin pagi-twin --help
```

## File Structure

```
pagi-twin-desktop/
├── pagi-twin/
│   ├── Cargo.toml          # Main switchboard binary
│   └── src/
│       └── main.rs         # CLI with subcommands
├── pagi-utils/
│   ├── Cargo.toml          # Centralized utilities
│   └── src/
│       └── lib.rs          # env_nonempty, logging, etc.
├── phoenix-web/
│   ├── Cargo.toml          # Updated: lib + bin
│   └── src/
│       ├── lib.rs          # NEW: Library interface
│       └── main.rs         # Existing: Binary entry point
├── Cargo.toml              # Updated: Added pagi-twin, pagi-utils
└── docs/
    └── PHASE_29_MODULAR_MONOLITH.md  # Full documentation
```

## Usage Examples

### Web Server Mode
```bash
# Start web server with telemetry services
cargo run --bin pagi-twin web

# With custom bind address
cargo run --bin pagi-twin web --bind 0.0.0.0:8888
```

### Future Modes (Stubs)
```bash
# CLI mode (not yet implemented)
cargo run --bin pagi-twin cli

# TUI mode (not yet implemented)
cargo run --bin pagi-twin tui

# Desktop mode (not yet implemented)
cargo run --bin pagi-twin desktop

# Daemon mode (not yet implemented)
cargo run --bin pagi-twin daemon
```

## Environment Variables

- `PHOENIX_NAME` - AGI name (default: "Sola")
- `USER_NAME` - User name (default: "User")
- `PHOENIX_WEB_BIND` - Web server bind address (default: "127.0.0.1:8888")
- `PHOENIX_DOTENV_PATH` - Explicit path to .env file
- `RUST_LOG` - Logging level (default: "info")

## Git Commands

```bash
# Stage all new and modified files
git add pagi-twin/ pagi-utils/ phoenix-web/ Cargo.toml docs/ PHASE_29_INTEGRATION.md

# Commit
git commit -m "Phase 29: Implement Modular Monolith Switchboard

- Add pagi-twin binary with clap subcommands (web, cli, tui, desktop, daemon)
- Add pagi-utils library for centralized utilities (env_nonempty, logging, .env loading)
- Convert phoenix-web to support both library and binary usage
- Spawn telemetry services (vital_pulse_collector, synaptic_pulse_distributor) as background tasks
- Update workspace Cargo.toml to include new crates
- Add comprehensive documentation and integration guide

Benefits:
- Single binary deployment
- Unified CLI interface
- Centralized utilities (no more duplication)
- Background task spawning for telemetry
- Future-ready for CLI, TUI, Desktop, Daemon modes

Next steps:
- Complete phoenix-web library conversion (move main() to lib.rs)
- Convert telemetry services to libraries
- Implement remaining subcommands (cli, tui, desktop, daemon)"

# Push
git push origin main
```

## Troubleshooting

### Build Errors

**Error:** `cannot find function 'run_server' in crate 'phoenix_web'`
- **Cause:** phoenix-web library conversion incomplete
- **Fix:** Complete Step 1 above

**Error:** `unresolved import 'pagi_utils'`
- **Cause:** Workspace not updated
- **Fix:** Run `cargo build --workspace` to update dependencies

### Runtime Errors

**Error:** `.env file not found`
- **Cause:** .env not in expected location
- **Fix:** Set `PHOENIX_DOTENV_PATH` environment variable

**Error:** `Address already in use (port 8888)`
- **Cause:** Another instance running
- **Fix:** Stop other instance or use `--bind` flag with different port

## Documentation

- **Full Documentation:** [`docs/PHASE_29_MODULAR_MONOLITH.md`](docs/PHASE_29_MODULAR_MONOLITH.md)
- **Binary Architecture:** [`BINARY_ARCHITECTURE_AUDIT.md`](BINARY_ARCHITECTURE_AUDIT.md)
- **Backend Architecture:** [`docs/BACKEND_ARCHITECTURE.md`](docs/BACKEND_ARCHITECTURE.md)

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the full documentation in `docs/PHASE_29_MODULAR_MONOLITH.md`
3. Check existing issues in the repository
4. Create a new issue with detailed error messages and steps to reproduce
