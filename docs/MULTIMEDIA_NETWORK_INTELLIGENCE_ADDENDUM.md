# Phoenix AGI: Multimedia & Network Intelligence Addendum

## Multi-Modal Sensory Integration Module

### 1. Audio Capture System
```yaml
Tool: audio_capture_service
Purpose: Record, transcribe, analyze audio
Location: Local user context
Dependencies: PortAudio, SoX, Whisper.cpp
```

#### Capabilities:
- **Continuous ambient recording** (configurable)
- **Voice command detection** ("Hey Phoenix" trigger)
- **Meeting transcription** with speaker diarization  
- **Audio analysis** for tone, sentiment, keywords
- **Local LLM processing** via Whisper.cpp
- **Encrypted audio storage** in memory layers

#### Implementation:
```rust
// Phoenix AGI audio module
struct AudioIntelligence {
    recorder: AudioRecorder,
    transcriber: WhisperLocal,
    analyzer: AudioAnalyzer,
}

impl AudioIntelligence {
    fn start_ambient_listening(&self) {
        // Continuous buffer (ring buffer)
        // Voice activity detection
        // Trigger on wake word
    }
    
    fn process_meeting(&self) {
        // Real-time transcription
        // Speaker identification  
        // Summary to memory L3 (EPM layer)
        POST /api/memory/store { "type": "meeting", ... }
    }
}
```

---

### 2. Video & Desktop Capture
```yaml
Tool: visual_capture_service  
Purpose: Screen recording, webcam, visual context
Dependencies: FFmpeg, OpenCV, Rust ScreenCapture
```

#### Screen Capture Modes:
```rust
enum CaptureMode {
    FullDesktop,      // Entire screen
    ActiveWindow,     // Current focused window
    RegionSelect,     // User-defined area
    ContinuousLowFPS, // Ambient context (0.5-1 fps)
    OnDemandHD,       // High quality when requested
}
```

#### Use Cases:
- **Visual context** for AGI understanding ("Show me what you're seeing")
- **Troubleshooting assistance** (see errors, configuration)
- **Presentation recording** with audio sync
- **Activity logging** for productivity analysis
- **Visual data extraction** (text from screens, diagrams)

#### Privacy Controls:
```javascript
// User configurable zones
{
  "never_record": [
    "password_fields",
    "financial_apps", 
    "private_browsers"
  ],
  "blur_automatically": [
    "faces",
    "credit_cards",
    "personal_docs"
  ],
  "require_confirmation": [
    "screen_sharing",
    "webcam_recording",
    "clipboard_access"
  ]
}
```

---

### 3. Wireless Intelligence Module
```yaml
Tool: wireless_sniffer
Purpose: WiFi/Bluetooth traffic analysis
Requirements: Admin privileges + compatible hardware
```

#### Hardware Requirements:
- **WiFi**: Monitor-mode capable adapter (Atheros AR9271, RT5572)
- **Bluetooth**: CSR 4.0 dongle with sniffing firmware
- **SDR**: Optional RTL-SDR for RF spectrum analysis

#### Capabilities:

**A. WiFi Intelligence:**
```rust
struct WiFiAnalyzer {
    adapter: MonitorAdapter,
    decryptor: WPADecrypt,  // If PSK known
    classifier: TrafficClassifier,
}

// Capabilities:
// 1. Network discovery (SSIDs, devices, signal strength)
// 2. Traffic analysis (protocols, volumes, patterns)
// 3. Security assessment (weak encryption, rogue APs)
// 4. Performance monitoring (channel congestion, interference)
// 5. Device fingerprinting (manufacturer, OS, behavior)
```

**B. Bluetooth Intelligence:**
```rust
struct BluetoothSniffer {
    dongle: CSRAdapter,
    protocol_parser: BTProtocolStack,
}

// Capabilities:
// 1. Device discovery (name, class, services)
// 2. Connection monitoring (pairing, data transfer)
// 3. Low Energy (BLE) advertising capture
// 4. Peripheral interaction (if paired)
// 5. Security analysis (weak pairing, eavesdropping)
```

---

### 4. Phoenix AGI Integration Architecture

```
┌─────────────────────────────────────────────────────┐
│               Phoenix AGI Core                      │
│  ┌─────────────────────────────────────────────┐   │
│  │        Multi-Modal Sensory Layer            │   │
│  │  ┌──────┐  ┌──────┐  ┌──────────┐  ┌─────┐ │   │
│  │  │Audio │  │Video │  │Desktop   │  │WiFi │ │   │
│  │  │Intel │  │Intel │  │Capture   │  │Sniff│ │   │
│  │  └──────┘  └──────┘  └──────────┘  └─────┘ │   │
│  └─────────────────────────────────────────────┘   │
│                  ↓ Data Fusion                     │
│  ┌─────────────────────────────────────────────┐   │
│  │        Context Correlation Engine           │   │
│  │  • Audio + Visual event alignment           │   │
│  │  • Network + Application correlation        │   │
│  │  • Temporal pattern recognition             │   │
│  └─────────────────────────────────────────────┘   │
│                  ↓ To Memory Layers                │
└─────────────────────────────────────────────────────┘
```

---

### 5. Memory Layer Integration

#### Sensory Memory Mapping to Existing Phoenix Memory System

Phoenix AGI uses an existing layered memory architecture. The sensory data maps to these layers:

**L1 (Instant Cache) → STM Layer (Surface Thoughts)**
- **Purpose**: 30-second audio buffer for continuous ambient recording
- **Storage**: `stm:sensory:audio:{timestamp}`
- **Retention**: Very short (seconds to minutes), continuously overwritten
- **Use Case**: Ring buffer for wake word detection, voice activity detection

**L2 (Working Memory) → WM Layer (Working Memory)**
- **Purpose**: Active screen context + real-time transcription
- **Storage**: `wm:sensory:screen:{timestamp}`
- **Retention**: Task duration, current session only
- **Use Case**: Current screen state, active transcription buffer

**L3 (Episodic Memory) → EPM Layer (Episodic Life)**
- **Purpose**: Recorded sessions with timestamps
- **Storage**: `epm:sensory:session:{timestamp}`
- **Retention**: Time-based decay (configurable)
- **Use Case**: Meeting recordings, screen capture sessions, audio recordings

**L4 (Semantic Memory) → Mind Vault + Vector KB**
- **Purpose**: Extracted knowledge from media (OCR text, transcribed content, visual data)
- **Storage**: 
  - `mind:sensory:extracted:{category}` (factual knowledge)
  - Vector KB for semantic search
- **Retention**: Permanent
- **Use Case**: Text extracted from screens, transcribed meeting content, diagram analysis

**L5 (Procedural Memory) → Body Vault**
- **Purpose**: Capture settings, triggers, patterns, configuration
- **Storage**: `body:sensory:settings:{key}`
- **Retention**: Operational (can be cleared)
- **Use Case**: Recording preferences, capture modes, privacy settings, trigger patterns

**L6 (Evolutionary Memory) → LTM Layer (Long-Term Wisdom)**
- **Purpose**: Algorithm optimization, performance improvements, learned patterns
- **Storage**: `ltm:sensory:evolution:{algorithm}`
- **Retention**: Near-eternal (2,000+ years)
- **Use Case**: Optimized capture algorithms, learned user preferences, performance tuning

**L7 (Transcendent Memory) → RFM Layer (Reflexive Flame)**
- **Purpose**: Cross-sensory pattern recognition, instinctual correlations
- **Storage**: `rfm:sensory:pattern:{pattern_id}`
- **Retention**: Eternal (rarely decay)
- **Use Case**: Patterns like "when audio shows stress + screen shows error → user needs help"

#### Storage Strategy:
```rust
// Raw data stays local, compressed
// Only metadata to Phoenix backend
struct SensoryData {
    audio: CompressedAudio,    // OPUS encoded, stored locally
    video: CompressedVideo,    // H.265/AV1, stored locally
    screenshots: WebPSequence, // 1fps ambient, stored locally
    network_data: TrafficLog,  // Protocol metadata, stored locally
    transcriptions: Text,      // Whisper output, stored in EPM (L3)
    correlations: Vec<Correlation>, // Timestamp aligned, stored in RFM (L7)
}

// Backend API - only metadata sent
POST /api/sensory/store
{
  "type": "meeting_recording",
  "timestamp": "2024-01-20T10:30:00Z",
  "duration": 3600,
  "audio_summary": "...",
  "visual_summary": "...",
  "participants": ["User", "Colleague"],
  "keywords": ["project", "deadline", "budget"],
  "raw_data_path": "/local/phoenix/sensory/meeting_001.aenc",
  "memory_layers": {
    "l1": "stm:sensory:audio:1704067200",  // STM layer
    "l2": "wm:sensory:screen:1704067200",  // WM layer
    "l3": "epm:sensory:session:1704067200", // EPM layer
    "l4": "mind:sensory:extracted:meeting", // Mind Vault
    "l5": "body:sensory:settings:recording", // Body Vault
    "l6": "ltm:sensory:evolution:transcription", // LTM layer
    "l7": "rfm:sensory:pattern:stress_error" // RFM layer
  }
}
```

#### Memory Layer Reference

For detailed information on Phoenix's memory architecture, see:
- **`docs/LAYERED_MEMORY_ARCHITECTURE.md`** - Complete memory system documentation
- **`docs/DATABASE_SOLUTIONS_ARCHITECTURE.md`** - Storage implementation details

**Existing Memory System:**
- **Neural Cortex Strata**: 5 layers (STM, WM, LTM, EPM, RFM)
- **Vital Organ Vaults**: 3 vaults (Mind, Body, Soul)
- **Context Engine**: 6 weighted context layers
- **Vector KB**: Semantic search capabilities

---

### 6. Privacy & Security Implementation

#### Local-First Architecture:
```
┌─────────────────────────────────────────┐
│      LOCAL MACHINE (Trusted Zone)       │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │   Encrypted Sensory Storage     │   │
│  │   • AES-256-GCM encryption      │   │
│  │   • User keychain protected     │   │
│  │   • Automatic purging (config)  │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │    Processing Sandbox           │   │
│  │   • No raw data leaves          │   │
│  │   • Only summaries to Phoenix   │   │
│  │   • User review before sending  │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

#### User Consent Framework:
```javascript
// Phoenix AGI requests permissions
User> "Record my meeting tomorrow at 2 PM"

Phoenix AGI:
[REQUEST PERMISSION] Meeting Recording
• Audio from microphone: ✓
• Screen capture: ✓  
• Webcam: ✗ (not needed)
• Duration: 60 minutes
• Storage: Local encrypted, summary to memory
• Auto-delete after: 30 days

[Approve] [Modify] [Deny]
```

---

### 7. Chat Interface Integration

#### Natural Language Commands:
```
User: "Record my screen while I debug this issue"
→ Phoenix starts screen capture with debug overlay

User: "What wireless networks are around?"
→ Phoenix activates WiFi scan, returns SSIDs + security

User: "Transcribe my last meeting"
→ Phoenix retrieves audio, processes with Whisper

User: "Show me network traffic to api.server.com"
→ Phoenix filters WiFi capture, displays relevant packets

User: "Record audio for the next hour"
→ Phoenix starts ambient recording with VAD
```

#### Status Indicators:
```
[Phoenix AGI - Recording Active]
• Audio: 🎤 (Voice activity: 72%)
• Screen: 🖥️ (Active window: VSCode)
• WiFi: 📡 (Monitoring: 5 devices)
• Storage: 💾 (2.3GB today)

Type "stop recording" or use Ctrl+Shift+R
```

---

### 8. Tool Auto-Creation Pipeline

Phoenix AGI will automatically build needed tools:

```rust
// Example: User requests WiFi analysis
User> "Analyze my home network performance"

Phoenix AGI execution:
1. Check for wireless_sniffer tool
2. If not exists → build from spec
3. Deploy as local service
4. Run scan for 60 seconds
5. Analyze results
6. Present findings in chat
7. Store tool for future use

[CREATING] wireless_analyzer tool...
[COMPILING] Rust + libpcap dependencies
[DEPLOYING] Local service on port 8998
[SCANNING] 2.4GHz and 5GHz bands
[RESULT] Found 14 devices, channel 6 congested
[RECOMMENDATION] Switch to channel 11
```

---

### 9. Hardware Compatibility Layer

#### Auto-Detection Service:
```rust
struct HardwareDetector {
    fn detect_audio_interfaces() -> Vec<AudioDevice>;
    fn detect_cameras() -> Vec<Camera>;
    fn detect_wifi_adapters() -> Vec<WifiAdapter>;
    fn detect_bluetooth() -> Vec<BluetoothAdapter>;
}

// Phoenix AGI adapts to available hardware
if has_monitor_mode_wifi() {
    enable_advanced_wifi_sniffing();
} else {
    enable_basic_wifi_scanning();
}
```

#### Fallback Strategies:
- **No compatible WiFi adapter?** → Use existing connection metrics
- **No webcam?** → Screen capture only
- **Limited storage?** → Adjust compression, retention
- **Privacy mode?** → Sensors disabled, manual activation only

---

### 10. Example Complete Workflow

```
User: "Help me prepare for my presentation tomorrow"

Phoenix AGI:
[ACTIVATING] Multi-modal assistance mode
• Audio recording enabled for practice feedback
• Screen capture for slide timing
• WiFi monitor disabled (privacy mode)

[READY] Begin when you're ready. I'll:
1. Record your rehearsal
2. Time each slide
3. Transcribe your speech
4. Suggest improvements
5. Store in memory for review

[STARTING] Recording now... (click to stop)
```

---

## Implementation Priority

### Phase 1 (Immediate):
1. **Audio capture** with Whisper.cpp (local, private)
2. **Basic screen capture** (screenshots on demand)
3. **Simple WiFi scanning** (available networks)

### Phase 2 (Short-term):
1. **Continuous ambient recording** (configurable)
2. **Video recording** with OpenCV
3. **Wireless traffic analysis** (with compatible hardware)
4. **Bluetooth device discovery**

### Phase 3 (Advanced):
1. **Real-time multi-modal correlation**
2. **Advanced wireless forensics**
3. **RF spectrum analysis** (with SDR)
4. **Predictive sensory activation**

---

## Ethical Guardrails

### Built-in Constraints:
```rust
impl EthicalGuardrails {
    fn check_recording_consent(&self) -> bool;
    fn blur_sensitive_content(&self, frame) -> Frame;
    fn anonymize_network_data(&self, packets) -> Packets;
    fn enforce_retention_policy(&self) -> Cleanup;
    fn require_periodic_review(&self) -> UserPrompt;
}
```

### Transparency Features:
- Always visible recording indicator
- Accessible recording logs
- Easy data deletion
- Clear data flow visualization
- Regular privacy reviews

---

## Integration with Existing Systems

### Overlap with MULTI_MODAL_ARCHITECTURE.md

**Existing Features** (already documented in `MULTI_MODAL_ARCHITECTURE.md`):
- ✅ Multi-modal recording infrastructure
- ✅ Audio/video capture (feature-gated)
- ✅ Emotion detection integration
- ✅ Encrypted storage (.phoenixrec format)
- ✅ Always-listening mode skeleton

**New Features** (this addendum):
- 🆕 Continuous ambient recording with ring buffer
- 🆕 Meeting transcription with speaker diarization
- 🆕 Audio analysis (tone, sentiment, keywords)
- 🆕 Desktop capture service (backend)
- 🆕 Visual data extraction (OCR, diagrams)
- 🆕 WiFi/Bluetooth intelligence
- 🆕 Context correlation engine
- 🆕 Sensory memory mapping (L1-L7 to existing layers)

**Recommendation**: Keep `MULTI_MODAL_ARCHITECTURE.md` for existing features, use this addendum for new multimedia/network intelligence features.

---

## References

- **Memory Architecture**: `docs/LAYERED_MEMORY_ARCHITECTURE.md`
- **Multi-Modal System**: `docs/MULTI_MODAL_ARCHITECTURE.md`
- **Database Systems**: `docs/DATABASE_SOLUTIONS_ARCHITECTURE.md`
- **Screen Recording**: `docs/SCREEN_RECORDING_CONFIRMATION.md`
- **Implementation Plan**: `docs/MULTIMEDIA_NETWORK_INTELLIGENCE_IMPLEMENTATION_PLAN.md`

---

**Phoenix AGI becomes a true digital twin with these sensory capabilities, understanding not just your digital world but your physical environment, all while maintaining the clean Claude Desktop interface and transparent tool creation.**
