import { useEffect, useMemo, useState } from 'react';
import { getPhoenixApiBase } from '../env';

const BRAKE_UNTIL_KEY = 'pagi.regulatory_brake_until_ms';

export type InterventionResponse = {
  risk_score: number;
  title: string;
  exercise: string;
  recommended_seconds: number;
};

export function getBrakeUntilMs(): number | null {
  try {
    const raw = localStorage.getItem(BRAKE_UNTIL_KEY);
    if (!raw) return null;
    const n = Number(raw);
    if (!Number.isFinite(n) || n <= 0) return null;
    return n;
  } catch {
    return null;
  }
}

export function setBrakeUntilMs(untilMs: number | null) {
  try {
    if (!untilMs) {
      localStorage.removeItem(BRAKE_UNTIL_KEY);
      return;
    }
    localStorage.setItem(BRAKE_UNTIL_KEY, String(untilMs));
  } catch {
    // no-op
  }
}

export function fmtCountdown(sec: number) {
  const s = Math.max(0, Math.floor(sec));
  const m = Math.floor(s / 60);
  const r = s % 60;
  if (m <= 0) return `${r}s`;
  return `${m}m ${r}s`;
}

export function useRegulatoryBrake() {
  const [untilMs, setUntilMsState] = useState<number | null>(() => getBrakeUntilMs());
  const [nowMs, setNowMs] = useState(() => Date.now());

  useEffect(() => {
    const t = setInterval(() => setNowMs(Date.now()), 1000);
    return () => clearInterval(t);
  }, []);

  // External updates (e.g. another component starts the brake)
  useEffect(() => {
    const onStorage = (e: StorageEvent) => {
      if (e.key !== BRAKE_UNTIL_KEY) return;
      setUntilMsState(getBrakeUntilMs());
    };
    window.addEventListener('storage', onStorage);
    return () => window.removeEventListener('storage', onStorage);
  }, []);

  const secondsLeft = useMemo(() => {
    if (!untilMs) return 0;
    return Math.max(0, Math.ceil((untilMs - nowMs) / 1000));
  }, [untilMs, nowMs]);

  const blocked = secondsLeft > 0;

  useEffect(() => {
    if (untilMs && !blocked) {
      setBrakeUntilMs(null);
      setUntilMsState(null);
    }
  }, [blocked, untilMs]);

  const startBrake = (seconds: number) => {
    const s = Math.max(1, Math.floor(seconds));
    const u = Date.now() + s * 1000;
    setBrakeUntilMs(u);
    setUntilMsState(u);
  };

  const clearBrake = () => {
    setBrakeUntilMs(null);
    setUntilMsState(null);
  };

  return { blocked, secondsLeft, untilMs, startBrake, clearBrake };
}

export function useIntervention(riskScore: number | null) {
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [data, setData] = useState<InterventionResponse | null>(null);

  useEffect(() => {
    let alive = true;
    if (typeof riskScore !== 'number') {
      setData(null);
      setError(null);
      return;
    }

    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/intervention?risk=${riskScore}`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const json = (await res.json()) as InterventionResponse;
        if (!alive) return;
        setData(json);
      } catch (e: any) {
        if (!alive) return;
        setError(e?.message || 'Failed to load intervention');
      } finally {
        if (!alive) return;
        setLoading(false);
      }
    })();

    return () => {
      alive = false;
    };
  }, [PHOENIX_API_BASE, riskScore]);

  return { loading, error, data };
}
