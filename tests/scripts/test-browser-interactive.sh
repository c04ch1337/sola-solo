#!/bin/bash
# Interactive browser test - sends command and waits for all responses

{
  sleep 0.5
  echo '{"type":"speak","text":"system browser help"}'
  sleep 3
} | wscat -c ws://localhost:8888/ws 2>&1 | head -30
