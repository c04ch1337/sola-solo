/**
 * Memory Service for Phoenix Backend
 * 
 * Provides a type-safe abstraction over WebSocket memory operations.
 * Supports all three memory systems:
 * - Vital Organ Vaults (Mind, Body, Soul)
 * - Neural Cortex Strata (STM, WM, LTM, EPM, RFM)
 * - Vector KB (Semantic Search)
 */

import { WebSocketService } from './websocketService';

export type Vault = 'mind' | 'body' | 'soul';
export type CortexLayer = 'STM' | 'WM' | 'LTM' | 'EPM' | 'RFM';

export interface MemoryItem {
  key: string;
  value: string;
}

export interface CortexMemoryItem {
  key: string;
  layer: string;
  value: string;
}

export interface VectorMemoryResult {
  id: string;
  text: string;
  score: number;
  metadata: any;
}

export type MemoryResultCallback = (result: any) => void;

export class MemoryService {
  private ws: WebSocketService;
  private callbacks: Map<string, Set<MemoryResultCallback>> = new Map();

  constructor(ws: WebSocketService) {
    this.ws = ws;

    // Subscribe to all memory response types
    this.ws.on('memory_search_response', (response: any) => {
      this.notifyCallbacks('memory_search', response);
    });

    this.ws.on('memory_store_response', (response: any) => {
      this.notifyCallbacks('memory_store', response);
    });

    this.ws.on('memory_get_response', (response: any) => {
      this.notifyCallbacks('memory_get', response);
    });

    this.ws.on('memory_delete_response', (response: any) => {
      this.notifyCallbacks('memory_delete', response);
    });

    this.ws.on('memory_cortex_store_response', (response: any) => {
      this.notifyCallbacks('memory_cortex_store', response);
    });

    this.ws.on('memory_cortex_get_response', (response: any) => {
      this.notifyCallbacks('memory_cortex_get', response);
    });

    this.ws.on('memory_cortex_search_response', (response: any) => {
      this.notifyCallbacks('memory_cortex_search', response);
    });

    this.ws.on('memory_vector_store_response', (response: any) => {
      this.notifyCallbacks('memory_vector_store', response);
    });

    this.ws.on('memory_vector_search_response', (response: any) => {
      this.notifyCallbacks('memory_vector_search', response);
    });

    // Also listen for errors
    this.ws.on('error', (response: any) => {
      if (response.message?.includes('memory') || response.message?.includes('Memory')) {
        this.notifyCallbacks('error', response);
      }
    });
  }

  /**
   * Subscribe to memory operation results
   */
  onResult(operation: string, callback: MemoryResultCallback): () => void {
    if (!this.callbacks.has(operation)) {
      this.callbacks.set(operation, new Set());
    }
    this.callbacks.get(operation)!.add(callback);

    // Return unsubscribe function
    return () => {
      this.callbacks.get(operation)?.delete(callback);
    };
  }

  private notifyCallbacks(operation: string, result: any): void {
    const callbacks = this.callbacks.get(operation);
    if (callbacks) {
      callbacks.forEach(cb => {
        try {
          cb(result);
        } catch (error) {
          console.error('[MemoryService] Error in callback:', error);
        }
      });
    }
    // Also notify generic listeners
    const genericCallbacks = this.callbacks.get('*');
    if (genericCallbacks) {
      genericCallbacks.forEach(cb => {
        try {
          cb(result);
        } catch (error) {
          console.error('[MemoryService] Error in generic callback:', error);
        }
      });
    }
  }

  // ── Vault Operations (defaults to soul to match REST API) ──

  /**
   * Search memories in a vault
   */
  searchVault(query: string, limit: number = 10, vault: Vault = 'soul'): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_search', { query, limit, vault });
  }

  /**
   * Store a memory in a vault
   */
  storeVault(key: string, value: string, vault: Vault = 'soul'): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_store', { key, value, vault });
  }

  /**
   * Get a memory from a vault
   */
  getVault(key: string, vault: Vault = 'soul'): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_get', { key, vault });
  }

  /**
   * Delete a memory from a vault (only supported for soul vault)
   */
  deleteVault(key: string, vault: Vault = 'soul'): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_delete', { key, vault });
  }

  // ── Neural Cortex Strata (5-layer system) ──

  /**
   * Store a memory in a specific cortex layer
   */
  storeCortex(layer: CortexLayer, key: string, value: string): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_cortex_store', { layer, key, value });
  }

  /**
   * Get a memory from cortex (any layer)
   */
  getCortex(key: string): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_cortex_get', { key });
  }

  /**
   * Search memories in cortex by prefix
   */
  searchCortex(prefix: string, limit: number = 10): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_cortex_search', { prefix, limit });
  }

  // ── Vector KB (Semantic Search) ──

  /**
   * Store a memory in vector KB with semantic embeddings
   */
  storeVector(text: string, metadata: Record<string, any> = {}): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_vector_store', { text, metadata });
  }

  /**
   * Semantic search in vector KB
   */
  searchVector(query: string, k: number = 5): boolean {
    if (!this.ws.isConnected()) {
      console.warn('[MemoryService] WebSocket not connected');
      return false;
    }
    return this.ws.send('memory_vector_search', { query, k });
  }

  /**
   * Check if WebSocket is connected
   */
  isConnected(): boolean {
    return this.ws.isConnected();
  }
}
