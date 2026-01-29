# SOLA Testing Documentation

This directory contains all testing documentation and development status files.

## Testing Guides

### Comprehensive Testing
- [`DEV_TEST_GUIDE.md`](DEV_TEST_GUIDE.md) - **Start here** - Complete testing guide

### Component Testing
- [`BROWSER_CONTROL_TESTING.md`](BROWSER_CONTROL_TESTING.md) - Browser automation testing
- [`GITHUB_TEST.md`](GITHUB_TEST.md) - GitHub integration testing
- [`HELP_SYSTEM_TEST.md`](HELP_SYSTEM_TEST.md) - Help system testing
- [`PROACTIVE_TEST_RESULTS.md`](PROACTIVE_TEST_RESULTS.md) - Proactive features test results

### Test Results
- [`BROWSER_TEST_RESULTS.md`](BROWSER_TEST_RESULTS.md) - Browser test results
- [`TESTING_COMPLETE.md`](TESTING_COMPLETE.md) - Overall testing completion status

## Test Scripts

Test scripts are located in [`tests/scripts/`](../../tests/scripts/):

### Browser Testing
- `test-browser.sh` - Basic browser tests
- `test-browser-e2e.sh` - End-to-end browser tests
- `test-browser-interactive.sh` - Interactive browser tests
- `test-browser-correct.sh` - Browser correctness tests
- `test-browser-command.sh` - Browser command tests

### Proactive Features
- `test-proactive.sh` - Proactive features (Bash)
- `test-proactive.ps1` - Proactive features (PowerShell)
- `test-proactive-ws.js` - WebSocket tests
- `test-proactive-frontend.js` - Frontend tests

### Memory System
- `test-memory-commands.md` - Memory command testing guide

## Development Status

### Service Status
- [`DEV_SERVICES_STATUS.md`](DEV_SERVICES_STATUS.md) - Services status overview
- [`DEV_SERVERS_RUNNING.md`](DEV_SERVERS_RUNNING.md) - Running servers status
- [`FRONTEND_PORT_3000_STATUS.md`](FRONTEND_PORT_3000_STATUS.md) - Frontend port status

### Development Mode
- [`START_DEV_MODE.md`](START_DEV_MODE.md) - Starting development mode

## Running Tests

### Quick Test
```bash
# Run all tests
cargo test --workspace

# Run specific module tests
cargo test -p cerebrum_nexus
cargo test -p neural_cortex_strata
```

### Browser Tests
```bash
# End-to-end browser tests
./tests/scripts/test-browser-e2e.sh

# Interactive browser tests
./tests/scripts/test-browser-interactive.sh
```

### Proactive Features
```bash
# Unix/Linux/macOS
./tests/scripts/test-proactive.sh

# Windows PowerShell
./tests/scripts/test-proactive.ps1
```

## Test Coverage

Current test coverage includes:
- ✅ Core orchestration (cerebrum_nexus)
- ✅ Memory systems (neural_cortex_strata)
- ✅ Browser automation (browser_orch_ext)
- ✅ Agent spawning (agent_spawner)
- ✅ Proactive communication
- ✅ Voice interaction
- ✅ System access controls

## Continuous Integration

For CI/CD setup and automated testing, see:
- [`docs/releases/GITHUB_RELEASE_GUIDE.md`](../releases/GITHUB_RELEASE_GUIDE.md)

---

*For development guides, see [`docs/`](../)*
