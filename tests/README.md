# SOLA Tests

This directory contains all test scripts and testing utilities for SOLA.

## Test Scripts

All test scripts are located in [`scripts/`](scripts/):

### Browser Automation Tests
- `test-browser.sh` - Basic browser automation tests
- `test-browser-e2e.sh` - End-to-end browser tests
- `test-browser-interactive.sh` - Interactive browser testing
- `test-browser-correct.sh` - Browser correctness validation
- `test-browser-command.sh` - Browser command tests

### Proactive Features Tests
- `test-proactive.sh` - Proactive features (Bash)
- `test-proactive.ps1` - Proactive features (PowerShell)
- `test-proactive-ws.js` - WebSocket communication tests
- `test-proactive-frontend.js` - Frontend proactive features

### Memory System Tests
- `test-memory-commands.md` - Memory command testing documentation

## Running Tests

### Cargo Tests (Rust)
```bash
# Run all workspace tests
cargo test --workspace

# Run specific crate tests
cargo test -p cerebrum_nexus
cargo test -p neural_cortex_strata
cargo test -p agent_spawner
cargo test -p browser_orch_ext

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name
```

### Browser Tests
```bash
# Basic browser tests
./tests/scripts/test-browser.sh

# End-to-end tests
./tests/scripts/test-browser-e2e.sh

# Interactive testing
./tests/scripts/test-browser-interactive.sh
```

### Proactive Features
```bash
# Unix/Linux/macOS
./tests/scripts/test-proactive.sh

# Windows PowerShell
./tests/scripts/test-proactive.ps1

# WebSocket tests
node ./tests/scripts/test-proactive-ws.js

# Frontend tests
node ./tests/scripts/test-proactive-frontend.js
```

## Test Coverage

### Core Systems
- ✅ Cerebrum Nexus (orchestrator)
- ✅ Neural Cortex Strata (memory)
- ✅ Vital Organ Vaults (knowledge bases)
- ✅ Context Engine
- ✅ LLM Orchestrator

### Automation & Intelligence
- ✅ Browser automation (Playwright/Selenium)
- ✅ System access controls
- ✅ Agent spawning
- ✅ Ecosystem management

### Communication
- ✅ Proactive communication
- ✅ WebSocket connections
- ✅ Voice I/O
- ✅ Emotion detection

### Frontend
- ✅ Chat interface
- ✅ Memory panels
- ✅ Browser control
- ✅ Settings management

## Test Documentation

Comprehensive test documentation is available in [`docs/testing/`](../docs/testing/):
- [`DEV_TEST_GUIDE.md`](../docs/testing/DEV_TEST_GUIDE.md) - Complete testing guide
- [`BROWSER_CONTROL_TESTING.md`](../docs/testing/BROWSER_CONTROL_TESTING.md) - Browser testing
- [`TESTING_COMPLETE.md`](../docs/testing/TESTING_COMPLETE.md) - Test completion status

## Writing Tests

### Rust Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_async_example() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
Place integration tests in `tests/` directory within each crate:
```
crate_name/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

## Continuous Integration

Tests are automatically run on:
- Pull requests
- Commits to main branch
- Release builds

## Troubleshooting

### Test Failures

**Cargo tests:**
```bash
# Clean and rebuild
cargo clean
cargo test --workspace

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

**Browser tests:**
```bash
# Check browser drivers
which chromedriver
which geckodriver

# Update dependencies
cd browser_orch_ext
cargo update
```

**Node.js tests:**
```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

## Test Requirements

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Chrome/Chromium (for browser tests)
- ChromeDriver (for Selenium tests)

### Environment Variables
```env
# Test configuration
TEST_MODE=true
LOG_LEVEL=debug

# Browser testing
CHROME_DRIVER_PATH=/path/to/chromedriver
HEADLESS=true
```

## Contributing

When adding new features:
1. Write unit tests
2. Add integration tests if needed
3. Update test documentation
4. Ensure all tests pass before PR

---

*For development guides, see [`docs/`](../docs/)*
