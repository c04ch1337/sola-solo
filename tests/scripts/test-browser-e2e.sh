#!/bin/bash
# Browser Control End-to-End Test Script
#
# Validates Phase 16 end-to-end:
# - WS consent gate (per-connection)
# - Tier-2-only WS command execution
# - `system browser ...` routed through the shared command router (not raw shell)
#
# Usage:
#   ./test-browser-e2e.sh
#
# Env:
#   WS_URL=ws://localhost:8888/ws ./test-browser-e2e.sh

set -euo pipefail

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

WS_URL="${WS_URL:-ws://localhost:8888/ws}"

echo "=================================================="
echo "Browser Control End-to-End Test Suite (Phase 16)"
echo "=================================================="
echo ""

echo "Checking prerequisites..."

if ! ps aux | grep -E 'pagi-sola-web' | grep -v grep > /dev/null; then
  echo -e "${RED}✗ Backend not running${NC}"
  echo "Start backend: cd phoenix-web && cargo run"
  exit 1
fi
echo -e "${GREEN}✓ Backend running${NC}"

if ! ps aux | grep -E 'vite.*3000|npm.*dev' | grep -v grep > /dev/null; then
  echo -e "${RED}✗ Frontend not running${NC}"
  echo "Start frontend: cd frontend_desktop && npm run dev"
  exit 1
fi
echo -e "${GREEN}✓ Frontend running${NC}"

if ! curl -s http://localhost:8888/health | grep -q "ok"; then
  echo -e "${RED}✗ Backend health check failed${NC}"
  exit 1
fi
echo -e "${GREEN}✓ Backend healthy${NC}"

if ! curl -s http://localhost:3000 | grep -q "html"; then
  echo -e "${RED}✗ Frontend not accessible${NC}"
  exit 1
fi
echo -e "${GREEN}✓ Frontend accessible${NC}"

echo ""
echo "=================================================="
echo "Automated WebSocket Tests (Tier-2 + Consent)"
echo "=================================================="
echo ""

if ! command -v wscat &> /dev/null; then
  echo -e "${YELLOW}⚠ wscat not found, skipping WebSocket tests${NC}"
  echo "Install: npm install -g wscat"
else
  echo -e "${YELLOW}WS_URL: ${WS_URL}${NC}"
  echo -e "${YELLOW}Sending: system grant + system browser help/sessions/navigate/scrape${NC}"
  echo ""

  {
    # Give wscat time to establish the WS connection before we write the first frame.
    # If this is too short, the first message can be dropped and you'll see `consent_required`.
    sleep 1.5
    echo '{"type":"system","action":"grant"}'
    sleep 0.8

    # IMPORTANT: use WS `command` to exercise the shared internal command router.
    echo '{"type":"command","command":"system browser help"}'
    sleep 1.2

    echo '{"type":"command","command":"system browser sessions"}'
    sleep 1.2

    # Natural forms (no | url=... or | selector=... required)
    echo '{"type":"command","command":"system browser navigate https://news.ycombinator.com"}'
    sleep 2.5

    echo '{"type":"command","command":"system browser scrape https://example.com h1"}'
    sleep 2.5
  } | timeout 15 wscat -c "${WS_URL}" 2>&1 | tee /tmp/browser-test-ws.log

  if grep -q 'consent_required' /tmp/browser-test-ws.log; then
    echo -e "${RED}✗ Consent gate triggered (grant failed or Tier-2 disabled)${NC}"
  else
    echo -e "${GREEN}✓ Consent gate OK (no consent_required found)${NC}"
  fi

  if grep -q 'system\.browser\.' /tmp/browser-test-ws.log; then
    echo -e "${GREEN}✓ Browser messages observed${NC}"
  else
    echo -e "${YELLOW}⚠ No system.browser.* message observed; inspect /tmp/browser-test-ws.log and backend logs${NC}"
  fi
fi

echo ""
echo "=================================================="
echo "Manual Testing Instructions"
echo "=================================================="
echo ""
echo "1. Open UI: http://localhost:3000"
echo ""
echo "2. In chat, run:"
echo "   - system browser help"
echo "   - system browser sessions"
echo "   - system browser launch chrome"
echo "   - system browser navigate https://news.ycombinator.com"
echo "   - system browser scrape https://example.com h1"
echo ""
echo "3. Edge cases:"
echo "   - Without consent: should see consent_required error"
echo "   - Bad URL: should return a graceful error JSON"
echo ""
echo "Logs: /tmp/browser-test-ws.log"
