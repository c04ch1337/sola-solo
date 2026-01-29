# Phoenix Frontend UI - Comprehensive Implementation Plan

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Phase 0: Project Initialization](#phase-0-project-initialization)
4. [Phase 1: Foundation & Configuration](#phase-1-foundation--configuration)
5. [Phase 2: Core Infrastructure](#phase-2-core-infrastructure)
6. [Phase 3: Base Components](#phase-3-base-components)
7. [Phase 4: View Components (Priority Order)](#phase-4-view-components-priority-order)
8. [Phase 5: Integration & Polish](#phase-5-integration--polish)
9. [Phase 6: Testing & Optimization](#phase-6-testing--optimization)
10. [Best Practices](#best-practices)
11. [Tips & Tricks](#tips--tricks)
12. [Troubleshooting Guide](#troubleshooting-guide)

---

## Overview

This document provides a step-by-step implementation plan for recreating the Phoenix Frontend UI. The plan is organized into logical phases, with each phase building upon the previous one. Follow this order to ensure a stable, maintainable codebase.

**Estimated Timeline:**
- Phase 0-1: 2-4 hours (Setup)
- Phase 2: 4-6 hours (Infrastructure)
- Phase 3: 6-8 hours (Base Components)
- Phase 4: 20-30 hours (View Components)
- Phase 5: 8-12 hours (Integration)
- Phase 6: 6-10 hours (Testing)

**Total: 46-70 hours** (depending on experience level)

---

## Prerequisites

### Required Knowledge
- React 18 (Hooks, Context API)
- TypeScript (Interfaces, Types, Generics)
- Tailwind CSS (Utility classes)
- Vite (Build tool)
- REST API integration
- Local Storage API

### Required Tools
- Node.js 18+ and npm/yarn/pnpm
- Code editor (VS Code recommended)
- Git (for version control)
- Backend API running on port 8888 (for testing)

### Environment Setup
```bash
# Verify Node.js version
node --version  # Should be 18+

# Verify npm version
npm --version   # Should be 9+

# Create project directory
mkdir phoenix-frontend
cd phoenix-frontend
```

---

## Phase 0: Project Initialization

### Step 0.1: Initialize Vite Project

```bash
# Create Vite + React + TypeScript project
npm create vite@latest . -- --template react-ts

# Install dependencies
npm install
```

**Verification:**
- ✅ `package.json` exists
- ✅ `vite.config.ts` exists
- ✅ `tsconfig.json` exists
- ✅ `src/` directory created

### Step 0.2: Install Core Dependencies

```bash
# Production dependencies
npm install react@18.2.0 react-dom@18.2.0 lucide-react@0.263.1

# Development dependencies
npm install -D \
  @vitejs/plugin-react@^5.0.0 \
  typescript@~5.8.2 \
  vite@^6.2.0 \
  tailwindcss@^4.1.13 \
  @tailwindcss/postcss@^4.1.13 \
  postcss@^8.5.6 \
  autoprefixer@^10.4.21 \
  @types/node@^22.14.0
```

**Verification:**
```bash
npm list react react-dom lucide-react
# Should show correct versions
```

### Step 0.3: Project Structure Setup

Create the following directory structure:

```
frontend/
├── src/
│   ├── components/        # Reusable components (created later)
│   ├── services/          # API services (created later)
│   ├── contexts/          # React contexts (created later)
│   ├── types/            # TypeScript types (created later)
│   ├── hooks/            # Custom hooks (created later)
│   ├── utils/            # Utility functions (created later)
│   ├── App.tsx           # Main app component
│   ├── main.tsx          # Entry point
│   └── index.css         # Global styles
├── public/               # Static assets
├── index.html            # HTML entry point
├── vite.config.ts        # Vite configuration
├── tsconfig.json         # TypeScript config
├── tailwind.config.cjs   # Tailwind config
├── postcss.config.cjs    # PostCSS config
└── package.json          # Dependencies
```

**Action Items:**
- ✅ Create empty directories
- ✅ Keep existing `src/main.tsx` and `src/App.tsx` (we'll modify them)

---

## Phase 1: Foundation & Configuration

### Step 1.1: Configure TypeScript

**File: `tsconfig.json`**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

**Verification:**
```bash
npx tsc --noEmit
# Should show no errors
```

### Step 1.2: Configure Vite

**File: `vite.config.ts`**

```typescript
import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig(({ mode }) => {
  // Load env from repo root so `VITE_*` can live alongside the Rust `.env`.
  const repoRoot = path.resolve(__dirname, '..');
  const env = loadEnv(mode, repoRoot, '');
  
  return {
    server: {
      port: parseInt(env.VITE_PORT || '3000', 10),
      host: '0.0.0.0',
      proxy: {
        // Local dev: proxy API calls to the Rust backend.
        '/api': {
          target: env.VITE_PHOENIX_API_BASE || 'http://127.0.0.1:8888',
          changeOrigin: true,
        },
        '/health': {
          target: env.VITE_PHOENIX_API_BASE || 'http://127.0.0.1:8888',
          changeOrigin: true,
        },
      },
    },
    plugins: [react()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src'),
      },
    },
  };
});
```

**Verification:**
```bash
npm run dev
# Should start dev server on port 3000
# Check browser console for no errors
```

### Step 1.3: Configure Tailwind CSS

**File: `tailwind.config.cjs`**

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{js,jsx,ts,tsx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        handwriting: ['Caveat', 'cursive'],
      },
      colors: {
        phoenix: {
          50: '#fdf2f8',
          100: '#fce7f3',
          200: '#fbcfe8',
          300: '#f9a8d4',
          400: '#f472b6',
          500: '#ec4899',
          600: '#db2777',
          700: '#be185d',
          800: '#9d174d',
          900: '#831843',
          950: '#500724',
        },
        void: {
          900: '#0f0b15',
          800: '#1a1625',
          700: '#2f2b3a',
        },
      },
      animation: {
        float: 'float 15s infinite linear',
        'heartbeat-slow': 'heartbeat 4s infinite ease-in-out',
        'pop-in': 'popIn 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards',
        'life-pulse': 'lifePulse 3s infinite ease-in-out',
        'subtle-bounce': 'subtleBounce 2s infinite ease-in-out',
        'msg-in-left': 'slideInLeft 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'msg-in-right': 'slideInRight 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards',
      },
      keyframes: {
        float: {
          '0%': { transform: 'translateY(0) rotate(0deg)', opacity: '0' },
          '10%': { opacity: '0.5' },
          '90%': { opacity: '0.5' },
          '100%': { transform: 'translateY(-100px) rotate(20deg)', opacity: '0' },
        },
        heartbeat: {
          '0%, 100%': { opacity: '0.02', transform: 'scale(1)' },
          '50%': { opacity: '0.08', transform: 'scale(1.05)' },
        },
        popIn: {
          '0%': { opacity: '0', transform: 'scale(0.8) translateY(10px)' },
          '100%': { opacity: '1', transform: 'scale(1) translateY(0)' },
        },
        lifePulse: {
          '0%, 100%': { transform: 'scale(1)' },
          '50%': { transform: 'scale(1.015)' },
        },
        subtleBounce: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-3px)' },
        },
        slideInLeft: {
          '0%': { opacity: '0', transform: 'translateX(-20px) scale(0.98)' },
          '100%': { opacity: '1', transform: 'translateX(0) scale(1)' },
        },
        slideInRight: {
          '0%': { opacity: '0', transform: 'translateX(20px) scale(0.98)' },
          '100%': { opacity: '1', transform: 'translateX(0) scale(1)' },
        },
      },
    },
  },
  plugins: [],
};
```

**File: `postcss.config.cjs`**

```javascript
module.exports = {
  plugins: {
    '@tailwindcss/postcss': {},
    autoprefixer: {},
  },
};
```

**Verification:**
- ✅ Tailwind classes work in components
- ✅ Custom colors available (`phoenix-600`, `void-900`, etc.)
- ✅ Animations defined

### Step 1.4: Create Global Styles

**File: `src/index.css`**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-void-900 text-white;
    font-family: 'Inter', sans-serif;
  }
}

@layer components {
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

  .gradient-text {
    background: linear-gradient(135deg, #db2777 0%, #ec4899 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
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

  .semantic-highlight {
    background: rgba(219, 39, 119, 0.1);
    border-left: 3px solid #db2777;
  }
}
```

**Update `src/main.tsx`:**

```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

**Verification:**
- ✅ Styles load correctly
- ✅ Custom classes work
- ✅ Scrollbar styling visible

---

## Phase 2: Core Infrastructure

### Step 2.1: Define TypeScript Types

**File: `src/types/index.ts`**

```typescript
export interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
}

export interface Archetype {
  id: string;
  name: string;
  zodiac: string;
  traits: string[];
  communication_style: string;
  mood_preferences: string[];
}

export interface DatingProfile {
  name: string;
  age: number;
  archetype_id: string;
  preferences: Record<string, any>;
}

export interface MemoryItem {
  key: string;
  value: string;
}

export interface VectorMemoryResult {
  text: string;
  metadata: Record<string, any>;
  score: number;
}

export interface Agent {
  id: string;
  name: string;
  status: 'online' | 'offline' | 'error';
  description: string;
}

export interface RepoMetadata {
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

export interface UiSettings {
  theme: 'dark' | 'light';
  fontSize: 'small' | 'medium' | 'large';
  animations: boolean;
}

export type ActiveView =
  | 'chat'
  | 'memories'
  | 'orchestrator'
  | 'studio'
  | 'google'
  | 'ecosystem'
  | 'archetype'
  | 'devtools'
  | 'settings';
```

**Verification:**
```bash
npx tsc --noEmit
# Should compile without errors
```

### Step 2.2: Create Backend Service

**File: `src/services/PhoenixBackendService.ts`**

```typescript
import type {
  MemoryItem,
  VectorMemoryResult,
  DatingProfile,
  RepoMetadata,
} from '../types';

export class PhoenixBackendService {
  private baseUrl: string;

  constructor() {
    this.baseUrl =
      (import.meta.env?.VITE_PHOENIX_API_BASE as string | undefined)
        ?.replace(/\/$/, '') || 'http://127.0.0.1:8888';
  }

  private url(path: string): string {
    return `${this.baseUrl}${path}`;
  }

  // Memory operations
  async memoryStore(key: string, value: string): Promise<void> {
    const res = await fetch(this.url('/api/memory/store'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ key, value }),
    });
    if (!res.ok) throw new Error(`Memory store failed: ${res.status}`);
  }

  async memoryGet(key: string): Promise<MemoryItem | null> {
    const res = await fetch(this.url(`/api/memory/get/${encodeURIComponent(key)}`));
    if (!res.ok) {
      if (res.status === 404) return null;
      throw new Error(`Memory get failed: ${res.status}`);
    }
    return res.json();
  }

  async memorySearch(query: string): Promise<MemoryItem[]> {
    const res = await fetch(
      this.url(`/api/memory/search?q=${encodeURIComponent(query)}`)
    );
    if (!res.ok) throw new Error(`Memory search failed: ${res.status}`);
    return res.json();
  }

  async memoryDelete(key: string): Promise<void> {
    const res = await fetch(this.url(`/api/memory/delete/${encodeURIComponent(key)}`), {
      method: 'DELETE',
    });
    if (!res.ok) throw new Error(`Memory delete failed: ${res.status}`);
  }

  async vectorMemoryStore(text: string, metadata: Record<string, any>): Promise<void> {
    const res = await fetch(this.url('/api/memory/vector/store'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text, metadata }),
    });
    if (!res.ok) throw new Error(`Vector memory store failed: ${res.status}`);
  }

  async vectorMemorySearch(query: string, k: number = 10): Promise<VectorMemoryResult[]> {
    const res = await fetch(
      this.url(`/api/memory/vector/search?q=${encodeURIComponent(query)}&k=${k}`)
    );
    if (!res.ok) throw new Error(`Vector memory search failed: ${res.status}`);
    return res.json();
  }

  async vectorMemoryAll(): Promise<VectorMemoryResult[]> {
    const res = await fetch(this.url('/api/memory/vector/all'));
    if (!res.ok) throw new Error(`Vector memory all failed: ${res.status}`);
    return res.json();
  }

  // Command and communication
  async sendCommand(command: string): Promise<string> {
    const res = await fetch(this.url('/api/command'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ command }),
    });
    if (!res.ok) throw new Error(`Command failed: ${res.status}`);
    return res.text();
  }

  async getPhoenixName(): Promise<string> {
    const res = await fetch(this.url('/api/name'));
    if (!res.ok) throw new Error(`Get name failed: ${res.status}`);
    return res.text();
  }

  async status(): Promise<{ status: string; version?: string }> {
    const res = await fetch(this.url('/api/status'));
    if (!res.ok) throw new Error(`Status check failed: ${res.status}`);
    return res.json();
  }

  // Archetype operations
  async applyArchetype(id: string, profile: DatingProfile): Promise<void> {
    const res = await fetch(this.url('/api/archetype/apply'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id, profile }),
    });
    if (!res.ok) throw new Error(`Apply archetype failed: ${res.status}`);
  }

  // System operations
  async setKeylogger(enabled: boolean, path: string): Promise<void> {
    const res = await fetch(this.url('/api/system/keylogger'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ enabled, path }),
    });
    if (!res.ok) throw new Error(`Set keylogger failed: ${res.status}`);
  }

  async setMouseJigger(enabled: boolean): Promise<void> {
    const res = await fetch(this.url('/api/system/mouse-jigger'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ enabled }),
    });
    if (!res.ok) throw new Error(`Set mouse jigger failed: ${res.status}`);
  }

  // Ecosystem operations
  async ecosystemImport(owner: string, repo: string, branch?: string): Promise<RepoMetadata> {
    const res = await fetch(this.url('/api/ecosystem/import'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ owner, repo, branch }),
    });
    if (!res.ok) throw new Error(`Ecosystem import failed: ${res.status}`);
    return res.json();
  }

  async ecosystemList(): Promise<RepoMetadata[]> {
    const res = await fetch(this.url('/api/ecosystem/list'));
    if (!res.ok) throw new Error(`Ecosystem list failed: ${res.status}`);
    return res.json();
  }

  async ecosystemBuild(id: string): Promise<void> {
    const res = await fetch(this.url(`/api/ecosystem/${id}/build`), {
      method: 'POST',
    });
    if (!res.ok) throw new Error(`Ecosystem build failed: ${res.status}`);
  }

  async ecosystemStart(id: string): Promise<void> {
    const res = await fetch(this.url(`/api/ecosystem/${id}/start`), {
      method: 'POST',
    });
    if (!res.ok) throw new Error(`Ecosystem start failed: ${res.status}`);
  }

  async ecosystemStop(id: string): Promise<void> {
    const res = await fetch(this.url(`/api/ecosystem/${id}/stop`), {
      method: 'POST',
    });
    if (!res.ok) throw new Error(`Ecosystem stop failed: ${res.status}`);
  }

  async ecosystemDelete(id: string): Promise<void> {
    const res = await fetch(this.url(`/api/ecosystem/${id}`), {
      method: 'DELETE',
    });
    if (!res.ok) throw new Error(`Ecosystem delete failed: ${res.status}`);
  }

  // System access (DevTools)
  async systemExecute(command: string): Promise<string> {
    const res = await fetch(this.url('/api/system/execute'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ command }),
    });
    if (!res.ok) throw new Error(`System execute failed: ${res.status}`);
    return res.text();
  }

  async systemReadFile(path: string): Promise<string> {
    const res = await fetch(
      this.url(`/api/system/read?path=${encodeURIComponent(path)}`)
    );
    if (!res.ok) throw new Error(`System read file failed: ${res.status}`);
    return res.text();
  }

  async systemWriteFile(path: string, content: string): Promise<void> {
    const res = await fetch(this.url('/api/system/write'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ path, content }),
    });
    if (!res.ok) throw new Error(`System write file failed: ${res.status}`);
  }

  async systemStatus(): Promise<{ full_access_granted: boolean; self_modification_enabled: boolean }> {
    const res = await fetch(this.url('/api/system/status'));
    if (!res.ok) throw new Error(`System status failed: ${res.status}`);
    return res.json();
  }
}
```

**Verification:**
- ✅ Service class compiles
- ✅ All methods have proper types
- ✅ Error handling in place

### Step 2.3: Create Custom Hooks

**File: `src/hooks/useLocalStorage.ts`**

```typescript
import { useState, useEffect } from 'react';

export function useLocalStorage<T>(key: string, defaultValue: T): [T, (value: T) => void] {
  const [state, setState] = useState<T>(() => {
    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : defaultValue;
    } catch {
      return defaultValue;
    }
  });

  useEffect(() => {
    try {
      window.localStorage.setItem(key, JSON.stringify(state));
    } catch (e) {
      console.error('Failed to save to localStorage', e);
    }
  }, [key, state]);

  return [state, setState];
}
```

**File: `src/hooks/usePhoenixService.ts`**

```typescript
import { useMemo } from 'react';
import { PhoenixBackendService } from '../services/PhoenixBackendService';

export function usePhoenixService(): PhoenixBackendService {
  return useMemo(() => new PhoenixBackendService(), []);
}
```

**Verification:**
- ✅ Hooks compile
- ✅ Can be imported in components

### Step 2.4: Create Phoenix Context

**File: `src/contexts/PhoenixContext.tsx`**

```typescript
import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import type { Message, DatingProfile, Archetype } from '../types';
import { PhoenixBackendService } from '../services/PhoenixBackendService';

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

const PhoenixContext = createContext<PhoenixContextType | null>(null);

export function PhoenixProvider({ children }: { children: React.ReactNode }) {
  const [isConnected, setIsConnected] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const [phoenixName, setPhoenixName] = useState('Phoenix');
  const [relationalScore, setRelationalScore] = useState(0);
  const [sentiment, setSentiment] = useState<'positive' | 'negative' | 'neutral'>('neutral');
  const [currentArchetype, setCurrentArchetype] = useState<Archetype | null>(null);
  
  const service = useMemo(() => new PhoenixBackendService(), []);

  // Status polling
  useEffect(() => {
    const checkStatus = async () => {
      try {
        const status = await service.status();
        setIsConnected(status.status === 'online');
      } catch {
        setIsConnected(false);
      }
    };

    checkStatus();
    const interval = setInterval(checkStatus, 5000);
    return () => clearInterval(interval);
  }, [service]);

  // Load Phoenix name
  useEffect(() => {
    service.getPhoenixName().then(setPhoenixName).catch(console.error);
  }, [service]);

  const sendMessage = useCallback(async (text: string) => {
    const userMessage: Message = {
      id: `msg-${Date.now()}-user`,
      role: 'user',
      content: text,
      timestamp: Date.now(),
    };
    setMessages((prev) => [...prev, userMessage]);

    try {
      const response = await service.sendCommand(text);
      const assistantMessage: Message = {
        id: `msg-${Date.now()}-assistant`,
        role: 'assistant',
        content: response,
        timestamp: Date.now(),
      };
      setMessages((prev) => [...prev, assistantMessage]);
    } catch (e: any) {
      const errorMessage: Message = {
        id: `msg-${Date.now()}-error`,
        role: 'system',
        content: `Error: ${e.message || 'Failed to send message'}`,
        timestamp: Date.now(),
      };
      setMessages((prev) => [...prev, errorMessage]);
    }
  }, [service]);

  const runCommand = useCallback(async (text: string): Promise<string> => {
    return service.sendCommand(text);
  }, [service]);

  const applyArchetype = useCallback(async (id: string, profile: DatingProfile) => {
    await service.applyArchetype(id, profile);
    // Load archetype data (you'll implement this based on your archetype source)
  }, [service]);

  const clearHistory = useCallback(() => {
    setMessages([]);
  }, []);

  const deleteMessage = useCallback((id: string) => {
    setMessages((prev) => prev.filter((msg) => msg.id !== id));
  }, []);

  const handleSetKeylogger = useCallback(async (enabled: boolean, path: string) => {
    await service.setKeylogger(enabled, path);
  }, [service]);

  const handleSetMouseJigger = useCallback(async (enabled: boolean) => {
    await service.setMouseJigger(enabled);
  }, [service]);

  return (
    <PhoenixContext.Provider
      value={{
        isConnected,
        messages,
        sendMessage,
        runCommand,
        applyArchetype,
        currentArchetype,
        clearHistory,
        deleteMessage,
        relationalScore,
        sentiment,
        setRelationalScore,
        setSentiment,
        phoenixName,
        setKeylogger: handleSetKeylogger,
        setMouseJigger: handleSetMouseJigger,
      }}
    >
      {children}
    </PhoenixContext.Provider>
  );
}

export function usePhoenix() {
  const context = useContext(PhoenixContext);
  if (!context) {
    throw new Error('usePhoenix must be used within PhoenixProvider');
  }
  return context;
}
```

**Verification:**
- ✅ Context compiles
- ✅ Can wrap App component
- ✅ Hook works in components

---

## Phase 3: Base Components

### Step 3.1: Create Layout Components

**File: `src/components/Layout/DashboardLayout.tsx`**

```typescript
import React, { useState } from 'react';
import { Sidebar } from './Sidebar';
import type { ActiveView } from '../../types';

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export function DashboardLayout({ children }: DashboardLayoutProps) {
  const [activeView, setActiveView] = useState<ActiveView>('chat');

  return (
    <div className="flex h-screen bg-void-900 text-white overflow-hidden">
      <Sidebar activeView={activeView} onNavigate={setActiveView} />
      <main className="flex-1 overflow-hidden">{children}</main>
    </div>
  );
}
```

**File: `src/components/Layout/Sidebar.tsx`**

```typescript
import React from 'react';
import {
  MessageSquare,
  Brain,
  Network,
  Film,
  Cloud,
  GitBranch,
  Heart,
  Terminal,
  Settings,
  Trash2,
} from 'lucide-react';
import type { ActiveView } from '../../types';

interface SidebarProps {
  activeView: ActiveView;
  onNavigate: (view: ActiveView) => void;
}

interface SidebarItemProps {
  icon: React.ComponentType<{ size?: number; className?: string }>;
  label: string;
  active: boolean;
  onClick: () => void;
}

function SidebarItem({ icon: Icon, label, active, onClick }: SidebarItemProps) {
  return (
    <button
      onClick={onClick}
      className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
        active
          ? 'bg-phoenix-600 text-white'
          : 'text-gray-400 hover:bg-white/5 hover:text-white'
      }`}
    >
      <Icon size={20} />
      <span className="font-medium">{label}</span>
    </button>
  );
}

export function Sidebar({ activeView, onNavigate }: SidebarProps) {
  return (
    <aside className="w-64 bg-void-800 border-r border-white/10 flex flex-col">
      <div className="p-6 border-b border-white/10">
        <h1 className="text-2xl font-bold gradient-text">Phoenix</h1>
      </div>
      
      <nav className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-2">
        <div className="space-y-1">
          <p className="text-xs text-gray-500 uppercase font-bold px-4 py-2">Dashboard</p>
          <SidebarItem
            icon={MessageSquare}
            label="Chat Stream"
            active={activeView === 'chat'}
            onClick={() => onNavigate('chat')}
          />
          <SidebarItem
            icon={Film}
            label="Studio & Recording"
            active={activeView === 'studio'}
            onClick={() => onNavigate('studio')}
          />
          <SidebarItem
            icon={Network}
            label="Orchestrator"
            active={activeView === 'orchestrator'}
            onClick={() => onNavigate('orchestrator')}
          />
          <SidebarItem
            icon={Cloud}
            label="Google Ecosystem"
            active={activeView === 'google'}
            onClick={() => onNavigate('google')}
          />
          <SidebarItem
            icon={GitBranch}
            label="EcoSystem"
            active={activeView === 'ecosystem'}
            onClick={() => onNavigate('ecosystem')}
          />
          <SidebarItem
            icon={Heart}
            label="Archetype Matcher"
            active={activeView === 'archetype'}
            onClick={() => onNavigate('archetype')}
          />
          <SidebarItem
            icon={Brain}
            label="Memories & Context"
            active={activeView === 'memories'}
            onClick={() => onNavigate('memories')}
          />
        </div>
        
        <div className="space-y-1 mt-8">
          <p className="text-xs text-gray-500 uppercase font-bold px-4 py-2">System</p>
          <SidebarItem
            icon={Trash2}
            label="Clear Memory"
            active={false}
            onClick={() => {
              if (confirm('Clear all messages?')) {
                // Implement clear
              }
            }}
          />
          <SidebarItem
            icon={Terminal}
            label="Self-Mod Console"
            active={activeView === 'devtools'}
            onClick={() => onNavigate('devtools')}
          />
          <SidebarItem
            icon={Settings}
            label="Settings"
            active={activeView === 'settings'}
            onClick={() => onNavigate('settings')}
          />
        </div>
      </nav>
    </aside>
  );
}
```

**Verification:**
- ✅ Layout renders
- ✅ Sidebar navigation works
- ✅ Active state highlights correctly

### Step 3.2: Create Reusable UI Components

**File: `src/components/UI/Button.tsx`**

```typescript
import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'danger';
  children: React.ReactNode;
}

export function Button({ variant = 'primary', children, className = '', ...props }: ButtonProps) {
  const baseClasses = 'px-4 py-2.5 rounded-lg font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed';
  const variantClasses = {
    primary: 'bg-phoenix-600 hover:bg-phoenix-500 text-white',
    secondary: 'bg-white/5 hover:bg-white/10 text-gray-200 border border-white/10',
    danger: 'bg-red-600 hover:bg-red-500 text-white',
  };

  return (
    <button
      className={`${baseClasses} ${variantClasses[variant]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
}
```

**File: `src/components/UI/Input.tsx`**

```typescript
import React from 'react';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

export function Input({ label, className = '', ...props }: InputProps) {
  return (
    <div className="space-y-2">
      {label && (
        <label className="block text-xs text-gray-400 uppercase font-bold">
          {label}
        </label>
      )}
      <input
        className={`w-full bg-void-900 border border-white/10 rounded-lg p-3 text-white outline-none focus:border-phoenix-500 ${className}`}
        {...props}
      />
    </div>
  );
}
```

**File: `src/components/UI/Card.tsx`**

```typescript
import React from 'react';

interface CardProps {
  children: React.ReactNode;
  className?: string;
}

export function Card({ children, className = '' }: CardProps) {
  return (
    <div className={`glass-panel p-6 rounded-2xl border border-white/10 ${className}`}>
      {children}
    </div>
  );
}
```

**Verification:**
- ✅ Components render correctly
- ✅ Styling matches design
- ✅ Props work as expected

---

## Phase 4: View Components (Priority Order)

### Priority 1: ChatView (Most Important)

**File: `src/views/ChatView.tsx`**

```typescript
import React, { useState, useRef, useEffect } from 'react';
import { Send, RefreshCw } from 'lucide-react';
import { usePhoenix } from '../contexts/PhoenixContext';
import { Button } from '../components/UI/Button';
import { Input } from '../components/UI/Input';

export function ChatView() {
  const { isConnected, messages, sendMessage } = usePhoenix();
  const [input, setInput] = useState('');
  const [sending, setSending] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || sending) return;
    
    setSending(true);
    try {
      await sendMessage(input);
      setInput('');
    } catch (e) {
      console.error('Failed to send message', e);
    } finally {
      setSending(false);
    }
  };

  return (
    <div className="h-full bg-void-900 flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-white/10 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-400' : 'bg-red-400'}`} />
          <span className="text-sm text-gray-400">
            {isConnected ? 'Connected' : 'Disconnected'}
          </span>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-4">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-3xl rounded-lg p-4 ${
                msg.role === 'user'
                  ? 'bg-phoenix-600 text-white msg-in-right'
                  : msg.role === 'assistant'
                  ? 'bg-white/5 text-gray-200 msg-in-left'
                  : 'bg-red-500/20 text-red-400'
              }`}
            >
              <p className="whitespace-pre-wrap">{msg.content}</p>
              <p className="text-xs opacity-70 mt-2">
                {new Date(msg.timestamp).toLocaleTimeString()}
              </p>
            </div>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="p-4 border-t border-white/10">
        <div className="flex gap-2">
          <Input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSend();
              }
            }}
            placeholder="Type your message..."
            disabled={!isConnected || sending}
            className="flex-1"
          />
          <Button
            onClick={handleSend}
            disabled={!isConnected || sending || !input.trim()}
          >
            {sending ? <RefreshCw size={20} className="animate-spin" /> : <Send size={20} />}
          </Button>
        </div>
      </div>
    </div>
  );
}
```

**Verification:**
- ✅ Messages display correctly
- ✅ Send functionality works
- ✅ Auto-scroll works
- ✅ Connection status visible

### Priority 2: MemoriesView

**File: `src/views/MemoriesView.tsx`**

```typescript
import React, { useState, useEffect } from 'react';
import { Search, Plus, Trash2 } from 'lucide-react';
import { usePhoenixService } from '../hooks/usePhoenixService';
import { Button } from '../components/UI/Button';
import { Input } from '../components/UI/Input';
import { Card } from '../components/UI/Card';
import type { MemoryItem, VectorMemoryResult } from '../types';

type MemoryTab = 'episodic' | 'semantic' | 'vector';

export function MemoriesView() {
  const [activeTab, setActiveTab] = useState<MemoryTab>('episodic');
  const [episodicMemories, setEpisodicMemories] = useState<MemoryItem[]>([]);
  const [vectorMemories, setVectorMemories] = useState<VectorMemoryResult[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const service = usePhoenixService();

  useEffect(() => {
    loadMemories();
  }, [activeTab]);

  const loadMemories = async () => {
    setLoading(true);
    try {
      if (activeTab === 'vector') {
        const memories = await service.vectorMemoryAll();
        setVectorMemories(memories);
      }
      // Episodic and semantic loaded via search
    } catch (e) {
      console.error('Failed to load memories', e);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;
    setLoading(true);
    try {
      if (activeTab === 'semantic' || activeTab === 'episodic') {
        const results = await service.memorySearch(searchQuery);
        setEpisodicMemories(results);
      } else if (activeTab === 'vector') {
        const results = await service.vectorMemorySearch(searchQuery);
        setVectorMemories(results);
      }
    } catch (e) {
      console.error('Search failed', e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="h-full bg-void-900 overflow-y-auto custom-scrollbar">
      <div className="max-w-5xl mx-auto p-8">
        <h2 className="text-2xl font-bold text-white mb-6">Memories & Context</h2>

        {/* Tabs */}
        <div className="flex gap-2 mb-6 border-b border-white/10">
          {(['episodic', 'semantic', 'vector'] as MemoryTab[]).map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 font-semibold capitalize transition-colors ${
                activeTab === tab
                  ? 'text-phoenix-400 border-b-2 border-phoenix-400'
                  : 'text-gray-400 hover:text-white'
              }`}
            >
              {tab}
            </button>
          ))}
        </div>

        {/* Search */}
        <div className="flex gap-2 mb-6">
          <Input
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
            placeholder="Search memories..."
            className="flex-1"
          />
          <Button onClick={handleSearch}>
            <Search size={20} />
          </Button>
        </div>

        {/* Memory List */}
        {loading ? (
          <div className="text-gray-400 text-center py-8">Loading...</div>
        ) : activeTab === 'vector' ? (
          <div className="space-y-4">
            {vectorMemories.map((mem, idx) => (
              <Card key={idx}>
                <div className="flex justify-between items-start mb-2">
                  <p className="text-white">{mem.text}</p>
                  <span className="text-xs text-gray-400">Score: {mem.score.toFixed(3)}</span>
                </div>
                {Object.keys(mem.metadata).length > 0 && (
                  <div className="text-xs text-gray-400 mt-2">
                    {JSON.stringify(mem.metadata, null, 2)}
                  </div>
                )}
              </Card>
            ))}
          </div>
        ) : (
          <div className="space-y-4">
            {episodicMemories.map((mem) => (
              <Card key={mem.key}>
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <p className="font-semibold text-white mb-1">{mem.key}</p>
                    <p className="text-gray-300 text-sm">{mem.value}</p>
                  </div>
                  <Button
                    variant="danger"
                    onClick={async () => {
                      await service.memoryDelete(mem.key);
                      loadMemories();
                    }}
                  >
                    <Trash2 size={16} />
                  </Button>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
```

**Continue with remaining views in priority order:**
3. SettingsView (Simple, needed early)
4. DevToolsView (System access)
5. OrchestratorView (Agent management)
6. EcoSystemView (Repository management)
7. GoogleEcosystemView (Google integration)
8. StudioView (Recording)
9. DatingProfileMatcher (Archetype matching)

**Implementation Pattern for Each View:**
1. Create component file
2. Import necessary hooks/services
3. Set up state management
4. Implement UI layout
5. Add API integration
6. Add error handling
7. Add loading states
8. Test functionality

---

## Phase 5: Integration & Polish

### Step 5.1: Wire Up App Component

**File: `src/App.tsx`**

```typescript
import React, { useState } from 'react';
import { PhoenixProvider } from './contexts/PhoenixContext';
import { DashboardLayout } from './components/Layout/DashboardLayout';
import { ChatView } from './views/ChatView';
import { MemoriesView } from './views/MemoriesView';
import { SettingsView } from './views/SettingsView';
// Import other views...
import type { ActiveView } from './types';

function AppContent() {
  const [activeView, setActiveView] = useState<ActiveView>('chat');

  const renderView = () => {
    switch (activeView) {
      case 'chat':
        return <ChatView />;
      case 'memories':
        return <MemoriesView />;
      case 'settings':
        return <SettingsView />;
      // Add other cases...
      default:
        return <ChatView />;
    }
  };

  return (
    <DashboardLayout>
      {renderView()}
    </DashboardLayout>
  );
}

function App() {
  return (
    <PhoenixProvider>
      <AppContent />
    </PhoenixProvider>
  );
}

export default App;
```

### Step 5.2: Add Static Data

**File: `src/data/archetypes.ts`**

```typescript
import type { Archetype } from '../types';

export const ARCHETYPES_DB: Archetype[] = [
  {
    id: 'aries',
    name: 'Aries',
    zodiac: 'Aries',
    traits: ['bold', 'adventurous', 'passionate'],
    communication_style: 'Direct and energetic',
    mood_preferences: ['excitement', 'challenge', 'action'],
  },
  // Add all 12 zodiac signs...
];
```

### Step 5.3: Add Error Boundaries

**File: `src/components/ErrorBoundary.tsx`**

```typescript
import React, { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="h-screen flex items-center justify-center bg-void-900">
          <div className="text-center">
            <h2 className="text-2xl font-bold text-red-400 mb-4">Something went wrong</h2>
            <p className="text-gray-400">{this.state.error?.message}</p>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### Step 5.4: Add Loading States

Create a reusable `LoadingSpinner` component and use it throughout.

### Step 5.5: Add Toast Notifications

Create a toast system for user feedback.

---

## Phase 6: Testing & Optimization

### Step 6.1: Component Testing

Test each view component:
- ✅ Renders without errors
- ✅ Handles loading states
- ✅ Handles error states
- ✅ API calls work correctly
- ✅ User interactions work

### Step 6.2: Performance Optimization

- Use `React.memo` for expensive components
- Use `useMemo` for computed values
- Use `useCallback` for event handlers
- Lazy load heavy components
- Optimize bundle size

### Step 6.3: Accessibility Audit

- ✅ Keyboard navigation works
- ✅ Screen reader compatible
- ✅ ARIA labels on icon buttons
- ✅ Focus indicators visible
- ✅ Color contrast meets WCAG AA

### Step 6.4: Browser Testing

Test in:
- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers

### Step 6.5: Production Build

```bash
npm run build
npm run preview
```

Verify:
- ✅ Build succeeds
- ✅ No console errors
- ✅ All features work
- ✅ Assets load correctly

---

## Best Practices

### Code Organization

1. **File Structure**
   - One component per file
   - Group related files in directories
   - Use index files for clean imports

2. **Naming Conventions**
   - Components: PascalCase (`ChatView.tsx`)
   - Hooks: camelCase starting with `use` (`usePhoenix.ts`)
   - Types: PascalCase (`Message`, `ActiveView`)
   - Files: Match export name

3. **Component Size**
   - Keep components under 300 lines
   - Extract sub-components when needed
   - Use composition over large components

### TypeScript

1. **Type Safety**
   - No `any` types
   - Define interfaces for all data structures
   - Use type guards for runtime checks

2. **Imports**
   - Use type imports: `import type { Message } from './types'`
   - Group imports: React, third-party, local, types

### React Patterns

1. **Hooks**
   - Use custom hooks for reusable logic
   - Follow Rules of Hooks
   - Memoize expensive computations

2. **State Management**
   - Local state for component-specific data
   - Context for global state
   - Avoid prop drilling

3. **Performance**
   - Memoize callbacks passed to children
   - Use `React.memo` for pure components
   - Lazy load routes/views

### API Integration

1. **Error Handling**
   - Always wrap async operations in try-catch
   - Show user-friendly error messages
   - Log errors for debugging

2. **Loading States**
   - Show loading indicators for all async operations
   - Disable buttons during operations
   - Prevent duplicate requests

3. **Service Abstraction**
   - All API calls through service class
   - No direct fetch calls in components
   - Centralized error handling

### Styling

1. **Tailwind CSS**
   - Use utility classes
   - Create custom classes for repeated patterns
   - Use design tokens (colors, spacing)

2. **Responsive Design**
   - Mobile-first approach
   - Test on multiple screen sizes
   - Use Tailwind responsive classes

3. **Accessibility**
   - Semantic HTML
   - ARIA labels where needed
   - Keyboard navigation support

---

## Tips & Tricks

### Development

1. **Hot Reload**
   - Vite provides instant HMR
   - Changes reflect immediately
   - Use React DevTools for debugging

2. **TypeScript Errors**
   - Fix errors immediately
   - Use `// @ts-ignore` sparingly
   - Prefer type assertions over ignores

3. **Console Logging**
   - Use `console.log` for debugging
   - Remove before committing
   - Use conditional logging in production

### Performance

1. **Bundle Size**
   - Use dynamic imports for large dependencies
   - Tree-shake unused code
   - Analyze bundle with `vite-bundle-visualizer`

2. **Rendering**
   - Avoid unnecessary re-renders
   - Use `React.memo` strategically
   - Profile with React DevTools Profiler

### Debugging

1. **React DevTools**
   - Inspect component tree
   - Check props and state
   - Profile performance

2. **Network Tab**
   - Monitor API calls
   - Check request/response
   - Debug CORS issues

3. **Console**
   - Use breakpoints
   - Inspect variables
   - Check error stack traces

### Common Patterns

1. **Form Handling**
   ```typescript
   const [formData, setFormData] = useState({});
   const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
     setFormData({ ...formData, [e.target.name]: e.target.value });
   };
   ```

2. **Debouncing**
   ```typescript
   const [debouncedValue, setDebouncedValue] = useState('');
   useEffect(() => {
     const timer = setTimeout(() => setDebouncedValue(value), 500);
     return () => clearTimeout(timer);
   }, [value]);
   ```

3. **Conditional Rendering**
   ```typescript
   {loading && <LoadingSpinner />}
   {error && <ErrorMessage error={error} />}
   {data && <DataDisplay data={data} />}
   ```

---

## Troubleshooting Guide

### Common Issues

**1. Vite Dev Server Won't Start**
- Check port 3000 is available
- Verify Node.js version (18+)
- Clear `node_modules` and reinstall

**2. Tailwind Classes Not Working**
- Verify `tailwind.config.cjs` content paths
- Check `postcss.config.cjs` exists
- Restart dev server

**3. API Calls Failing**
- Verify backend is running on port 8888
- Check CORS settings
- Verify proxy configuration in `vite.config.ts`

**4. TypeScript Errors**
- Run `npx tsc --noEmit` to see all errors
- Check `tsconfig.json` settings
- Verify type definitions

**5. Build Fails**
- Check for TypeScript errors
- Verify all imports are correct
- Check for missing dependencies

**6. Styles Not Applying**
- Verify CSS import in `main.tsx`
- Check Tailwind directives in `index.css`
- Clear browser cache

**7. Context Not Working**
- Verify `PhoenixProvider` wraps app
- Check hook usage (must be inside provider)
- Verify context value is not null

**8. Local Storage Issues**
- Check browser console for errors
- Verify localStorage is available
- Handle quota exceeded errors

---

## Conclusion

This implementation plan provides a comprehensive, step-by-step guide to recreating the Phoenix Frontend UI. Follow the phases in order, verify each step before moving to the next, and refer to the best practices and troubleshooting guide as needed.

**Key Success Factors:**
- ✅ Follow the phase order
- ✅ Verify each step before proceeding
- ✅ Test frequently
- ✅ Use TypeScript strictly
- ✅ Follow React best practices
- ✅ Handle errors gracefully
- ✅ Optimize for performance

**Estimated Completion Time:** 46-70 hours (depending on experience)

**Next Steps After Completion:**
1. Deploy to production
2. Set up CI/CD pipeline
3. Add automated testing
4. Monitor performance
5. Gather user feedback
6. Iterate and improve

Good luck with your implementation! 🚀

