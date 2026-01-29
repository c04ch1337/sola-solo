# Frontend LLM Integration Guide

**For Frontend Developers**

This guide explains how Phoenix AGI connects to LLM providers and what frontend developers need to know.

---

## Key Points for Frontend Developers

### ✅ **DO NOT Connect to LLM Providers Directly**

**Important**: The frontend should **NEVER** make direct API calls to LLM providers (OpenRouter, OpenAI, Anthropic, etc.). All LLM interactions must go through the Phoenix backend API.

### ✅ **Use Backend API Endpoints**

All LLM interactions happen through these backend endpoints:
- `POST /api/speak` - Natural language interaction
- `POST /api/command` - Command execution (routes to LLM for natural language)

---

## How LLM Integration Works

### Architecture Overview

```
┌─────────────────┐
│   Frontend      │
│   (React/TS)    │
└────────┬────────┘
         │
         │ POST /api/speak
         │ { "user_input": "Hello" }
         │
         ▼
┌─────────────────┐
│  Phoenix Web    │
│  Backend (Rust) │
│  Port: 8888     │
└────────┬────────┘
         │
         │ LLMOrchestrator.speak()
         │
         ▼
┌─────────────────┐
│  OpenRouter API │
│  (Default LLM)  │
│  Port: 443      │
└─────────────────┘
```

### Flow Diagram

1. **User types message** in frontend
2. **Frontend sends** `POST /api/speak` to backend
3. **Backend processes** request through `LLMOrchestrator`
4. **Backend calls** OpenRouter API (or other LLM provider)
5. **Backend receives** LLM response
6. **Backend returns** JSON response to frontend
7. **Frontend displays** the response

---

## OpenRouter as Default LLM Provider

### What is OpenRouter?

**OpenRouter** is a unified API gateway that provides access to **500+ AI models** from various providers:
- OpenAI (GPT-4, GPT-3.5, o1, etc.)
- Anthropic (Claude 3 Opus, Sonnet, Haiku, etc.)
- Google (Gemini Pro, etc.)
- Meta (Llama, etc.)
- And many more...

### Why OpenRouter?

1. **Single API Key**: One API key for all models
2. **Model Flexibility**: Easy switching between models
3. **Cost Optimization**: Automatic fallback to cheaper models
4. **Unified Interface**: Same API for all providers

### Configuration

**Backend Configuration** (in `.env` file):
```bash
# Required: OpenRouter API Key
OPENROUTER_API_KEY=sk-or-v1-your-key-here

# Optional: Model Selection
DEFAULT_LLM_MODEL=openai/gpt-4o-mini
FALLBACK_LLM_MODEL=openai/gpt-4o-mini
```

**Frontend Configuration**: None required! The frontend doesn't need to know about OpenRouter or API keys.

---

## Frontend Implementation

### Example: Sending a Message

```typescript
// ✅ CORRECT: Send to backend API
async function sendMessage(userInput: string) {
  const response = await fetch('/api/speak', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      user_input: userInput,
      dad_emotion_hint: 'curious', // optional
      mode: 'casual' // optional
    })
  });
  
  const result = await response.json();
  return result.message; // LLM response
}
```

### Example: React Hook

```typescript
import { useState } from 'react';

function useChat() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);

  const sendMessage = async (text: string) => {
    setLoading(true);
    try {
      const response = await fetch('/api/speak', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ user_input: text })
      });
      
      const result = await response.json();
      
      if (result.type === 'chat.reply') {
        setMessages(prev => [
          ...prev,
          { role: 'user', content: text },
          { role: 'assistant', content: result.message }
        ]);
      } else if (result.type === 'error') {
        console.error('LLM Error:', result.message);
        // Handle error (e.g., LLM offline)
      }
    } catch (error) {
      console.error('Network Error:', error);
    } finally {
      setLoading(false);
    }
  };

  return { messages, sendMessage, loading };
}
```

---

## What Frontend Developers Need to Know

### ✅ **What You Should Do**

1. **Use Backend Endpoints**: Always use `/api/speak` or `/api/command`
2. **Handle Responses**: Parse JSON responses and display messages
3. **Error Handling**: Check for `llm_status` in `/api/status` response
4. **User Feedback**: Show loading states and error messages

### ❌ **What You Should NOT Do**

1. **Don't Call LLM APIs Directly**: No OpenAI, Anthropic, or OpenRouter API calls from frontend
2. **Don't Store API Keys**: Never put LLM API keys in frontend code
3. **Don't Bypass Backend**: All LLM interactions must go through backend

---

## Response Format

### Success Response

```json
{
  "type": "chat.reply",
  "message": "Hello! How can I help you today?"
}
```

### Error Response (LLM Offline)

```json
{
  "type": "error",
  "message": "LLM is offline (missing OPENROUTER_API_KEY)."
}
```

### Error Response (Other)

```json
{
  "type": "error",
  "message": "Error description here"
}
```

---

## Checking LLM Status

### Check if LLM is Available

```typescript
async function checkLLMStatus() {
  const response = await fetch('/api/status');
  const status = await response.json();
  
  if (status.llm_status === 'online') {
    // LLM is ready to use
    return true;
  } else {
    // LLM is offline (missing API key or error)
    console.warn('LLM is offline:', status);
    return false;
  }
}
```

### Display Status to User

```typescript
function ChatInterface() {
  const [llmOnline, setLlmOnline] = useState(false);
  
  useEffect(() => {
    const checkStatus = async () => {
      const status = await fetch('/api/status').then(r => r.json());
      setLlmOnline(status.llm_status === 'online');
    };
    
    checkStatus();
    const interval = setInterval(checkStatus, 5000);
    return () => clearInterval(interval);
  }, []);
  
  return (
    <div>
      {!llmOnline && (
        <div className="warning">
          LLM is offline. Please configure OPENROUTER_API_KEY in backend.
        </div>
      )}
      {/* Chat interface */}
    </div>
  );
}
```

---

## Model Selection (Backend Configuration)

### Model Tiers

The backend supports model tier shortcuts:

- `:free` → Free models (e.g., `anthropic/claude-4-sonnet:free`)
- `:floor` → Best free/low-cost models (default: `openai/gpt-4o-mini`)
- `:nitro` → Premium models (e.g., `openai/o1-preview`)
- Custom model ID (e.g., `anthropic/claude-3-opus`)

### Configuration (Backend Only)

```bash
# In .env file (backend)
DEFAULT_LLM_MODEL=openai/gpt-4o-mini
FALLBACK_LLM_MODEL=anthropic/claude-3-haiku
```

**Note**: Frontend doesn't need to know about model selection. The backend handles it automatically.

---

## Error Handling

### Common Errors

1. **LLM Offline**: Missing `OPENROUTER_API_KEY` in backend
   - **Solution**: User needs to configure API key in backend `.env` file

2. **Network Error**: Backend unreachable
   - **Solution**: Check if backend is running on port 8888

3. **Rate Limit**: Too many requests to OpenRouter
   - **Solution**: Backend automatically retries with fallback models

### Error Handling Example

```typescript
async function sendMessageWithErrorHandling(text: string) {
  try {
    // Check LLM status first
    const statusRes = await fetch('/api/status');
    const status = await statusRes.json();
    
    if (status.llm_status !== 'online') {
      throw new Error('LLM is offline. Please configure OPENROUTER_API_KEY.');
    }
    
    // Send message
    const response = await fetch('/api/speak', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_input: text })
    });
    
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    
    const result = await response.json();
    
    if (result.type === 'error') {
      throw new Error(result.message);
    }
    
    return result.message;
    
  } catch (error) {
    console.error('Error sending message:', error);
    // Display user-friendly error message
    return `Error: ${error.message}`;
  }
}
```

---

## Getting OpenRouter API Key

### For Backend Configuration

1. Visit https://openrouter.ai/keys
2. Sign up or log in
3. Create a new API key
4. Copy the key (format: `sk-or-v1-...`)
5. Add to backend `.env` file:
   ```bash
   OPENROUTER_API_KEY=sk-or-v1-your-key-here
   ```

### Updating API Key via Frontend

Users can update the API key through the frontend UI:

```typescript
async function updateAPIKey(apiKey: string) {
  const response = await fetch('/api/config', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      openrouter_api_key: apiKey
    })
  });
  
  const result = await response.json();
  return result.llm_status === 'online';
}
```

---

## Summary for Frontend Developers

### ✅ **What You Need to Know**

1. **All LLM interactions go through backend** (`/api/speak` or `/api/command`)
2. **OpenRouter is the default LLM provider** (handled by backend)
3. **No API keys in frontend code** (all in backend `.env`)
4. **Check `llm_status` in `/api/status`** before making requests
5. **Handle errors gracefully** (LLM offline, network errors, etc.)

### ✅ **What You Don't Need to Know**

1. **OpenRouter API details** (handled by backend)
2. **Model selection logic** (handled by backend)
3. **API key management** (handled by backend)
4. **Fallback mechanisms** (handled by backend)

### ✅ **Your Responsibilities**

1. **Send user input** to `/api/speak` endpoint
2. **Display LLM responses** to users
3. **Handle loading states** during requests
4. **Show error messages** when LLM is offline
5. **Provide good UX** for chat interactions

---

## Quick Reference

### Essential Endpoints

```typescript
// Check LLM status
GET /api/status
→ { llm_status: "online" | "offline" }

// Send message to LLM
POST /api/speak
→ { type: "chat.reply", message: "..." }

// Execute command (routes to LLM)
POST /api/command
→ { type: "chat.reply", message: "..." }
```

### Essential Code Pattern

```typescript
// 1. Check status
const status = await fetch('/api/status').then(r => r.json());

// 2. Send message if online
if (status.llm_status === 'online') {
  const result = await fetch('/api/speak', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ user_input: message })
  }).then(r => r.json());
  
  // 3. Display response
  displayMessage(result.message);
}
```

---

## Additional Resources

- **Full API Documentation**: See `docs/FRONTEND_API_CONNECTIONS.md`
- **Backend Setup**: See `SETUP.md`
- **OpenRouter Documentation**: https://openrouter.ai/docs

---

**Remember**: The frontend is a **presentation layer**. All LLM logic, API calls, and model management happens in the backend. Your job is to provide a great user experience for interacting with the LLM through the backend API.
