#!/bin/bash
# Test browser commands via WebSocket

echo "=== Testing: system browser help ==="
(echo '{"type":"speak","text":"system browser help"}'; sleep 2) | wscat -c ws://localhost:8888/ws 2>&1

echo ""
echo "=== Testing: system browser sessions ==="
(echo '{"type":"speak","text":"system browser sessions"}'; sleep 2) | wscat -c ws://localhost:8888/ws 2>&1
