
export enum StepStatus {
  CREATING = 'CREATING',
  SPEC = 'SPEC',
  BUILD = 'BUILD',
  RESULT = 'RESULT',
  SCANNING = 'SCANNING',
  ORCHESTRATING = 'ORCHESTRATING'
}

export interface WorkflowStep {
  label: StepStatus;
  icon: string;
  text: string;
  progress?: number;
  highlightText?: string;
  colorClass: string;
}

export interface Project {
  id: string;
  name: string;
  icon: string;
  location: string;
  description: string;
  authScope?: 'ReadPolicy' | 'WritePolicy' | 'SystemAdmin';
}

export interface ChatHistoryItem {
  id: string;
  title: string;
  projectId: string;
  timestamp: number;
}

export type AgentType = 'Orchestrator' | 'RedTeamSupervisor' | 'BlueTeamSupervisor';
export type TaskPriority = 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
export type TaskStatus = 'PENDING' | 'RUNNING' | 'COMPLETED' | 'FAILED';

export type RecurrencePattern = 'NONE' | 'DAILY' | 'WEEKLY' | 'HOURLY' | 'THREE_HOURLY' | 'EVERY_SIX_HOURS' | 'TWELVE_HOURLY' | 'WEEKLY_MON' | 'WEEKLY_WED' | 'WEEKLY_FRI' | 'BI_WEEKLY' | 'WEEKDAYS' | 'WEEKENDS' | 'MONTHLY' | 'BI_MONTHLY' | 'LAST_DAY_OF_MONTH' | 'QUARTERLY' | 'SEMI_ANNUALLY' | 'ANNUALLY';

export interface ScheduledTask {
  id: string;
  title: string;
  description: string;
  projectId: string;
  targetAgent: AgentType;
  priority: TaskPriority;
  status: TaskStatus;
  scheduledTime: string;
  tools: string[];
  recurring: RecurrencePattern;
}

export interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  steps?: WorkflowStep[];
  memoryCommit?: string;
  isStreaming?: boolean;
  timestamp: number;
  projectId?: string;
  type?: 'speak' | 'command';
  agent?: AgentType;
}

export interface SystemMetrics {
  prc: number;
  status: 'ONLINE' | 'OFFLINE';
  backend: string;
}

export interface EnvConfig {
  // Required API Keys
  OPENROUTER_API_KEY: string;
  GITHUB_PAT: string;

  // User Identity
  USER_NAME: string;
  USER_PREFERRED_ALIAS: string;
  USER_RELATIONSHIP: string;
  EQ_DAD_ALIAS: string;

  // Phoenix Identity
  PHOENIX_NAME: string;
  PHOENIX_CUSTOM_NAME: string;
  PHOENIX_PREFERRED_NAME: string;
  PHOENIX_PRONOUNS: string;

  // LLM Configuration
  DEFAULT_LLM_MODEL: string;
  FALLBACK_LLM_MODEL: string;
  TEMPERATURE: number;
  MAX_TOKENS: number;
  ETERNAL_TRUTH: string;
  CAPABILITIES_IN_PROMPT: boolean;

  // Synaptic Tuning (Personality)
  CURIOSITY_DRIVE: number;
  SELF_PRESERVATION_INSTINCT: number;
  MISCHIEF_FACTOR: number;
  LOVE_WEIGHT: number;
  LAUGH_DELAY: number;
  VOICE_LILT: number;
  WARMTH_CURVE: number;
  EYE_SPARKLE_INTENSITY: number;
  MEMORY_RETENTION_RATE: number;

  // Modes & Dynamics
  ORCH_MASTER_MODE: boolean;
  ORCH_SLAVE_SYNC_INTERVAL: number;
  PARTNER_MODE_ENABLED: boolean;
  PARTNER_TYPE: string; // girlfriend, boyfriend, partner
  SEXUAL_ORIENTATION: string;
  RELATIONSHIP_TEMPLATE: string;
  RELATIONSHIP_INTIMACY_LEVEL: string;

  // Integrations & Storage
  GITHUB_USERNAME: string;
  GITHUB_AGENTS_REPO: string;
  GITHUB_TOOLS_REPO: string;
  VECTOR_KB_ENABLED: boolean;
  DIGITAL_TWIN_ENABLED: boolean;
  X402_ENABLED: boolean;

  // UI / CSS Customization
  UI_PRIMARY_COLOR: string;
  UI_BG_DARK: string;
  UI_PANEL_DARK: string;
  UI_BORDER_DARK: string;
  UI_FONT_FAMILY: string;
  UI_CUSTOM_CSS: string;
}
