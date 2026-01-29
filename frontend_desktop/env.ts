// frontend_desktop/env.ts
// Centralized frontend environment access.

/**
 * Returns the Phoenix backend base URL.
 *
 * Requirement: do not rely on hardcoded defaults; force explicit configuration.
 */
export function getPhoenixApiBase(): string {
  const raw = import.meta.env.VITE_PHOENIX_API_URL;
  if (!raw || !String(raw).trim()) {
    throw new Error(
      'VITE_PHOENIX_API_URL is not set. Set it to your backend base URL (e.g. http://localhost:8888).'
    );
  }
  return String(raw).replace(/\/$/, '');
}

/**
 * Returns the Phoenix backend WebSocket URL.
 * Uses VITE_PHOENIX_WS_URL if provided; otherwise derives from VITE_PHOENIX_API_URL.
 */
export function getPhoenixWsUrl(): string {
  const explicit = import.meta.env.VITE_PHOENIX_WS_URL;
  if (explicit && String(explicit).trim()) {
    return String(explicit).replace(/\/$/, '');
  }

  const api = new URL(getPhoenixApiBase());
  api.protocol = api.protocol === 'https:' ? 'wss:' : 'ws:';
  api.pathname = '/ws';
  api.search = '';
  api.hash = '';
  return api.toString();
}

