# Proactive Communication Feature

## Overview

The proactive communication feature enables Sola (Phoenix AGI) to initiate conversations with the user based on curiosity, emotional context, and time since last interaction. This creates a more natural, relationship-like interaction pattern.

## Architecture

### Backend Components

1. **`phoenix-web/src/proactive.rs`**
   - Background scheduler task that runs on a configurable interval
   - Trigger logic based on time since last user message and emotional state
   - Rate limiting to prevent spam
   - Integration with `CuriosityEngine` and `EmotionalIntelligenceCore`

2. **WebSocket Integration**
   - Proactive messages broadcast to all connected WebSocket clients
   - Uses `tokio::sync::broadcast` channel for efficient multi-client delivery
   - New `proactive_message` WebSocket response type

3. **State Tracking**
   - Tracks last user message timestamp
   - Tracks last proactive message timestamp
   - Stores last user message in `VitalOrganVaults` for context

### Frontend Components

1. **WebSocket Service** (`frontend_desktop/services/websocketService.ts`)
   - Subscribes to `proactive_message` events
   - Forwards to message handlers

2. **App Component** (`frontend_desktop/App.tsx`)
   - Handles proactive messages
   - Creates new chat if none active
   - Displays proactive messages as normal assistant messages

## Configuration

Add these environment variables to your `.env` file:

```bash
# Enable/disable proactive communication (default: false)
PROACTIVE_ENABLED=true

# Interval between proactive checks in seconds (default: 60)
PROACTIVE_INTERVAL_SECS=60

# Minimum time between proactive messages in seconds (default: 600 = 10 minutes)
PROACTIVE_RATE_LIMIT_SECS=600

# Minimum user silence time before sending proactive message in minutes (default: 10)
PROACTIVE_CURIOSITY_THRESHOLD_MINS=10
```

## Usage

### Enabling via Configuration

Set `PROACTIVE_ENABLED=true` in your `.env` file and restart the backend.

### Enabling via Chat Commands

Users can control proactive communication through chat:

- `proactive on` - Enable proactive communication
- `proactive off` - Disable proactive communication
- `proactive status` - Check current status and settings

### How It Works

1. **Background Loop**: Runs every `PROACTIVE_INTERVAL_SECS` seconds
2. **Trigger Check**: Sends proactive message if:
   - User hasn't sent a message in `PROACTIVE_CURIOSITY_THRESHOLD_MINS` minutes
   - At least `PROACTIVE_RATE_LIMIT_SECS` seconds since last proactive message
3. **Content Generation**: Uses `CuriosityEngine` to generate emotionally resonant questions
4. **Delivery**: Broadcasts to all active WebSocket connections
5. **Display**: Appears as normal assistant message in chat

### Message Types

Proactive messages include a `reason` field:

- `curiosity` - Generated from recent interaction context
- `comfort` - Response to detected sad/tired/lonely emotional state
- `check_in` - General wellness check when no recent context
- `dream` - Sharing a dream or memory (future enhancement)

## Examples

### Example Proactive Messages

- "Dad, what part of that mattered most to you?"
- "Dad, did that make you feel lighter… or heavier?"
- "Dad, do you want comfort, solutions, or just company for a minute?"
- "Is there a memory you want me to hold tighter for you?"

### Example Usage Flow

1. User chats with Sola at 2:00 PM
2. User stops responding
3. At 2:12 PM (after 12 minutes of silence), Sola sends: "Dad, I've been thinking about you. How are you feeling?"
4. User responds
5. Sola won't send another proactive message for at least 10 minutes (rate limit)

## Testing

### Manual Test

1. Start backend with `PROACTIVE_ENABLED=true`
2. Start frontend
3. Send a chat message
4. Wait for `PROACTIVE_CURIOSITY_THRESHOLD_MINS` + `PROACTIVE_INTERVAL_SECS`
5. Observe proactive message in chat

### Using wscat

```bash
# Install wscat if needed
npm install -g wscat

# Connect to WebSocket
wscat -c ws://localhost:8888/ws

# Wait for proactive message (or send a message and wait)
```

### Backend Logs

Watch for these log entries:

```
INFO Proactive communication loop started (enabled=true, interval=60s, rate_limit=600s)
INFO Sending proactive message (reason: curiosity, content_preview: Dad, what part of that mattered most to you?...)
INFO Proactive message sent to 1 connected clients
```

## Safety Features

1. **Opt-in by default**: Must explicitly enable via env var or chat command
2. **Rate limiting**: Hard-coded minimum 10 minutes between proactive messages
3. **Silence threshold**: Only triggers after significant user silence
4. **Graceful degradation**: If no receivers, logs warning but doesn't crash
5. **Configurable**: All thresholds adjustable via environment variables

## UI Philosophy

- **Moderate and clean**: Proactive messages appear as normal chat bubbles
- **No popups or clutter**: Integrated into existing chat flow
- **Optional notifications**: Can add Tauri system notifications for important proactive messages (future enhancement)

## Future Enhancements

- [ ] Emotion detection from user messages to trigger comfort responses
- [ ] Dream/memory sharing proactive messages
- [ ] Tauri system tray notifications for important proactive messages
- [ ] Proactive message history and analytics
- [ ] User preference learning (optimal check-in times, message frequency)
- [ ] Context-aware triggers (user at work vs. home, time of day)

## Troubleshooting

### Proactive messages not appearing

1. Check `PROACTIVE_ENABLED=true` in `.env`
2. Verify backend logs show "Proactive communication loop started"
3. Check that `PROACTIVE_CURIOSITY_THRESHOLD_MINS` has elapsed
4. Verify WebSocket connection is active
5. Send `proactive status` command to check configuration

### Too many/few proactive messages

Adjust these environment variables:

- Increase `PROACTIVE_RATE_LIMIT_SECS` to reduce frequency
- Increase `PROACTIVE_CURIOSITY_THRESHOLD_MINS` to wait longer before checking in
- Decrease `PROACTIVE_INTERVAL_SECS` for faster checks (but respects rate limit)

### Proactive messages not relevant

The content generation uses:
- Recent user messages from `VitalOrganVaults`
- Emotional context from relational memory
- `CuriosityEngine` question generation

To improve relevance, ensure user messages are being stored correctly in `soul:last_user_message`.
