// frontend_mobile/src/env.ts

export function getPhoenixApiBase(): string {
  const raw = import.meta.env.VITE_PHOENIX_API_URL;
  if (!raw || !String(raw).trim()) {
    throw new Error(
      'VITE_PHOENIX_API_URL is not set. Set it to your backend base URL (e.g. http://localhost:8888).'
    );
  }
  return String(raw).replace(/\/$/, '');
}
