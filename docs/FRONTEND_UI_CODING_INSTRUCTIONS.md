# Frontend UI - IDE Agent Coding Custom Instructions

## Purpose

This document provides comprehensive coding instructions for IDE agents (like Cursor AI) working on the Phoenix Frontend UI. It establishes patterns, conventions, and best practices to ensure consistent, maintainable code.

---

## Architecture Overview

### Technology Stack
- **React**: 18.2.0 (Functional components with hooks)
- **TypeScript**: ~5.8.2 (Strict type checking)
- **Vite**: ^6.2.0 (Build tool and dev server)
- **Tailwind CSS**: ^4.1.13 (Utility-first styling)
- **Lucide React**: 0.263.1 (Icon library)

### File Structure
```
frontend/
├── index.tsx          # Main application (3341 lines - all views/components)
├── devtools.tsx       # DevTools view component
├── vite.config.ts     # Vite configuration
├── package.json       # Dependencies
├── tsconfig.json      # TypeScript config
├── tailwind.config.cjs # Tailwind configuration
├── styles.css         # Global styles
├── index.html         # HTML entry point
└── public/            # Static assets
```

### Key Design Principles

1. **Chat-First**: Chat interface is primary, always accessible
2. **Relationship-Centric**: Emotional connection drives UX decisions
3. **Modular Views**: Each view is a separate component
4. **Context-Based State**: Global state via React Context
5. **Service Abstraction**: All backend calls through `PhoenixBackendService`
6. **Type Safety**: Full TypeScript coverage

---

## Code Organization Patterns

### 1. Component Structure

**Standard Component Pattern**:
```typescript
// Import statements (grouped)
import React, { useState, useEffect } from 'react';
import { IconName } from 'lucide-react';

// Type definitions (if component-specific)
interface ComponentProps {
  prop1: string;
  prop2?: number;
}

// Component implementation
const ComponentName: React.FC<ComponentProps> = ({ prop1, prop2 }) => {
  // Hooks (state, effects, etc.)
  const [state, setState] = useState<string>('');
  
  useEffect(() => {
    // Effect logic
  }, []);
  
  // Event handlers
  const handleAction = () => {
    // Handler logic
  };
  
  // Render
  return (
    <div className="...">
      {/* JSX */}
    </div>
  );
};
```

### 2. View Components

**Location**: All views in `frontend/index.tsx`

**Naming Convention**: `{Feature}View` (e.g., `ChatView`, `MemoriesView`)

**Structure**:
```typescript
const FeatureView = () => {
  // 1. Context access
  const { isConnected, messages, sendMessage } = useContext(PhoenixContext)!;
  
  // 2. Local state
  const [localState, setLocalState] = useState<Type>(initialValue);
  
  // 3. Service instance
  const phoenixService = useMemo(() => new PhoenixBackendService(), []);
  
  // 4. Effects
  useEffect(() => {
    // Initialization logic
  }, []);
  
  // 5. Event handlers
  const handleAction = async () => {
    try {
      await phoenixService.method();
      // Update UI
    } catch (e) {
      // Error handling
    }
  };
  
  // 6. Render
  return (
    <div className="h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar">
      {/* View content */}
    </div>
  );
};
```

### 3. Backend Service Integration

**Always use `PhoenixBackendService` class** - Never make direct fetch calls.

**Pattern**:
```typescript
// ✅ CORRECT
const phoenixService = useMemo(() => new PhoenixBackendService(), []);

const handleStore = async () => {
  try {
    await phoenixService.memoryStore(key, value);
    // Update UI
  } catch (e) {
    // Handle error
  }
};

// ❌ WRONG - Don't do this
const handleStore = async () => {
  const res = await fetch('/api/memory/store', { ... });
  // Direct fetch bypasses service abstraction
};
```

**Service Methods Available**:
- `memoryStore(key, value)` - Store memory
- `memoryGet(key)` - Get memory
- `memorySearch(query)` - Search memories
- `memoryDelete(key)` - Delete memory
- `vectorMemoryStore(text, metadata)` - Store vector memory
- `vectorMemorySearch(query, k?)` - Vector search
- `vectorMemoryAll()` - List all vector memories
- `sendCommand(command)` - Send command
- `getPhoenixName()` - Get Phoenix name
- `status()` - Get system status
- `applyArchetype(id, profile)` - Apply archetype
- `setKeylogger(enabled, path)` - Configure keylogger
- `setMouseJigger(enabled)` - Configure mouse jigger

### 4. Context Usage

**Always use `PhoenixContext` for global state**:

```typescript
// ✅ CORRECT
const { isConnected, messages, sendMessage } = useContext(PhoenixContext)!;

// ❌ WRONG - Don't create new context
const [localMessages, setLocalMessages] = useState<Message[]>([]);
```

**Available Context Values**:
- `isConnected: boolean` - Backend connection status
- `messages: Message[]` - Message history
- `sendMessage: (text: string) => Promise<void>` - Send message
- `runCommand: (text: string) => Promise<string>` - Run command
- `applyArchetype: (id: string, profile: DatingProfile) => Promise<void>`
- `currentArchetype: Archetype | null`
- `clearHistory: () => void`
- `deleteMessage: (id: string) => void`
- `relationalScore: number`
- `sentiment: 'positive' | 'negative' | 'neutral'`
- `setRelationalScore: (score: number) => void`
- `setSentiment: (sentiment: 'positive' | 'negative' | 'neutral') => void`
- `phoenixName: string`
- `setKeylogger: (enabled: boolean, path: string) => Promise<void>`
- `setMouseJigger: (enabled: boolean) => Promise<void>`

---

## Styling Guidelines

### 1. Tailwind CSS Classes

**Use Tailwind utility classes** - Avoid inline styles except for dynamic values.

**Color Palette**:
```typescript
// Phoenix colors
className="bg-phoenix-500 text-phoenix-600 border-phoenix-400"

// Void (dark) colors
className="bg-void-900 border-void-800 text-void-700"

// Glass effects
className="glass-panel" // Defined in styles.css
className="glass-card"  // Defined in styles.css
```

**Common Patterns**:
```typescript
// Container
<div className="h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar">

// Card/Panel
<div className="glass-panel p-6 rounded-2xl border border-white/10">

// Button (Primary)
<button className="bg-phoenix-600 hover:bg-phoenix-500 text-white rounded-lg py-2.5 font-semibold">

// Button (Secondary)
<button className="bg-white/5 hover:bg-white/10 text-gray-200 rounded-lg border border-white/10">

// Input
<input className="w-full bg-void-900 border border-white/10 rounded-lg p-3 text-white outline-none focus:border-phoenix-500">

// Text (Primary)
<span className="text-white font-semibold">

// Text (Secondary)
<span className="text-gray-400 text-sm">
```

### 2. Responsive Design

**Mobile-First Approach**:
```typescript
// Mobile: full width, Desktop: max-width
<div className="w-full lg:max-w-5xl mx-auto p-4 lg:p-8">

// Mobile: column, Desktop: row
<div className="flex flex-col lg:flex-row gap-4">

// Mobile: hidden, Desktop: visible
<div className="hidden lg:block">
```

### 3. Custom Classes

**Defined in `styles.css`**:
- `.glass-panel` - Glass morphism effect
- `.glass-card` - Card with hover effects
- `.gradient-text` - Gradient text effect
- `.custom-scrollbar` - Custom scrollbar styling
- `.semantic-highlight` - Semantic memory highlighting

**Use these classes** - Don't recreate them.

---

## Component Patterns

### 1. Form Components

**Pattern**:
```typescript
const FormComponent = () => {
  const [formData, setFormData] = useState<FormType>(initialValue);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      await phoenixService.method(formData);
      // Success handling
    } catch (e: any) {
      setError(e.message || 'Operation failed');
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {/* Form fields */}
      {error && <div className="text-red-400 text-sm">{error}</div>}
      <button type="submit" disabled={loading}>
        {loading ? 'Processing...' : 'Submit'}
      </button>
    </form>
  );
};
```

### 2. List Components

**Pattern**:
```typescript
const ListComponent = () => {
  const [items, setItems] = useState<ItemType[]>([]);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    const loadItems = async () => {
      try {
        const data = await phoenixService.getItems();
        setItems(data);
      } catch (e) {
        console.error('Failed to load items', e);
      } finally {
        setLoading(false);
      }
    };
    loadItems();
  }, []);
  
  if (loading) {
    return <div className="text-gray-400">Loading...</div>;
  }
  
  return (
    <div className="space-y-2">
      {items.map(item => (
        <div key={item.id} className="glass-card p-4 rounded-lg">
          {/* Item content */}
        </div>
      ))}
    </div>
  );
};
```

### 3. Modal Components

**Pattern**:
```typescript
interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm?: () => void;
  title: string;
  message: string;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, onConfirm, title, message }) => {
  if (!isOpen) return null;
  
  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-void-900 border border-white/10 rounded-2xl p-6 max-w-sm w-full">
        <h3 className="text-xl font-bold text-white mb-2">{title}</h3>
        <p className="text-gray-400 mb-6 text-sm">{message}</p>
        <div className="flex space-x-3 justify-end">
          <button onClick={onClose} className="px-4 py-2 text-gray-400 hover:text-white">
            Cancel
          </button>
          {onConfirm && (
            <button onClick={onConfirm} className="px-4 py-2 bg-phoenix-600 text-white rounded-lg">
              Confirm
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
```

### 4. Status Indicators

**Pattern**:
```typescript
// Connection status
<div className={`flex items-center gap-2 ${isConnected ? 'text-green-400' : 'text-red-400'}`}>
  <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-400' : 'bg-red-400'}`} />
  <span>{isConnected ? 'Connected' : 'Disconnected'}</span>
</div>

// Loading state
{loading && (
  <div className="flex items-center gap-2 text-gray-400">
    <RefreshCw size={16} className="animate-spin" />
    <span>Loading...</span>
  </div>
)}
```

---

## API Integration Patterns

### 1. Error Handling

**Always handle errors gracefully**:

```typescript
const handleAction = async () => {
  try {
    setLoading(true);
    setError(null);
    const result = await phoenixService.method();
    // Success handling
  } catch (e: any) {
    setError(e.message || 'Operation failed');
    console.error('Action failed:', e);
  } finally {
    setLoading(false);
  }
};
```

### 2. Loading States

**Always show loading feedback**:

```typescript
const [loading, setLoading] = useState(false);

// In button
<button disabled={loading}>
  {loading ? 'Processing...' : 'Submit'}
</button>

// Or with spinner
{loading && <RefreshCw size={16} className="animate-spin" />}
```

### 3. Response Parsing

**Backend returns JSON strings** - Parse carefully:

```typescript
const response = await phoenixService.sendCommand(command);
let displayContent = response;

try {
  const json = JSON.parse(response);
  if (json.message) {
    displayContent = json.message;
  } else if (json.data) {
    displayContent = "Received structured data";
  }
} catch (e) {
  // Not JSON, use as-is
}
```

### 4. Status Polling

**Use intervals for status updates**:

```typescript
useEffect(() => {
  const checkStatus = async () => {
    try {
      const status = await phoenixService.status();
      setIsConnected(status.status === 'online');
    } catch (e) {
      setIsConnected(false);
    }
  };
  
  checkStatus();
  const interval = setInterval(checkStatus, 5000); // Every 5 seconds
  return () => clearInterval(interval);
}, []);
```

---

## TypeScript Guidelines

### 1. Type Definitions

**Define types at top of file or in separate types file**:

```typescript
// ✅ CORRECT - Clear type definition
interface MemoryItem {
  key: string;
  value: string;
}

interface MemorySearchResponse {
  items: MemoryItem[];
  count: number;
}

// ❌ WRONG - Using 'any'
const handleData = (data: any) => { ... };
```

### 2. Optional Properties

**Use optional properties with `?`**:

```typescript
interface ComponentProps {
  required: string;
  optional?: number;
  withDefault?: boolean; // Default to false if not provided
}
```

### 3. Type Assertions

**Avoid type assertions when possible**:

```typescript
// ✅ CORRECT - Type guard
if (typeof value === 'string') {
  // value is string here
}

// ❌ WRONG - Unsafe assertion
const str = value as string;
```

### 4. Null Safety

**Handle null/undefined explicitly**:

```typescript
// ✅ CORRECT
const name = phoenixName || 'Phoenix';
const archetype = currentArchetype?.name ?? 'Unknown';

// ❌ WRONG
const name = phoenixName; // Could be undefined
```

---

## State Management Patterns

### 1. Local State

**Use `useState` for component-local state**:

```typescript
const [localState, setLocalState] = useState<Type>(initialValue);
```

### 2. Global State

**Use `PhoenixContext` for global state**:

```typescript
const { messages, sendMessage } = useContext(PhoenixContext)!;
```

### 3. Persistent State

**Use `useLocalStorageJsonState` for browser persistence**:

```typescript
const [settings, setSettings] = useLocalStorageJsonState<UiSettings>(
  'phoenix.ui.settings',
  DEFAULT_UI_SETTINGS
);
```

### 4. Derived State

**Use `useMemo` for computed values**:

```typescript
const filteredItems = useMemo(() => {
  return items.filter(item => item.active);
}, [items]);
```

---

## Common Tasks

### 1. Adding a New View

**Steps**:
1. Create view component in `index.tsx`:
```typescript
const NewView = () => {
  // Component implementation
};
```

2. Add to `activeView` type:
```typescript
const [activeView, setActiveView] = useState<
  'chat' | 'memories' | ... | 'newview'
>('chat');
```

3. Add sidebar item:
```typescript
<SidebarItem 
  icon={NewIcon} 
  label="New View" 
  active={activeView === 'newview'} 
  onClick={() => handleNavigation('newview')} 
/>
```

4. Add to render:
```typescript
{activeView === 'newview' && <NewView />}
```

### 2. Adding a New API Endpoint

**Steps**:
1. Add method to `PhoenixBackendService`:
```typescript
async newMethod(param: string): Promise<ResponseType> {
  const res = await fetch(this.url('/api/new-endpoint'), {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ param }),
  });
  if (!res.ok) {
    throw new Error(`Request failed: ${res.status}`);
  }
  return res.json();
}
```

2. Use in component:
```typescript
const result = await phoenixService.newMethod(value);
```

### 3. Adding a New Form Field

**Pattern**:
```typescript
<div className="space-y-2">
  <label className="block text-xs text-gray-400 uppercase font-bold mb-1">
    Field Label
  </label>
  <input
    type="text"
    value={formData.field}
    onChange={(e) => setFormData({ ...formData, field: e.target.value })}
    className="w-full bg-void-900 border border-white/10 rounded-lg p-3 text-white outline-none focus:border-phoenix-500"
    placeholder="Placeholder text"
  />
</div>
```

### 4. Adding Error Display

**Pattern**:
```typescript
{error && (
  <div className="text-red-400 text-xs font-mono bg-red-500/10 border border-red-500/20 rounded-lg p-3">
    {error}
  </div>
)}
```

### 5. Adding Success Message

**Pattern**:
```typescript
{success && (
  <div className="text-green-400 text-xs font-mono bg-green-500/10 border border-green-500/20 rounded-lg p-3">
    Operation successful
  </div>
)}
```

---

## Best Practices

### 1. Component Organization

✅ **DO**:
- Keep components focused and single-purpose
- Extract reusable components
- Use descriptive component names
- Group related functionality

❌ **DON'T**:
- Create monolithic components (>500 lines)
- Duplicate code across components
- Use generic names like `Component` or `View`

### 2. Performance

✅ **DO**:
- Use `useMemo` for expensive computations
- Use `useCallback` for event handlers passed to children
- Lazy load heavy components
- Debounce search inputs

❌ **DON'T**:
- Create unnecessary re-renders
- Fetch data on every render
- Use inline object/array creation in props

### 3. Accessibility

✅ **DO**:
- Use semantic HTML elements
- Add `aria-label` for icon-only buttons
- Ensure keyboard navigation works
- Provide focus indicators

❌ **DON'T**:
- Use `<div>` for buttons (use `<button>`)
- Skip focus management
- Ignore screen reader support

### 4. Error Handling

✅ **DO**:
- Always wrap async operations in try/catch
- Display user-friendly error messages
- Log errors to console for debugging
- Provide fallback UI for errors

❌ **DON'T**:
- Swallow errors silently
- Show raw error messages to users
- Ignore network failures

### 5. Code Style

✅ **DO**:
- Use consistent indentation (2 spaces)
- Use meaningful variable names
- Add comments for complex logic
- Follow existing code patterns

❌ **DON'T**:
- Mix tabs and spaces
- Use abbreviations (e.g., `msg` instead of `message`)
- Leave TODO comments without context
- Break existing patterns

---

## Things to Avoid

### 1. Direct API Calls

❌ **Don't do this**:
```typescript
const res = await fetch('/api/memory/store', {
  method: 'POST',
  body: JSON.stringify({ key, value }),
});
```

✅ **Do this instead**:
```typescript
await phoenixService.memoryStore(key, value);
```

### 2. Bypassing Context

❌ **Don't do this**:
```typescript
const [localMessages, setLocalMessages] = useState<Message[]>([]);
```

✅ **Do this instead**:
```typescript
const { messages } = useContext(PhoenixContext)!;
```

### 3. Inline Styles (except dynamic)

❌ **Don't do this**:
```typescript
<div style={{ backgroundColor: '#0f0b15', padding: '16px' }}>
```

✅ **Do this instead**:
```typescript
<div className="bg-[#0f0b15] p-4">
```

### 4. Any Types

❌ **Don't do this**:
```typescript
const handleData = (data: any) => { ... };
```

✅ **Do this instead**:
```typescript
interface DataType {
  field: string;
}
const handleData = (data: DataType) => { ... };
```

### 5. Missing Error Handling

❌ **Don't do this**:
```typescript
const result = await phoenixService.method();
// No error handling
```

✅ **Do this instead**:
```typescript
try {
  const result = await phoenixService.method();
} catch (e) {
  setError(e.message || 'Operation failed');
}
```

---

## Testing Considerations

### 1. Component Testing

**When creating components, ensure they are testable**:
- Extract logic into separate functions
- Use props for dependencies
- Avoid side effects in render

### 2. Mock Services

**For testing, mock `PhoenixBackendService`**:
```typescript
const mockService = {
  memoryStore: jest.fn().mockResolvedValue(undefined),
  memoryGet: jest.fn().mockResolvedValue({ key: 'test', value: 'value' }),
  // ... other methods
};
```

### 3. Test Data

**Use realistic test data**:
```typescript
const mockMessage: Message = {
  id: 'test-1',
  role: 'user',
  content: 'Test message',
  timestamp: Date.now(),
};
```

---

## Environment Configuration

### Environment Variables

**Frontend-specific (VITE_ prefix)**:
- `VITE_PORT` - Dev server port (default: 3000)
- `VITE_PHOENIX_API_BASE` - Backend URL (default: `http://127.0.0.1:8888`)

**Usage**:
```typescript
const PHOENIX_API_BASE = 
  ((import.meta as any).env?.VITE_PHOENIX_API_BASE as string | undefined)
    ?.replace(/\/$/, '') || '';
```

### Development vs Production

**Development**:
- Vite dev server on port 3000
- Proxy `/api/*` to backend
- Hot Module Replacement (HMR)

**Production**:
- Build to `frontend/dist/`
- Backend serves static files
- No proxy needed

---

## Common Patterns Reference

### 1. Conditional Rendering

```typescript
// Simple condition
{isLoading && <LoadingSpinner />}

// Ternary
{error ? <ErrorMessage error={error} /> : <SuccessMessage />}

// Multiple conditions
{status === 'loading' && <Loading />}
{status === 'error' && <Error />}
{status === 'success' && <Success />}
```

### 2. List Rendering

```typescript
{items.map(item => (
  <div key={item.id} className="...">
    {/* Item content */}
  </div>
))}

// With empty state
{items.length === 0 ? (
  <div className="text-gray-400 text-center py-8">No items found</div>
) : (
  items.map(item => <Item key={item.id} item={item} />)
)}
```

### 3. Event Handlers

```typescript
// Inline
<button onClick={() => handleClick(id)}>Click</button>

// With preventDefault
<form onSubmit={(e) => { e.preventDefault(); handleSubmit(); }}>

// Async handler
const handleAsync = async () => {
  setLoading(true);
  try {
    await phoenixService.method();
  } finally {
    setLoading(false);
  }
};
```

### 4. Input Handling

```typescript
// Controlled input
<input
  value={value}
  onChange={(e) => setValue(e.target.value)}
  onKeyDown={(e) => {
    if (e.key === 'Enter') {
      handleSubmit();
    }
  }}
/>

// Textarea
<textarea
  value={content}
  onChange={(e) => setContent(e.target.value)}
  rows={5}
/>
```

### 5. Modal/Dialog Pattern

```typescript
const [isOpen, setIsOpen] = useState(false);

{isOpen && (
  <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/80">
    <div className="bg-void-900 border border-white/10 rounded-2xl p-6">
      {/* Modal content */}
      <button onClick={() => setIsOpen(false)}>Close</button>
    </div>
  </div>
)}
```

---

## Debugging Tips

### 1. Console Logging

**Use console.log for debugging**:
```typescript
console.log('State:', state);
console.log('Props:', props);
console.error('Error:', error);
```

**Remove console.logs before committing** (or use conditional logging).

### 2. React DevTools

**Use React DevTools to inspect**:
- Component tree
- Props and state
- Hook values
- Performance

### 3. Network Tab

**Check Network tab for API calls**:
- Verify request/response
- Check status codes
- Inspect payloads
- Debug CORS issues

### 4. TypeScript Errors

**Fix TypeScript errors immediately**:
- Use IDE hints
- Check type definitions
- Verify imports
- Ensure null safety

---

## File-Specific Guidelines

### index.tsx

**This is a large file (3341 lines)** - When adding code:

1. **Find the right section**:
   - Types/Interfaces (lines 83-214)
   - Static Data (lines 247-366)
   - Services (lines 368-682)
   - Helper Components (lines 684-836)
   - View Components (lines 837-3150)
   - Layout (lines 3150-3340)

2. **Add to appropriate section**:
   - New types → Types section
   - New view → View Components section
   - New helper → Helper Components section

3. **Maintain organization** - Don't add code randomly.

### devtools.tsx

**Separate file for DevTools view**:
- Keep it focused on system access
- Don't mix with other views
- Follow same patterns as main views

### styles.css

**Global styles only**:
- Custom classes (`.glass-panel`, etc.)
- Animations
- Scrollbar styling
- Don't add component-specific styles here

---

## Quick Reference

### Common Imports

```typescript
import React, { useState, useEffect, useRef, useContext, useMemo } from 'react';
import { IconName } from 'lucide-react';
import { PhoenixContext } from './index'; // If in separate file
```

### Common Class Names

```typescript
// Containers
"h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar"
"max-w-5xl mx-auto p-8"

// Cards/Panels
"glass-panel p-6 rounded-2xl border border-white/10"

// Buttons
"bg-phoenix-600 hover:bg-phoenix-500 text-white rounded-lg py-2.5 font-semibold"
"bg-white/5 hover:bg-white/10 text-gray-200 rounded-lg border border-white/10"

// Inputs
"w-full bg-void-900 border border-white/10 rounded-lg p-3 text-white outline-none focus:border-phoenix-500"

// Text
"text-white font-semibold"
"text-gray-400 text-sm"
"text-phoenix-400"
```

### Common Icons

```typescript
import {
  MessageSquare,  // Chat
  Heart,          // Relationship
  Settings,       // Settings
  Activity,       // Activity/Monitoring
  Brain,          // Memory
  Terminal,       // DevTools
  Network,        // Orchestrator
  Cloud,          // Google
  GitBranch,      // Ecosystem
  Film,           // Studio
  // ... more in index.tsx
} from 'lucide-react';
```

---

## When Adding New Features

### Checklist

1. ✅ **Define Types**: Create TypeScript interfaces
2. ✅ **Add Service Method**: Extend `PhoenixBackendService` if needed
3. ✅ **Create Component**: Follow component patterns
4. ✅ **Add to Navigation**: Update sidebar if needed
5. ✅ **Handle Errors**: Add error handling
6. ✅ **Add Loading States**: Show loading feedback
7. ✅ **Test**: Verify functionality
8. ✅ **Style**: Use Tailwind classes
9. ✅ **Document**: Add comments for complex logic

### Example: Adding a New Feature View

```typescript
// 1. Define types (if needed)
interface FeatureData {
  id: string;
  name: string;
}

// 2. Create view component
const FeatureView = () => {
  const { isConnected } = useContext(PhoenixContext)!;
  const [data, setData] = useState<FeatureData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const phoenixService = useMemo(() => new PhoenixBackendService(), []);
  
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const result = await phoenixService.getFeatureData();
        setData(result);
      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };
    loadData();
  }, []);
  
  return (
    <div className="h-full bg-[#0f0b15] overflow-y-auto custom-scrollbar">
      <div className="max-w-5xl mx-auto p-8">
        <h2 className="text-2xl font-bold text-white mb-6">Feature</h2>
        
        {loading && <div className="text-gray-400">Loading...</div>}
        {error && <div className="text-red-400">{error}</div>}
        
        <div className="space-y-4">
          {data.map(item => (
            <div key={item.id} className="glass-panel p-4 rounded-lg">
              {item.name}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

// 3. Add to activeView type
type ActiveView = 'chat' | 'memories' | ... | 'feature';

// 4. Add sidebar item
<SidebarItem 
  icon={FeatureIcon} 
  label="Feature" 
  active={activeView === 'feature'} 
  onClick={() => handleNavigation('feature')} 
/>

// 5. Add to render
{activeView === 'feature' && <FeatureView />}
```

---

## Troubleshooting

### Common Issues

**1. Backend not responding**
- Check backend is running on port 8888
- Verify `VITE_PHOENIX_API_BASE` is correct
- Check CORS settings

**2. Styles not applying**
- Verify Tailwind classes are correct
- Check `tailwind.config.cjs` includes file paths
- Ensure CSS is imported in `index.tsx`

**3. TypeScript errors**
- Check type definitions
- Verify imports
- Ensure null safety

**4. State not updating**
- Check dependency arrays in `useEffect`
- Verify state setters are called
- Check for stale closures

**5. API calls failing**
- Verify endpoint exists in backend
- Check request format
- Verify authentication if needed

---

## Summary

### Key Principles

1. **Always use `PhoenixBackendService`** for API calls
2. **Always use `PhoenixContext`** for global state
3. **Always use Tailwind classes** for styling
4. **Always handle errors** gracefully
5. **Always show loading states** for async operations
6. **Always use TypeScript types** (no `any`)
7. **Always follow existing patterns** in the codebase

### Code Quality Checklist

Before submitting code, ensure:
- ✅ TypeScript compiles without errors
- ✅ No console.logs (or conditional)
- ✅ Error handling in place
- ✅ Loading states implemented
- ✅ Responsive design considered
- ✅ Accessibility basics covered
- ✅ Follows existing code style
- ✅ No direct API calls (use service)
- ✅ No bypassing context
- ✅ Proper null/undefined handling

---

*These instructions are designed to help IDE agents work effectively with the Phoenix Frontend UI codebase. Follow these patterns for consistent, maintainable code.*

