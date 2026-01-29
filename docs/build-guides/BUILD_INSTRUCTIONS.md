# Backend Services Build and Startup Instructions

## Current Status

✅ **service-orchestrator-rs**: Built and running (PID: check with `pgrep -f service-orchestrator-rs`)

⚠️ **phoenix-web**: Requires ALSA development libraries to build

## Installing ALSA (Required for phoenix-web)

The `phoenix-web` backend service depends on `digital_twin`, which requires ALSA for audio capabilities.

Install ALSA development libraries:

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev pkg-config
```

## Building Backend Services

Once ALSA is installed, build the backend services:

```bash
# Build all workspace members
cargo build --workspace

# Or build specific services
cargo build --bin pagi-sola-web
cargo build --bin service-orchestrator-rs
```

## Starting Backend Services

### Start phoenix-web (Web API Server)

```bash
cargo run --bin pagi-sola-web
```

This will start the web server on `http://127.0.0.1:8888` (default port).

### Start service-orchestrator-rs (Service Orchestrator)

```bash
cargo run --bin service-orchestrator-rs
```

This service manages scheduled jobs and social media integrations.

## Running Services in Background

To run services in the background:

```bash
# Start phoenix-web in background
cargo run --bin pagi-sola-web > phoenix-web.log 2>&1 &

# Start service-orchestrator-rs in background  
cargo run --bin service-orchestrator-rs > service-orchestrator.log 2>&1 &
```

## Checking Service Status

```bash
# Check if services are running
pgrep -f pagi-sola-web
pgrep -f service-orchestrator-rs

# View logs
tail -f phoenix-web.log
tail -f service-orchestrator.log
```

## Stopping Services

```bash
# Stop by process name
pkill -f pagi-sola-web
pkill -f service-orchestrator-rs

# Or stop by PID
kill <PID>
```
