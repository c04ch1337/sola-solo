#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "[sola-solo] starting backend (:8888)…"
(cd "$ROOT_DIR/backend" && cargo run) &
BACK_PID=$!

echo "[sola-solo] starting frontend (:5173)…"
(cd "$ROOT_DIR/frontend" && npm run dev -- --host 127.0.0.1 --port 3000 --strictPort) &
FRONT_PID=$!

cleanup() {
  echo "[sola-solo] stopping…"
  kill "$FRONT_PID" 2>/dev/null || true
  kill "$BACK_PID" 2>/dev/null || true
}

trap cleanup EXIT

wait
