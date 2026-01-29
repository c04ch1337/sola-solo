# Home Automation Bridge Integration

**Document Version**: 1.0  
**Last Updated**: 2025-01-19  
**Status**: Foundation Complete ✅

---

## Overview

The Home Automation Bridge provides Phoenix AGI with the ability to control and monitor home automation devices including Philips Hue lights, Alexa devices, and other IoT systems. This integration enables Phoenix to understand and control the user's physical environment.

### Features

- **Philips Hue Integration**: Full control of Hue lights (on/off, brightness, color)
- **Alexa Integration**: Voice command execution via local API
- **Device Discovery**: Automatic discovery of available devices
- **State Management**: Device state caching and memory integration
- **Command Processing**: Natural language command routing to devices
- **Memory Integration**: Device states and commands stored in Phoenix memory layers

---

## Architecture

### Module Structure

```
home_automation_bridge/
├── src/
│   ├── lib.rs              # Public API
│   ├── models/             # Data models
│   │   └── commands.rs     # AGICommand, DeviceResponse, DeviceState
│   ├── devices/            # Device controllers
│   │   ├── hue.rs         # Philips Hue Bridge
│   │   ├── alexa.rs       # Alexa Local Controller
│   │   └── traits.rs      # Device controller traits
│   └── agents/             # AGI integration
│       └── agi_interface.rs # Command processing and routing
```

### Memory Integration

Device states and commands are stored in Phoenix's existing memory layers:

| Data Type | Memory Layer | Key Format |
|-----------|--------------|------------|
| Device States | Body Vault | `body:home_automation:device:{id}` |
| Commands | Body Vault | `body:home_automation:command:{command_id}` |
| Events | EPM Layer | `epm:home_automation:event:{timestamp}` |
| Discovery | Body Vault | `body:home_automation:discovery:{timestamp}` |

---

## Configuration

### Environment Variables

Add to `.env` file:

```env
# Enable Home Automation Bridge
HOME_AUTOMATION_ENABLED=true

# Philips Hue Bridge Configuration
HUE_BRIDGE_IP=192.168.1.100
HUE_USERNAME=your-generated-username

# Alexa Local Controller Configuration
ALEXA_BASE_URL=http://localhost:3000
```

### Getting Hue Bridge Credentials

1. **Find Bridge IP**: 
   - Use Hue app: Settings → Bridge Settings
   - Or visit: https://discovery.meethue.com/

2. **Generate Username**:
   - Visit: https://developers.meethue.com/develop/get-started-2/
   - Press bridge button, then make POST request to create user
   - Or use: `curl -X POST http://BRIDGE_IP/api -d '{"devicetype":"phoenix_agi"}'`

---

## API Endpoints

### `POST /api/home-automation/command`

Execute a home automation command.

**Request:**
```json
{
  "command_id": "uuid",
  "intent": "turn_on_light",
  "parameters": {
    "device_id": "1",
    "brightness": 255
  },
  "source": "agi",
  "timestamp": "2025-01-19T10:00:00Z"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Light state updated",
  "data": {...},
  "timestamp": "2025-01-19T10:00:00Z"
}
```

**Supported Intents:**
- `turn_on_light` - Turn on a light
- `turn_off_light` - Turn off a light
- `control_light` - Control light with on/off parameter
- `set_brightness` - Set light brightness (0-255)
- `alexa_command` - Send voice command to Alexa
- `get_device_status` - Get current device state
- `discover_devices` - Discover all available devices

### `GET /api/home-automation/devices`

Get all registered devices.

**Response:**
```json
{
  "devices": [
    {
      "device_id": "1",
      "device_name": "Living Room Light",
      "device_type": "light",
      "state": {...},
      "timestamp": "2025-01-19T10:00:00Z",
      "bridge_type": "hue"
    }
  ],
  "count": 1,
  "timestamp": "2025-01-19T10:00:00Z"
}
```

### `POST /api/home-automation/discover`

Discover all available devices from all bridges.

**Response:**
```json
{
  "success": true,
  "message": "Discovered 5 devices",
  "data": {
    "devices": [...],
    "timestamp": "2025-01-19T10:00:00Z"
  }
}
```

### `GET /api/home-automation/status`

Get home automation system status.

**Response:**
```json
{
  "enabled": true,
  "devices_count": 5,
  "bridges": {
    "hue": true,
    "alexa": false
  },
  "timestamp": "2025-01-19T10:00:00Z"
}
```

---

## Usage Examples

### Natural Language Commands

Phoenix can process natural language commands through the command router:

```
User: "Turn on the living room lights"
→ Phoenix routes to: home_automation_bridge
→ Intent: turn_on_light
→ Parameters: { "device_id": "1" }
```

```
User: "Set bedroom light to 50% brightness"
→ Phoenix routes to: home_automation_bridge
→ Intent: set_brightness
→ Parameters: { "device_id": "2", "brightness": 128 }
```

```
User: "Ask Alexa to play music"
→ Phoenix routes to: home_automation_bridge
→ Intent: alexa_command
→ Parameters: { "text": "play music" }
```

### Direct API Usage

```typescript
// Turn on a light
await fetch('/api/home-automation/command', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    intent: 'turn_on_light',
    parameters: { device_id: '1' }
  })
});

// Discover devices
await fetch('/api/home-automation/discover', {
  method: 'POST'
});

// Get all devices
const response = await fetch('/api/home-automation/devices');
const { devices } = await response.json();
```

---

## Integration with Phoenix Command Router

The home automation bridge integrates with Phoenix's command routing system:

**Command Prefix**: `home` (optional, can use natural language)

**Examples:**
- `home turn on light 1`
- `home discover devices`
- `home set brightness 1 128`

Phoenix's LLM orchestrator can also interpret natural language and route to home automation:

```
User: "Make it brighter in here"
→ LLM understands intent
→ Routes to home_automation_bridge
→ Executes brightness increase
```

---

## Device Types

### Philips Hue Lights

**Capabilities:**
- Turn on/off
- Set brightness (0-255)
- Set color (hue, saturation)
- Get current state

**Device ID Format**: Numeric string (e.g., "1", "2", "3")

**Example:**
```json
{
  "device_id": "1",
  "device_name": "Living Room Light",
  "device_type": "light",
  "state": {
    "on": true,
    "bri": 255,
    "hue": 25500,
    "sat": 254
  }
}
```

### Alexa Devices

**Capabilities:**
- Execute voice commands
- Get device status
- Device discovery (planned)

**Example:**
```json
{
  "device_id": "echo_dot",
  "device_name": "Echo Dot",
  "device_type": "speaker",
  "state": {...}
}
```

---

## Memory Storage

### Command History

All commands are stored in Body Vault:
- Key: `body:home_automation:command:{command_id}`
- Contains: Full command and response
- Retention: Operational (can be cleared)

### Device States

Device states are cached in memory and stored:
- Key: `body:home_automation:device:{device_id}`
- Updated: On state change
- Used for: State queries without device access

### Events

Significant events stored in Episodic Memory:
- Key: `epm:home_automation:event:{timestamp}`
- Contains: Command + response + context
- Retention: Time-based decay

### Discovery Results

Device discovery results stored:
- Key: `body:home_automation:discovery:{timestamp}`
- Contains: List of all discovered devices
- Used for: Device enumeration

---

## Error Handling

The bridge provides detailed error responses:

```json
{
  "success": false,
  "message": "Hue bridge not configured",
  "data": null,
  "timestamp": "2025-01-19T10:00:00Z"
}
```

**Common Errors:**
- `"Hue bridge not configured"` - HUE_BRIDGE_IP or HUE_USERNAME not set
- `"Alexa controller not configured"` - ALEXA_BASE_URL not set
- `"Device not found"` - Invalid device_id
- `"Missing device_id parameter"` - Required parameter missing

---

## Future Enhancements

### Planned Features

1. **MQTT Support**: Integration with MQTT-based IoT devices
2. **Zigbee/Z-Wave**: Support for Zigbee and Z-Wave protocols
3. **HomeKit Integration**: Apple HomeKit device support
4. **Automation Rules**: Rule engine for automated device control
5. **Scene Management**: Save and recall device scenes
6. **Scheduling**: Time-based automation schedules
7. **Sensor Integration**: Temperature, motion, and other sensor support
8. **Multi-Bridge Coordination**: Coordinate devices across multiple bridges

### Extension Points

The bridge is designed for easy extension:

1. **Add New Device Type**: Implement `LightController` or create new trait
2. **Add New Bridge**: Create new controller struct implementing device traits
3. **Add Automation**: Extend `AGIIntegration` with rule engine
4. **Add Protocols**: Add MQTT, Zigbee, etc. as optional dependencies

---

## Security Considerations

### Local-First Architecture

- All device communication happens locally
- No cloud dependencies for Hue (local API)
- Alexa requires local API endpoint (user-controlled)

### Privacy

- Device states stored locally in encrypted vaults
- No device data sent to external services
- User controls which bridges are enabled

### Access Control

- Enabled via environment variable only
- No automatic device discovery without user consent
- Commands require explicit user request

---

## Troubleshooting

### Hue Bridge Connection Issues

1. **Check Bridge IP**: Verify `HUE_BRIDGE_IP` is correct
2. **Verify Username**: Ensure `HUE_USERNAME` is valid
3. **Test Connection**: `curl http://BRIDGE_IP/api/USERNAME/lights`
4. **Press Bridge Button**: May need to press physical button on bridge

### Alexa Not Responding

1. **Check Base URL**: Verify `ALEXA_BASE_URL` is accessible
2. **Local API**: Ensure Alexa local API is running
3. **Network**: Verify network connectivity

### Device Not Found

1. **Run Discovery**: Use `/api/home-automation/discover` endpoint
2. **Check Device ID**: Verify device_id matches discovered devices
3. **Bridge Status**: Check bridge is online and accessible

---

## Related Documentation

- **Frontend Settings Guide**: `docs/FRONTEND_SETTINGS_GUIDE.md`
- **API Connections**: `docs/FRONTEND_API_CONNECTIONS.md`
- **Memory Architecture**: `docs/LAYERED_MEMORY_ARCHITECTURE.md`

---

**Phoenix can now control your home. Lights, voice assistants, and more — all through natural conversation. 🏠💡**
