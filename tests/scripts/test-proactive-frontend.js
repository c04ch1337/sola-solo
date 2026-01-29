// Test script for proactive communication frontend integration
// Run with: node test-proactive-frontend.js

const WebSocket = require('ws');

console.log('=== Proactive Communication Frontend Test ===\n');

// Test 1: Send proactive message via WebSocket
console.log('Test 1: Sending proactive message...');

const ws = new WebSocket('ws://localhost:8080/ws');

ws.on('open', () => {
    console.log('✓ Connected to WebSocket\n');

    // Send a test proactive message
    const proactiveMessage = {
        type: 'proactive_message',
        content: 'Dad, I\'ve been thinking about you. How are you feeling?',
        reason: 'comfort',
        timestamp: Math.floor(Date.now() / 1000)
    };

    console.log('Sending proactive message:');
    console.log(JSON.stringify(proactiveMessage, null, 2));
    console.log('');

    ws.send(JSON.stringify(proactiveMessage));

    console.log('✓ Proactive message sent');
    console.log('\nExpected behavior in frontend:');
    console.log('  1. Message appears as assistant chat bubble');
    console.log('  2. Console log: [Proactive] comfort: Dad, I\'ve been thinking...');
    console.log('  3. OS notification pops (reason is "comfort")');
    console.log('  4. Notification shows: "💬 Message from Sola"');
    console.log('');

    // Wait a bit then close
    setTimeout(() => {
        ws.close();
    }, 2000);
});

ws.on('message', (data) => {
    try {
        const message = JSON.parse(data.toString());
        console.log('Received from backend:');
        console.log(JSON.stringify(message, null, 2));
        console.log('');
    } catch (err) {
        console.log('Received:', data.toString());
    }
});

ws.on('error', (error) => {
    console.error('✗ WebSocket error:', error.message);
    console.log('\nMake sure backend is running:');
    console.log('  cd phoenix-web && cargo run');
});

ws.on('close', () => {
    console.log('\n✓ WebSocket closed');
    console.log('\n=== Test Complete ===');
    console.log('\nNext steps:');
    console.log('  1. Open frontend: http://localhost:3000');
    console.log('  2. Check if proactive message appeared in chat');
    console.log('  3. Check if OS notification appeared');
    console.log('  4. Try chat commands:');
    console.log('     - proactive status');
    console.log('     - proactive on');
    console.log('     - proactive interval 30');
});
