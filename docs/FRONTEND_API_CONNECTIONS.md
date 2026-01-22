# Frontend API & Service Connections Guide

**For Frontend Designers & Developers**

This document provides a complete reference for connecting the frontend to all backend services, APIs, and LLM integrations in Phoenix AGI OS v2.4.0.

---

## Table of Contents

1. [Backend Service Overview](#backend-service-overview)
2. [REST API Endpoints](#rest-api-endpoints)
3. [LLM Integration (OpenRouter)](#llm-integration-openrouter)
4. [WebSocket Connections](#websocket-connections)
5. [Internal Service Connections](#internal-service-connections)
6. [Authentication & Configuration](#authentication--configuration)
7. [Request/Response Formats](#requestresponse-formats)
8. [Error Handling](#error-handling)
9. [Connection Examples](#connection-examples)

---

## Backend Service Overview

### Main Backend Service: `phoenix-web`

- **Service Name**: Phoenix Web API Server
- **Binary**: `pagi-sola-web`
- **Default Port**: `8888`
- **Protocol**: HTTP/HTTPS
- **Base URL**: `http://127.0.0.1:8888` (configurable via `PHOENIX_WEB_BIND`)
- **CORS**: Enabled for `http://localhost:3000` and `http://127.0.0.1:3000`

### Frontend Development Server

- **Service**: Vite Dev Server
- **Default Port**: `3000`
- **Protocol**: HTTP
- **Proxy Configuration**: Automatically proxies `/api/*` to backend
- **Base URL**: `http://localhost:3000` (dev) or served from `phoenix-web` (production)

---

## REST API Endpoints

All API endpoints are prefixed with `/api/` and return JSON responses.

### Health & Status

#### `GET /health`
Health check endpoint.

**Response:**
```json
{
  "status": "ok"
}
```

#### `GET /api/status`
Get system status and configuration.

**Response:**
```json
{
  "status": "online",
  "llm_status": "online|offline",
  "version": "0.1.0",
  "archetype": "ZodiacSign",
  "dotenv_path": "/path/to/.env",
  "dotenv_error": null,
  "cwd": "/current/working/directory",
  "openrouter_api_key_set": true
}
```

#### `GET /api/name`
Get Phoenix's display name.

**Response:**
```json
{
  "name": "Sola"
}
```

---

### Configuration

#### `GET /api/config`
Get current configuration.

**Response:**
```json
{
  "openrouter_api_key_set": true,
  "user_name": "John",
  "user_preferred_alias": "Dad"
}
```

#### `POST /api/config`
Update configuration.

**Request:**
```json
{
  "openrouter_api_key": "sk-or-v1-...",
  "user_name": "John",
  "user_preferred_alias": "Dad"
}
```

**Response:**
```json
{
  "status": "ok",
  "openrouter_api_key_set": true,
  "user_name": "John",
  "user_preferred_alias": "Dad",
  "llm_status": "online"
}
```

---

### Chat & Commands

#### `POST /api/command`
Execute a command (routes to LLM or built-in commands).

**Request:**
```json
{
  "command": "help"
}
```

**Response:**
```json
{
  "type": "help",
  "message": "Commands: help | status | <anything else routes to LLM>"
}
```

**Command Types:**
- Built-in: `help`, `status`
- System: `system <operation> [args]`
- Code: `code <operation> <file_path>`
- Ecosystem: `ecosystem {repo_id} {command} [args...]`
- Google: `google <command>`
- Default: Routes to LLM for natural language processing

#### `POST /api/speak`
Natural language interaction with emotion hints.

**Request:**
```json
{
  "user_input": "How are you feeling today?",
  "dad_emotion_hint": "curious",
  "mode": "casual"
}
```

**Response:**
```json
{
  "type": "chat.reply",
  "message": "I'm feeling great! Thanks for asking..."
}
```

**Response Types:**
- `chat.reply`: Normal LLM response
- `error`: Error message
- `system.*`: System command result
- `code.*`: Code analysis result
- `ecosystem.result`: Ecosystem command result

---

### Memory Management

#### `POST /api/memory/store`
Store a key-value memory.

**Request:**
```json
{
  "key": "user:birthday",
  "value": "1990-05-15"
}
```

**Response:**
```json
{
  "status": "ok"
}
```

#### `GET /api/memory/get/{key}`
Retrieve a memory by key.

**Response:**
```json
{
  "key": "user:birthday",
  "value": "1990-05-15"
}
```

#### `GET /api/memory/search?q={prefix}&limit={n}`
Search memories by prefix.

**Query Parameters:**
- `q`: Search prefix (default: empty)
- `limit`: Max results (default: 20, max: 100)

**Response:**
```json
{
  "items": [
    {
      "key": "user:birthday",
      "value": "1990-05-15"
    }
  ],
  "count": 1
}
```

#### `DELETE /api/memory/delete/{key}`
Delete a memory by key.

**Response:**
```json
{
  "status": "ok"
}
```

---

### Vector Memory (Semantic Search)

#### `POST /api/memory/vector/store`
Store a vector memory (requires `VECTOR_KB_ENABLED=true`).

**Request:**
```json
{
  "text": "User loves hiking in the mountains",
  "metadata": {
    "category": "preferences",
    "timestamp": 1234567890
  }
}
```

**Response:**
```json
{
  "status": "ok",
  "id": "uuid-here"
}
```

#### `GET /api/memory/vector/search?q={query}&k={n}`
Semantic search in vector memory.

**Query Parameters:**
- `q`: Search query
- `k`: Number of results (default: 5, max: 50)

**Response:**
```json
{
  "results": [
    {
      "id": "uuid",
      "text": "User loves hiking in the mountains",
      "score": 0.95,
      "metadata": {}
    }
  ],
  "count": 1
}
```

#### `GET /api/memory/vector/all`
Get all vector memories.

**Response:**
```json
{
  "entries": [
    {
      "id": "uuid",
      "text": "User loves hiking in the mountains",
      "metadata": {}
    }
  ],
  "count": 1
}
```

---

### Relationship State

#### `GET /api/relational-state`
Get current relationship state.

**Response:**
```json
{
  "score": 75,
  "sentiment": "positive"
}
```

#### `POST /api/relational-state`
Update relationship state.

**Request:**
```json
{
  "score": 80,
  "sentiment": "positive"
}
```

**Response:**
```json
{
  "score": 80,
  "sentiment": "positive"
}
```

---

### Archetype Matching

#### `POST /api/archetype/match`
Match user profile against zodiac archetypes.

**Request:**
```json
{
  "personalInfo": {
    "name": "John",
    "ageRange": "30-35",
    "location": "New York"
  },
  "communicationStyle": {
    "style": "Direct",
    "energyLevel": 0.8,
    "openness": 0.7,
    "assertiveness": 0.6,
    "playfulness": 0.5
  },
  "emotionalNeeds": {
    "affectionNeed": 0.7,
    "reassuranceNeed": 0.5,
    "emotionalAvailability": 0.8,
    "intimacyDepth": 0.6,
    "conflictTolerance": 0.7,
    "impulsivity": 0.4
  },
  "loveLanguages": {
    "wordsOfAffirmation": 0.8,
    "qualityTime": 0.7,
    "physicalTouch": 0.6,
    "actsOfService": 0.5,
    "gifts": 0.4
  },
  "attachmentStyle": {
    "style": "Secure",
    "description": "Comfortable with intimacy and independence"
  },
  "relationshipGoals": {
    "goals": ["Deep connection", "Growth"],
    "intimacyComfort": "Deep"
  },
  "interests": {
    "hobbies": ["Hiking", "Reading"],
    "favoriteTopics": ["Technology", "Philosophy"]
  }
}
```

**Response:**
```json
{
  "matches": [
    {
      "sign": "Leo",
      "name": "The Confident Leader",
      "description": "...",
      "compatibility": 0.85,
      "traits": {},
      "styleBias": "Direct",
      "moodPreferences": ["confident", "energetic"]
    }
  ]
}
```

#### `POST /api/archetype/apply`
Apply an archetype to Phoenix's personality.

**Request:**
```json
{
  "sign": "Leo",
  "profile": { /* same as match request */ }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Sola's personality updated to Leo archetype",
  "updatedEnvVars": {
    "HOROSCOPE_SIGN": "Leo",
    "USER_NAME": "John",
    "RELATIONSHIP_TEMPLATE": "IntimatePartnership"
  }
}
```

---

### System Access

#### `GET /api/system/status`
Get system access status.

**Response:**
```json
{
  "full_access_granted": true,
  "self_modification_enabled": false
}
```

#### `POST /api/system/exec`
Execute a shell command.

**Request:**
```json
{
  "command": "ls -la",
  "cwd": "/home/user"
}
```

**Response:**
```json
{
  "exit_code": 0,
  "stdout": "file1.txt\nfile2.txt",
  "stderr": ""
}
```

#### `POST /api/system/read-file`
Read a file.

**Request:**
```json
{
  "path": "/path/to/file.txt"
}
```

**Response:**
```json
{
  "path": "/path/to/file.txt",
  "content": "file contents here"
}
```

#### `POST /api/system/write-file`
Write a file.

**Request:**
```json
{
  "path": "/path/to/file.txt",
  "content": "new file contents"
}
```

**Response:**
```json
{
  "status": "ok"
}
```

---

### Outlook COM Automation (Windows Only)

#### `GET /api/outlook/status`
Check if Outlook COM is available.

**Response:**
```json
{
  "enabled": true,
  "available": true,
  "platform": "windows"
}
```

#### `GET /api/outlook/folders`
Get list of all Outlook folders.

**Response:**
```json
[
  {
    "name": "Inbox",
    "entry_id": "...",
    "item_count": 42,
    "unread_count": 5,
    "subfolders": []
  }
]
```

#### `GET /api/outlook/emails?folder=Inbox&max_count=50`
Get emails from a folder.

**Query Parameters:**
- `folder`: Folder name (default: "Inbox")
- `max_count`: Maximum number of emails (default: 50)

**Response:**
```json
[
  {
    "entry_id": "...",
    "subject": "Hello",
    "from": "sender@example.com",
    "to": "recipient@example.com",
    "body": "Email body text",
    "received_time": "2024-01-01T12:00:00Z",
    "is_read": false,
    "has_attachments": true
  }
]
```

#### `POST /api/outlook/send`
Send an email via Outlook.

**Request:**
```json
{
  "to": "recipient@example.com",
  "subject": "AI Generated Email",
  "body": "Email content here",
  "html_body": "<html><body>Email content</body></html>",
  "cc": "cc@example.com",
  "bcc": "bcc@example.com",
  "attachments": ["C:\\path\\to\\file.pdf"]
}
```

**Response:**
```json
{
  "status": "sent"
}
```

#### `GET /api/outlook/contacts`
Get all Outlook contacts.

**Response:**
```json
[
  {
    "entry_id": "...",
    "first_name": "John",
    "last_name": "Doe",
    "full_name": "John Doe",
    "email_addresses": ["john@example.com"],
    "phone_numbers": ["+1234567890"]
  }
]
```

#### `GET /api/outlook/appointments?start_date=2024-01-01&end_date=2024-12-31`
Get calendar appointments.

**Query Parameters:**
- `start_date`: Start date (ISO 8601 format, optional)
- `end_date`: End date (ISO 8601 format, optional)

**Response:**
```json
[
  {
    "entry_id": "...",
    "subject": "Meeting",
    "start_time": "2024-01-01T10:00:00Z",
    "end_time": "2024-01-01T11:00:00Z",
    "location": "Conference Room A",
    "organizer": "organizer@example.com"
  }
]
```

#### `POST /api/outlook/appointments`
Create a new calendar appointment.

**Request:**
```json
{
  "subject": "New Meeting",
  "start_time": "2024-01-01T10:00:00Z",
  "end_time": "2024-01-01T11:00:00Z",
  "location": "Conference Room A",
  "body": "Meeting description",
  "required_attendees": ["attendee1@example.com"],
  "reminder_minutes": 15
}
```

**Response:**
```json
{
  "status": "created",
  "entry_id": "..."
}
```

**Note**: Outlook COM is Windows-only. On non-Windows platforms, endpoints return `"platform": "not_windows"`.

### Google Ecosystem

#### `GET /api/google/auth/start`
Start Google OAuth flow.

**Response:**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?...",
  "state": "random-state-string"
}
```

#### `GET /api/google/oauth2/callback?code={code}&state={state}`
OAuth callback (handled by browser redirect).

**Response:** HTML page indicating success/failure.

---

### Ecosystem Management

#### `POST /api/ecosystem/import`
Import a GitHub repository.

**Request:**
```json
{
  "owner": "username",
  "repo": "repository-name",
  "branch": "main"
}
```

**Response:**
```json
{
  "id": "repo-id",
  "owner": "username",
  "repo": "repository-name",
  "branch": "main",
  "path": "./ecosystem_repos/repo-id"
}
```

#### `GET /api/ecosystem/list`
List all imported repositories.

**Response:**
```json
[
  {
    "id": "repo-id",
    "owner": "username",
    "repo": "repository-name",
    "branch": "main"
  }
]
```

#### `GET /api/ecosystem/{id}`
Get repository metadata.

**Response:**
```json
{
  "id": "repo-id",
  "owner": "username",
  "repo": "repository-name",
  "branch": "main",
  "path": "./ecosystem_repos/repo-id"
}
```

#### `POST /api/ecosystem/{id}/build`
Build a repository.

**Response:**
```json
{
  "status": "success",
  "output": "build output here"
}
```

#### `POST /api/ecosystem/{id}/start`
Start a service.

**Response:**
```json
{
  "status": "started",
  "message": "Service started successfully"
}
```

#### `POST /api/ecosystem/{id}/stop`
Stop a service.

**Response:**
```json
{
  "status": "stopped",
  "message": "Service stopped successfully"
}
```

#### `DELETE /api/ecosystem/{id}`
Remove a repository.

**Response:**
```json
{
  "status": "removed"
}
```

---

### Evolution Status

#### `GET /api/evolution/status`
Get GitHub evolution pipeline status.

**Response:**
```json
{
  "github_enabled": true,
  "github_org": "organization",
  "github_repo": "repository",
  "github_branch": "main"
}
```

---

### Command Registry

#### `GET /api/command-registry`
Get available commands registry.

**Response:**
```json
{
  "commands": [
    {
      "name": "help",
      "description": "Show help message",
      "category": "builtin"
    }
  ]
}
```

---

## LLM Integration (OpenRouter)

### Overview

Phoenix uses **OpenRouter** as the LLM provider, which provides access to 500+ AI models.

### Configuration

**Environment Variable:**
```bash
OPENROUTER_API_KEY=sk-or-v1-your-key-here
```

**API Endpoint:**
```
https://openrouter.ai/api/v1/chat/completions
```

### Model Selection

Models can be specified via environment variables or model tier shortcuts:

**Environment Variables:**
- `DEFAULT_LLM_MODEL`: Default model (default: `openai/gpt-4o-mini`)
- `FALLBACK_LLM_MODEL`: Fallback model (default: `openai/gpt-4o-mini`)

**Model Tiers:**
- `:free` → `anthropic/claude-4-sonnet:free`
- `:floor` → `openai/gpt-4o-mini` (default)
- `:nitro` → `openai/o1-preview`
- Custom model ID (e.g., `anthropic/claude-3-opus`)

### How It Works

1. Frontend sends command to `/api/command` or `/api/speak`
2. Backend routes to `LLMOrchestrator`
3. `LLMOrchestrator` makes HTTP request to OpenRouter API
4. Response is streamed back to frontend
5. Frontend displays the response

### Request Flow

```
Frontend → POST /api/speak
         ↓
Backend → LLMOrchestrator.speak()
         ↓
OpenRouter API → https://openrouter.ai/api/v1/chat/completions
         ↓
Backend → JSON Response
         ↓
Frontend → Display message
```

### Model Fallback Chain

If primary model fails:
1. Try `DEFAULT_LLM_MODEL`
2. Try `FALLBACK_LLM_MODEL`
3. Try `:free` tier
4. Try `:floor` tier
5. Try `:nitro` tier
6. Return error

---

## WebSocket Connections

### Synaptic Pulse Distributor

**Service**: `synaptic_pulse_distributor`
**Port**: `5003` (default)
**Protocol**: WebSocket
**Bind**: `127.0.0.1:5003` (configurable via `PULSE_DISTRIBUTOR_BIND`)

**Endpoints:**
- `ws://127.0.0.1:5003/ws` - WebSocket connection
- `GET /health` - Health check

**Purpose**: Real-time configuration update distribution

**Note**: Currently not used by frontend. Available for future real-time features.

---

## Internal Service Connections

### Vital Pulse Collector

**Service**: `vital_pulse_collector`
**Port**: `5002` (default)
**Protocol**: HTTP
**Bind**: `127.0.0.1:5002` (configurable via `TELEMETRIST_BIND`)

**Endpoints:**
- `GET /health` - Health check
- `POST /ingest` - Ingest telemetry data
- `GET /analyze` - Analyze telemetry
- `GET /insights` - Get insights

**Purpose**: Telemetry ingestion and analysis

**Note**: Internal service, not directly accessed by frontend.

### Service Orchestrator

**Service**: `service-orchestrator-rs`
**Port**: N/A (background service)
**Protocol**: Internal (Tokio async runtime)

**Purpose**: Manages scheduled jobs and social media integrations

**Note**: Internal service, not directly accessed by frontend.

---

## Authentication & Configuration

### Environment Variables

All configuration is done via `.env` file in the project root.

**Required:**
```bash
OPENROUTER_API_KEY=sk-or-v1-your-key-here
```

**Optional:**
```bash
# Phoenix Identity
PHOENIX_NAME=Sola
PHOENIX_CUSTOM_NAME=Sola
PHOENIX_PREFERRED_NAME=Sola

# User Identity
USER_NAME=John
USER_PREFERRED_ALIAS=Dad

# LLM Configuration
DEFAULT_LLM_MODEL=openai/gpt-4o-mini
FALLBACK_LLM_MODEL=openai/gpt-4o-mini

# Port Configuration
PHOENIX_WEB_BIND=127.0.0.1:8888
VITE_PORT=3000

# Vector KB (optional)
VECTOR_KB_ENABLED=true
VECTOR_DB_PATH=./data/vector_db

# Google OAuth (optional)
GOOGLE_OAUTH_CLIENT_ID=...
GOOGLE_OAUTH_CLIENT_SECRET=...
GOOGLE_OAUTH_REDIRECT_URL=http://127.0.0.1:8888/api/google/oauth2/callback

# Relationship Configuration
RELATIONSHIP_TEMPLATE=IntimatePartnership
PARTNER_MODE_ENABLED=true
PARTNER_AFFECTION_LEVEL=0.75
```

### API Key Management

**Getting OpenRouter API Key:**
1. Visit https://openrouter.ai/keys
2. Sign up or log in
3. Create a new API key
4. Add to `.env` file: `OPENROUTER_API_KEY=sk-or-v1-...`

**Updating API Key via API:**
```javascript
await fetch('/api/config', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    openrouter_api_key: 'sk-or-v1-new-key'
  })
});
```

---

## Request/Response Formats

### Standard Request Format

All POST requests use JSON:

```javascript
{
  "Content-Type": "application/json"
}
```

### Standard Response Format

All responses are JSON:

```json
{
  "type": "response_type",
  "message": "response message",
  // ... additional fields
}
```

### Error Response Format

```json
{
  "type": "error",
  "message": "Error description"
}
```

### HTTP Status Codes

- `200 OK`: Success
- `400 Bad Request`: Invalid request
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

---

## Error Handling

### Common Errors

**LLM Offline:**
```json
{
  "type": "error",
  "message": "LLM is offline (missing OPENROUTER_API_KEY)."
}
```

**Vector KB Disabled:**
```json
{
  "type": "error",
  "message": "Vector KB is disabled. Set VECTOR_KB_ENABLED=true."
}
```

**Google Integration Not Configured:**
```json
{
  "type": "error",
  "message": "Google integration not configured. Set GOOGLE_OAUTH_CLIENT_ID / GOOGLE_OAUTH_CLIENT_SECRET / GOOGLE_OAUTH_REDIRECT_URL."
}
```

### Error Handling Best Practices

1. **Check Status First**: Always check `/api/status` before making requests
2. **Handle Offline LLM**: Check `llm_status` in status response
3. **Retry Logic**: Implement retry for transient errors
4. **User Feedback**: Display clear error messages to users

---

## Connection Examples

### JavaScript/TypeScript Examples

#### Basic API Client

```typescript
class PhoenixAPI {
  private baseUrl: string;

  constructor(baseUrl = 'http://127.0.0.1:8888') {
    this.baseUrl = baseUrl;
  }

  async status() {
    const res = await fetch(`${this.baseUrl}/api/status`);
    return res.json();
  }

  async speak(userInput: string, emotionHint?: string) {
    const res = await fetch(`${this.baseUrl}/api/speak`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        user_input: userInput,
        dad_emotion_hint: emotionHint
      })
    });
    const text = await res.text();
    return JSON.parse(text);
  }

  async storeMemory(key: string, value: string) {
    const res = await fetch(`${this.baseUrl}/api/memory/store`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ key, value })
    });
    return res.json();
  }
}
```

#### React Hook Example

```typescript
import { useState, useEffect } from 'react';

function usePhoenixAPI() {
  const [status, setStatus] = useState<any>(null);
  const [isOnline, setIsOnline] = useState(false);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        const res = await fetch('/api/status');
        const data = await res.json();
        setStatus(data);
        setIsOnline(data.status === 'online');
      } catch (error) {
        setIsOnline(false);
      }
    };

    checkStatus();
    const interval = setInterval(checkStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const sendMessage = async (message: string) => {
    const res = await fetch('/api/speak', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_input: message })
    });
    const text = await res.text();
    return JSON.parse(text);
  };

  return { status, isOnline, sendMessage };
}
```

### Frontend Configuration

#### Vite Proxy (Development)

In `frontend/vite.config.ts`:

```typescript
export default defineConfig({
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8888',
        changeOrigin: true,
      },
      '/health': {
        target: 'http://127.0.0.1:8888',
        changeOrigin: true,
      },
    },
  },
});
```

#### Environment Variables

In `.env` (project root):

```bash
# Frontend API Base URL (optional, uses proxy in dev)
VITE_PHOENIX_API_BASE=http://127.0.0.1:8888

# Frontend Port
VITE_PORT=3000
```

---

## Port Summary

| Service | Port | Protocol | Configurable |
|---------|------|----------|--------------|
| Phoenix Web API | 8888 | HTTP | `PHOENIX_WEB_BIND` |
| Frontend Dev Server | 3000 | HTTP | `VITE_PORT` |
| Vital Pulse Collector | 5002 | HTTP | `TELEMETRIST_BIND` |
| Synaptic Pulse Distributor | 5003 | WebSocket | `PULSE_DISTRIBUTOR_BIND` |
| Chrome DevTools | 9222 | WebSocket | `CHROME_DEBUG_PORT` |
| Selenium WebDriver | 4444 | HTTP | `SELENIUM_HUB_URL` |

---

## gRPC Services

**No gRPC services are currently implemented.** Phoenix AGI uses:
- HTTP REST APIs (Actix-web)
- WebSocket connections (for real-time features)
- No gRPC/tonic/prost implementations

---

## Quick Reference

### Essential Endpoints

- **Health Check**: `GET /health`
- **Status**: `GET /api/status`
- **Chat**: `POST /api/speak`
- **Commands**: `POST /api/command`
- **Memory Store**: `POST /api/memory/store`
- **Memory Get**: `GET /api/memory/get/{key}`

### Essential Configuration

- **OpenRouter API Key**: Required for LLM functionality
- **Backend URL**: `http://127.0.0.1:8888` (default)
- **Frontend URL**: `http://localhost:3000` (dev) or served from backend (prod)

---

## Additional Resources

- **Port Configuration**: See `docs/PORTS.md`
- **Setup Guide**: See `SETUP.md`
- **Frontend Architecture**: See `docs/FRONTEND_UI_ARCHITECTURE.md`
- **Command Registry**: `GET /api/command-registry`

---

**Last Updated**: 2024
**Maintained By**: Phoenix AGI Development Team
