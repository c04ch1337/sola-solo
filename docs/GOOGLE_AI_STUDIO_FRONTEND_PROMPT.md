# Google AI Studio Prompt: Recreate Phoenix Frontend UI

## Prompt

Create a complete React + TypeScript frontend application for Phoenix AGI, a relationship-centric AI companion interface. This is a single-page application (SPA) with multiple specialized views accessible via sidebar navigation.

### Project Setup

**Technology Stack:**
- React 18.2.0 (functional components with hooks)
- TypeScript ~5.8.2 (strict mode)
- Vite ^6.2.0 (build tool and dev server)
- Tailwind CSS ^4.1.13 (utility-first styling)
- Lucide React 0.263.1 (icon library)

**Project Structure:**
```
frontend/
├── index.tsx          # Main application file (all components)
├── devtools.tsx       # DevTools view component
├── vite.config.ts     # Vite configuration with proxy
├── package.json       # Dependencies
├── tsconfig.json      # TypeScript config
├── tailwind.config.cjs # Tailwind configuration
├── styles.css         # Global styles and animations
├── index.html         # HTML entry point
└── public/            # Static assets
```

**Vite Configuration:**
- Dev server on port 3000 (configurable via VITE_PORT)
- Proxy `/api/*` and `/health` to `http://127.0.0.1:8888`
- React plugin enabled
- TypeScript support

### Core Design Principles

1. **Chat-First**: Chat interface is the primary interaction method, always accessible
2. **Relationship-Centric**: Emotional connection and intimacy levels drive UX decisions
3. **Modular Views**: Each feature has its own view component
4. **Context-Based State**: Global state managed via React Context
5. **Service Abstraction**: All backend API calls through `PhoenixBackendService` class
6. **Type Safety**: Full TypeScript coverage, no `any` types
7. **Dark Theme**: Void/black background (#0f0b15) with Phoenix pink accents (#db2777)

### Type Definitions

Create these TypeScript interfaces:

```typescript
interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
}

interface Archetype {
  id: string;
  name: string;
  zodiac: string;
  traits: string[];
  communication_style: string;
  mood_preferences: string[];
}

interface DatingProfile {
  name: string;
  age: number;
  archetype_id: string;
  preferences: Record<string, any>;
}

interface MemoryItem {
  key: string;
  value: string;
}

interface VectorMemoryResult {
  text: string;
  metadata: Record<string, any>;
  score: number;
}

interface Agent {
  id: string;
  name: string;
  status: 'online' | 'offline' | 'error';
  description: string;
}

interface RepoMetadata {
  id: string;
  name: string;
  owner: string;
  url: string;
  build_system: 'Cargo' | 'Npm' | 'Pip' | 'Make' | 'Docker' | 'Unknown';
  build_status: 'NotBuilt' | 'Building' | 'Built' | 'BuildFailed';
  service_status: 'Stopped' | 'Starting' | 'Running' | 'Stopping' | 'Error';
  port?: number;
  commands: string[];
}

interface UiSettings {
  theme: 'dark' | 'light';
  fontSize: 'small' | 'medium' | 'large';
  animations: boolean;
}
```

### PhoenixBackendService Class

Create a service class that handles all backend communication:

```typescript
class PhoenixBackendService {
  private baseUrl: string;
  
  constructor() {
    this.baseUrl = (import.meta.env?.VITE_PHOENIX_API_BASE as string | undefined)
      ?.replace(/\/$/, '') || 'http://127.0.0.1:8888';
  }
  
  private url(path: string): string {
    return `${this.baseUrl}${path}`;
  }
  
  // Memory operations
  async memoryStore(key: string, value: string): Promise<void>
  async memoryGet(key: string): Promise<MemoryItem | null>
  async memorySearch(query: string): Promise<MemoryItem[]>
  async memoryDelete(key: string): Promise<void>
  async vectorMemoryStore(text: string, metadata: Record<string, any>): Promise<void>
  async vectorMemorySearch(query: string, k?: number): Promise<VectorMemoryResult[]>
  async vectorMemoryAll(): Promise<VectorMemoryResult[]>
  
  // Command and communication
  async sendCommand(command: string): Promise<string>
  async getPhoenixName(): Promise<string>
  async status(): Promise<{ status: string; version?: string }>
  
  // Archetype operations
  async applyArchetype(id: string, profile: DatingProfile): Promise<void>
  
  // System operations
  async setKeylogger(enabled: boolean, path: string): Promise<void>
  async setMouseJigger(enabled: boolean): Promise<void>
  
  // Ecosystem operations
  async ecosystemImport(owner: string, repo: string, branch?: string): Promise<RepoMetadata>
  async ecosystemList(): Promise<RepoMetadata[]>
  async ecosystemBuild(id: string): Promise<void>
  async ecosystemStart(id: string): Promise<void>
  async ecosystemStop(id: string): Promise<void>
  async ecosystemDelete(id: string): Promise<void>
  
  // System access (DevTools)
  async systemExecute(command: string): Promise<string>
  async systemReadFile(path: string): Promise<string>
  async systemWriteFile(path: string, content: string): Promise<void>
  async systemStatus(): Promise<{ full_access_granted: boolean; self_modification_enabled: boolean }>
}
```

### PhoenixContext

Create a React Context for global state:

```typescript
interface PhoenixContextType {
  isConnected: boolean;
  messages: Message[];
  sendMessage: (text: string) => Promise<void>;
  runCommand: (text: string) => Promise<string>;
  applyArchetype: (id: string, profile: DatingProfile) => Promise<void>;
  currentArchetype: Archetype | null;
  clearHistory: () => void;
  deleteMessage: (id: string) => void;
  relationalScore: number;
  sentiment: 'positive' | 'negative' | 'neutral';
  setRelationalScore: (score: number) => void;
  setSentiment: (sentiment: 'positive' | 'negative' | 'neutral') => void;
  phoenixName: string;
  setKeylogger: (enabled: boolean, path: string) => Promise<void>;
  setMouseJigger: (enabled: boolean) => Promise<void>;
}

const PhoenixContext = React.createContext<PhoenixContextType | null>(null);
```

### Main Application Structure

**DashboardLayout Component:**
- Root component with sidebar navigation and main content area
- Sidebar on left (fixed width ~240px)
- Main content area on right (flexible)
- Dark background (#0f0b15)

**Sidebar Navigation:**
- Dashboard section:
  - Chat Stream (MessageSquare icon)
  - Studio & Recording (Film icon)
  - Orchestrator (Network icon)
  - Google Ecosystem (Cloud icon)
  - EcoSystem (GitBranch icon)
  - Archetype Matcher (Heart icon)
  - Memories & Context (Brain icon)
- System section:
  - Clear Memory (Trash2 icon)
  - Self-Mod Console (Terminal icon)
  - Settings (Settings icon)

**View Components (all in index.tsx):**

1. **ChatView** - Primary chat interface
   - Message list with user/assistant distinction
   - Input field at bottom
   - Auto-scroll to latest message
   - Connection status indicator
   - Message animations (slide-in-left for user, slide-in-right for assistant)

2. **MemoriesView** - Memory browser
   - Tabs: Episodic, Semantic, Vector
   - Search functionality
   - Create/Edit/Delete operations
   - Vector memory semantic search

3. **OrchestratorView** - ORCH agent management
   - List of agents with status
   - Start/Stop controls
   - Agent details

4. **StudioView** - Recording interface
   - Audio/Video/Screen recording controls
   - Recording status
   - Session management

5. **GoogleEcosystemView** - Google services integration
   - Gmail, Drive, Calendar sections
   - OAuth status
   - Service controls

6. **EcoSystemView** - GitHub repository manager
   - Import form (owner/repo/branch)
   - Repository list with status cards
   - Build/Start/Stop controls
   - Service status indicators

7. **DatingProfileMatcher** - Archetype matching
   - Archetype selection
   - Profile creation form
   - Match results

8. **DevToolsView** - System access console
   - Command execution
   - File read/write
   - System status display

9. **Settings** - UI preferences
   - Theme selection
   - Font size
   - Animation toggle

### Styling Guidelines

**Color Palette:**
- Background: `#0f0b15` (void-900)
- Secondary background: `#1a1625` (void-800)
- Border: `rgba(255, 255, 255, 0.1)` (white/10)
- Primary accent: `#db2777` (phoenix-600)
- Text primary: `#ffffff`
- Text secondary: `#9ca3af` (gray-400)

**Tailwind Classes:**
- Container: `h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar`
- Card/Panel: `glass-panel p-6 rounded-2xl border border-white/10`
- Button Primary: `bg-phoenix-600 hover:bg-phoenix-500 text-white rounded-lg py-2.5 font-semibold`
- Button Secondary: `bg-white/5 hover:bg-white/10 text-gray-200 rounded-lg border border-white/10`
- Input: `w-full bg-void-900 border border-white/10 rounded-lg p-3 text-white outline-none focus:border-phoenix-500`

**Custom CSS Classes (in styles.css):**
```css
.glass-panel {
  background: rgba(15, 11, 21, 0.7);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.glass-card {
  background: rgba(15, 11, 21, 0.5);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.glass-card:hover {
  background: rgba(15, 11, 21, 0.8);
  border-color: rgba(219, 39, 119, 0.3);
}

.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(15, 11, 21, 0.5);
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(219, 39, 119, 0.5);
  border-radius: 4px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(219, 39, 119, 0.7);
}
```

**Animations:**
- Message slide-in: `msg-in-left` (user), `msg-in-right` (assistant)
- Pop-in: `pop-in` for modals
- Heartbeat: `heartbeat-slow` for connection status
- Life pulse: `life-pulse` for active elements

### Component Patterns

**View Component Structure:**
```typescript
const FeatureView = () => {
  // 1. Context access
  const { isConnected, messages } = useContext(PhoenixContext)!;
  
  // 2. Local state
  const [localState, setLocalState] = useState<Type>(initialValue);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // 3. Service instance
  const phoenixService = useMemo(() => new PhoenixBackendService(), []);
  
  // 4. Effects
  useEffect(() => {
    // Initialization
  }, []);
  
  // 5. Event handlers
  const handleAction = async () => {
    setLoading(true);
    setError(null);
    try {
      await phoenixService.method();
    } catch (e: any) {
      setError(e.message || 'Operation failed');
    } finally {
      setLoading(false);
    }
  };
  
  // 6. Render
  return (
    <div className="h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar">
      <div className="max-w-5xl mx-auto p-8">
        {/* View content */}
      </div>
    </div>
  );
};
```

### Key Features Implementation

**1. Chat Interface:**
- Real-time message display
- Auto-scroll to bottom
- Message input with Enter key support
- Connection status indicator (green/red dot)
- Message role styling (user right-aligned, assistant left-aligned)
- Timestamp display
- Message animations

**2. Memory Management:**
- Three memory types: Episodic (key-value), Semantic (searchable), Vector (embeddings)
- Search functionality for semantic and vector memories
- Create/Edit/Delete operations
- Memory list with cards
- Vector memory semantic search with score display

**3. Status Polling:**
- Poll backend status every 5 seconds
- Update connection indicator
- Handle disconnection gracefully

**4. Local Storage:**
- Persist UI settings
- Persist message history (optional)
- Use `useLocalStorageJsonState` hook pattern

**5. Error Handling:**
- Try-catch blocks for all async operations
- User-friendly error messages
- Error state display (red text, error icon)
- Loading states for all async operations

**6. Responsive Design:**
- Mobile-first approach
- Sidebar collapses on mobile
- Flexible layouts with Tailwind responsive classes
- Max-width containers for readability

### Static Data

**Archetypes Database:**
Create an array of 12 zodiac archetypes (Aries through Pisces) with:
- id, name, zodiac sign
- traits array
- communication_style
- mood_preferences array

**Available Tools:**
Array of tool names for display in OrchestratorView

### Helper Functions

**useLocalStorageJsonState Hook:**
```typescript
function useLocalStorageJsonState<T>(
  key: string,
  defaultValue: T
): [T, (value: T) => void] {
  const [state, setState] = useState<T>(() => {
    try {
      const item = localStorage.getItem(key);
      return item ? JSON.parse(item) : defaultValue;
    } catch {
      return defaultValue;
    }
  });
  
  const setValue = (value: T) => {
    try {
      localStorage.setItem(key, JSON.stringify(value));
      setState(value);
    } catch (e) {
      console.error('Failed to save to localStorage', e);
    }
  };
  
  return [state, setValue];
}
```

### Implementation Checklist

1. ✅ Set up Vite project with React + TypeScript
2. ✅ Install dependencies (React, Tailwind, Lucide React)
3. ✅ Configure Vite with proxy to backend
4. ✅ Create type definitions
5. ✅ Create PhoenixBackendService class
6. ✅ Create PhoenixContext and Provider
7. ✅ Create DashboardLayout with Sidebar
8. ✅ Create all View components
9. ✅ Implement ChatView with message handling
10. ✅ Implement MemoriesView with tabs
11. ✅ Implement status polling
12. ✅ Add error handling throughout
13. ✅ Add loading states
14. ✅ Style with Tailwind CSS
15. ✅ Add custom CSS classes
16. ✅ Add animations
17. ✅ Implement local storage for settings
18. ✅ Add responsive design
19. ✅ Test all API integrations
20. ✅ Add connection status indicators

### Additional Requirements

- **No direct fetch calls**: All API calls must go through PhoenixBackendService
- **No inline styles**: Use Tailwind classes (except for dynamic values)
- **Type safety**: No `any` types, proper TypeScript interfaces
- **Error handling**: All async operations wrapped in try-catch
- **Loading states**: Show loading indicators for all async operations
- **Accessibility**: Semantic HTML, ARIA labels for icon-only buttons
- **Performance**: Use useMemo and useCallback where appropriate
- **Code organization**: Group related functionality, clear component structure

### Expected Behavior

1. **On Load:**
   - Check backend connection status
   - Load saved UI settings
   - Initialize PhoenixContext
   - Start status polling

2. **Navigation:**
   - Clicking sidebar items switches views
   - Active view highlighted in sidebar
   - Smooth transitions between views

3. **Chat:**
   - Messages appear in real-time
   - Auto-scroll to latest message
   - Enter key sends message
   - Connection status visible

4. **Memory Operations:**
   - Create/Read/Update/Delete operations
   - Search works across memory types
   - Vector search shows similarity scores

5. **Error States:**
   - Connection errors show red indicator
   - API errors display user-friendly messages
   - Loading states prevent duplicate requests

### Output

Generate the complete frontend application with:
- All files in the correct structure
- Complete TypeScript types
- All view components implemented
- Styling with Tailwind CSS
- Custom CSS for glass effects and animations
- Full error handling and loading states
- Responsive design
- Local storage integration
- Status polling
- All API service methods

The application should be production-ready and follow all the patterns and principles outlined above.

---

## Usage Instructions

Copy the prompt above into Google AI Studio (Gemini) and request it to generate the complete frontend application. The AI will create all necessary files with proper structure, types, components, and styling.

**After generation:**
1. Review the generated code
2. Install dependencies: `npm install`
3. Start dev server: `npm run dev`
4. Verify backend connection (should proxy to port 8888)
5. Test all views and functionality

**Customization:**
- Adjust colors in Tailwind config
- Modify animations in styles.css
- Add new views following the same patterns
- Extend PhoenixBackendService with new API methods

