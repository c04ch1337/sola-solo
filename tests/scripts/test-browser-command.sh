#!/bin/bash
# Test browser using direct command type (bypasses LLM)

echo "=== Test 1: Browser Help (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser help"}'
  sleep 3
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 2: Browser Sessions (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser sessions"}'
  sleep 3
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 3: Browser Screenshot (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser screenshot"}'
  sleep 5
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 4: Browser Click (via command type) ==="
echo "NOTE: run 'system browser navigate <url>' first in the UI (or add it here) so the selector exists."
{
  sleep 0.5
  echo '{"type":"command","command":"system browser click a"}'
  sleep 5
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 5: Browser Type (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser type input hello"}'
  sleep 5
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 6: Browser Scroll (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser scroll 0 500"}'
  sleep 2
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 7: Browser Keypress (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser keypress PageDown"}'
  sleep 2
} | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Test 8: Browser Wait (via command type) ==="
{
  sleep 0.5
  echo '{"type":"command","command":"system browser wait body 5000"}'
  sleep 6
} | wscat -c ws://localhost:8888/ws 2>&1
