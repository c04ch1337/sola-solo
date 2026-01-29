# Frontend Desktop - Phoenix Orchestrator Integration

## Overview

The `frontend_desktop` has been integrated with the Phoenix Orchestrator backend, replacing the Google Gemini API dependency.

## Changes Made

### 1. Service Layer
- **Created**: `services/phoenixService.ts`
  - Replaces `services/geminiService.ts`
  - Implements `apiSpeak()` and `apiCommand()` functions
  - Connects to Phoenix backend at `http://localhost:8888` (configurable via `VITE_PHOENIX_API_URL`)

### 2. Configuration
- **Updated**: `vite.config.ts`
  - Added proxy configuration for `/api` and `/health` endpoints
  - Proxies requests to Phoenix backend (port 8888)
  - Removed Gemini API key configuration

### 3. Application Code
- **Updated**: `App.tsx`
  - Removed `@google/genai` import
  - Updated imports to use `phoenixService` instead of `geminiService`
  - Disabled live voice mode and dictation (requires backend audio intelligence integration)
  - Kept audio processing helpers for future integration

### 4. Dependencies
- **Updated**: `package.json`
  - Removed `@google/genai` dependency
  - Kept React, React DOM, and markdown rendering dependencies

## API Endpoints Used

### `/api/speak`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "user_input": "string",
    "dad_emotion_hint": "optional string",
    "mode": "optional string"
  }
  ```
- **Response**: JSON string containing `{"type": "chat.reply", "message": "..."}`

### `/api/command`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "command": "string"
  }
  ```
- **Response**: JSON string with command execution results

### `/health`
- **Method**: GET
- **Response**: Health check status

### `/api/status`
- **Method**: GET
- **Response**: Phoenix system status

## Environment Variables

Create a `.env` file in `frontend_desktop/` directory:

```bash
# Phoenix Backend API URL
VITE_PHOENIX_API_URL=http://localhost:8888
```

## Running the Frontend

1. **Install dependencies**:
   ```bash
   cd frontend_desktop
   npm install
   ```

2. **Start Phoenix backend** (in another terminal):
   ```bash
   cd ..
   cargo run -p phoenix-web
   ```

3. **Start frontend dev server**:
   ```bash
   cd frontend_desktop
   npm run dev
   ```

4. **Access the frontend**:
   - Open `http://localhost:3000` in your browser
   - The frontend will proxy API requests to the Phoenix backend

## Features Status

### ✅ Implemented
- Text-based chat interface
- Command execution
- Project management
- Chat history
- Settings panel
- Scheduler view

### ⚠️ Pending Backend Integration
- **Live Voice Mode**: Requires Phoenix audio intelligence API integration
- **Dictation Mode**: Requires Phoenix audio intelligence API integration
- **Real-time Audio Streaming**: Requires WebSocket or streaming API support

## Future Enhancements

1. **Audio Intelligence Integration**
   - Connect to `/api/audio/start-recording` and `/api/audio/stop-recording`
   - Implement WebSocket for real-time audio streaming
   - Add TTS support for assistant responses

2. **WebSocket Support**
   - Add WebSocket connection for real-time updates
   - Stream command execution progress
   - Live status updates

3. **Memory Integration**
   - Connect to `/api/memory/*` endpoints
   - Display memory context in chat
   - Memory browser interface

4. **System Status**
   - Real-time backend status monitoring
   - Connection health indicators
   - Backend metrics display

## Troubleshooting

### Backend Connection Issues
- Ensure Phoenix backend is running on port 8888
- Check `VITE_PHOENIX_API_URL` in `.env` file
- Verify CORS settings in Phoenix backend (should allow `http://localhost:3000`)

### API Errors
- Check browser console for detailed error messages
- Verify backend is responding to `/health` endpoint
- Check network tab for failed requests

### Build Issues
- Run `npm install` to ensure all dependencies are installed
- Clear `node_modules` and reinstall if needed
- Check Node.js version (requires Node 18+)
