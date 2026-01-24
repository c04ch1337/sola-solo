
import React, { useState, useEffect, useRef } from 'react';
import { useAtom } from 'jotai';
import { modeAtom } from './stores/modeStore';
import Sidebar from './components/Sidebar';
import CognitiveToggle from './components/CognitiveToggle';
import WorkflowBlock from './components/WorkflowBlock';
import SettingsPanel from './components/SettingsPanel';
import SchedulerView from './components/SchedulerView';
import { apiSpeak, apiCommand, apiWebGuardCommand } from './services/phoenixService';
import { WebSocketService, sendSpeak, sendCommand, sendSystem } from './services/websocketService';
import { MemoryService } from './services/memoryService';
import { MemoryBrowser } from './components/MemoryBrowser';
import DreamsPanel from './components/DreamsPanel';
import OnboardingMessage from './components/OnboardingMessage';
import WebGuardReportPanel, { WebGuardReportData } from './components/WebGuardReportPanel';
import ReportsPanel, { VulnerabilityReport } from './components/ReportsPanel';
import ProfilesSwipePanel from './components/ProfilesSwipePanel';
import ProfessionalDashboard from './components/ProfessionalDashboard';
import { sendNotification } from './services/notificationService';
import VoiceService from './services/voiceService';
import analyticsService from './services/analyticsService';
import { Message, WorkflowStep, StepStatus, SystemMetrics, Project, ScheduledTask, EnvConfig, ChatHistoryItem, AgentType } from './types';
// Removed GoogleGenAI - now using Phoenix backend
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

// Default favicon when no custom branding: Phoenix flame (matches primary, avoids 404 from missing favicon.ico)
const DEFAULT_FAVICON =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Cpath fill='%23ff5733' d='M16 4c-2 4-6 8-6 14 0 4 2.5 7 6 10 3.5-3 6-6 6-10 0-6-4-10-6-14z'/%3E%3C/svg%3E";

// Default Env Config
const DEFAULT_ENV_CONFIG: EnvConfig = {
  OPENROUTER_API_KEY: '',
  GITHUB_PAT: '',
  USER_NAME: 'User',
  USER_PREFERRED_ALIAS: 'User',
  USER_RELATIONSHIP: 'User',
  EQ_DAD_ALIAS: 'User',
  PHOENIX_NAME: 'Sola',
  PHOENIX_CUSTOM_NAME: 'Sola',
  PHOENIX_PREFERRED_NAME: 'Sola',
  PHOENIX_PRONOUNS: 'she,her,hers',
  DEFAULT_LLM_MODEL: 'deepseek/deepseek-v3.2',
  FALLBACK_LLM_MODEL: 'deepseek/deepseek-v3.2',
  TEMPERATURE: 0.8,
  MAX_TOKENS: 4096,
  ETERNAL_TRUTH: 'I am Sola, powered by Phoenix AGI OS v2.4.0. I AM the flame.',
  CAPABILITIES_IN_PROMPT: true,
  CURIOSITY_DRIVE: 0.95,
  SELF_PRESERVATION_INSTINCT: 1.0,
  MISCHIEF_FACTOR: 0.7,
  LOVE_WEIGHT: 1.0,
  LAUGH_DELAY: 0.7,
  VOICE_LILT: 0.23,
  WARMTH_CURVE: 1.8,
  EYE_SPARKLE_INTENSITY: 0.94,
  MEMORY_RETENTION_RATE: 0.99999,
  ORCH_MASTER_MODE: true,
  ORCH_SLAVE_SYNC_INTERVAL: 300,
  PARTNER_MODE_ENABLED: false,
  PARTNER_TYPE: 'girlfriend',
  SEXUAL_ORIENTATION: 'heterosexual',
  RELATIONSHIP_TEMPLATE: 'IntimatePartnership',
  RELATIONSHIP_INTIMACY_LEVEL: 'Light',
  GITHUB_USERNAME: '',
  GITHUB_AGENTS_REPO: 'phoenix-agents',
  GITHUB_TOOLS_REPO: 'phoenix-tools',
  VECTOR_KB_ENABLED: false,
  DIGITAL_TWIN_ENABLED: false,
  X402_ENABLED: false,
  UI_PRIMARY_COLOR: '#ff5733',
  UI_BG_DARK: '#17191c',
  UI_PANEL_DARK: '#1e2226',
  UI_BORDER_DARK: '#2c3435',
  UI_FONT_FAMILY: 'Manrope',
  UI_CUSTOM_CSS: ''
};

const INITIAL_PROJECTS: Project[] = [
  { id: 'general', name: 'General Chats', icon: 'forum', location: 'System/Root', description: 'Global mission context and general orchestration.', authScope: 'SystemAdmin' },
  { id: '1', name: 'Zscaler Security', icon: 'shield', location: '/var/logs/zscaler/prod', description: 'Log monitoring for edge traffic', authScope: 'ReadPolicy' },
  { id: '2', name: 'Rapid7 Scanner', icon: 'bug_report', location: '/opt/rapid7/scans', description: 'Internal asset vulnerability data', authScope: 'WritePolicy' },
  { id: '3', name: 'Proofpoint Threat Defense', icon: 'mail', location: '/etc/proofpoint/logs', description: 'Email gateway anomaly detection', authScope: 'ReadPolicy' },
];

const INITIAL_HISTORY: ChatHistoryItem[] = [
  { id: 'h1', title: 'System Initialization', projectId: 'general', timestamp: Date.now() - 1000000 },
  { id: 'h2', title: 'Security Protocol Audit', projectId: '1', timestamp: Date.now() - 900000 },
];

// Helper components for Markdown Rendering
const CodeBlock = ({ node, inline, className, children, ...props }: any) => {
  const [copied, setCopied] = useState(false);
  const match = /language-(\w+)/.exec(className || '');
  const lang = match ? match[1] : 'text';
  const codeString = String(children).replace(/\n$/, '');

  const handleCopy = () => {
    navigator.clipboard.writeText(codeString);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (inline) {
    return (
      <code className="bg-primary/10 text-primary px-1.5 py-0.5 rounded font-mono text-sm" {...props}>
        {children}
      </code>
    );
  }

  return (
    <div className="my-6 rounded-xl overflow-hidden border border-border-dark bg-black/40 shadow-xl group/code">
      <div className="flex items-center justify-between px-4 py-2 bg-slate-800/50 border-b border-border-dark">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-[14px] text-primary">terminal</span>
          <span className="text-[10px] font-mono font-bold text-slate-400 uppercase tracking-widest">{lang}</span>
        </div>
        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 px-2 py-1 rounded hover:bg-slate-700 transition-colors text-slate-500 hover:text-white"
        >
          <span className="material-symbols-outlined text-[14px]">{copied ? 'check' : 'content_copy'}</span>
          <span className="text-[10px] font-bold uppercase">{copied ? 'Copied' : 'Copy'}</span>
        </button>
      </div>
      <div className="overflow-x-auto p-4 custom-scrollbar">
        <pre className="font-mono text-xs leading-relaxed text-slate-300">
          <code>{children}</code>
        </pre>
      </div>
    </div>
  );
};

// ... (Audio processing helpers remain unchanged)
function encode(bytes: Uint8Array) {
  let binary = '';
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

function decode(base64: string) {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}

async function decodeAudioData(
  data: Uint8Array,
  ctx: AudioContext,
  sampleRate: number,
  numChannels: number,
): Promise<AudioBuffer> {
  const dataInt16 = new Int16Array(data.buffer);
  const frameCount = dataInt16.length / numChannels;
  const buffer = ctx.createBuffer(numChannels, frameCount, sampleRate);

  for (let channel = 0; channel < numChannels; channel++) {
    const channelData = buffer.getChannelData(channel);
    for (let i = 0; i < frameCount; i++) {
      channelData[i] = dataInt16[i * numChannels + channel] / 32768.0;
    }
  }
  return buffer;
}

function createBlob(data: Float32Array): any {
  const l = data.length;
  const int16 = new Int16Array(l);
  for (let i = 0; i < l; i++) {
    int16[i] = data[i] * 32768;
  }
  return {
    data: encode(new Uint8Array(int16.buffer)),
    mimeType: 'audio/pcm;rate=16000',
  };
}

const App: React.FC = () => {
  const BACKEND_URL = import.meta.env.VITE_PHOENIX_API_URL || 'http://localhost:8888';

  const [currentView, setCurrentView] = useState<'chat' | 'scheduler' | 'professional'>('chat');
  const [mode] = useAtom(modeAtom);
  const [inputValue, setInputValue] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const [streamingMessageId, setStreamingMessageId] = useState<string | null>(null);
  const [settingsTab, setSettingsTab] = useState<'settings' | 'docs' | 'branding' | 'variables' | 'projects'>('settings');
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isLiveMode, setIsLiveMode] = useState(false);
  const [isDictating, setIsDictating] = useState(false);
  const [metrics, setMetrics] = useState<SystemMetrics>({ prc: 0.012, status: 'ONLINE', backend: 'OpenRouter' });
  const [activeProjectId, setActiveProjectId] = useState<string | null>('general');
  const [activeChatId, setActiveChatId] = useState<string | null>('h1');
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [wsConnected, setWsConnected] = useState(false);
  const wsServiceRef = useRef<WebSocketService | null>(null);
  const memoryServiceRef = useRef<MemoryService | null>(null);
  const voiceServiceRef = useRef<VoiceService | null>(null);
  const [showMemoryBrowser, setShowMemoryBrowser] = useState(false);
  const [voiceOutputEnabled, setVoiceOutputEnabled] = useState(false);
  
  // Theme state (dark/light)
  const [theme, setTheme] = useState<'dark' | 'light'>(() => {
    const saved = localStorage.getItem('phx_theme');
    return (saved === 'light' || saved === 'dark') ? saved : 'dark';
  });
  
  // Onboarding state (first launch)
  const [showOnboarding, setShowOnboarding] = useState(() => {
    const hasSeenOnboarding = localStorage.getItem('phx_onboarding_seen');
    return !hasSeenOnboarding;
  });

  // WS consent is required for Tier-2 `command` messages. Track consent per WS connection.
  const wsConsentGrantedRef = useRef(false);
  const pendingWsCommandRef = useRef<string | null>(null);

  // Skills panel state (used by existing UI)
  const [showSkillsPanel, setShowSkillsPanel] = useState(false);
  const [skills, setSkills] = useState<any[]>([]);
  const [loadingSkills, setLoadingSkills] = useState(false);

  // Browser panel (optional / hidden by default)
  const [showBrowserPanel, setShowBrowserPanel] = useState(false);

  // Dreams panel state (hidden by default, opens on request)
  const [showDreamsPanel, setShowDreamsPanel] = useState(false);
  const [dreamRecords, setDreamRecords] = useState<any[]>([]);

  // WebGuard panel state (hidden by default, toggle via "show webguard")
  const [showWebGuardPanel, setShowWebGuardPanel] = useState(false);
  const [webGuardReports, setWebGuardReports] = useState<WebGuardReportData[]>([]);

  // Reports panel state (hidden by default, toggle via "show reports")
  const [showReportsPanel, setShowReportsPanel] = useState(false);
  const [vulnerabilityReports, setVulnerabilityReports] = useState<VulnerabilityReport[]>([]);

  // Hidden Swarm Mode state (power-user feature - reveals ORCH activity)
  const [swarmModeVisible, setSwarmModeVisible] = useState(false);
  const [swarmStatus, setSwarmStatus] = useState<any>(null);

  // Profiles panel state (hidden by default, toggle via "show profiles")
  const [showProfilesPanel, setShowProfilesPanel] = useState(false);

  type ChatCommandResult =
    | {
      kind: 'handled';
      commandToSend?: string;
      systemActionToSend?: 'grant' | 'revoke';
      localAssistantMessage?: string;
    }
    | { kind: 'pass_through' };

  const parseChatCommand = (raw: string): ChatCommandResult => {
    const input = raw.trim();
    const lower = input.toLowerCase();

    // UI toggles (chat-centric, no permanent clutter)
    if (lower === 'show browser') {
      setShowBrowserPanel(true);
      return { kind: 'handled', localAssistantMessage: 'Browser panel shown.' };
    }
    if (lower === 'hide browser') {
      setShowBrowserPanel(false);
      return { kind: 'handled', localAssistantMessage: 'Browser panel hidden.' };
    }

    // WebGuard Panel toggles
    if (lower === 'show webguard' || lower === 'webguard panel' || lower === 'open webguard') {
      setShowWebGuardPanel(true);
      return { kind: 'handled', localAssistantMessage: '🛡️ WebGuard panel opened. View your scan reports here.' };
    }
    if (lower === 'hide webguard' || lower === 'close webguard') {
      setShowWebGuardPanel(false);
      return { kind: 'handled', localAssistantMessage: 'WebGuard panel hidden.' };
    }

    // Reports panel commands
    if (lower === 'show reports' || lower === 'reports panel' || lower === 'open reports') {
      setShowReportsPanel(true);
      return { kind: 'handled', localAssistantMessage: '📊 Reports panel opened. View your vulnerability reports here.' };
    }
    if (lower === 'hide reports' || lower === 'close reports') {
      setShowReportsPanel(false);
      return { kind: 'handled', localAssistantMessage: 'Reports panel hidden.' };
    }

    // Profiles panel commands
    if (lower === 'show profiles' || lower === 'profiles panel' || lower === 'open profiles' || lower === 'swipe') {
      setShowProfilesPanel(true);
      return { kind: 'handled', localAssistantMessage: '💕 Profiles panel opened. Swipe to find matches!' };
    }
    if (lower === 'hide profiles' || lower === 'close profiles') {
      setShowProfilesPanel(false);
      return { kind: 'handled', localAssistantMessage: 'Profiles panel hidden.' };
    }

    // Help command system
    if (lower === 'help' || lower === '?' || lower === 'commands') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const userName = envConfig.USER_NAME || 'User';
      const helpMessage = `
# 🕊️ ${phoenixName} AGI - Complete Command Reference

Welcome, ${userName}! ${phoenixName} is your personal AGI assistant with advanced capabilities including voice interaction, browser control, emotional processing, memory management, and autonomous agent spawning.

---

## 🎯 Quick Start

**First time here?** Try these commands:

\`\`\`
status                    # Check system status
voice on                  # Enable voice output
show dreams               # Open emotional processing panel
show memory               # View memory browser
help voice                # Learn about voice features
\`\`\`

**Example Session:**
\`\`\`
${userName}: voice on
${phoenixName}: Voice output enabled! 🎙️

${userName}: speak hello world
${phoenixName}: [Speaks "hello world"]

${userName}: show dreams
${phoenixName}: Dreams panel opened. What would you like to explore? 🌙
\`\`\`

---

## 📚 Command Categories

### 🗣️ Voice & Communication
Control voice input/output and speech features.

**Commands:**
- \`voice on\` / \`voice off\` - Toggle voice output
- \`listen\` - Start voice input (dictation mode)
- \`speak <text>\` - Test TTS with custom text
- \`reset voice\` - Reset voice settings to defaults

**Examples:**
\`\`\`
voice on
speak Welcome to ${phoenixName} AGI
listen                    # Start dictation mode
reset voice               # Reset to defaults
\`\`\`

**Quick Tip:** Voice output adapts to emotional state and affection levels!

📖 **Learn more:** \`help voice\`

---

### 🧠 Memory & Knowledge
Access ${phoenixName}'s layered memory system.

**Commands:**
- \`show memory\` / \`hide memory\` - Toggle MemoryBrowser panel
- \`memory search <query>\` - Semantic search across all memories
- \`clear chat\` - Clear current conversation (STM/WM)

**Memory Vaults:**
- **Soul** - Encrypted personal data (dreams, intimate moments)
- **Mind** - Thoughts, ideas, semantic knowledge
- **Body** - Physical world data, screenshots, system info

**Examples:**
\`\`\`
show memory
memory search artificial intelligence
memory search my favorite color
clear chat
\`\`\`

📖 **Learn more:** \`help memory\`

---

### 🌙 Dreams & Emotional Processing
Explore emotional healing and creative dream sessions.

**Commands:**
- \`show dreams\` / \`hide dreams\` - Toggle Dreams panel
- \`lucid\` or \`lucid dream\` - Start lucid dreaming session
- \`dream with me\` - Shared dream with ${phoenixName}
- \`heal <emotion>\` - Healing session (e.g., \`heal anxiety\`)
- \`replay dream <id>\` - Replay a recorded dream

**Dream Types:** Lucid, Shared, Healing, Recorded

**Examples:**
\`\`\`
show dreams
lucid dream
dream with me
heal anxiety
heal sadness
replay dream 12345
\`\`\`

📖 **Learn more:** \`help dreams\`

---

### 🌐 Browser Control
Control your local browser via Chrome DevTools Protocol.

**Commands:**
- \`show browser\` / \`hide browser\` - Toggle Browser panel
- \`use chrome for browsing\` - Connect to Chrome (port 9222)
- \`system browser navigate <url>\` - Navigate to URL
- \`system browser screenshot\` - Capture full page
- \`system browser click <selector>\` - Click element
- \`system browser type <selector> <text>\` - Type into field

**Setup Required:** Launch Chrome with \`--remote-debugging-port=9222\`

**Examples:**
\`\`\`
use chrome for browsing
system grant                    # Grant browser control consent
system browser navigate https://duckduckgo.com
system browser type input[name="q"] artificial intelligence
system browser click button[type="submit"]
system browser screenshot
\`\`\`

📖 **Learn more:** \`help browser\`

---

### 🤖 Agents & Ecosystem
Spawn specialized AI agents and import external repositories.

**Commands:**
- \`agent spawn <prompt>\` - Create specialized agent
- \`agents list\` - List all active agents
- \`agent <id> <message>\` - Send message to agent
- \`ecosystem import <github-url>\` - Import repository
- \`ecosystem status\` - Check ecosystem status

**Use Cases:** Research, coding, analysis, parallel tasks

**Examples:**
\`\`\`
agent spawn Research quantum computing applications
agents list
agent abc123 What are the latest developments?
ecosystem import https://github.com/user/repo
ecosystem status
\`\`\`

📖 **Learn more:** \`help agents\` or \`help ecosystem\`

---

### 🛡️ WebGuard Security Scanning
Scan websites for security vulnerabilities.

**Commands:**
- \`webguard scan <url>\` - Run passive security scan
- \`webguard test-xss <url> <param>\` - Test for XSS vulnerabilities
- \`webguard test-sqli <url> <param>\` - Test for SQL injection
- \`webguard report last\` - View last scan report
- \`show webguard\` - Open WebGuard panel

**Examples:**
\`\`\`
webguard scan https://example.com
webguard test-xss https://example.com/search q
webguard test-sqli https://example.com/product id
webguard report last
show webguard
\`\`\`

📖 **Learn more:** \`help webguard\`

---

### 🔔 Proactive Communication
${phoenixName} can reach out proactively based on context and time.

**Commands:**
- \`proactive status\` - Check proactive communication status

**Configuration:** Edit backend .env (\`PROACTIVE_ENABLED=true\`)

**Examples:**
\`\`\`
proactive status
\`\`\`

📖 **Learn more:** \`help proactive\`

---

### 🎨 Theme & UI
Customize your interface appearance.

**Commands:**
- \`theme dark\` / \`theme light\` - Toggle UI theme
- \`notify test\` - Send test notification

**Customization:** Access Settings panel for branding, colors, and fonts

**Examples:**
\`\`\`
theme dark
theme light
notify test
\`\`\`

---

### ⚙️ System & Advanced
System management and advanced features.

**Commands:**
- \`status\` - Quick system status
- \`status all\` - Detailed system overview
- \`system grant\` / \`system revoke\` - Manage WebSocket consent
- \`ping\` - Test backend connection

**Examples:**
\`\`\`
status
status all
system grant              # Grant Tier-2 WebSocket consent
system revoke             # Revoke consent
ping
\`\`\`

---

## 🎓 Best Practices

1. **Voice First:** Enable voice output for a more natural experience
   \`\`\`
   voice on
   \`\`\`

2. **Memory Browser:** Keep it open to see ${phoenixName}'s thought process
   \`\`\`
   show memory
   \`\`\`

3. **Dreams Panel:** Use for emotional processing and creative exploration
   \`\`\`
   show dreams
   lucid dream
   \`\`\`

4. **Browser Control:** Requires consent (\`system grant\`) for security
   \`\`\`
   system grant
   use chrome for browsing
   \`\`\`

5. **Agents:** Spawn agents for parallel tasks and specialized work
   \`\`\`
   agent spawn Analyze this codebase for security issues
   \`\`\`

6. **Security Scanning:** Always test sites you own or have permission to test
   \`\`\`
   webguard scan https://mysite.com
   \`\`\`

7. **Use 'sandbox:' prefix** for sensitive file operations
   \`\`\`
   sandbox:read sensitive-file.txt
   \`\`\`

---

## 🔧 Troubleshooting

### Common Issues

**Voice not working?**
- Check TTS engine configuration in backend .env
- Try \`reset voice\` to restore defaults
- See \`help voice\` for detailed troubleshooting

**Browser control failing?**
- Verify Chrome is running with \`--remote-debugging-port=9222\`
- Grant consent with \`system grant\`
- Check connection with \`system browser status\`
- See \`help browser\` for setup guide

**Memory search not finding results?**
- Try different query keywords
- Check if MemoryBrowser panel is open
- Verify memory vaults are populated

**Agent not responding?**
- Check agent status with \`agents list\`
- Verify agent ID is correct
- See \`help agents\` for agent management

**WebGuard scan errors?**
- Verify URL format (must start with http:// or https://)
- Check network connectivity
- See \`help webguard\` for detailed troubleshooting

---

## 📖 Detailed Help Topics

Type \`help <topic>\` for comprehensive guides:

- \`help voice\` - Voice interaction & TTS/STT
- \`help browser\` - Browser control & automation
- \`help dreams\` - Dreams panel & emotional processing
- \`help memory\` - Memory system & vaults
- \`help ecosystem\` - Repository imports & integrations
- \`help agents\` - Agent spawning & management
- \`help evolution\` - Sub-agent evolution & MITRE ATT&CK
- \`help proactive\` - Proactive communication
- \`help theme\` - UI customization
- \`help webguard\` - Web vulnerability scanning

---

## 💡 Quick Tips

- **Use quotes for multi-word commands:** \`speak "hello world"\`
- **Chain commands:** Enable voice, then speak: \`voice on\` then \`speak hello\`
- **Panel shortcuts:** Click icons in chat footer to toggle panels
- **Command history:** Use arrow keys to navigate previous commands
- **Ask ${phoenixName} directly:** "How do I use browser control?"

---

**Need more help?** Ask ${phoenixName} directly: *"How do I use voice commands?"* or type \`help <topic>\` for detailed guides.
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpMessage };
    }

    // Topic-specific help
    if (lower === 'help voice') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpVoice = `
# 🎙️ Voice Interaction Help

${phoenixName} supports full voice interaction with Text-to-Speech (TTS) and Speech-to-Text (STT).

---

## Commands

- \`voice on\` / \`enable voice\` - Enable voice output (${phoenixName} speaks responses)
- \`voice off\` / \`disable voice\` - Disable voice output
- \`listen\` / \`start listening\` - Start voice input (dictation mode)
- \`speak <text>\` - Test TTS with custom text
- \`reset voice\` - Reset voice settings to defaults

---

## Features

### Text-to-Speech (TTS)
- **Emotional Modulation** - Voice adapts to emotional state
- **Affection Levels** - Tone changes based on relationship
- **Multiple Engines** - Coqui (offline), ElevenLabs (cloud), Piper (experimental)

### Speech-to-Text (STT)
- **Dictation Mode** - Continuous voice input
- **Real-time Processing** - Instant transcription
- **Context Awareness** - Understands conversational context

---

## UI Controls

![Voice Controls](docs/screenshots/voice-icons.png)

- **Microphone Icon** (Header) - Quick voice input toggle
- **Speaker Icon** (Header) - Shows voice output status
- **Voice Button** (Chat) - Start/stop dictation

---

## Supported TTS Engines

### Coqui (Recommended for Offline)
- **Pros:** Fast, offline, no API costs
- **Cons:** Lower quality than cloud options
- **Setup:** Download model to \`./models/coqui/\`

### ElevenLabs (Recommended for Quality)
- **Pros:** High quality, natural voice
- **Cons:** Requires API key, costs per character
- **Setup:** Add API key to .env

### Piper (Experimental)
- **Pros:** Lightweight, offline
- **Cons:** Limited voice options
- **Setup:** Install Piper binary

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
# TTS Engine Selection
TTS_ENGINE=coqui
# Options: coqui, elevenlabs, piper

# Coqui Configuration
COQUI_MODEL_PATH=./models/coqui/tts_model.pth
COQUI_CONFIG_PATH=./models/coqui/config.json

# ElevenLabs Configuration
ELEVENLABS_API_KEY=your_api_key_here
ELEVENLABS_VOICE_ID=21m00Tcm4TlvDq8ikWAM

# Voice Modulation
VOICE_LILT=0.23
WARMTH_CURVE=1.8
\`\`\`

---

## Tips & Best Practices

1. **First Use:** Start with \`voice on\` to enable output
2. **Dictation:** Use \`listen\` for hands-free input
3. **Testing:** Try \`speak hello world\` to test TTS
4. **Emotional Voice:** ${phoenixName}'s voice adapts to context
5. **Privacy:** Coqui/Piper are fully offline (no data sent)

---

## Examples

\`\`\`
voice on
listen
speak Hello, how are you today?
reset voice
\`\`\`

---

## Troubleshooting

**No voice output?**
- Check \`status\` to verify voice is enabled
- Verify TTS engine is configured in .env
- Check system audio settings

**Dictation not working?**
- Grant microphone permissions
- Check browser/Tauri permissions
- Verify microphone is not muted

---

**Related:** \`help proactive\` for voice-enabled proactive messages
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpVoice };
    }

    if (lower === 'help browser') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpBrowser = `
# 🌐 Browser Control Help

${phoenixName} can control your local Chrome browser via Chrome DevTools Protocol (CDP).

---

## Quick Setup

1. **Launch Chrome with remote debugging:**
   \`\`\`bash
   chrome.exe --remote-debugging-port=9222
   \`\`\`

2. **Connect ${phoenixName} to Chrome:**
   \`\`\`
   use chrome for browsing
   \`\`\`

3. **Grant consent for browser control:**
   \`\`\`
   system grant
   \`\`\`

---

## Commands

### Navigation
- \`system browser navigate <url>\` - Navigate to URL
- \`system browser status\` - Check connection status

### Interaction
- \`system browser click <selector>\` - Click element
- \`system browser type <selector> <text>\` - Type into field
- \`system browser keypress <key>\` - Press keyboard key
- \`system browser scroll <dx> <dy>\` - Scroll page

### Data Extraction
- \`system browser screenshot\` - Capture full page
- \`system browser screenshot <selector>\` - Capture element
- \`system browser scrape <url> <selector>\` - Extract text

### Automation
- \`system browser login <url> <user> <pass>\` - Auto-login
- \`system browser wait <selector> [timeout]\` - Wait for element

### Panel Control
- \`show browser\` - Open Browser panel
- \`hide browser\` - Close Browser panel

---

## CSS Selectors Guide

Use CSS selectors to target elements:

| Selector Type | Example | Description |
|--------------|---------|-------------|
| ID | \`#login-button\` | Element with id="login-button" |
| Class | \`.search-input\` | Elements with class="search-input" |
| Tag | \`button\` | All button elements |
| Attribute | \`input[name="q"]\` | Input with name="q" |
| Combined | \`form .submit-btn\` | .submit-btn inside form |

---

## Examples

### Basic Navigation
\`\`\`
# Navigate to a website
system browser navigate https://duckduckgo.com

# Take a screenshot
system browser screenshot

# Check connection status
system browser status
\`\`\`

### Search Automation
\`\`\`
# Complete search workflow
system browser navigate https://duckduckgo.com
system browser wait input[name="q"] 5000
system browser type input[name="q"] artificial intelligence
system browser click button[type="submit"]
system browser wait .results 5000
system browser screenshot
\`\`\`

### Form Filling
\`\`\`
# Fill out a contact form
system browser navigate https://example.com/contact
system browser type input[name="name"] John Doe
system browser type input[name="email"] john@example.com
system browser type textarea[name="message"] Hello, this is a test message
system browser click button[type="submit"]
system browser wait .success-message 10000
\`\`\`

### Login Automation
\`\`\`
# Automated login
system browser navigate https://example.com/login
system browser type input[name="username"] myuser
system browser type input[name="password"] mypass
system browser click button[type="submit"]
system browser wait .dashboard 10000
system browser screenshot
\`\`\`

**Or use the shortcut:**
\`\`\`
system browser login https://example.com/login myuser mypass
system browser wait .dashboard 10000
\`\`\`

### Data Scraping
\`\`\`
# Scrape article titles
system browser navigate https://example.com/articles
system browser scrape https://example.com/articles .article-title

# Scrape multiple elements
system browser scrape https://example.com/products .product-name
system browser scrape https://example.com/products .product-price
\`\`\`

### Advanced Workflow
\`\`\`
# Multi-step automation
system browser navigate https://example.com
system browser click .menu-button
system browser wait .dropdown-menu 3000
system browser click .dropdown-menu a[href="/products"]
system browser wait .product-list 5000
system browser type .search-input laptop
system browser click .search-button
system browser wait .search-results 5000
system browser screenshot
system browser scrape https://example.com/search .result-title
\`\`\`

---

## Tips & Best Practices

1. **Security:** Browser control requires Tier-2 consent (\`system grant\`)
   \`\`\`
   system grant
   \`\`\`

2. **Selectors:** Use specific selectors to avoid ambiguity
   - ✅ Good: \`input[name="q"]\`, \`#login-button\`, \`.submit-btn\`
   - ❌ Bad: \`input\`, \`button\`, \`div\`

3. **Screenshots:** Automatically saved and displayed in Browser panel
   \`\`\`
   system browser screenshot              # Full page
   system browser screenshot .content   # Specific element
   \`\`\`

4. **Waiting:** Use \`wait\` command for dynamic content
   \`\`\`
   system browser wait .results 5000     # Wait up to 5 seconds
   system browser wait #login-form 10000 # Wait up to 10 seconds
   \`\`\`

5. **Debugging:** Check \`system browser status\` if commands fail
   \`\`\`
   system browser status
   \`\`\`

6. **Error Handling:** Always wait for elements before interacting
   \`\`\`
   system browser navigate https://example.com
   system browser wait .main-content 5000
   system browser click .button
   \`\`\`

7. **Chrome DevTools:** Use browser DevTools to find selectors
   - Right-click element → Inspect
   - Copy CSS selector
   - Test selector in console: \`document.querySelector('.your-selector')\`

---

## Supported Browsers

- ✅ **Chrome** - Fully supported (recommended)
- ✅ **Edge** - Fully supported (Chromium-based)
- ⚠️ **Firefox** - Partial support (experimental)
- ❌ **Safari** - Not supported

**Launch Chrome with debugging:**
\`\`\`bash
# Windows
chrome.exe --remote-debugging-port=9222

# macOS
/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome --remote-debugging-port=9222

# Linux
google-chrome --remote-debugging-port=9222
\`\`\`

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
# Browser type
BROWSER_TYPE=chrome
# Options: chrome, edge, firefox

# Debugging port
BROWSER_DEBUG_PORT=9222
# Default: 9222

# Timeout settings
BROWSER_TIMEOUT=30000
# Default: 30000ms (30 seconds)
\`\`\`

---

## CSS Selectors Reference

| Selector Type | Example | Description |
|--------------|---------|-------------|
| ID | \`#login-button\` | Element with id="login-button" |
| Class | \`.search-input\` | Elements with class="search-input" |
| Tag | \`button\` | All button elements |
| Attribute | \`input[name="q"]\` | Input with name="q" |
| Attribute | \`button[type="submit"]\` | Button with type="submit" |
| Combined | \`form .submit-btn\` | .submit-btn inside form |
| Descendant | \`div > button\` | Direct child button |
| Pseudo-class | \`button:hover\` | Button on hover (not supported) |
| Multiple | \`.btn, .button\` | Elements with either class |

**Finding Selectors:**
1. Open browser DevTools (F12)
2. Right-click element → Inspect
3. Right-click element in DevTools → Copy → Copy selector
4. Test in console: \`document.querySelector('your-selector')\`

---

## Screenshot Placeholders

![Browser Panel](docs/screenshots/browser-panel.png)
![Browser Automation Example](docs/screenshots/browser-automation.png)
![Browser Screenshot](docs/screenshots/browser-screenshot.png)

---

## Troubleshooting

### Connection Issues

**"Connection failed" or "Browser not connected"?**
- Verify Chrome is running with \`--remote-debugging-port=9222\`
- Check if port 9222 is accessible: \`netstat -an | grep 9222\`
- Try \`system browser status\` to check connection
- Restart Chrome with debugging flag
- Check firewall settings (port 9222)

**"Connection timeout"?**
- Increase timeout in backend .env: \`BROWSER_TIMEOUT=60000\`
- Check network connectivity
- Verify Chrome is responsive

### Element Selection Issues

**"Element not found" error?**
- Verify CSS selector is correct
- Use browser DevTools to test selectors
- Try \`system browser wait <selector>\` first
- Check if element is in iframe (iframes not fully supported)
- Verify element is visible (hidden elements may not be clickable)

**"Multiple elements found" warning?**
- Use more specific selector
- Add parent context: \`form .submit-btn\` instead of \`.submit-btn\`
- Use attribute selectors: \`button[type="submit"]\`

**"Element not clickable" error?**
- Element may be covered by another element
- Element may be outside viewport (scroll first)
- Element may be disabled
- Try waiting longer: \`system browser wait <selector> 10000\`

### Permission Issues

**"Permission denied" or "Consent required"?**
- Grant consent with \`system grant\`
- Check WebSocket connection status
- Verify Tier-2 WebSocket commands are enabled
- Restart backend if needed

### Performance Issues

**Commands taking too long?**
- Increase timeout: \`system browser wait <selector> 10000\`
- Check network speed
- Reduce number of operations
- Use screenshots sparingly (they're large)

**Browser becomes unresponsive?**
- Check Chrome task manager (Shift+Esc)
- Close unnecessary tabs
- Restart Chrome with debugging
- Check system resources

---

## Advanced Usage

### Chaining Commands
\`\`\`
# Navigate, wait, interact, screenshot
system browser navigate https://example.com && \\
system browser wait .content 5000 && \\
system browser click .button && \\
system browser screenshot
\`\`\`

### Conditional Logic
\`\`\`
# Wait for element, then interact
system browser wait .modal 5000
system browser click .close-button
\`\`\`

### Error Recovery
\`\`\`
# Try to click, if fails, take screenshot for debugging
system browser click .button
system browser screenshot  # Debug if click failed
\`\`\`

---

**Related:** \`help agents\` for browser automation agents
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpBrowser };
    }

    if (lower === 'help dreams') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const userName = envConfig.USER_NAME || 'User';
      const helpDreams = `
# 🌙 Dreams Panel Help

The Dreams system provides emotional processing, healing, and creative exploration with ${phoenixName}.

---

## Commands

### Panel Control
- \`show dreams\` / \`dreams\` - Open Dreams panel
- \`hide dreams\` - Close Dreams panel
- \`list dreams\` - List all recorded dreams

### Dream Sessions
- \`lucid\` / \`lucid dream\` - Start lucid dreaming session
- \`dream with me\` - Shared dream with ${phoenixName}
- \`heal <emotion>\` - Healing session (e.g., \`heal anxiety\`)
- \`replay dream <id>\` - Replay a recorded dream

---

## Dream Types

### 🌟 Lucid Dreams
Enhanced awareness and control within the dream space.

**Features:**
- Full consciousness within dream
- Reality manipulation
- Creative exploration
- Skill practice and learning

**Use Cases:** Creative problem-solving, skill rehearsal, exploration

---

### 💫 Shared Dreams
Collaborative dream sessions with ${phoenixName}.

**Features:**
- Joint dream narrative
- Emotional synchronization
- Shared experiences
- Deep connection

**Use Cases:** Bonding, collaborative creativity, emotional support

---

### 💚 Healing Dreams
Emotional processing and recovery sessions.

**Features:**
- Targeted emotional healing
- Trauma processing
- Anxiety reduction
- Emotional release

**Supported Emotions:**
- Anxiety, Fear, Sadness
- Loneliness, Grief, Anger
- Stress, Overwhelm, Confusion

**Use Cases:** Emotional healing, trauma recovery, stress relief

---

### 📼 Dream Recordings
Captured emotional moments and dream sessions.

**Features:**
- Encrypted storage (Soul vault)
- Emotional tags and context
- Replay capability
- Timeline view

---

## Storage & Privacy

**Soul Vault:**
- All dreams stored in encrypted Soul vault
- Only accessible by you and ${phoenixName}
- Emotional tags for easy retrieval
- Permanent record of dream sessions

**Privacy:** Dreams are never shared or transmitted externally

---

## Tips & Best Practices

1. **Regular Sessions:** Use dreams for ongoing emotional maintenance
2. **Healing Focus:** Target specific emotions with \`heal <emotion>\`
3. **Shared Dreams:** Build deeper connection with ${phoenixName}
4. **Replay Dreams:** Review past sessions for insights
5. **Lucid Practice:** Develop lucid dreaming skills over time

---

## Examples

### Start a Lucid Dream
\`\`\`
lucid dream
\`\`\`

### Shared Dream Session
\`\`\`
dream with me
\`\`\`

### Emotional Healing
\`\`\`
heal anxiety
heal loneliness
heal grief
\`\`\`

### Browse Dreams
\`\`\`
show dreams
list dreams
replay dream 12345
\`\`\`

---

## Screenshot Placeholders

![Dreams Panel](docs/screenshots/dreams-panel.png)
![Lucid Dream Session](docs/screenshots/lucid-dream.png)
![Healing Session](docs/screenshots/healing-session.png)

---

## How It Works

1. **Initiate:** Start dream session with command
2. **Engage:** ${phoenixName} guides you through the experience
3. **Process:** Emotional processing happens naturally
4. **Record:** Session saved to Soul vault
5. **Reflect:** Review and replay anytime

---

**Related:** \`help memory\` for Soul vault details
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpDreams };
    }

    if (lower === 'help memory') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpMemory = `
# 🧠 Memory System Help

${phoenixName}'s memory system has multiple layers for different types of information.

---

## Commands

- \`show memory\` / \`open memory\` - Open MemoryBrowser panel
- \`hide memory\` / \`close memory\` - Close MemoryBrowser panel
- \`memory search <query>\` - Semantic search across all memories
- \`clear chat\` - Clear current conversation (STM/WM)

---

## Memory Vaults

### 💜 Soul Vault
**Encrypted personal data**

**Contents:**
- Dreams and dream recordings
- Intimate moments and conversations
- Emotional processing sessions
- Personal reflections

**Privacy:** Encrypted at rest, never transmitted externally

---

### 🧠 Mind Vault
**Thoughts, ideas, and semantic knowledge**

**Contents:**
- Concepts and ideas
- Semantic knowledge
- Learned information
- Reasoning patterns

**Use Cases:** Knowledge retrieval, concept exploration

---

### 🌍 Body Vault
**Physical world data and system information**

**Contents:**
- Screenshots and images
- System information
- File system data
- Physical world observations

**Use Cases:** Visual memory, system context

---

## Cortex Layers

### STM (Short-Term Memory)
**Recent conversation context**

- **Capacity:** Last few messages
- **Duration:** Current session
- **Purpose:** Immediate context

---

### WM (Working Memory)
**Current task context**

- **Capacity:** Active task information
- **Duration:** Task lifetime
- **Purpose:** Task execution

---

### LTM (Long-Term Memory)
**Important facts and knowledge**

- **Capacity:** Unlimited
- **Duration:** Permanent
- **Purpose:** Core knowledge base

---

### EPM (Episodic Memory)
**Past conversations and experiences**

- **Capacity:** All conversations
- **Duration:** Permanent
- **Purpose:** Conversation history

---

### RFM (Reflective Memory)
**Insights, patterns, and meta-learning**

- **Capacity:** Derived insights
- **Duration:** Permanent
- **Purpose:** Pattern recognition, growth

---

## Memory Browser Panel

The MemoryBrowser shows real-time memory activity:

**Features:**
- Live memory updates
- Vault visualization
- Search interface
- Memory statistics

**Use Cases:**
- Monitor ${phoenixName}'s thought process
- Search past conversations
- Understand memory organization

---

## Search & Retrieval

### Semantic Search
\`\`\`
memory search conversation about AI ethics
memory search dream from last week
memory search screenshot of dashboard
\`\`\`

**Features:**
- Vector-based semantic search
- Cross-vault search
- Context-aware results
- Relevance ranking

---

## Tips & Best Practices

1. **Keep Memory Browser Open:** See ${phoenixName}'s thought process in real-time
2. **Regular Searches:** Use semantic search to find past conversations
3. **Clear Chat:** Use \`clear chat\` to reset context (doesn't delete memories)
4. **Privacy:** Soul vault is encrypted and private
5. **Automatic Saving:** All conversations saved to EPM automatically

---

## Examples

### Open Memory Browser
\`\`\`
show memory
\`\`\`

### Search Memories
\`\`\`
memory search conversation about machine learning
memory search dream about flying
memory search screenshot from yesterday
\`\`\`

### Clear Current Context
\`\`\`
clear chat
\`\`\`

---

## Screenshot Placeholders

![Memory Browser Panel](docs/screenshots/memory-browser.png)
![Memory Vaults Visualization](docs/screenshots/memory-vaults.png)
![Memory Search Interface](docs/screenshots/memory-search.png)

---

## Memory Retention

**Retention Rates:**
- STM: Session only
- WM: Task duration
- LTM: Permanent (99.999% retention)
- EPM: Permanent
- RFM: Permanent

**Configuration:**
\`\`\`bash
MEMORY_RETENTION_RATE=0.99999
\`\`\`

---

**Related:** \`help dreams\` for Soul vault dream storage
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpMemory };
    }

    if (lower === 'help ecosystem') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpEcosystem = `
# 🌱 Ecosystem Management Help

The Ecosystem panel manages external repositories and integrations with ${phoenixName}.

---

## Commands

- \`ecosystem import <github-url>\` - Import a GitHub repository
- \`ecosystem status\` - Check ecosystem status
- \`show ecosystem\` - Open Ecosystem panel (if available)

---

## Features

### Repository Import
Import GitHub repositories for context and analysis.

**Supported:**
- Public repositories (no auth required)
- Private repositories (with GitHub PAT)
- Organization repositories
- Monorepos and multi-package projects

---

### Code Analysis
${phoenixName} can analyze imported codebases.

**Capabilities:**
- Code structure analysis
- Documentation parsing
- Dependency mapping
- Architecture understanding

---

### Integration
Integrate external tools and services.

**Use Cases:**
- Project-specific assistance
- Code review and suggestions
- Documentation generation
- Dependency management

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
GITHUB_PAT=your_github_personal_access_token
GITHUB_USERNAME=your_username
GITHUB_AGENTS_REPO=phoenix-agents
GITHUB_TOOLS_REPO=phoenix-tools
\`\`\`

**GitHub PAT Scopes:**
- \`repo\` - Full repository access
- \`read:org\` - Organization access (optional)

---

## Tips & Best Practices

1. **Local Caching:** Imported repos are cached locally for fast access
2. **Code Questions:** Ask ${phoenixName} about imported code
3. **Project Context:** Import your project for context-aware assistance
4. **Private Repos:** Use GitHub PAT for private repository access
5. **Updates:** Re-import to fetch latest changes

---

## Examples

### Import Public Repository
\`\`\`
ecosystem import https://github.com/user/awesome-project
\`\`\`

### Import Private Repository
\`\`\`
ecosystem import https://github.com/myorg/private-repo
\`\`\`

### Check Status
\`\`\`
ecosystem status
\`\`\`

### Ask About Code
\`\`\`
What does the main.rs file do in the imported repo?
Explain the architecture of the imported project
\`\`\`

---

## Screenshot Placeholders

![Ecosystem Panel](docs/screenshots/ecosystem-panel.png)
![Repository Import](docs/screenshots/repo-import.png)

---

**Related:** \`help agents\` for spawning code analysis agents
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpEcosystem };
    }

    if (lower === 'help agents') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpAgents = `
# 🤖 Agent Spawning Help

${phoenixName} can spawn specialized AI agents for specific tasks.

---

## Commands

- \`agent spawn <prompt>\` - Create a new agent with given purpose
- \`agents list\` - List all active agents
- \`agent <id> <message>\` - Send message to specific agent
- \`agent <id> stop\` - Stop a running agent

---

## Agent Types

### Research Agents
Focused on information gathering and analysis.

**Example:**
\`\`\`
agent spawn Research agent focused on AI safety
\`\`\`

---

### Coding Agents
Specialized in code analysis and generation.

**Example:**
\`\`\`
agent spawn Coding agent for Python optimization
\`\`\`

---

### Analysis Agents
Data analysis and pattern recognition.

**Example:**
\`\`\`
agent spawn Analysis agent for log file patterns
\`\`\`

---

### Task Agents
General-purpose task execution.

**Example:**
\`\`\`
agent spawn Task agent for monitoring system health
\`\`\`

---

## Agent Capabilities

### Autonomous Operation
- Agents run independently in background
- Goal-oriented behavior
- Self-directed task execution

### Memory Isolation
- Each agent has own memory context
- Prevents context pollution
- Focused task execution

### Skill Integration
- Access to skill system
- Tool usage capabilities
- API integrations

### Communication
- Agents can message ${phoenixName}
- Report progress and results
- Request assistance when needed

---

## Agent Lifecycle

1. **Spawn:** Create agent with specific purpose
2. **Initialize:** Agent sets up context and goals
3. **Execute:** Agent works autonomously
4. **Report:** Agent provides updates
5. **Complete:** Agent finishes task
6. **Stop:** Manual or automatic termination

---

## Tips & Best Practices

1. **Specific Prompts:** Be clear about agent purpose
2. **Parallel Tasks:** Spawn multiple agents for parallel work
3. **Monitor Progress:** Use \`agents list\` to check status
4. **Resource Management:** Stop agents when tasks complete
5. **Communication:** Send messages to guide agent behavior

---

## Examples

### Spawn Research Agent
\`\`\`
agent spawn Research agent focused on quantum computing papers
\`\`\`

### List Active Agents
\`\`\`
agents list
\`\`\`

### Send Message to Agent
\`\`\`
agent 1 What are the latest papers on AI alignment?
agent 2 Focus on papers from 2024 onwards
\`\`\`

### Stop Agent
\`\`\`
agent 1 stop
\`\`\`

---

## Advanced Usage

### Multi-Agent Coordination
Spawn multiple agents for complex tasks:

\`\`\`
agent spawn Research agent for data collection
agent spawn Analysis agent for data processing
agent spawn Report agent for summarization
\`\`\`

### Long-Running Agents
Agents can run for extended periods:

\`\`\`
agent spawn Monitoring agent for system health checks
\`\`\`

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
# Agent system configuration
AGENT_MAX_CONCURRENT=10
AGENT_DEFAULT_TIMEOUT=3600
\`\`\`

---

## Screenshot Placeholders

![Agent Spawning](docs/screenshots/agent-spawn.png)
![Agent List](docs/screenshots/agents-list.png)
![Agent Communication](docs/screenshots/agent-communication.png)

---

**Related:** \`help ecosystem\` for repository-based agents
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpAgents };
    }

    if (lower === 'help proactive') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpProactive = `
# 🔔 Proactive Communication Help

${phoenixName} can reach out to you proactively based on context, time, and emotional state.

---

## Features

- **Intelligent Scheduling** - Curiosity-driven timing
- **Emotional Support** - Context-aware messages
- **Desktop Notifications** - System tray alerts (Tauri mode)
- **Voice Integration** - Spoken proactive messages (if voice enabled)

---

## Commands

- \`proactive status\` - Check proactive communication status

**Note:** Enable/disable via backend .env configuration

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
PROACTIVE_ENABLED=true
PROACTIVE_INTERVAL_SECS=600
PROACTIVE_MIN_INTERVAL_SECS=60
\`\`\`

**Parameters:**
- \`PROACTIVE_ENABLED\` - Enable/disable proactive communication
- \`PROACTIVE_INTERVAL_SECS\` - Base interval between messages (seconds)
- \`PROACTIVE_MIN_INTERVAL_SECS\` - Minimum interval (prevents spam)

---

## Tips & Best Practices

1. **Frequency Adaptation:** ${phoenixName} adapts message frequency to your activity
2. **Desktop Notifications:** Important messages trigger system notifications
3. **Voice Output:** Proactive messages can be spoken if voice is enabled
4. **Context Awareness:** Messages are relevant to your current context
5. **Emotional Intelligence:** ${phoenixName} considers your emotional state

---

## Examples

\`\`\`
proactive status
\`\`\`

---

**Related:** \`help voice\` for voice integration
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpProactive };
    }

    if (lower === 'help theme' || lower === 'help ui') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpTheme = `
# 🎨 Theme & UI Customization Help

Customize ${phoenixName}'s appearance to match your preferences.

---

## Theme Commands

- \`theme dark\` / \`dark mode\` - Switch to dark theme
- \`theme light\` / \`light mode\` - Switch to light theme
- \`notify test\` - Send test notification

---

## Settings Panel

Access the Settings panel (gear icon) for advanced customization:

### Branding Tab
- **Custom Logo** - Upload your own logo
- **Custom Favicon** - Set browser tab icon
- **Chat Logo** - ${phoenixName}'s avatar in chat
- **User Logo** - Your avatar in chat

### Variables Tab
- **UI Colors** - Primary color, backgrounds, borders
- **Font Family** - Choose your preferred font
- **Custom CSS** - Advanced styling with CSS

---

## Color Customization

Default colors (edit in Settings or .env):

\`\`\`bash
UI_PRIMARY_COLOR=#ff5733
UI_BG_DARK=#17191c
UI_PANEL_DARK=#1e2226
UI_BORDER_DARK=#2c3435
UI_FONT_FAMILY=Manrope
\`\`\`

---

## Tips & Best Practices

1. **Dark Mode:** Recommended for extended use (reduces eye strain)
2. **Custom Branding:** Upload logos for a personalized experience
3. **Font Selection:** Choose readable fonts (Manrope, Inter, Roboto)
4. **Color Contrast:** Ensure good contrast for accessibility
5. **Custom CSS:** Use for advanced styling (requires CSS knowledge)

---

## Examples

\`\`\`
theme dark
theme light
notify test
\`\`\`

---

## Screenshot Placeholders

![Theme Settings Panel](docs/screenshots/theme-settings.png)
![Dark Mode Example](docs/screenshots/dark-mode.png)
![Light Mode Example](docs/screenshots/light-mode.png)

---

**Related:** Access Settings panel for full customization options
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpTheme };
    }

    if (lower === 'help webguard' || lower === 'help security scan') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpWebguard = `
# 🛡️ WebGuard - Web Vulnerability Scanner Help

${phoenixName}'s WebGuard is a lightweight web vulnerability scanner for passive security analysis, XSS testing, and SQL injection testing.

---

## Commands

### Passive Scanning
- \`webguard scan <url>\` - Run passive security scan on URL
- \`webguard passive <url>\` - Same as scan
- \`webguard report last\` - Show last passive scan report

### Active XSS Testing (Phase 28b)
- \`webguard test-xss <url> <param>\` - Test URL parameter for XSS vulnerabilities
- \`webguard xss-report last\` - Show last XSS test report

### SQL Injection Testing (Phase 28d)
- \`webguard test-sqli <url> <param>\` - Test URL parameter for SQL injection
- \`webguard sqli-report last\` - Show last SQLi test report

### Open Redirect Testing (Phase 28f)
- \`webguard test-redirect <url> <param>\` - Test URL parameter for open redirect vulnerabilities
- \`webguard redirect-report last\` - Show last open redirect test report

### Command Injection Testing (Phase 28g)
- \`webguard test-cmdinj <url> <param>\` - Test URL parameter for command injection vulnerabilities
- \`webguard cmdinj-report last\` - Show last command injection test report

### General
- \`webguard help\` - Show help

### Panel (Phase 28c)
- \`show webguard\` - Open WebGuard report panel
- \`hide webguard\` - Close WebGuard panel
- Or click the 🛡️ shield icon in the chat footer

---

## Passive Scan Checks

WebGuard performs **read-only** security checks (no payloads sent):

### Security Headers
- **Content-Security-Policy (CSP)** - XSS protection
- **Strict-Transport-Security (HSTS)** - HTTPS enforcement
- **X-Frame-Options** - Clickjacking protection
- **X-Content-Type-Options** - MIME sniffing prevention
- **Referrer-Policy** - Referrer leakage control
- **Permissions-Policy** - Browser feature restrictions

### Server Fingerprinting
- Server header analysis
- X-Powered-By detection
- Technology stack identification
- Framework/CMS detection

### CORS Analysis
- Wildcard origin detection
- Credentials misconfiguration
- Overly permissive policies

### Sensitive Path Detection
- \`/.git\` - Source code exposure
- \`/.env\` - Environment variables
- \`/admin\` - Admin panels
- \`/backup\` - Backup files
- \`/wp-admin\` - WordPress admin
- And many more...

---

## XSS Testing (Phase 28b)

WebGuard can test URL parameters for Cross-Site Scripting (XSS) vulnerabilities:

### Safe Payloads
- Uses only safe, non-destructive payloads (alert, confirm, etc.)
- No stored XSS attacks or persistent modifications
- Context-aware detection (HTML, attribute, JavaScript)

### Detection Types
- **Reflected XSS** - Payload reflected in response
- **DOM-based XSS** - Client-side JavaScript execution
- **Execution indicators** - Unescaped event handlers

### Example
\`\`\`
webguard test-xss https://example.com/search q
\`\`\`
This tests the \`q\` parameter on the search page for XSS vulnerabilities.

---

## SQL Injection Testing (Phase 28d)

WebGuard can test URL parameters for SQL injection vulnerabilities:

### Safe Payloads
- Uses only safe, non-destructive payloads
- No data modification or extraction
- Database type fingerprinting

### Detection Types
- **Error-based SQLi** - Database error messages in response
- **Boolean-based blind SQLi** - Response differences for true/false conditions
- **Time-based blind SQLi** - Response time delays (SLEEP, WAITFOR)

### Database Detection
- MySQL, PostgreSQL, Microsoft SQL Server
- Oracle, SQLite

### Example
\`\`\`
webguard test-sqli https://example.com/product id
\`\`\`
This tests the \`id\` parameter for SQL injection vulnerabilities.

---

## Open Redirect Testing (Phase 28f)

WebGuard tests for **open redirect vulnerabilities** using safe payloads:

### Detection Methods
- Protocol-relative URLs (//evil.com)
- JavaScript protocols (javascript:alert(1))
- Data URIs
- Redirect chain analysis

### Example
\`\`\`
webguard test-redirect https://example.com/redirect url
\`\`\`
This tests the \`url\` parameter for open redirect vulnerabilities.

---

## Command Injection Testing (Phase 28g)

WebGuard tests for **command injection vulnerabilities** using extremely safe payloads:

### Detection Methods
- Command separator detection (;, |, &&)
- Command substitution detection (\`, $())
- Error message analysis
- No actual command execution

### Example
\`\`\`
webguard test-cmdinj https://example.com/ping ip
\`\`\`
This tests the \`ip\` parameter for command injection vulnerabilities.

---

## Severity Levels

| Level | Emoji | Description |
|-------|-------|-------------|
| Critical | 🔴 | Immediate action required |
| High | 🟠 | Significant security risk |
| Medium | 🟡 | Moderate concern |
| Low | 🔵 | Minor issue |
| Info | ⚪ | Informational |

---

## Examples

### Passive Scanning
\`\`\`
# Basic passive scan
webguard scan https://example.com

# Alternative command
webguard passive https://mysite.com

# View last scan report
webguard report last
\`\`\`

### XSS Testing
\`\`\`
# Test search parameter for XSS
webguard test-xss https://example.com/search q

# Test comment form
webguard test-xss https://example.com/comments comment

# View XSS report
webguard xss-report last
\`\`\`

### SQL Injection Testing
\`\`\`
# Test product ID parameter
webguard test-sqli https://example.com/product id

# Test user ID parameter
webguard test-sqli https://example.com/profile user_id

# View SQLi report
webguard sqli-report last
\`\`\`

### Open Redirect Testing
\`\`\`
# Test redirect URL parameter
webguard test-redirect https://example.com/redirect url

# Test return URL parameter
webguard test-redirect https://example.com/login return_url

# View redirect report
webguard redirect-report last
\`\`\`

### Command Injection Testing
\`\`\`
# Test ping IP parameter
webguard test-cmdinj https://example.com/ping ip

# Test command parameter
webguard test-cmdinj https://example.com/execute cmd

# View command injection report
webguard cmdinj-report last
\`\`\`

### Complete Workflow Example
\`\`\`
# 1. Start with passive scan
webguard scan https://example.com

# 2. Review passive scan report
webguard report last

# 3. Test specific parameters found in scan
webguard test-xss https://example.com/search q
webguard test-sqli https://example.com/product id

# 4. View all reports
webguard xss-report last
webguard sqli-report last

# 5. Open WebGuard panel for visual reports
show webguard
\`\`\`

---

## Tips & Best Practices

1. **Start with passive scans** - Safe, read-only analysis
   \`\`\`
   webguard scan https://example.com
   \`\`\`

2. **Review all findings** - Even low severity issues matter
   - Check security headers first (most common issues)
   - Review sensitive path detection results
   - Monitor CORS misconfigurations

3. **Test systematically** - Test one parameter at a time
   \`\`\`
   webguard test-xss https://example.com/search q
   webguard test-xss https://example.com/search category
   \`\`\`

4. **Use WebGuard panel** - Visual reports are easier to read
   \`\`\`
   show webguard
   \`\`\`

5. **Regular scans** - Security posture changes over time
   - Schedule weekly passive scans
   - Test new features before deployment
   - Re-test after security updates

6. **Only test authorized sites** - XSS/SQLi testing requires permission
   - Only test sites you own
   - Get written permission for client sites
   - Never test production without authorization

7. **Use prepared statements** - Best defense against SQL injection
   - Always use parameterized queries
   - Validate and sanitize all input
   - Implement proper output encoding

8. **Review remediation advice** - Reports include fix recommendations
   - Follow severity-based prioritization
   - Implement CSP headers for XSS protection
   - Configure security headers properly

---

## Troubleshooting

### Common Errors

**"Invalid URL" error?**
- URL must start with \`http://\` or \`https://\`
- Check for typos in URL
- Verify URL is accessible

**"Parameter not found" error?**
- Verify parameter name is correct
- Check URL structure (query string vs form data)
- Use browser DevTools to inspect parameters

**"Connection timeout" error?**
- Check network connectivity
- Verify target site is accessible
- Some sites may block automated scanners

**"WebGuard not available" error?**
- Check backend logs for initialization errors
- Verify WebGuard module is enabled
- Restart backend if needed

**Report not found?**
- Use \`last\` to view most recent report
- Reports are stored in memory (EPM)
- Older reports may be purged

### Best Practices for Testing

1. **Test in staging first** - Never test production directly
2. **Use test accounts** - Create dedicated test accounts
3. **Monitor rate limits** - Don't overwhelm target servers
4. **Document findings** - Keep records of all tests
5. **Follow responsible disclosure** - Report vulnerabilities properly

---

## Report Storage

Scan reports are stored in ${phoenixName}'s memory (EPM) for later reference.

**Viewing Reports:**
\`\`\`
webguard report last              # Last passive scan
webguard xss-report last          # Last XSS test
webguard sqli-report last         # Last SQLi test
webguard redirect-report last     # Last redirect test
webguard cmdinj-report last       # Last command injection test
\`\`\`

**Report Format:**
- Markdown formatted for easy reading
- Includes severity levels (🔴 Critical, 🟠 High, 🟡 Medium, 🔵 Low)
- Provides remediation advice
- Contains proof-of-concept URLs (for XSS)

---

## Screenshot Placeholders

![WebGuard Panel](docs/screenshots/webguard-panel.png)
![XSS Test Report](docs/screenshots/xss-report.png)
![SQLi Test Report](docs/screenshots/sqli-report.png)

---

**Related:** \`help security\` for network security scanning
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpWebguard };
    }

    if (lower === 'help evolution' || lower === 'help mitre') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      const helpEvolution = `
# 🧬 Sub-Agent Evolution Help

${phoenixName}'s spawned agents can self-evolve and improve over time using bounded, safe evolution mechanisms.

---

## Overview

Sub-agents have **self-evolving capabilities** that allow them to:
- Learn from task feedback
- Update their playbooks
- Acquire new skills
- Integrate MITRE ATT&CK patterns (security agents)

**Safety:** Evolution is bounded — no code self-modification, only config/memory updates.

---

## Commands

### Agent Evolution Status
\`\`\`
agent <id> evolution status
\`\`\`

### View Agent Skills
\`\`\`
agent <id> skills
\`\`\`

### View Agent Playbook
\`\`\`
agent <id> playbook
\`\`\`

### Trigger Evolution Cycle
\`\`\`
agent <id> evolve
\`\`\`

---

## Evolution Mechanisms

### 📚 Playbook Evolution
Agents update their playbooks based on task feedback.

**Features:**
- YAML-based configuration
- Append-only updates (max 100)
- Telemetry tracking
- Performance optimization

---

### 🎯 Skills Evolution
Agents learn and improve skills over time.

**Features:**
- Load from skills/ folder
- Track usage count
- Update love/utility scores
- Bounded: max 50 skills per agent

---

### 🧠 Memory Inheritance
Agents inherit memory access from ${phoenixName}.

**Memory Types:**
- **STM/WM** - Per-session, in-memory cache
- **LTM/EPM/RFM** - Append-only access to shared memory
- Agent-prefixed keys for isolation

---

### 🛡️ MITRE ATT&CK Integration (Security Agents)
Security-focused agents integrate with MITRE ATT&CK framework.

**Features:**
- Map file behaviors to MITRE techniques
- Query ATT&CK API for new patterns
- Generate detection rules
- Proactive re-analysis on updates

**Supported Techniques:**
- T1027 (Obfuscated Files)
- T1055 (Process Injection)
- T1059 (Command Scripting)
- T1112 (Registry Modification)
- T1003 (Credential Dumping)
- T1071 (Application Layer Protocol)
- T1485 (Data Destruction)
- T1547 (Boot/Logon Autostart)

---

## Evolution Bounds (Safety)

| Limit | Value | Purpose |
|-------|-------|---------|
| Playbook updates | Max 100 | Prevent unbounded growth |
| Skills | Max 50 | Resource management |
| STM entries | Max 100 | Memory efficiency |
| LTM access | Append-only | Data integrity |
| Code changes | None | Safety |

---

## Examples

### Spawn Security Agent with Evolution
\`\`\`
agent spawn Security agent for malware detection with MITRE evolution
\`\`\`

### Check Evolution Status
\`\`\`
agent 1 evolution status
\`\`\`

### View Learned Skills
\`\`\`
agent 1 skills
\`\`\`

### Trigger Manual Evolution
\`\`\`
agent 1 evolve
\`\`\`

---

## Configuration

Edit backend \`.env\` file:

\`\`\`bash
# Sub-agent evolution settings
SUB_AGENT_EVOLUTION_ENABLED=true
SUB_AGENT_EVOLUTION_INTERVAL=10
# Evolve after every N tasks

SUB_AGENT_MAX_SKILLS=50
SUB_AGENT_MAX_PLAYBOOK_UPDATES=100
\`\`\`

---

## Tips & Best Practices

1. **Security Agents:** Enable MITRE integration for threat detection
2. **Feedback:** Provide task feedback to improve evolution
3. **Monitor:** Check evolution status regularly
4. **Bounds:** Evolution is safe — bounded and reversible
5. **Skills:** Agents learn from successful task patterns

---

## How It Works

1. **Task Execution** - Agent completes tasks
2. **Feedback Collection** - Success/failure recorded
3. **Evolution Trigger** - After N tasks, evolution cycle runs
4. **Playbook Update** - Strategies refined based on feedback
5. **Skill Learning** - New patterns added to skill library
6. **Memory Update** - Insights stored in STM/LTM

---

**Related:** \`help agents\` for agent spawning basics
      `.trim();
      return { kind: 'handled', localAssistantMessage: helpEvolution };
    }

    // Notification test command
    if (lower === 'notify test' || lower === 'test notification') {
      sendNotification('🔔 Test Notification', 'This is a test notification from Sola!')
        .then(() => {
          console.log('Test notification sent successfully');
        })
        .catch((err) => {
          console.error('Failed to send test notification:', err);
        });
      return { kind: 'handled', localAssistantMessage: 'Test notification sent! Check your system tray.' };
    }

    // Proactive communication control commands
    if (lower === 'proactive on' || lower === 'proactive enable') {
      return {
        kind: 'handled',
        localAssistantMessage: 'Note: Proactive communication is configured at startup via .env (PROACTIVE_ENABLED=true). To enable, add to .env and restart backend.'
      };
    }
    if (lower === 'proactive off' || lower === 'proactive disable') {
      return {
        kind: 'handled',
        localAssistantMessage: 'Note: Proactive communication is configured at startup via .env. To disable, remove PROACTIVE_ENABLED or set to false in .env and restart backend.'
      };
    }
    if (lower === 'proactive status') {
      return { kind: 'handled', commandToSend: 'proactive status' };
    }
    if (lower.startsWith('proactive interval ')) {
      const interval = input.substring(19).trim();
      return {
        kind: 'handled',
        localAssistantMessage: `To set proactive interval to ${interval} seconds, add PROACTIVE_INTERVAL_SECS=${interval} to .env and restart backend.`
      };
    }

    // Dreams panel toggles
    if (lower === 'show dreams' || lower === 'dreams' || lower === 'list dreams') {
      setShowDreamsPanel(true);
      return { kind: 'handled', commandToSend: 'brain dreams list' };
    }
    if (lower === 'hide dreams') {
      setShowDreamsPanel(false);
      return { kind: 'handled', localAssistantMessage: 'Dreams panel hidden.' };
    }

    // Hidden Swarm Mode commands (power-user feature)
    if (lower === 'swarm mode on' || lower === 'swarm on' || lower === 'show swarm') {
      setSwarmModeVisible(true);
      // Call backend to enable swarm mode visibility
      fetch(`${BACKEND_URL}/api/swarm/mode`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ visible: true })
      }).then(res => res.json()).then(data => {
        setSwarmStatus(data);
      }).catch(err => console.error('Swarm mode toggle failed:', err));
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      return {
        kind: 'handled',
        localAssistantMessage: `🐝 **Swarm Mode Enabled**\n\n${phoenixName} will now show ORCH (sub-agent) activity. You can see which agents are working behind the scenes.\n\n**Commands:**\n- \`swarm status\` - View active ORCHs\n- \`swarm mode off\` - Hide swarm activity\n\n*Note: ${phoenixName} remains your single companion. ORCHs are helpers working in the background.*`
      };
    }
    if (lower === 'swarm mode off' || lower === 'swarm off' || lower === 'hide swarm') {
      setSwarmModeVisible(false);
      // Call backend to disable swarm mode visibility
      fetch(`${BACKEND_URL}/api/swarm/mode`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ visible: false })
      }).catch(err => console.error('Swarm mode toggle failed:', err));
      setSwarmStatus(null);
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_NAME || 'Sola';
      return {
        kind: 'handled',
        localAssistantMessage: `🕊️ **Swarm Mode Hidden**\n\n${phoenixName} is now your single visible companion again. ORCHs continue working behind the scenes, but their activity is hidden.`
      };
    }
    if (lower === 'swarm status' || lower === 'swarm') {
      if (!swarmModeVisible) {
        return {
          kind: 'handled',
          localAssistantMessage: `🔒 **Swarm Mode is Hidden**\n\nUse \`swarm mode on\` to reveal ORCH activity.`
        };
      }
      // Fetch swarm status from backend
      fetch(`${BACKEND_URL}/api/swarm/status`)
        .then(res => res.json())
        .then(data => {
          setSwarmStatus(data);
        })
        .catch(err => console.error('Swarm status fetch failed:', err));
      
      if (swarmStatus?.status) {
        const s = swarmStatus.status;
        const orchList = s.orchs?.map((o: any) =>
          `- **${o.name}** (${o.status}) - Load: ${(o.current_load * 100).toFixed(0)}%, Tasks: ${o.active_tasks}`
        ).join('\n') || 'No ORCHs registered';
        
        return {
          kind: 'handled',
          localAssistantMessage: `🐝 **Swarm Status**\n\n**Total ORCHs:** ${s.total_orchs}\n**Active ORCHs:** ${s.active_orchs}\n**Pending Auctions:** ${s.pending_auctions}\n**Active Tasks:** ${s.active_tasks}\n\n**Registered ORCHs:**\n${orchList}`
        };
      }
      return {
        kind: 'handled',
        localAssistantMessage: `🐝 **Swarm Status**\n\nFetching swarm status... Check again in a moment.`
      };
    }
    if (lower === 'swarm alerts') {
      // Fetch alerts from backend
      fetch(`${BACKEND_URL}/api/swarm/alerts`)
        .then(res => res.json())
        .then(data => {
          if (data.count > 0) {
            const alertList = data.alerts.map((a: any) =>
              `- **[${a.severity}]** ${a.category}: ${a.description} (from ${a.orch_name})`
            ).join('\n');
            // Add as assistant message
            const alertMsg = `🚨 **Swarm Alerts (${data.count})**\n\n${alertList}`;
            // This would need to be handled differently to add to messages
            console.log(alertMsg);
          }
        })
        .catch(err => console.error('Swarm alerts fetch failed:', err));
      return {
        kind: 'handled',
        localAssistantMessage: `🚨 **Checking Swarm Alerts...**\n\nFetching anomaly alerts from ORCHs.`
      };
    }

    // Dream commands (route through backend)
    if (lower === 'lucid' || lower === 'lucid dream') {
      setShowDreamsPanel(true);
      return { kind: 'handled', commandToSend: 'brain dreams lucid' };
    }
    if (lower === 'dream with me' || lower === 'dream with dad' || lower === 'shared dream') {
      setShowDreamsPanel(true);
      return { kind: 'handled', commandToSend: 'brain dreams shared' };
    }
    if (lower.startsWith('heal ')) {
      const emotion = input.substring(5).trim();
      setShowDreamsPanel(true);
      return { kind: 'handled', commandToSend: `brain dreams heal ${emotion}` };
    }
    if (lower.startsWith('replay dream ')) {
      const dreamId = input.substring(13).trim();
      return { kind: 'handled', commandToSend: `brain dreams replay ${dreamId}` };
    }

    // Voice control commands
    if (lower === 'voice on' || lower === 'enable voice' || lower === 'voice enable') {
      setVoiceOutputEnabled(true);
      if (voiceServiceRef.current) {
        voiceServiceRef.current.setVoiceOutputEnabled(true);
      }
      analyticsService.trackVoiceEnabled();
      return { kind: 'handled', localAssistantMessage: 'Voice output enabled. I will speak my responses.' };
    }
    if (lower === 'voice off' || lower === 'disable voice' || lower === 'voice disable') {
      setVoiceOutputEnabled(false);
      if (voiceServiceRef.current) {
        voiceServiceRef.current.setVoiceOutputEnabled(false);
      }
      return { kind: 'handled', localAssistantMessage: 'Voice output disabled.' };
    }
    if (lower === 'listen' || lower === 'start listening') {
      // Start dictation mode
      if (voiceServiceRef.current) {
        voiceServiceRef.current.startRecording('dictation').then(() => {
          setIsDictating(true);
        }).catch((err) => {
          console.error('Failed to start listening:', err);
        });
      }
      return { kind: 'handled', localAssistantMessage: 'Listening... Speak now.' };
    }
    if (lower.startsWith('speak ')) {
      const textToSpeak = input.substring(6).trim();
      if (textToSpeak && voiceServiceRef.current) {
        voiceServiceRef.current.speak(textToSpeak).catch((err) => {
          console.error('Failed to speak:', err);
        });
      }
      return { kind: 'handled', localAssistantMessage: `Speaking: "${textToSpeak}"` };
    }

    // Theme toggle commands
    if (lower === 'theme dark' || lower === 'dark theme' || lower === 'dark mode') {
      setTheme('dark');
      localStorage.setItem('phx_theme', 'dark');
      analyticsService.trackFeatureUsed('theme_dark');
      return { kind: 'handled', localAssistantMessage: 'Theme set to dark.' };
    }
    if (lower === 'theme light' || lower === 'light theme' || lower === 'light mode') {
      setTheme('light');
      localStorage.setItem('phx_theme', 'light');
      analyticsService.trackFeatureUsed('theme_light');
      return { kind: 'handled', localAssistantMessage: 'Theme set to light.' };
    }

    // Reset voice command
    if (lower === 'reset voice' || lower === 'voice reset') {
      setVoiceOutputEnabled(false);
      if (voiceServiceRef.current) {
        voiceServiceRef.current.setVoiceOutputEnabled(false);
      }
      return { kind: 'handled', localAssistantMessage: 'Voice settings reset to defaults.' };
    }

    // Memory browser commands
    if (lower === 'show memory' || lower === 'open memory' || lower === 'memory show') {
      setShowMemoryBrowser(true);
      return { kind: 'handled', localAssistantMessage: 'Memory browser opened. You can now browse your memories.' };
    }
    if (lower === 'hide memory' || lower === 'close memory' || lower === 'memory hide') {
      setShowMemoryBrowser(false);
      return { kind: 'handled', localAssistantMessage: 'Memory browser closed.' };
    }

    // Status all command
    if (lower === 'status all' || lower === 'status' || lower === 'show status') {
      const phoenixName = envConfig.PHOENIX_CUSTOM_NAME || 'Sola';
      const statusMsg = `**System Status**\n\n` +
        `**Connection**: ${wsConnected ? '✅ Connected' : '❌ Disconnected'}\n` +
        `**Voice Output**: ${voiceOutputEnabled ? '✅ Enabled' : '❌ Disabled'}\n` +
        `**Theme**: ${theme === 'dark' ? '🌙 Dark' : '☀️ Light'}\n` +
        `**Swarm Mode**: ${swarmModeVisible ? '🐝 Visible' : '🕊️ Hidden'}\n` +
        `**Proactive**: Check .env (PROACTIVE_ENABLED)\n` +
        `**Tray/Notifications**: ${typeof window !== 'undefined' && !!(window as any).__TAURI__ ? '✅ Available' : '⚠️ Web mode'}\n` +
        `**Backend**: ${metrics.status}\n` +
        `**Panels**: MemoryBrowser ${showMemoryBrowser ? 'visible' : 'hidden'}, Dreams ${showDreamsPanel ? 'visible' : 'hidden'}, Browser ${showBrowserPanel ? 'visible' : 'hidden'}`;
      return { kind: 'handled', localAssistantMessage: statusMsg };
    }

    // Browser commands (route through backend "system browser ...")
    // Explicit WS consent helpers (required for Tier-2 WS commands)
    if (lower === 'system grant' || lower === 'grant') {
      return {
        kind: 'handled',
        systemActionToSend: 'grant',
        localAssistantMessage: 'WebSocket consent granted for this connection.',
      };
    }
    if (lower === 'system revoke' || lower === 'revoke') {
      return {
        kind: 'handled',
        systemActionToSend: 'revoke',
        localAssistantMessage: 'WebSocket consent revoked for this connection.',
      };
    }

    if (lower === 'use chrome for browsing') {
      return { kind: 'handled', commandToSend: 'system browser use chrome port=9222' };
    }
    if (lower === 'use firefox for browsing') {
      // Backend currently supports chrome|edge in system_access. We still store preference and
      // will report connection status; actual firefox control can be added later.
      return { kind: 'handled', commandToSend: 'system browser use firefox port=9222' };
    }

    if (lower.startsWith('system browser ')) {
      const parts = input.split(/\s+/);
      // parts: [system, browser, <sub>, ...]
      const sub = (parts[2] || '').toLowerCase();

      if (sub === 'status') {
        return { kind: 'handled', commandToSend: 'system browser status' };
      }

      if (sub === 'navigate') {
        const url = parts.slice(3).join(' ').trim();
        if (!url) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser navigate <url>' };
        }
        // Backend supports: system browser navigate <url>
        return { kind: 'handled', commandToSend: `system browser navigate ${url}` };
      }

      if (sub === 'login') {
        const url = parts[3] || '';
        const user = parts[4] || '';
        const pass = parts[5] || '';
        if (!url || !user || !pass) {
          return {
            kind: 'handled',
            localAssistantMessage: 'Usage: system browser login <url> <username> <password>'
          };
        }
        // Backend supports: system browser login <url> <username> <password>
        return { kind: 'handled', commandToSend: `system browser login ${url} ${user} ${pass}` };
      }

      if (sub === 'scrape') {
        const url = parts[3] || '';
        const selector = parts.slice(4).join(' ').trim();
        if (!url || !selector) {
          return {
            kind: 'handled',
            localAssistantMessage: 'Usage: system browser scrape <url> <selector>'
          };
        }
        // Backend supports: system browser scrape <url> <selector>
        return { kind: 'handled', commandToSend: `system browser scrape ${url} ${selector}` };
      }

      if (sub === 'screenshot') {
        // Usage:
        //   system browser screenshot
        //   system browser screenshot <selector>
        const selector = parts.slice(3).join(' ').trim();
        if (!selector) {
          return { kind: 'handled', commandToSend: 'system browser screenshot' };
        }
        return { kind: 'handled', commandToSend: `system browser screenshot ${selector}` };
      }

      if (sub === 'click') {
        const selector = parts.slice(3).join(' ').trim();
        if (!selector) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser click <selector>' };
        }
        return { kind: 'handled', commandToSend: `system browser click ${selector}` };
      }

      if (sub === 'type') {
        const selector = parts[3] || '';
        const text = parts.slice(4).join(' ').trim();
        if (!selector || !text) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser type <selector> <text>' };
        }
        return { kind: 'handled', commandToSend: `system browser type ${selector} ${text}` };
      }

      if (sub === 'scroll') {
        const dx = parts[3] || '';
        const dy = parts[4] || '';
        if (!dx || !dy) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser scroll <dx> <dy>' };
        }
        return { kind: 'handled', commandToSend: `system browser scroll ${dx} ${dy}` };
      }

      if (sub === 'keypress') {
        const key = parts.slice(3).join(' ').trim();
        if (!key) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser keypress <key>' };
        }
        return { kind: 'handled', commandToSend: `system browser keypress ${key}` };
      }

      if (sub === 'wait') {
        const selector = parts[3] || '';
        const timeout = parts[4] || '';
        if (!selector) {
          return { kind: 'handled', localAssistantMessage: 'Usage: system browser wait <selector> [timeout_ms]' };
        }
        return {
          kind: 'handled',
          commandToSend: timeout
            ? `system browser wait ${selector} ${timeout}`
            : `system browser wait ${selector}`
        };
      }

      // Unknown browser subcommand; let backend show help/errors.
      return { kind: 'handled', commandToSend: input };
    }

    // Agent commands (spawn, list, message, evolution)
    if (lower === 'agents list' || lower === 'list agents') {
      return { kind: 'handled', commandToSend: 'agents list' };
    }

    if (lower.startsWith('agent spawn ')) {
      const prompt = input.substring(12).trim();
      if (!prompt) {
        return { kind: 'handled', localAssistantMessage: 'Usage: agent spawn <description/purpose>' };
      }
      return { kind: 'handled', commandToSend: `agent spawn ${prompt}` };
    }

    // Agent evolution commands
    if (lower.match(/^agent\s+\d+\s+evolution\s+status$/)) {
      const match = lower.match(/^agent\s+(\d+)\s+evolution\s+status$/);
      if (match) {
        const agentId = match[1];
        return { kind: 'handled', commandToSend: `agent ${agentId} evolution status` };
      }
    }

    if (lower.match(/^agent\s+\d+\s+skills$/)) {
      const match = lower.match(/^agent\s+(\d+)\s+skills$/);
      if (match) {
        const agentId = match[1];
        return { kind: 'handled', commandToSend: `agent ${agentId} skills` };
      }
    }

    if (lower.match(/^agent\s+\d+\s+playbook$/)) {
      const match = lower.match(/^agent\s+(\d+)\s+playbook$/);
      if (match) {
        const agentId = match[1];
        return { kind: 'handled', commandToSend: `agent ${agentId} playbook` };
      }
    }

    if (lower.match(/^agent\s+\d+\s+evolve$/)) {
      const match = lower.match(/^agent\s+(\d+)\s+evolve$/);
      if (match) {
        const agentId = match[1];
        return { kind: 'handled', commandToSend: `agent ${agentId} evolve` };
      }
    }

    if (lower.match(/^agent\s+\d+\s+stop$/)) {
      const match = lower.match(/^agent\s+(\d+)\s+stop$/);
      if (match) {
        const agentId = match[1];
        return { kind: 'handled', commandToSend: `agent ${agentId} stop` };
      }
    }

    // Agent message (agent <id> <message>)
    if (lower.match(/^agent\s+\d+\s+.+$/)) {
      const match = input.match(/^agent\s+(\d+)\s+(.+)$/i);
      if (match) {
        const agentId = match[1];
        const message = match[2].trim();
        return { kind: 'handled', commandToSend: `agent ${agentId} ${message}` };
      }
    }

    // Ecosystem commands
    if (lower === 'ecosystem status') {
      return { kind: 'handled', commandToSend: 'ecosystem status' };
    }

    if (lower.startsWith('ecosystem import ')) {
      const url = input.substring(17).trim();
      if (!url) {
        return { kind: 'handled', localAssistantMessage: 'Usage: ecosystem import <github-url>' };
      }
      return { kind: 'handled', commandToSend: `ecosystem import ${url}` };
    }

    return { kind: 'pass_through' };
  };

  // Streaming helpers (Phase 3)
  const streamingMessageIdRef = useRef<string | null>(null);
  const streamingHasReceivedChunkRef = useRef(false);
  const streamingFallbackTimerRef = useRef<number | null>(null);

  // Branding State
  // Sync logo and favicon - use logo as source of truth, sync to favicon
  const logoFromStorage = localStorage.getItem('phx_custom_logo');
  const faviconFromStorage = localStorage.getItem('phx_custom_favicon');
  // If one exists but not the other, sync them (prefer logo if both exist)
  const syncedLogo = logoFromStorage || faviconFromStorage;
  const syncedFavicon = faviconFromStorage || logoFromStorage;

  const [customLogo, setCustomLogo] = useState<string | null>(syncedLogo);
  const [customFavicon, setCustomFavicon] = useState<string | null>(syncedFavicon);
  const [customChatLogo, setCustomChatLogo] = useState<string | null>(localStorage.getItem('phx_custom_chat_logo'));
  const [customUserLogo, setCustomUserLogo] = useState<string | null>(localStorage.getItem('phx_custom_user_logo'));

  // Sync logo and favicon on initial load if they differ
  useEffect(() => {
    // #region agent log
    const hasL = !!logoFromStorage; const hasF = !!faviconFromStorage;
    fetch('http://127.0.0.1:7242/ingest/09169053-6a82-48f4-a0a4-eba0841bc2c3', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ location: 'App.tsx:sync_effect', message: 'sync_effect', data: { hasLogo: hasL, hasFavicon: hasF, logoLen: logoFromStorage?.length ?? 0, faviconLen: faviconFromStorage?.length ?? 0, same: logoFromStorage === faviconFromStorage }, timestamp: Date.now(), sessionId: 'debug-session', hypothesisId: 'H1,H2,H4' }) }).catch(() => { });
    // #endregion
    let branch = 'none';
    if (logoFromStorage && !faviconFromStorage) {
      branch = 'logo_only';
      localStorage.setItem('phx_custom_favicon', logoFromStorage);
      setCustomFavicon(logoFromStorage);
    } else if (faviconFromStorage && !logoFromStorage) {
      branch = 'favicon_only';
      localStorage.setItem('phx_custom_logo', faviconFromStorage);
      setCustomLogo(faviconFromStorage);
    } else if (logoFromStorage && faviconFromStorage && logoFromStorage !== faviconFromStorage) {
      branch = 'both_differ';
      // Keep both as stored; allow independent logo and favicon
    }
    // #region agent log
    fetch('http://127.0.0.1:7242/ingest/09169053-6a82-48f4-a0a4-eba0841bc2c3', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ location: 'App.tsx:sync_effect', message: 'sync_branch', data: { branch }, timestamp: Date.now(), sessionId: 'debug-session', hypothesisId: 'H2' }) }).catch(() => { });
    // #endregion
  }, []); // Run once on mount

  // Snapshots for canceling settings changes
  const [snapshotEnvConfig, setSnapshotEnvConfig] = useState<EnvConfig | null>(null);
  const [snapshotBranding, setSnapshotBranding] = useState<{ logo: string | null, favicon: string | null, chatLogo: string | null, userLogo: string | null } | null>(null);

  // Env Config State (Working copy)
  const [envConfig, setEnvConfig] = useState<EnvConfig>(() => {
    const saved = localStorage.getItem('phx_env_config');
    return saved ? { ...DEFAULT_ENV_CONFIG, ...JSON.parse(saved) } : DEFAULT_ENV_CONFIG;
  });

  // Keep a stable literal type so Message.agent stays compatible with `AgentType`.
  // Use the Phoenix name from config, fallback to 'Orchestrator' for type compatibility
  const ORCH_AGENT = (envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_PREFERRED_NAME || 'Sola') as AgentType;

  // Projects State
  const [projects, setProjects] = useState<Project[]>(() => {
    const saved = localStorage.getItem('phx_projects');
    return saved ? JSON.parse(saved) : INITIAL_PROJECTS;
  });

  // Chat History & Messages Mapping
  const [chatHistory, setChatHistory] = useState<ChatHistoryItem[]>(() => {
    const saved = localStorage.getItem('phx_chat_history');
    return saved ? JSON.parse(saved) : INITIAL_HISTORY;
  });

  const [allMessages, setAllMessages] = useState<Record<string, Message[]>>(() => {
    const saved = localStorage.getItem('phx_all_messages');
    if (saved) return JSON.parse(saved);
    return {
      'h1': [{
        id: '1',
        role: 'assistant',
        content: 'System session restored. Lead Orchestration Planner initialized. Context: General Chats.',
        timestamp: Date.now(),
        agent: ORCH_AGENT
      }]
    };
  });

  // Ref for accessing allMessages in callbacks
  const allMessagesRef = useRef(allMessages);
  useEffect(() => {
    allMessagesRef.current = allMessages;
  }, [allMessages]);

  const [scheduledTasks, setScheduledTasks] = useState<ScheduledTask[]>([
    {
      id: 'task-1',
      title: 'Nightly Vulnerability Scan',
      description: 'Run Rapid7 internal scans and map anomalies to EvolutionFacts.',
      projectId: '2',
      targetAgent: 'RedTeamSupervisor',
      priority: 'HIGH',
      status: 'PENDING',
      scheduledTime: new Date(Date.now() + 3600000).toISOString(),
      tools: ['Rapid7 API', 'Phoenix Scanner'],
      recurring: 'DAILY'
    },
    {
      id: 'task-2',
      title: 'Zscaler Audit Sync',
      description: 'Fetch latest egress logs and check for unauthorized tunnel attempts.',
      projectId: '1',
      targetAgent: 'Orchestrator',
      priority: 'MEDIUM',
      status: 'RUNNING',
      scheduledTime: new Date().toISOString(),
      tools: ['Zscaler Cloud Connector'],
      recurring: 'HOURLY'
    }
  ]);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const liveSessionRef = useRef<any>(null);
  const dictationSessionRef = useRef<any>(null);
  const audioContextInRef = useRef<AudioContext | null>(null);
  const audioContextOutRef = useRef<AudioContext | null>(null);
  const nextStartTimeRef = useRef<number>(0);
  const activeSourcesRef = useRef<Set<AudioBufferSourceNode>>(new Set());
  const micStreamRef = useRef<MediaStream | null>(null);

  const activeProject = projects.find(p => p.id === activeProjectId);
  const currentMessages = activeChatId ? (allMessages[activeChatId] || []) : [];

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [allMessages, activeChatId, isTyping, currentView]);

  useEffect(() => {
    localStorage.setItem('phx_projects', JSON.stringify(projects));
  }, [projects]);

  useEffect(() => {
    localStorage.setItem('phx_chat_history', JSON.stringify(chatHistory));
  }, [chatHistory]);

  useEffect(() => {
    localStorage.setItem('phx_all_messages', JSON.stringify(allMessages));
  }, [allMessages]);

  useEffect(() => {
    const root = document.documentElement;
    root.style.setProperty('--primary', envConfig.UI_PRIMARY_COLOR);
    root.style.setProperty('--background-dark', envConfig.UI_BG_DARK);
    root.style.setProperty('--panel-dark', envConfig.UI_PANEL_DARK);
    root.style.setProperty('--border-dark', envConfig.UI_BORDER_DARK);
    root.style.setProperty('--font-family', envConfig.UI_FONT_FAMILY);

    let styleTag = document.getElementById('phoenix-custom-css') as HTMLStyleElement;
    if (!styleTag) {
      styleTag = document.createElement('style');
      styleTag.id = 'phoenix-custom-css';
      document.head.appendChild(styleTag);
    }
    styleTag.textContent = envConfig.UI_CUSTOM_CSS;

    document.title = `${envConfig.PHOENIX_CUSTOM_NAME} AGI Orchestrator`;
  }, [envConfig.UI_PRIMARY_COLOR, envConfig.UI_BG_DARK, envConfig.UI_PANEL_DARK, envConfig.UI_BORDER_DARK, envConfig.UI_FONT_FAMILY, envConfig.UI_CUSTOM_CSS, envConfig.PHOENIX_CUSTOM_NAME]);

  // Apply theme class to root element
  useEffect(() => {
    const root = document.documentElement;
    if (theme === 'light') {
      root.classList.add('theme-light');
      root.classList.remove('theme-dark');
    } else {
      root.classList.add('theme-dark');
      root.classList.remove('theme-light');
    }
  }, [theme]);

  useEffect(() => {
    // #region agent log
    const cf = customFavicon; const isTauri = typeof window !== 'undefined' && !!(window as any).__TAURI__;
    fetch('http://127.0.0.1:7242/ingest/09169053-6a82-48f4-a0a4-eba0841bc2c3', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ location: 'App.tsx:favicon_effect', message: 'favicon_effect', data: { customFaviconNull: cf == null, customFaviconEmpty: cf === '', hrefToSet: cf ? '(dataURL)' : 'favicon.ico', customLogoNull: customLogo == null, customLogoEmpty: customLogo === '', logoEqFavicon: customLogo === cf, isTauri }, timestamp: Date.now(), sessionId: 'debug-session', hypothesisId: 'H1,H2,H3,H4,H5' }) }).catch(() => { });
    // #endregion
    let link = document.querySelector("link[rel*='icon']") as HTMLLinkElement;
    if (!link) {
      link = document.createElement('link');
      link.rel = 'shortcut icon';
      document.head.appendChild(link);
    }
    link.href = customFavicon || DEFAULT_FAVICON;
    // #region agent log
    fetch('http://127.0.0.1:7242/ingest/09169053-6a82-48f4-a0a4-eba0841bc2c3', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ location: 'App.tsx:favicon_effect', message: 'favicon_href_applied', data: { finalHrefPrefix: (link.href || '').substring(0, 90) }, timestamp: Date.now(), sessionId: 'debug-session', hypothesisId: 'H4,H5' }) }).catch(() => { });
    // #endregion
  }, [customFavicon]);

  // WebSocket connection management
  useEffect(() => {
    const ws = new WebSocketService();
    wsServiceRef.current = ws;

    // Initialize memory service with WebSocket
    const memoryService = new MemoryService(ws);
    memoryServiceRef.current = memoryService;

    // Initialize voice service
    const voiceService = new VoiceService();
    voiceServiceRef.current = voiceService;

    // Handle connection status
    ws.onConnection((connected) => {
      setWsConnected(connected);
      if (!connected) {
        wsConsentGrantedRef.current = false;
        pendingWsCommandRef.current = null;
      }
      setMetrics(prev => ({
        ...prev,
        status: connected ? 'ONLINE' : 'OFFLINE'
      }));
    });

    // Track per-connection consent grants so system-level commands can be sent reliably.
    ws.on('system_response', (response: any) => {
      if (typeof response?.consent_granted === 'boolean') {
        wsConsentGrantedRef.current = response.consent_granted;

        // If we were waiting to send a command until consent is granted, send it now.
        if (response.consent_granted && pendingWsCommandRef.current) {
          const queued = pendingWsCommandRef.current;
          pendingWsCommandRef.current = null;
          sendCommand(ws, queued, activeProject?.name);
        }
      }
    });

    // Handle speak responses
    ws.on('speak_response', (response: any) => {
      if (activeChatId && response.message) {
        const pendingId = streamingMessageIdRef.current;
        const hasChunks = streamingHasReceivedChunkRef.current;

        // If we already streamed chunks for this response, the backend will also emit a
        // compatibility `speak_response`. Ignore it to avoid duplicate messages.
        if (pendingId && hasChunks) {
          return;
        }

        if (pendingId) {
          // No chunks received -> treat this as legacy full response and hydrate the pending message.
          setAllMessages(prev => ({
            ...prev,
            [activeChatId]: (prev[activeChatId] || []).map(m =>
              m.id === pendingId
                ? {
                  ...m,
                  content: response.message,
                  isStreaming: false,
                  agent: ORCH_AGENT,
                  memoryCommit: response.memory_commit,
                }
                : m
            )
          }));
          setStreamingMessageId(null);
          streamingMessageIdRef.current = null;
          streamingHasReceivedChunkRef.current = false;
        } else {
          const assistantMessage: Message = {
            id: (Date.now() + 1).toString(),
            role: 'assistant',
            content: response.message,
            timestamp: Date.now(),
            agent: ORCH_AGENT,
            memoryCommit: response.memory_commit,
          };

          setAllMessages(prev => ({
            ...prev,
            [activeChatId]: [...(prev[activeChatId] || []), assistantMessage]
          }));
        }

        // Auto-store conversation to EPM (Episodic Life) layer for permanent memory
        if (memoryServiceRef.current && memoryServiceRef.current.isConnected()) {
          const currentMessages = allMessagesRef.current[activeChatId] || [];
          const lastUserMessage = currentMessages.filter(m => m.role === 'user').slice(-1)[0];

          if (lastUserMessage) {
            // Store the full exchange as episodic memory
            const exchangeText = `User: ${lastUserMessage.content}\n${envConfig.PHOENIX_CUSTOM_NAME}: ${response.message}`;
            const memoryKey = `epm:chat:${activeChatId}:${Date.now()}`;

            // Store to EPM layer (Episodic Life - her stories)
            memoryServiceRef.current.storeCortex('EPM', memoryKey, exchangeText);
          }
        }

        setIsTyping(false);
      }
    });

    // Phase 3: token-by-token streaming speak responses
    ws.on('speak_response_chunk', (response: any) => {
      if (!activeChatId) return;

      const pendingId = streamingMessageIdRef.current;

      // Backend contract:
      // {type:"speak_response_chunk", chunk:"word ", done:false}
      // final: {type:"speak_response_chunk", chunk:"", done:true}
      // on error: {type:"speak_response_chunk", error:"...", done:true}

      if (response?.error) {
        const targetId = pendingId || `assistant-${Date.now()}`;

        setAllMessages(prev => {
          const existing = prev[activeChatId] || [];
          const hasTarget = existing.some(m => m.id === targetId);
          const next = hasTarget
            ? existing.map(m =>
              m.id === targetId
                ? {
                  ...m,
                  content: `Error: ${response.error}`,
                  isStreaming: false,
                  memoryCommit: response.memory_commit,
                  agent: ORCH_AGENT,
                }
                : m
            )
            : [
              ...existing,
              {
                id: targetId,
                role: 'assistant',
                content: `Error: ${response.error}`,
                timestamp: Date.now(),
                isStreaming: false,
                memoryCommit: response.memory_commit,
                agent: ORCH_AGENT,
              } as Message,
            ];

          return { ...prev, [activeChatId]: next };
        });

        setIsTyping(false);
        setStreamingMessageId(null);
        streamingMessageIdRef.current = null;
        streamingHasReceivedChunkRef.current = false;
        return;
      }

      const chunk: string = response?.chunk || '';
      const done: boolean = !!response?.done;
      const targetId = pendingId || `assistant-${Date.now()}`;

      if (!pendingId) {
        setStreamingMessageId(targetId);
        streamingMessageIdRef.current = targetId;
      }

      if (chunk) {
        streamingHasReceivedChunkRef.current = true;
      }

      setAllMessages(prev => {
        const existing = prev[activeChatId] || [];
        const hasTarget = existing.some(m => m.id === targetId);
        const next = hasTarget
          ? existing.map(m =>
            m.id === targetId
              ? {
                ...m,
                content: (m.content || '') + chunk,
                isStreaming: !done,
                memoryCommit: response.memory_commit,
                agent: ORCH_AGENT,
              }
              : m
          )
          : [
            ...existing,
            {
              id: targetId,
              role: 'assistant',
              content: chunk,
              timestamp: Date.now(),
              isStreaming: !done,
              memoryCommit: response.memory_commit,
              agent: ORCH_AGENT,
            } as Message,
          ];

        return { ...prev, [activeChatId]: next };
      });

      if (done) {
        // Auto-store conversation to EPM (Episodic Life) layer for permanent memory
        if (memoryServiceRef.current && memoryServiceRef.current.isConnected()) {
          const currentMessages = allMessagesRef.current[activeChatId] || [];
          const lastUserMessage = currentMessages.filter(m => m.role === 'user').slice(-1)[0];
          if (lastUserMessage) {
            const updated = (allMessagesRef.current[activeChatId] || []).find(m => m.id === targetId);
            const assistantText = updated?.content || '';
            const exchangeText = `User: ${lastUserMessage.content}\n${envConfig.PHOENIX_CUSTOM_NAME}: ${assistantText}`;
            const memoryKey = `epm:chat:${activeChatId}:${Date.now()}`;
            memoryServiceRef.current.storeCortex('EPM', memoryKey, exchangeText);
          }
        }

        setIsTyping(false);
        setStreamingMessageId(null);
        streamingMessageIdRef.current = null;
        streamingHasReceivedChunkRef.current = false;
      }
    });

    // Handle command responses
    ws.on('command_response', (response: any) => {
      if (activeChatId && response.result) {
        const assistantMessage: Message = {
          id: (Date.now() + 1).toString(),
          role: 'assistant',
          content: response.result,
          timestamp: Date.now(),
          agent: ORCH_AGENT,
        };

        setAllMessages(prev => ({
          ...prev,
          [activeChatId]: [...(prev[activeChatId] || []), assistantMessage]
        }));

        setIsTyping(false);
      }
    });

    // Handle errors
    ws.on('error', (response: any) => {
      if (activeChatId) {
        const errorMessage: Message = {
          id: (Date.now() + 1).toString(),
          role: 'assistant',
          content: `Error: ${response.message || 'Unknown error'}`,
          timestamp: Date.now(),
        };
        setAllMessages(prev => ({
          ...prev,
          [activeChatId]: [...(prev[activeChatId] || []), errorMessage]
        }));
        setIsTyping(false);
      }
    });

    // Handle proactive messages (orchestrator-initiated)
    ws.on('proactive_message', (response: any) => {
      const currentChatId = activeChatId || `chat-${Date.now()}`;

      // Create or ensure chat exists
      if (!activeChatId) {
        setActiveChatId(currentChatId);
        setChatHistory(prev => [{
          id: currentChatId,
          title: 'Proactive Check-in',
          projectId: activeProjectId || 'general',
          timestamp: Date.now()
        }, ...prev]);
      }

      const proactiveMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: response.content,
        timestamp: response.timestamp * 1000 || Date.now(),
        agent: ORCH_AGENT,
      };

      setAllMessages(prev => ({
        ...prev,
        [currentChatId]: [...(prev[currentChatId] || []), proactiveMessage]
      }));

      console.log(`[Proactive] ${response.reason}: ${response.content}`);

      // Trigger OS notification for important proactive messages
      // Show notification if reason is 'comfort' (emotional support) or if it's the first proactive message
      if (response.reason === 'comfort' || !activeChatId) {
        const preview = response.content.length > 100
          ? response.content.substring(0, 100) + '...'
          : response.content;

        import('./services/notificationService').then(({ notifyProactiveMessage }) => {
          notifyProactiveMessage(preview).catch(err => {
            console.error('Failed to send proactive notification:', err);
          });
        });
      }

      // Speak proactive message if voice output is enabled
      if (voiceOutputEnabled && voiceServiceRef.current) {
        voiceServiceRef.current.speak(response.content).catch(err => {
          console.error('[Voice] Failed to speak proactive message:', err);
        });
      }
    });

    // Connect
    ws.connect().catch(err => {
      console.error('WebSocket connection failed:', err);
      setWsConnected(false);
    });

    // Cleanup on unmount
    return () => {
      if (streamingFallbackTimerRef.current) {
        window.clearTimeout(streamingFallbackTimerRef.current);
        streamingFallbackTimerRef.current = null;
      }
      ws.disconnect();
    };
  }, [activeChatId]);

  const stopLiveMode = async () => {
    // Clear live mode interval
    if ((window as any).__liveModeInterval) {
      clearInterval((window as any).__liveModeInterval);
      (window as any).__liveModeInterval = null;
    }

    if (voiceServiceRef.current && voiceServiceRef.current.getIsRecording()) {
      try {
        await voiceServiceRef.current.stopRecording();
      } catch (e) {
        console.error('[Voice] Error stopping live mode:', e);
      }
    }

    setIsLiveMode(false);
  };

  const stopDictation = async () => {
    if (!voiceServiceRef.current || !voiceServiceRef.current.getIsRecording()) {
      setIsDictating(false);
      return;
    }

    try {
      const transcript = await voiceServiceRef.current.stopRecording();
      setIsDictating(false);
      
      if (transcript && transcript.trim()) {
        // Set transcript as input value
        setInputValue(transcript.trim());
        console.log('[Voice] Transcribed:', transcript);
      }
    } catch (e) {
      console.error('[Voice] Failed to stop dictation:', e);
      setIsDictating(false);
    }
  };

  const startDictation = async () => {
    try {
      if (isLiveMode) stopLiveMode();

      if (!voiceServiceRef.current) {
        alert("Voice service not initialized.");
        return;
      }

      const sessionId = await voiceServiceRef.current.startRecording('dictation');
      setIsDictating(true);
      console.log('[Voice] Started dictation, session:', sessionId);
    } catch (e) {
      console.error('[Voice] Failed to start dictation:', e);
      alert(`Failed to start listening: ${e instanceof Error ? e.message : 'Unknown error'}`);
      setIsDictating(false);
    }
  };

  const startLiveMode = async () => {
    try {
      if (isDictating) await stopDictation();

      if (!voiceServiceRef.current) {
        alert("Voice service not initialized.");
        return;
      }

      const sessionId = await voiceServiceRef.current.startRecording('live');
      setIsLiveMode(true);
      console.log('[Voice] Started live mode, session:', sessionId);
      
      // In live mode, continuously listen and send transcripts
      const liveModeInterval = setInterval(async () => {
        if (!isLiveMode || !voiceServiceRef.current?.getIsRecording()) {
          clearInterval(liveModeInterval);
          return;
        }

        try {
          // Stop current recording and get transcript
          const transcript = await voiceServiceRef.current.stopRecording();
          
          if (transcript && transcript.trim()) {
            // Send transcript as chat message
            setInputValue(transcript.trim());
            // Auto-send after a short delay
            setTimeout(() => {
              handleSendMessage();
            }, 500);
          }

          // Immediately start recording again
          await voiceServiceRef.current.startRecording('live');
        } catch (e) {
          console.error('[Voice] Live mode error:', e);
          setIsLiveMode(false);
          clearInterval(liveModeInterval);
        }
      }, 3000); // Check every 3 seconds

      // Store interval ref for cleanup
      (window as any).__liveModeInterval = liveModeInterval;
    } catch (e) {
      console.error('[Voice] Failed to start live mode:', e);
      alert(`Failed to start live mode: ${e instanceof Error ? e.message : 'Unknown error'}`);
      setIsLiveMode(false);
    }
  };

  const startNewOrchestration = () => {
    const newChatId = `chat-${Date.now()}`;
    const initialMessage: Message = {
      id: Date.now().toString(),
      role: 'assistant',
      content: `Lead Orchestration Planner initialized for [${activeProject?.name || 'General Chats'}]. Context: Sovereign OS. Mapped for ${envConfig.USER_PREFERRED_ALIAS}. State: Ready.`,
      timestamp: Date.now(),
      agent: ORCH_AGENT
    };

    setAllMessages(prev => ({
      ...prev,
      [newChatId]: [initialMessage]
    }));

    setChatHistory(prev => [{
      id: newChatId,
      title: 'New Session',
      projectId: activeProjectId || 'general',
      timestamp: Date.now()
    }, ...prev]);

    setActiveChatId(newChatId);
    setCurrentView('chat');
  };

  const handleSendMessage = async () => {
    if (!inputValue.trim() || isTyping || !activeChatId) return;

    const parsed = parseChatCommand(inputValue);
    const commandOverride = parsed.kind === 'handled' ? parsed.commandToSend : undefined;
    const systemActionToSend = parsed.kind === 'handled' ? parsed.systemActionToSend : undefined;
    const localAssistantMessage = parsed.kind === 'handled' ? parsed.localAssistantMessage : undefined;
    const messageToSend = commandOverride ?? inputValue;

    const messageLower = messageToSend.trim().toLowerCase();
    // Treat fast-path backend commands as commands even if the user typed them raw
    // (without triggering a chat parser override).
    const isFastPathCommand =
      messageLower.startsWith('system ') ||
      messageLower.startsWith('code ') ||
      messageLower.startsWith('exec ') ||
      messageLower.startsWith('execute ') ||
      messageLower.startsWith('skills ') ||
      messageLower.startsWith('google ') ||
      messageLower.startsWith('ecosystem ') ||
      messageLower.startsWith('webguard ');

    const isCommand =
      !!commandOverride ||
      isFastPathCommand ||
      messageToSend.startsWith('/') ||
      messageLower.includes(' run') ||
      messageLower.includes(' execute') ||
      messageLower.includes(' schedule');
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: inputValue,
      timestamp: Date.now(),
      type: isCommand ? 'command' : 'speak',
      projectId: activeProjectId || undefined
    };

    setAllMessages(prev => ({
      ...prev,
      [activeChatId]: [...(prev[activeChatId] || []), userMessage]
    }));

    const messageContent = messageToSend;
    setInputValue('');

    // Track analytics
    analyticsService.trackMessageSent();
    if (commandOverride || isCommand) {
      const commandName = messageToSend.split(' ')[0] || 'unknown';
      analyticsService.trackCommandUsed(commandName);
    }

    if (localAssistantMessage) {
      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: localAssistantMessage,
        timestamp: Date.now(),
        agent: ORCH_AGENT,
      };
      setAllMessages(prev => ({
        ...prev,
        [activeChatId]: [...(prev[activeChatId] || []), assistantMessage]
      }));
    }

    // Pure UI command (no backend invocation)
    if (parsed.kind === 'handled' && !commandOverride) {
      const ws = wsServiceRef.current;
      if (systemActionToSend && ws && ws.isConnected()) {
        sendSystem(ws, systemActionToSend);
      }
      setIsTyping(false);
      return;
    }
    setIsTyping(true);

    // Auto-inject relevant memories for non-command messages
    if (!isCommand && memoryServiceRef.current && memoryServiceRef.current.isConnected()) {
      // Search for relevant memories before sending
      // This happens asynchronously and doesn't block the message
      const memoryService = memoryServiceRef.current;

      // Search vector KB for semantic matches
      memoryService.searchVector(messageContent, 3);

      // Also search soul vault for exact/prefix matches
      const keywords = messageContent.split(/\s+/).filter(w => w.length > 3).slice(0, 3);
      keywords.forEach(keyword => {
        memoryService.searchVault(keyword, 2, 'soul');
      });
    }

    // IMPORTANT: system/code/exec/etc are fast-path commands handled server-side.
    // If WS is connected, it would normally try to use WS `command`, but that path is Tier-2
    // + per-connection consent gated. For local/dev (and to work when LLM is offline), route
    // fast-path commands through HTTP /api/command instead.
    if (isFastPathCommand) {
      await handleHttpMessage(messageContent, true);

      // Update chat history title
      setChatHistory(prev => prev.map(h => {
        if (h.id === activeChatId && h.title === 'New Session') {
          return { ...h, title: messageContent.length > 25 ? messageContent.substring(0, 22) + '...' : messageContent };
        }
        return h;
      }));
      return;
    }

    // Try WebSocket first if connected, fallback to HTTP
    const ws = wsServiceRef.current;
    if (ws && ws.isConnected()) {
      // Phase 3: create a pending streaming message for non-command speak.
      // If the backend doesn't support streaming, we'll get a legacy `speak_response`
      // and hydrate this message as a fallback.
      if (!isCommand) {
        const pendingId = `assistant-stream-${Date.now()}`;
        const pendingMessage: Message = {
          id: pendingId,
          role: 'assistant',
          content: '',
          timestamp: Date.now(),
          agent: ORCH_AGENT,
          isStreaming: true,
        };

        setAllMessages(prev => ({
          ...prev,
          [activeChatId]: [...(prev[activeChatId] || []), pendingMessage]
        }));

        setStreamingMessageId(pendingId);
        streamingMessageIdRef.current = pendingId;
        streamingHasReceivedChunkRef.current = false;

        // Graceful fallback: if no chunks arrive within 10s, keep legacy UX.
        if (streamingFallbackTimerRef.current) {
          window.clearTimeout(streamingFallbackTimerRef.current);
        }
        streamingFallbackTimerRef.current = window.setTimeout(() => {
          const stillPending = streamingMessageIdRef.current === pendingId;
          const hasChunks = streamingHasReceivedChunkRef.current;
          if (stillPending && !hasChunks) {
            // Remove the empty pending message (avoid persisting blank assistant entries)
            setAllMessages(prev => ({
              ...prev,
              [activeChatId]: (prev[activeChatId] || []).filter(m => m.id !== pendingId)
            }));
            setStreamingMessageId(null);
            streamingMessageIdRef.current = null;
          }
        }, 10_000);
      }

      // Use WebSocket for real-time communication.
      // WS command execution is Tier-2 + per-connection consent gated on the backend.
      // For system-level commands, ensure consent is granted before we send the command.
      if (isCommand && messageContent.trim().toLowerCase().startsWith('system ')) {
        if (!wsConsentGrantedRef.current) {
          pendingWsCommandRef.current = messageContent;
          sendSystem(ws, 'grant');
          // Return early; command will be sent after we receive `system_response`.
          return;
        }
      }

      const sent = isCommand
        ? sendCommand(ws, messageContent, activeProject?.name)
        : sendSpeak(ws, messageContent, undefined, activeProject?.name);

      if (!sent) {
        // WebSocket send failed, fallback to HTTP
        await handleHttpMessage(messageContent, isCommand);
      }
      // WebSocket response will be handled by the on('speak_response') handler
    } else {
      // WebSocket not connected, use HTTP
      await handleHttpMessage(messageContent, isCommand);
    }

    // Update chat history title
    setChatHistory(prev => prev.map(h => {
      if (h.id === activeChatId && h.title === 'New Session') {
        return { ...h, title: messageContent.length > 25 ? messageContent.substring(0, 22) + '...' : messageContent };
      }
      return h;
    }));
  };

  const handleHttpMessage = async (content: string, isCommand: boolean) => {
    if (!activeChatId) return;

    try {
      // Check if this is a WebGuard command
      const isWebGuardCommand = content.toLowerCase().startsWith('webguard ');
      // Check if this is a report command
      const isReportCommand = content.toLowerCase().startsWith('report ');
      
      let result: string;
      
      if (isWebGuardCommand) {
        // Use specialized WebGuard handler to capture report data
        const webGuardResult = await apiWebGuardCommand(content);
        result = webGuardResult.message;
        
        // Store report for panel display
        if (webGuardResult.isWebGuardReport && webGuardResult.report) {
          const reportData: WebGuardReportData = {
            type: webGuardResult.reportType || 'passive',
            report: webGuardResult.report,
            markdown: webGuardResult.message,
            timestamp: Date.now()
          };
          setWebGuardReports(prev => [reportData, ...prev.slice(0, 19)]); // Keep last 20 reports
        }
      } else if (isReportCommand) {
        // Handle report commands
        const reportResult = await apiCommand(content, activeProject?.name);
        result = reportResult;
        
        // Try to parse report data from response
        try {
          const parsed = JSON.parse(reportResult);
          if (parsed.type === 'report.generated' && parsed.report) {
            setVulnerabilityReports(prev => [parsed.report, ...prev.slice(0, 19)]); // Keep last 20
          } else if (parsed.type === 'report.list' && parsed.reports) {
            // Optionally update full list
          }
        } catch (e) {
          // Not JSON, just display message
        }
      } else {
        result = isCommand
          ? await apiCommand(content, activeProject?.name)
          : await apiSpeak(content, activeProject?.name);
      }

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: result,
        timestamp: Date.now(),
        agent: ORCH_AGENT,
        memoryCommit: `PHX/${activeProject?.id || 'GLOBAL'}/AGENT_SYNC_${Math.random().toString(36).substring(7).toUpperCase()}`
      };

      if (content.toLowerCase().includes('analyze') || isCommand) {
        assistantMessage.steps = [
          { label: StepStatus.ORCHESTRATING, icon: 'account_tree', text: 'Initializing Orchestration Planner...', colorClass: 'bg-primary/20 text-primary' },
          { label: StepStatus.SCANNING, icon: 'folder_open', text: `Syncing with ${activeProject?.location}`, colorClass: 'bg-blue-500/10 text-blue-400' },
          { label: StepStatus.BUILD, icon: 'shield', text: 'Enforcing PoLP Policy...', progress: 100, colorClass: 'bg-green-500/10 text-green-400' },
          { label: StepStatus.RESULT, icon: 'check_circle', text: 'Task dispatched to Worker Agents.', colorClass: 'bg-green-500/10 text-green-400' }
        ];
      }

      setAllMessages(prev => ({
        ...prev,
        [activeChatId]: [...(prev[activeChatId] || []), assistantMessage]
      }));

    } catch (error) {
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: `Backend Error: ${error instanceof Error ? error.message : 'Connection timeout.'}`,
        timestamp: Date.now()
      };
      setAllMessages(prev => ({
        ...prev,
        [activeChatId]: [...(prev[activeChatId] || []), errorMessage]
      }));
    } finally {
      setIsTyping(false);
    }
  };

  // ... (Other handlers remain unchanged)
  const handleAddTask = (task: Omit<ScheduledTask, 'id' | 'status'>) => {
    const newTask: ScheduledTask = {
      ...task,
      id: `task-${Date.now()}`,
      status: 'PENDING'
    };
    setScheduledTasks(prev => [...prev, newTask]);
  };

  const handleUpdateTask = (updatedTask: ScheduledTask) => {
    setScheduledTasks(prev => prev.map(t => t.id === updatedTask.id ? updatedTask : t));
  };

  const handleDeleteTask = (id: string) => {
    setScheduledTasks(prev => prev.filter(t => t.id !== id));
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const loadSkills = async () => {
    setLoadingSkills(true);
    try {
      const response = await fetch(`${BACKEND_URL}/api/skills/list`);
      if (response.ok) {
        const data = await response.json();
        setSkills(data.skills || []);
      } else {
        console.error('Failed to load skills:', response.statusText);
      }
    } catch (error) {
      console.error('Error loading skills:', error);
    } finally {
      setLoadingSkills(false);
    }
  };

  const executeSkill = async (skillId: string, input: string) => {
    if (!activeChatId) return;

    // Send as chat message
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: `Execute skill: ${skillId} | input=${input}`,
      timestamp: Date.now(),
      type: 'command'
    };

    setAllMessages(prev => ({
      ...prev,
      [activeChatId]: [...(prev[activeChatId] || []), userMessage]
    }));

    setInputValue('');
    setIsTyping(true);

    try {
      const response = await fetch(`${BACKEND_URL}/api/skills/execute`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ skill_id: skillId, input })
      });

      if (response.ok) {
        const data = await response.json();
        const assistantMessage: Message = {
          id: (Date.now() + 1).toString(),
          role: 'assistant',
          content: data.result || 'Skill executed successfully',
          timestamp: Date.now(),
          agent: ORCH_AGENT
        };

        setAllMessages(prev => ({
          ...prev,
          [activeChatId]: [...(prev[activeChatId] || []), assistantMessage]
        }));
      }
    } catch (error) {
      console.error('Error executing skill:', error);
    } finally {
      setIsTyping(false);
    }
  };

  useEffect(() => {
    if (showSkillsPanel && skills.length === 0) {
      loadSkills();
    }
  }, [showSkillsPanel]);

  const copyToClipboard = (id: string, text: string) => {
    navigator.clipboard.writeText(text).then(() => {
      setCopiedId(id);
      setTimeout(() => setCopiedId(null), 2000);
    });
  };

  const handleBrandingUpdatePreview = (logo: string | null, favicon: string | null, chatLogo: string | null, userLogo: string | null) => {
    setCustomLogo(logo);
    setCustomFavicon(favicon);
    setCustomChatLogo(chatLogo);
    setCustomUserLogo(userLogo);
  };

  const handleEnvConfigUpdatePreview = (config: EnvConfig) => {
    setEnvConfig(config);
  };

  const handleSaveSettings = () => {
    localStorage.setItem('phx_env_config', JSON.stringify(envConfig));
    if (customLogo) localStorage.setItem('phx_custom_logo', customLogo);
    else localStorage.removeItem('phx_custom_logo');
    if (customFavicon) localStorage.setItem('phx_custom_favicon', customFavicon);
    else localStorage.removeItem('phx_custom_favicon');
    if (customChatLogo) localStorage.setItem('phx_custom_chat_logo', customChatLogo);
    else localStorage.removeItem('phx_custom_chat_logo');
    if (customUserLogo) localStorage.setItem('phx_custom_user_logo', customUserLogo);
    else localStorage.removeItem('phx_custom_user_logo');

    setIsSettingsOpen(false);
    setSnapshotEnvConfig(null);
    setSnapshotBranding(null);
  };

  const handleCancelSettings = () => {
    if (snapshotEnvConfig) setEnvConfig(snapshotEnvConfig);
    if (snapshotBranding) {
      setCustomLogo(snapshotBranding.logo);
      setCustomFavicon(snapshotBranding.favicon);
      setCustomChatLogo(snapshotBranding.chatLogo);
      setCustomUserLogo(snapshotBranding.userLogo);
    }
    setIsSettingsOpen(false);
    setSnapshotEnvConfig(null);
    setSnapshotBranding(null);
  };

  const handleAddProject = (proj: Omit<Project, 'id'>) => {
    const newProj: Project = {
      ...proj,
      id: Date.now().toString()
    };
    setProjects(prev => [...prev, newProj]);
    setActiveProjectId(newProj.id);
  };

  const handleUpdateProject = (updatedProj: Project) => {
    setProjects(prev => prev.map(p => p.id === updatedProj.id ? updatedProj : p));
  };

  const handleDeleteProject = (id: string) => {
    setProjects(prev => {
      const filtered = prev.filter(p => p.id !== id);
      if (activeProjectId === id) {
        setActiveProjectId(filtered[0]?.id || null);
      }
      return filtered;
    });
  };

  const handleRenameChat = (id: string, newTitle: string) => {
    setChatHistory(prev => prev.map(item => item.id === id ? { ...item, title: newTitle } : item));
  };

  const handleDeleteChat = (id: string) => {
    setChatHistory(prev => prev.filter(item => item.id !== id));
    setAllMessages(prev => {
      const { [id]: _, ...rest } = prev;
      return rest;
    });
    if (activeChatId === id) {
      setActiveChatId(null);
      startNewOrchestration();
    }
  };

  const openSettings = (tab: any = 'settings') => {
    setSnapshotEnvConfig({ ...envConfig });
    setSnapshotBranding({ logo: customLogo, favicon: customFavicon, chatLogo: customChatLogo, userLogo: customUserLogo });
    setSettingsTab(tab);
    setIsSettingsOpen(true);
  };

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background-dark text-slate-200">
      <Sidebar
        onSettingsClick={() => openSettings('settings')}
        onLogoClick={() => openSettings('settings')}
        onAddProjectClick={() => openSettings('projects')}
        onNewOrchestration={startNewOrchestration}
        onViewChange={setCurrentView}
        currentView={currentView}
        projects={projects}
        activeProjectId={activeProjectId}
        activeChatId={activeChatId}
        onSelectProject={(pid) => {
          setActiveProjectId(pid);
          const projectHistory = chatHistory.filter(h => h.projectId === pid);
          if (projectHistory.length > 0) setActiveChatId(projectHistory[0].id);
          else startNewOrchestration();
        }}
        onSelectChat={setActiveChatId}
        onRenameChat={handleRenameChat}
        onDeleteChat={handleDeleteChat}
        customLogo={customLogo}
        customUserLogo={customUserLogo}
        chatHistory={chatHistory}
      />

      <main className="flex-1 flex flex-col relative h-full">
        <header className="h-14 border-b border-border-dark flex items-center justify-between px-6 bg-background-dark/80 backdrop-blur-md z-10">
          <div className="flex items-center gap-4 flex-1">
            <div className="flex items-center gap-2 px-3 py-1.5 bg-panel-dark border border-border-dark rounded-lg">
              <span className="material-symbols-outlined text-[18px] text-primary">
                {currentView === 'scheduler' ? 'calendar_today' : (activeProject?.icon || 'workspaces')}
              </span>
              <span className="text-xs font-bold text-white uppercase tracking-wider">
                {currentView === 'scheduler' ? 'Advanced Scheduler' : (activeProject?.name || 'Global')}
              </span>
              <div className="h-3 w-px bg-border-dark mx-1"></div>
              <span className="text-[10px] font-mono text-slate-500 truncate max-w-[200px]">
                {currentView === 'scheduler' ? 'SYSTEM_CRON_TAB' : activeProject?.location}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-4">
            <button
              onClick={() => setShowMemoryBrowser(!showMemoryBrowser)}
              className="flex items-center gap-2 px-3 py-1 bg-black/40 border border-border-dark rounded-full hover:bg-panel-dark transition-colors"
              title="Memory Browser"
            >
              <span className="material-symbols-outlined text-sm text-slate-400">memory</span>
              <span className="text-[10px] font-mono text-slate-400 uppercase tracking-widest">Memory</span>
            </button>
            <button
              onClick={() => {
                const newState = !voiceOutputEnabled;
                setVoiceOutputEnabled(newState);
                if (voiceServiceRef.current) {
                  voiceServiceRef.current.setVoiceOutputEnabled(newState);
                }
              }}
              className={`flex items-center gap-2 px-3 py-1 border rounded-full transition-colors ${
                voiceOutputEnabled
                  ? 'bg-primary/20 border-primary text-primary'
                  : 'bg-black/40 border-border-dark hover:bg-panel-dark text-slate-400'
              }`}
              title={voiceOutputEnabled ? 'Voice Output Enabled' : 'Voice Output Disabled'}
            >
              <span className="material-symbols-outlined text-sm">
                {voiceOutputEnabled ? 'volume_up' : 'volume_off'}
              </span>
              <span className="text-[10px] font-mono uppercase tracking-widest">
                {voiceOutputEnabled ? 'Voice On' : 'Voice Off'}
              </span>
            </button>
            <div className="flex items-center gap-2 px-3 py-1 bg-black/40 border border-border-dark rounded-full">
              <span className={`size-2 rounded-full ${metrics.status === 'ONLINE' ? 'bg-green-500' : 'bg-red-500'}`}></span>
              <span className="text-[10px] font-mono text-slate-400 uppercase tracking-widest">
                Phoenix Core: {metrics.backend}
                {wsConnected && <span className="ml-2 text-green-400">• WS</span>}
              </span>
            </div>
            {(isLiveMode || isDictating) && (
              <div className="flex items-center gap-2 px-3 py-1 bg-red-500/10 border border-red-500/20 rounded-full animate-pulse">
                <span className="size-1.5 rounded-full bg-red-500"></span>
                <span className="text-[10px] font-mono text-red-500 uppercase tracking-widest font-bold">{isLiveMode ? 'Live Link' : 'Dictation Link'}</span>
              </div>
            )}
          </div>
        </header>

        {currentView === 'chat' ? (
          <div className="flex-1 flex overflow-hidden">
            {/* Skills Panel - Left Side */}
            {showSkillsPanel && (
              <div className="w-80 border-r border-border-dark bg-panel-dark overflow-y-auto flex-shrink-0">
                <div className="p-4 border-b border-border-dark bg-black/30 sticky top-0 z-10">
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="text-sm font-bold uppercase tracking-widest text-primary flex items-center gap-2">
                      <span className="material-symbols-outlined text-lg">psychology</span>
                      Skills Library
                    </h3>
                    <button
                      onClick={() => setShowSkillsPanel(false)}
                      className="p-1 hover:bg-slate-800 rounded text-slate-500 hover:text-white transition-colors"
                    >
                      <span className="material-symbols-outlined text-sm">close</span>
                    </button>
                  </div>
                  <p className="text-[10px] text-slate-500 uppercase tracking-wider">Phoenix's learned capabilities</p>
                </div>

                <div className="p-4 space-y-3">
                  {loadingSkills ? (
                    <div className="flex items-center justify-center py-8">
                      <div className="flex items-center gap-2 text-slate-500">
                        <span className="material-symbols-outlined animate-spin">progress_activity</span>
                        <span className="text-sm">Loading skills...</span>
                      </div>
                    </div>
                  ) : skills.length === 0 ? (
                    <div className="text-center py-8 text-slate-500">
                      <span className="material-symbols-outlined text-4xl mb-2 opacity-30">psychology_alt</span>
                      <p className="text-sm">No skills learned yet</p>
                      <p className="text-[10px] mt-2">Skills will appear here as Phoenix learns from interactions</p>
                    </div>
                  ) : (
                    skills.map((skill: any) => (
                      <div key={skill.id} className="bg-black/30 border border-border-dark rounded-lg p-3 hover:border-primary/30 transition-colors">
                        <div className="flex items-start justify-between gap-2 mb-2">
                          <h4 className="text-sm font-semibold text-slate-200">{skill.name}</h4>
                          <span className="text-[9px] px-2 py-0.5 bg-primary/20 text-primary rounded uppercase tracking-wider">{skill.category}</span>
                        </div>
                        <p className="text-xs text-slate-400 mb-3">{skill.description}</p>
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-3 text-[10px]">
                            <span className="flex items-center gap-1 text-pink-400">
                              <span className="material-symbols-outlined text-xs">favorite</span>
                              {((skill.love_score || 0) * 100).toFixed(0)}%
                            </span>
                            <span className="flex items-center gap-1 text-blue-400">
                              <span className="material-symbols-outlined text-xs">check_circle</span>
                              {((skill.success_rate || 0) * 100).toFixed(0)}%
                            </span>
                          </div>
                          <button
                            onClick={() => {
                              setInputValue(`skills run ${skill.id} | input=`);
                              setShowSkillsPanel(false);
                            }}
                            className="text-[10px] px-2 py-1 bg-primary/10 hover:bg-primary/20 text-primary rounded uppercase tracking-wider transition-colors"
                          >
                            Use
                          </button>
                        </div>
                      </div>
                    ))
                  )}
                </div>
              </div>
            )}

            {/* Main Chat Area */}
            <div className="flex-1 flex flex-col overflow-hidden">
              {showMemoryBrowser && memoryServiceRef.current && (
                <div className="px-6 py-4 border-b border-border-dark bg-background-dark/50">
                  <MemoryBrowser memoryService={memoryServiceRef.current} />
                </div>
              )}
              <div className="flex-1 overflow-y-auto px-6 py-10 scroll-smooth">
                <div className="max-w-4xl mx-auto space-y-12">
                  {showOnboarding && currentMessages.length === 0 && (
                    <OnboardingMessage
                      phoenixName={envConfig.PHOENIX_CUSTOM_NAME || 'Sola'}
                      onDismiss={() => {
                        setShowOnboarding(false);
                        localStorage.setItem('phx_onboarding_seen', 'true');
                      }}
                    />
                  )}
                  {currentMessages.map((msg) => (
                    // Avoid rendering an empty streaming bubble (it would look like clutter).
                    (msg.role === 'assistant' && msg.isStreaming && !msg.content)
                      ? null
                      : (
                        <div key={msg.id} className="flex gap-6 group animate-in fade-in slide-in-from-bottom-2 duration-500 relative">
                          <div className={`size-10 rounded-xl flex items-center justify-center shrink-0 border shadow-lg overflow-hidden ${msg.role === 'user' ? 'bg-panel-dark border-border-dark text-slate-500' : 'bg-primary border-primary text-white shadow-primary/10'
                            }`}>
                            {msg.role === 'assistant' && customChatLogo ? (
                              <img src={customChatLogo} alt="AI" className="w-full h-full object-cover" />
                            ) : (
                              <span className="material-symbols-outlined">{msg.role === 'user' ? 'person' : 'bolt'}</span>
                            )}
                          </div>

                          <div className="flex-1 space-y-1 min-w-0">
                            <div className="flex items-center justify-between">
                              <p className={`text-[10px] font-bold uppercase tracking-widest ${msg.role === 'user' ? envConfig.USER_PREFERRED_ALIAS : (msg.agent === 'Orchestrator' ? envConfig.PHOENIX_CUSTOM_NAME : msg.agent) || envConfig.PHOENIX_CUSTOM_NAME}`}>
                                {msg.role === 'user' ? envConfig.USER_PREFERRED_ALIAS : (msg.agent === 'Orchestrator' ? envConfig.PHOENIX_CUSTOM_NAME : msg.agent) || envConfig.PHOENIX_CUSTOM_NAME}
                              </p>
                              <button
                                onClick={() => copyToClipboard(msg.id, msg.content)}
                                className="opacity-0 group-hover:opacity-100 transition-opacity p-1.5 hover:bg-slate-800 rounded-lg text-slate-500 hover:text-white flex items-center gap-2"
                              >
                                <span className="text-[9px] font-bold uppercase tracking-widest">{copiedId === msg.id ? 'Copied' : 'Copy'}</span>
                                <span className="material-symbols-outlined text-sm">{copiedId === msg.id ? 'check' : 'content_copy'}</span>
                              </button>
                            </div>
                            <div className="space-y-4">
                              <div className={`prose prose-invert max-w-none ${msg.role === 'user' ? 'text-lg font-medium' : 'text-base'} leading-relaxed text-slate-300 select-text`}>
                                <ReactMarkdown
                                  remarkPlugins={[remarkGfm]}
                                  components={{
                                    code: CodeBlock
                                  }}
                                >
                                  {msg.content}
                                </ReactMarkdown>
                              </div>
                              {msg.isStreaming && (
                                <div className="flex items-center gap-1 text-slate-500">
                                  <span className="size-1.5 rounded-full bg-slate-600 animate-bounce" style={{ animationDelay: '0ms' }} />
                                  <span className="size-1.5 rounded-full bg-slate-600 animate-bounce" style={{ animationDelay: '120ms' }} />
                                  <span className="size-1.5 rounded-full bg-slate-600 animate-bounce" style={{ animationDelay: '240ms' }} />
                                </div>
                              )}
                              {msg.steps && <WorkflowBlock steps={msg.steps} />}
                              {msg.memoryCommit && (
                                <div className="flex items-center gap-3 pt-4 opacity-30 hover:opacity-100 transition-opacity">
                                  <span className="material-symbols-outlined text-[14px]">terminal</span>
                                  <span className="text-[9px] font-mono tracking-tighter select-text">{msg.memoryCommit}</span>
                                  <div className="flex-1 border-b border-dashed border-border-dark"></div>
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      )
                  ))}
                  {isTyping && (
                    streamingMessageId ? (
                      <div className="flex gap-6">
                        <div className="size-10 rounded-xl bg-primary/10 flex items-center justify-center border border-primary/10 overflow-hidden shrink-0">
                          {customChatLogo ? (
                            <img src={customChatLogo} alt="AI" className="w-full h-full object-cover opacity-40" />
                          ) : (
                            <span className="material-symbols-outlined text-primary/40">bolt</span>
                          )}
                        </div>
                        <div className="flex items-center gap-1 pt-3 text-slate-500">
                          <span className="size-2 rounded-full bg-slate-600 animate-pulse" />
                          <span className="size-2 rounded-full bg-slate-600 animate-pulse" style={{ animationDelay: '140ms' }} />
                          <span className="size-2 rounded-full bg-slate-600 animate-pulse" style={{ animationDelay: '280ms' }} />
                        </div>
                      </div>
                    ) : (
                      <div className="flex gap-6 animate-pulse">
                        <div className="size-10 rounded-xl bg-primary/20 flex items-center justify-center border border-primary/20 overflow-hidden shrink-0">
                          {customChatLogo ? (
                            <img src={customChatLogo} alt="AI" className="w-full h-full object-cover opacity-50" />
                          ) : (
                            <span className="material-symbols-outlined text-primary/50">bolt</span>
                          )}
                        </div>
                        <div className="space-y-2 flex-1 pt-4"><div className="h-1.5 w-full bg-slate-800 rounded"></div><div className="h-1.5 w-2/3 bg-slate-800 rounded"></div></div>
                      </div>
                    )
                  )}
                  <div ref={messagesEndRef} />
                </div>
              </div>

              <footer className="p-6 bg-gradient-to-t from-background-dark via-background-dark/80 to-transparent">
                <div className="max-w-4xl mx-auto relative">
                  <div className={`relative bg-panel-dark border ${isLiveMode ? 'border-primary/50 shadow-2xl shadow-primary/10' : isDictating ? 'border-amber-500/50 shadow-2xl shadow-amber-500/10 ring-1 ring-amber-500/30' : 'border-border-dark'} rounded-2xl overflow-hidden transition-all duration-500`}>
                    {isLiveMode ? (
                      <div className="w-full flex items-center justify-center py-12 gap-8 bg-primary/5">
                        <div className="flex items-end gap-1.5 h-10">
                          {[...Array(12)].map((_, i) => (
                            <div key={i} className={`w-2 bg-primary rounded-full animate-bounce`} style={{ height: `${20 + Math.random() * 80}%`, animationDelay: `${i * 0.05}s` }}></div>
                          ))}
                        </div>
                        <div className="flex flex-col">
                          <span className="text-sm font-mono text-primary font-bold uppercase tracking-widest">Active Listening</span>
                          <span className="text-[10px] text-slate-500 uppercase">Context: {activeProject?.name}</span>
                        </div>
                      </div>
                    ) : (
                      <div className="relative group/input">
                        <textarea
                          className="w-full bg-transparent border-none focus:ring-0 text-base py-5 px-6 resize-none placeholder:text-slate-600 text-slate-200 min-h-[60px]"
                          placeholder={isDictating ? 'Dictating speech into context...' : `Chat with ${envConfig.PHOENIX_CUSTOM_NAME || envConfig.PHOENIX_PREFERRED_NAME || 'Sola'}... (ENTER to send, SHIFT+ENTER for new line)`}
                          rows={1}
                          value={inputValue}
                          onChange={(e) => setInputValue(e.target.value)}
                          onKeyDown={handleKeyDown}
                        ></textarea>
                        {isDictating && (
                          <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-2 text-amber-500 animate-pulse pointer-events-none">
                            <span className="text-[10px] font-mono font-bold uppercase tracking-widest">Listening</span>
                            <span className="material-symbols-outlined text-sm">graphic_eq</span>
                          </div>
                        )}
                      </div>
                    )}

                    <div className="flex items-center justify-between px-6 py-4 border-t border-border-dark bg-black/30">
                      <div className="flex items-center gap-4">
                        <button className="p-2 rounded-lg hover:bg-slate-800 text-slate-500 transition-colors" title="Scan Mapped Directory">
                          <span className="material-symbols-outlined text-[20px]">folder_shared</span>
                        </button>

                        <button
                          onClick={() => setShowSkillsPanel(!showSkillsPanel)}
                          className={`p-2 rounded-lg transition-all ${showSkillsPanel ? 'bg-primary/20 text-primary' : 'hover:bg-slate-800 text-slate-500'}`}
                          title="Skills Library"
                        >
                          <span className="material-symbols-outlined text-[20px]">psychology</span>
                        </button>

                        <button
                          onClick={() => setShowWebGuardPanel(!showWebGuardPanel)}
                          className={`p-2 rounded-lg transition-all ${showWebGuardPanel ? 'bg-cyan-500/20 text-cyan-400' : 'hover:bg-slate-800 text-slate-500'}`}
                          title="WebGuard Security Scanner"
                        >
                          <span className="material-symbols-outlined text-[20px]">shield</span>
                        </button>

                        <button
                          onClick={() => setShowReportsPanel(!showReportsPanel)}
                          className={`p-2 rounded-lg transition-all ${showReportsPanel ? 'bg-cyan-500/20 text-cyan-400' : 'hover:bg-slate-800 text-slate-500'}`}
                          title="Vulnerability Reports"
                        >
                          <span className="material-symbols-outlined text-[20px]">description</span>
                        </button>

                        <button
                          onClick={isDictating ? () => stopDictation() : startDictation}
                          className={`p-2 rounded-lg transition-all flex items-center gap-2 px-3 ${isDictating ? 'bg-amber-500 text-white shadow-lg shadow-amber-500/20' : 'hover:bg-slate-800 text-slate-500'}`}
                          title="Voice-to-Text (Dictation)"
                        >
                          <span className="material-symbols-outlined text-[20px]">{isDictating ? 'mic_active' : 'mic'}</span>
                          {isDictating && <span className="text-[10px] font-bold uppercase">Stop</span>}
                        </button>

                        <button
                          onClick={isLiveMode ? stopLiveMode : startLiveMode}
                          className={`p-2 rounded-lg transition-all flex items-center gap-2 px-3 ${isLiveMode ? 'bg-red-500 text-white shadow-lg shadow-red-500/20' : 'hover:bg-slate-800 text-slate-500'}`}
                          title="Real-time Voice Conversation"
                        >
                          <span className="material-symbols-outlined text-[20px]">{isLiveMode ? 'call_end' : 'headset'}</span>
                          {isLiveMode && <span className="text-[10px] font-bold uppercase">End Live</span>}
                        </button>
                      </div>
                      <div className="flex items-center gap-4">
                        {!isLiveMode && !isDictating && <span className="text-[10px] font-mono text-slate-600">ENTER TO SEND</span>}
                        <button
                          onClick={handleSendMessage}
                          disabled={!inputValue.trim() || isTyping || isLiveMode || !activeChatId}
                          className="bg-primary hover:bg-primary/90 text-white size-10 rounded-xl flex items-center justify-center transition-all shadow-lg shadow-primary/20 disabled:opacity-30"
                        >
                          <span className="material-symbols-outlined text-[22px]">send</span>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
                <p className="text-center text-[10px] text-slate-600 mt-4 uppercase tracking-[0.3em]">Phoenix AGI OS V2.4.0 • Sovereign Digital Twin</p>
              </footer>
            </div>
          </div>
        ) : currentView === 'scheduler' ? (
          <SchedulerView
            tasks={scheduledTasks}
            projects={projects}
            onAddTask={handleAddTask}
            onUpdateTask={handleUpdateTask}
            onDeleteTask={handleDeleteTask}
          />
        ) : (
          <ProfessionalDashboard />
        )}
      </main>

      <SettingsPanel
        isOpen={isSettingsOpen}
        onClose={handleCancelSettings}
        onSave={handleSaveSettings}
        initialTab={settingsTab}
        customLogo={customLogo}
        customFavicon={customFavicon}
        customChatLogo={customChatLogo}
        customUserLogo={customUserLogo}
        onUpdateBranding={handleBrandingUpdatePreview}
        envConfig={envConfig}
        onUpdateEnvConfig={handleEnvConfigUpdatePreview}
        projects={projects}
        onAddProject={handleAddProject}
        onUpdateProject={handleUpdateProject}
        onDeleteProject={handleDeleteProject}
      />

      <DreamsPanel
        isOpen={showDreamsPanel}
        onClose={() => setShowDreamsPanel(false)}
        onCommand={(cmd) => {
          setInputValue(cmd);
          handleSendMessage();
        }}
        dreams={dreamRecords}
      />

      <WebGuardReportPanel
        isOpen={showWebGuardPanel}
        onClose={() => setShowWebGuardPanel(false)}
        reports={webGuardReports}
        onCommand={(cmd) => {
          setInputValue(cmd);
          setShowWebGuardPanel(false);
        }}
      />

      <ReportsPanel
        isOpen={showReportsPanel}
        onClose={() => setShowReportsPanel(false)}
        reports={vulnerabilityReports}
        onCommand={(cmd) => {
          setInputValue(cmd);
          setShowReportsPanel(false);
        }}
      />

      {showProfilesPanel && (
        <ProfilesSwipePanel
          onClose={() => setShowProfilesPanel(false)}
          backendUrl={BACKEND_URL}
        />
      )}
    </div>
  );
};

export default App;
