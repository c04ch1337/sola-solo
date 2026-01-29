# Phoenix Desktop Frontend

Desktop frontend interface for Phoenix AGI Orchestrator.

## Overview

This is a React + TypeScript frontend that connects to the Phoenix Orchestrator backend. It provides a chat-first interface for interacting with Phoenix AGI, managing projects, scheduling tasks, and configuring system settings.

## Prerequisites

- Node.js 18+ 
- Phoenix backend running on port 8888 (default)

## Setup

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Configure backend URL** (optional):
   Create a `.env` file:
   ```bash
   VITE_PHOENIX_API_URL=http://localhost:8888
   ```

3. **Start Phoenix backend** (in another terminal):
   ```bash
   cd ..
   cargo run -p phoenix-web
   ```

4. **Run the frontend**:
   ```bash
   npm run dev
   ```

5. **Access the application**:
   Open `http://localhost:3000` in your browser

## Features

- **Chat Interface**: Primary interaction with Phoenix AGI
- **Project Management**: Organize conversations by project context
- **Command Execution**: Run system commands through Phoenix
- **Scheduler**: Schedule recurring tasks and orchestration
- **Settings Panel**: Configure Phoenix personality, UI, and integrations
- **Chat History**: Persistent conversation history per project

## Integration

This frontend is fully integrated with the Phoenix Orchestrator backend. See [INTEGRATION.md](./INTEGRATION.md) for detailed integration documentation.

## Development

- **Dev Server**: `npm run dev` (runs on port 3000)
- **Build**: `npm run build`
- **Preview**: `npm run preview`

## API Endpoints

The frontend communicates with Phoenix backend via:
- `/api/speak` - Chat messages
- `/api/command` - Command execution
- `/api/status` - System status
- `/health` - Health check
