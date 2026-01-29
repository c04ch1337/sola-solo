/**
 * WebSocket Service for Phoenix Backend
 * 
 * Provides real-time bi-directional communication with the Phoenix Orchestrator backend.
 * Supports chat, commands, memory operations, and status queries.
 */

import { getPhoenixApiBase, getPhoenixWsUrl } from '../env';

// Derive WebSocket URL from API URL if not explicitly set
function getWebSocketUrl(): string {
  // If explicitly set, use it; otherwise derive from VITE_PHOENIX_API_URL.
  return getPhoenixWsUrl();
}

const PHOENIX_WS_URL = getWebSocketUrl();

// Touch the API base at module init so missing env fails fast in dev.
// (Also guarantees we do not silently fall back to hardcoded URLs.)
void getPhoenixApiBase();

export interface WebSocketMessage {
  type:
    | 'speak'
    | 'command'
    | 'system'
    | 'memory_search'
    | 'memory_store'
    | 'memory_get'
    | 'status'
    | 'ping'
    | 'proactive_control'; // For controlling proactive messages
  [key: string]: any;
}

export interface WebSocketResponse {
  type: string;
  [key: string]: any;
}

export type MessageHandler = (response: WebSocketResponse) => void;
export type ConnectionHandler = (connected: boolean) => void;

export class WebSocketService {
  private ws: WebSocket | null = null;
  private url: string;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private listeners: Map<string, Set<MessageHandler>> = new Map();
  private connectionListeners: Set<ConnectionHandler> = new Set();
  private isConnecting = false;
  private shouldReconnect = true;

  constructor(url?: string) {
    this.url = url || PHOENIX_WS_URL;
  }

  /**
   * Connect to the WebSocket server
   */
  connect(): Promise<void> {
    if (this.isConnecting || (this.ws && this.ws.readyState === WebSocket.OPEN)) {
      return Promise.resolve();
    }

    return new Promise((resolve, reject) => {
      this.isConnecting = true;
      this.shouldReconnect = true;

      try {
        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
          this.isConnecting = false;
          this.reconnectAttempts = 0;
          this.reconnectDelay = 1000;
          this.notifyConnectionListeners(true);
          console.log('[WebSocket] Connected to Phoenix backend');
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const response: WebSocketResponse = JSON.parse(event.data);
            
            // Handle special connection message
            if (response.type === 'connected') {
              console.log('[WebSocket] Connection confirmed:', response);
              return;
            }

            // Notify all listeners for this message type
            this.notifyListeners(response.type, response);
            
            // Also notify generic listeners
            this.notifyListeners('*', response);
          } catch (error) {
            console.error('[WebSocket] Failed to parse message:', error, event.data);
          }
        };

        this.ws.onerror = (error) => {
          this.isConnecting = false;
          console.error('[WebSocket] Connection error:', error);
          this.notifyConnectionListeners(false);
          reject(error);
        };

        this.ws.onclose = (event) => {
          this.isConnecting = false;
          this.notifyConnectionListeners(false);
          console.log('[WebSocket] Connection closed:', event.code, event.reason);

          // Attempt to reconnect if we should
          if (this.shouldReconnect && this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = this.reconnectDelay * this.reconnectAttempts;
            console.log(`[WebSocket] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
            
            setTimeout(() => {
              if (this.shouldReconnect) {
                this.connect().catch(err => {
                  console.error('[WebSocket] Reconnection failed:', err);
                });
              }
            }, delay);
          } else if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.error('[WebSocket] Max reconnection attempts reached');
          }
        };
      } catch (error) {
        this.isConnecting = false;
        reject(error);
      }
    });
  }

  /**
   * Disconnect from the WebSocket server
   */
  disconnect(): void {
    this.shouldReconnect = false;
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.notifyConnectionListeners(false);
  }

  /**
   * Send a message to the server
   */
  send(type: string, data: any): boolean {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      console.warn('[WebSocket] Cannot send message: not connected');
      return false;
    }

    try {
      const message: WebSocketMessage = { type: type as any, ...data };
      this.ws.send(JSON.stringify(message));
      return true;
    } catch (error) {
      console.error('[WebSocket] Failed to send message:', error);
      return false;
    }
  }

  /**
   * Subscribe to messages of a specific type
   */
  on(type: string, handler: MessageHandler): void {
    if (!this.listeners.has(type)) {
      this.listeners.set(type, new Set());
    }
    this.listeners.get(type)!.add(handler);
  }

  /**
   * Unsubscribe from messages of a specific type
   */
  off(type: string, handler: MessageHandler): void {
    this.listeners.get(type)?.delete(handler);
  }

  /**
   * Subscribe to connection state changes
   */
  onConnection(handler: ConnectionHandler): void {
    this.connectionListeners.add(handler);
  }

  /**
   * Unsubscribe from connection state changes
   */
  offConnection(handler: ConnectionHandler): void {
    this.connectionListeners.delete(handler);
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  /**
   * Get connection state
   */
  getState(): 'connecting' | 'open' | 'closing' | 'closed' {
    if (!this.ws) return 'closed';
    switch (this.ws.readyState) {
      case WebSocket.CONNECTING:
        return 'connecting';
      case WebSocket.OPEN:
        return 'open';
      case WebSocket.CLOSING:
        return 'closing';
      case WebSocket.CLOSED:
        return 'closed';
      default:
        return 'closed';
    }
  }

  private notifyListeners(type: string, response: WebSocketResponse): void {
    const handlers = this.listeners.get(type);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(response);
        } catch (error) {
          console.error('[WebSocket] Error in message handler:', error);
        }
      });
    }
  }

  private notifyConnectionListeners(connected: boolean): void {
    this.connectionListeners.forEach(handler => {
      try {
        handler(connected);
      } catch (error) {
        console.error('[WebSocket] Error in connection handler:', error);
      }
    });
  }
}

// Convenience methods for common operations

/**
 * Send a chat message via WebSocket
 */
export function sendSpeak(
  ws: WebSocketService,
  userInput: string,
  mode?: string,
  projectContext?: string
): boolean {
  return ws.send('speak', {
    user_input: userInput,
    mode,
    project_context: projectContext,
  });
}

/**
 * Send a command via WebSocket
 */
export function sendCommand(
  ws: WebSocketService,
  command: string,
  projectContext?: string
): boolean {
  return ws.send('command', {
    command,
    project_context: projectContext,
  });
}

/**
 * Send a system action over WebSocket (used for per-connection consent: grant/revoke).
 */
export function sendSystem(ws: WebSocketService, action: 'grant' | 'revoke'): boolean {
  return ws.send('system', { action });
}

/**
 * Search memories via WebSocket
 */
export function sendMemorySearch(
  ws: WebSocketService,
  query: string,
  limit?: number
): boolean {
  return ws.send('memory_search', {
    query,
    limit,
  });
}

/**
 * Store a memory via WebSocket
 */
export function sendMemoryStore(
  ws: WebSocketService,
  key: string,
  value: string
): boolean {
  return ws.send('memory_store', {
    key,
    value,
  });
}

/**
 * Get a memory via WebSocket
 */
export function sendMemoryGet(ws: WebSocketService, key: string): boolean {
  return ws.send('memory_get', { key });
}

/**
 * Request status via WebSocket
 */
export function sendStatus(ws: WebSocketService): boolean {
  return ws.send('status', {});
}
