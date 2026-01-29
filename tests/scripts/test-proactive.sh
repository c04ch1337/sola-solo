#!/bin/bash
# test-proactive.sh
# Test script for proactive communication feature

set -e

echo "======================================"
echo "Proactive Communication Test Script"
echo "======================================"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ .env file not found. Please create one from .env.example"
    exit 1
fi

echo "✓ .env file found"
echo ""

# Check if backend is running
echo "Checking if Phoenix backend is running..."
if curl -s http://localhost:8888/health > /dev/null 2>&1; then
    echo "✓ Backend is running"
else
    echo "❌ Backend is not running. Please start it first:"
    echo "   cd phoenix-web && cargo run"
    exit 1
fi
echo ""

# Check proactive settings in .env
echo "Checking proactive settings in .env:"
if grep -q "PROACTIVE_ENABLED=true" .env; then
    echo "✓ PROACTIVE_ENABLED=true"
else
    echo "⚠️  PROACTIVE_ENABLED not set to true"
    echo "   Add this line to .env: PROACTIVE_ENABLED=true"
    echo "   Then restart the backend"
fi

INTERVAL=$(grep "PROACTIVE_INTERVAL_SECS=" .env | cut -d'=' -f2 || echo "60 (default)")
RATE_LIMIT=$(grep "PROACTIVE_RATE_LIMIT_SECS=" .env | cut -d'=' -f2 || echo "600 (default)")
THRESHOLD=$(grep "PROACTIVE_CURIOSITY_THRESHOLD_MINS=" .env | cut -d'=' -f2 || echo "10 (default)")

echo "   - Interval: ${INTERVAL}s"
echo "   - Rate limit: ${RATE_LIMIT}s"
echo "   - Curiosity threshold: ${THRESHOLD} min"
echo ""

# Test WebSocket connection
echo "Testing WebSocket connection..."
if command -v wscat &> /dev/null; then
    echo "✓ wscat is installed"
    echo ""
    echo "To manually test proactive messages:"
    echo "1. Run: wscat -c ws://localhost:8888/ws"
    echo "2. Send a test message: {\"type\":\"speak\",\"user_input\":\"hello\"}"
    echo "3. Wait ${THRESHOLD} minutes + ${INTERVAL}s for proactive message"
    echo ""
else
    echo "⚠️  wscat not installed (optional for manual testing)"
    echo "   Install with: npm install -g wscat"
    echo ""
fi

# Test via REST API
echo "Testing via REST API..."
echo "Sending test message to establish last user message time..."
RESPONSE=$(curl -s -X POST http://localhost:8888/api/speak \
    -H "Content-Type: application/json" \
    -d '{"user_input": "Test message for proactive communication"}' || echo '{"error": "failed"}')

if echo "$RESPONSE" | grep -q '"message"'; then
    echo "✓ REST API working"
    echo "Response: $(echo "$RESPONSE" | jq -r '.message' 2>/dev/null || echo "$RESPONSE" | head -c 100)"
else
    echo "❌ REST API test failed"
    echo "Response: $RESPONSE"
fi
echo ""

# Instructions for testing
echo "======================================"
echo "Testing Instructions"
echo "======================================"
echo ""
echo "1. Automatic test (wait for proactive message):"
echo "   - Open frontend (http://localhost:3000)"
echo "   - Send a chat message"
echo "   - Wait ${THRESHOLD} minutes + ${INTERVAL}s"
echo "   - Proactive message should appear in chat"
echo ""
echo "2. Manual test via chat commands:"
echo "   - Type: proactive status"
echo "   - Type: proactive on"
echo "   - Type: proactive off"
echo ""
echo "3. Check backend logs:"
echo "   - Look for: 'Proactive communication loop started'"
echo "   - Look for: 'Sending proactive message'"
echo "   - Look for: 'Proactive message sent to N connected clients'"
echo ""
echo "4. Reduce wait time for testing (optional):"
echo "   - Add to .env: PROACTIVE_CURIOSITY_THRESHOLD_MINS=1"
echo "   - Add to .env: PROACTIVE_INTERVAL_SECS=30"
echo "   - Restart backend"
echo ""

echo "Test script complete!"
