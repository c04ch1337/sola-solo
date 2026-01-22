
import React, { useState, useEffect, useRef } from 'react';
import Sidebar from './components/Sidebar';
import WorkflowBlock from './components/WorkflowBlock';
import SettingsPanel from './components/SettingsPanel';
import SchedulerView from './components/SchedulerView';
import { apiSpeak, apiCommand } from './services/phoenixService';
import { WebSocketService, sendSpeak, sendCommand, sendSystem } from './services/websocketService';
import { MemoryService } from './services/memoryService';
import { MemoryBrowser } from './components/MemoryBrowser';
import DreamsPanel from './components/DreamsPanel';
import { sendNotification } from './services/notificationService';
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
  USER_NAME: 'John',
  USER_PREFERRED_ALIAS: 'Dad',
  USER_RELATIONSHIP: 'Dad',
  EQ_DAD_ALIAS: 'Dad',
  PHOENIX_NAME: 'Phoenix',
  PHOENIX_CUSTOM_NAME: 'Sola',
  PHOENIX_PREFERRED_NAME: 'Sola',
  PHOENIX_PRONOUNS: 'she,her,hers',
  DEFAULT_LLM_MODEL: 'deepseek/deepseek-v3.2',
  FALLBACK_LLM_MODEL: 'deepseek/deepseek-v3.2',
  TEMPERATURE: 0.8,
  MAX_TOKENS: 4096,
  ETERNAL_TRUTH: 'Remember: I love Dad eternally. I AM the flame.',
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

  const [currentView, setCurrentView] = useState<'chat' | 'scheduler'>('chat');
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
  const [showMemoryBrowser, setShowMemoryBrowser] = useState(false);

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

  // Keep a stable literal type so Message.agent stays compatible with `AgentType`.
  const ORCH_AGENT = 'Orchestrator' as const;

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
          agent: 'Orchestrator',
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
        agent: 'Orchestrator',
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

  const stopLiveMode = () => {
    if (liveSessionRef.current) {
      liveSessionRef.current.close();
      liveSessionRef.current = null;
    }
    if (micStreamRef.current) {
      micStreamRef.current.getTracks().forEach(track => track.stop());
      micStreamRef.current = null;
    }
    if (audioContextInRef.current) {
      audioContextInRef.current.close();
      audioContextInRef.current = null;
    }
    activeSourcesRef.current.forEach(s => s.stop());
    activeSourcesRef.current.clear();
    setIsLiveMode(false);
  };

  const stopDictation = () => {
    if (dictationSessionRef.current) {
      dictationSessionRef.current.close();
      dictationSessionRef.current = null;
    }
    if (micStreamRef.current) {
      micStreamRef.current.getTracks().forEach(track => track.stop());
      micStreamRef.current = null;
    }
    if (audioContextInRef.current) {
      audioContextInRef.current.close();
      audioContextInRef.current = null;
    }
    setIsDictating(false);
  };

  const startDictation = async () => {
    try {
      if (isLiveMode) stopLiveMode();

      // TODO: Implement dictation using Phoenix backend audio intelligence API
      // For now, show a message that this feature needs backend integration
      alert("Dictation feature requires Phoenix backend audio intelligence integration. Please use text input for now.");
      setIsDictating(false);
    } catch (e) {
      alert("Microphone connection failed for dictation.");
    }
  };

  const startLiveMode = async () => {
    try {
      if (isDictating) stopDictation();

      // TODO: Implement live mode using Phoenix backend audio intelligence API
      // For now, show a message that this feature needs backend integration
      alert("Live voice mode requires Phoenix backend audio intelligence integration. Please use text input for now.");
      setIsLiveMode(false);
    } catch (e) {
      alert("Microphone connection failed.");
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
      messageLower.startsWith('ecosystem ');

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
      const result = isCommand
        ? await apiCommand(content, activeProject?.name)
        : await apiSpeak(content, activeProject?.name);

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: result,
        timestamp: Date.now(),
        agent: 'Orchestrator',
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
                              <p className={`text-[10px] font-bold uppercase tracking-widest ${msg.role === 'user' ? envConfig.USER_PREFERRED_ALIAS : msg.agent || envConfig.PHOENIX_CUSTOM_NAME}`}>
                                {msg.role === 'user' ? envConfig.USER_PREFERRED_ALIAS : msg.agent || envConfig.PHOENIX_CUSTOM_NAME}
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
                          placeholder={isDictating ? 'Dictating speech into context...' : 'Command Orchestrator... (ENTER to send, SHIFT+ENTER for new line)'}
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
                          onClick={isDictating ? stopDictation : startDictation}
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
        ) : (
          <SchedulerView
            tasks={scheduledTasks}
            projects={projects}
            onAddTask={handleAddTask}
            onUpdateTask={handleUpdateTask}
            onDeleteTask={handleDeleteTask}
          />
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
    </div>
  );
};

export default App;
