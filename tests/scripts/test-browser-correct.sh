#!/bin/bash
# Test browser commands with correct WebSocket format

echo "=== Test 1: Browser Help ==="
{
  sleep 0.5
  echo '{"type":"speak","user_input":"system browser help"}'
  sleep 4
} | wscat -c ws://localhost:8888/ws 2>&1 | grep -v "^connected" | head -40

echo ""
echo "=== Test 2: Browser Sessions ==="
{
  sleep 0.5
  echo '{"type":"speak","user_input":"system browser sessions"}'
  sleep 4
} | wscat -c ws://localhost:8888/ws 2>&1 | grep -v "^connected" | head -40
