#!/usr/bin/env node
// Quick WebSocket test for proactive communication

const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8888/ws');

ws.on('open', () => {
  console.log('✓ WebSocket connected');
  console.log('Sending test message...');
  
  // Send a test speak message
  ws.send(JSON.stringify({
    type: 'speak',
    user_input: 'Hello Sola, testing proactive communication'
  }));
  
  console.log('Message sent. Waiting for proactive response...');
  console.log('(Should receive proactive message after ~90 seconds of silence)');
  console.log('');
});

ws.on('message', (data) => {
  try {
    const msg = JSON.parse(data.toString());
    
    if (msg.type === 'connected') {
      console.log(`✓ Connection confirmed: ${msg.message}`);
    } else if (msg.type === 'proactive_message') {
      console.log('');
      console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
      console.log('🎉 PROACTIVE MESSAGE RECEIVED!');
      console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
      console.log(`Reason: ${msg.reason}`);
      console.log(`Content: ${msg.content}`);
      console.log(`Timestamp: ${new Date(msg.timestamp * 1000).toLocaleString()}`);
      console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
      console.log('');
      console.log('✅ Proactive communication test PASSED!');
      process.exit(0);
    } else if (msg.type === 'speak_response_chunk') {
      if (msg.done) {
        console.log('✓ Speak response received');
        console.log(`  Waiting for proactive message (${new Date().toLocaleTimeString()})...`);
      }
    } else {
      console.log(`→ ${msg.type}: ${JSON.stringify(msg).substring(0, 100)}`);
    }
  } catch (e) {
    console.log('Raw message:', data.toString());
  }
});

ws.on('error', (error) => {
  console.error('❌ WebSocket error:', error.message);
  process.exit(1);
});

ws.on('close', () => {
  console.log('WebSocket closed');
});

// Timeout after 120 seconds
setTimeout(() => {
  console.log('');
  console.log('⏱️  Test timeout (120s). Proactive message not received.');
  console.log('This might mean:');
  console.log('  - Rate limit not met yet (check PROACTIVE_RATE_LIMIT_SECS)');
  console.log('  - Silence threshold not met (check PROACTIVE_CURIOSITY_THRESHOLD_MINS)');
  console.log('  - No active broadcast receivers');
  process.exit(1);
}, 120000);

console.log('Starting proactive communication test...');
console.log('Connecting to ws://localhost:8888/ws');
