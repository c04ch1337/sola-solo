export type L9Stage = 'Denial' | 'Anger' | 'Bargaining' | 'Depression' | 'Acceptance';
export type ContextTag = 'Home' | 'Transit' | 'Work' | 'Other';

export type MobileL9Log = {
  id: string;
  created_at_ms: number;
  stage: L9Stage;
  energy_level: number; // 0..100
  intensity: number; // 0..100
  tag: ContextTag;
  text: string;
};

export type SyncStatus = 'online' | 'offline' | 'syncing';

export type SyncState = {
  status: SyncStatus;
  /** If offline due to a failed request, show a countdown until next retry. */
  retryingInSec: number;
};

import { getPhoenixApiBase } from './env';

const QUEUE_KEY = 'l9.mobile.queue.v1';

function safeJsonParse<T>(raw: string | null): T | null {
  if (!raw) return null;
  try {
    return JSON.parse(raw) as T;
  } catch {
    return null;
  }
}

function loadQueue(): MobileL9Log[] {
  const parsed = safeJsonParse<MobileL9Log[]>(localStorage.getItem(QUEUE_KEY));
  if (!Array.isArray(parsed)) return [];
  return parsed.filter(Boolean);
}

function saveQueue(q: MobileL9Log[]) {
  localStorage.setItem(QUEUE_KEY, JSON.stringify(q));
}

function uid() {
  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

let state: SyncState = { status: 'offline', retryingInSec: 0 };
let listeners: Array<(s: SyncState) => void> = [];
let retryUntilMs: number | null = null;

function computeRetryingInSec() {
  if (!retryUntilMs) return 0;
  return Math.max(0, Math.ceil((retryUntilMs - Date.now()) / 1000));
}

function emit() {
  const next: SyncState = { ...state, retryingInSec: computeRetryingInSec() };
  listeners.forEach((l) => l(next));
}

function setState(patch: Partial<SyncState>) {
  state = { ...state, ...patch };
  emit();
}

export function getSyncState(): SyncState {
  return { ...state, retryingInSec: computeRetryingInSec() };
}

export function subscribeSyncState(cb: (s: SyncState) => void) {
  listeners.push(cb);
  cb(getSyncState());
  return () => {
    listeners = listeners.filter((x) => x !== cb);
  };
}

/** Back-compat: older UI can subscribe to just a status string. */
export function subscribeSyncStatus(cb: (s: SyncStatus) => void) {
  return subscribeSyncState((s) => cb(s.status));
}

export function enqueueLog(input: Omit<MobileL9Log, 'id' | 'created_at_ms'>): MobileL9Log {
  const entry: MobileL9Log = {
    id: uid(),
    created_at_ms: Date.now(),
    ...input,
  };

  const q = loadQueue();
  q.push(entry);
  saveQueue(q);
  return entry;
}

export function peekQueue(): MobileL9Log[] {
  return loadQueue();
}

export async function syncOnce(): Promise<{ sent: number; remaining: number }> {
  const base = getPhoenixApiBase();
  const url = `${String(base).replace(/\/$/, '')}/api/counselor/events`;

  // Respect backoff window when the last attempt failed.
  if (retryUntilMs && Date.now() < retryUntilMs) {
    setState({ status: 'offline' });
    return { sent: 0, remaining: loadQueue().length };
  }

  const q = loadQueue();
  if (!q.length) {
    retryUntilMs = null;
    setState({ status: 'online', retryingInSec: 0 });
    return { sent: 0, remaining: 0 };
  }

  setState({ status: 'syncing', retryingInSec: 0 });

  let sent = 0;
  const remaining: MobileL9Log[] = [];

  for (const item of q) {
    // Payload aligned to backend GriefEvent; mobile uses a single tag -> context_tags[0]
    const payload = {
      stage: item.stage,
      intensity: item.intensity,
      energy_level: item.energy_level,
      context_tags: [item.tag],
      text: item.text,
      timestamp_ms: item.created_at_ms,
      source: 'mobile_pwa',
    };

    try {
      const res = await fetch(url, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        remaining.push(item);
        retryUntilMs = Date.now() + 30_000;
        setState({ status: 'offline' });
        continue;
      }

      sent += 1;
      retryUntilMs = null;
      setState({ status: 'online', retryingInSec: 0 });
    } catch {
      remaining.push(item);
      retryUntilMs = Date.now() + 30_000;
      setState({ status: 'offline' });
    }
  }

  saveQueue(remaining);
  return { sent, remaining: remaining.length };
}

let loopStarted = false;

export function startBackgroundSync(intervalMs = 4000) {
  if (loopStarted) return;
  loopStarted = true;

  const tick = async () => {
    try {
      await syncOnce();
    } catch {
      retryUntilMs = Date.now() + 30_000;
      setState({ status: 'offline' });
    }
  };

  // Fast tick so retry countdown can update smoothly.
  tick();
  window.setInterval(() => {
    // Refresh countdown even when we are in backoff.
    emit();
    void tick();
  }, Math.max(1000, Math.floor(intervalMs)));
}
