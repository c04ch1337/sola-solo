#!/usr/bin/env bash
set -euo pipefail

echo "Launching Sola AGI (Phoenix AGI OS v2.4.0) Web UI backend..."
cargo run --bin phoenix-web

