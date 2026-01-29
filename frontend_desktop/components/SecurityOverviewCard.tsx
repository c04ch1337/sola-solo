import React, { useEffect, useMemo, useState } from 'react';

type VaultHealth = {
  unlocked: boolean;
  profiles_count: number;
  last_rotation_ms: number | null;
  rotation_overdue: boolean;
};

function formatTs(ms: number | null): string {
  if (!ms) return 'Never';
  try {
    return new Date(ms).toLocaleString();
  } catch {
    return String(ms);
  }
}

export default function SecurityOverviewCard() {
  const [health, setHealth] = useState<VaultHealth>({
    unlocked: false,
    profiles_count: 0,
    last_rotation_ms: null,
    rotation_overdue: false,
  });
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const poll = async () => {
      try {
        // Prefer Tauri bridge if present.
        const api = (window as any).solaApp;
        if (api?.getVaultHealth) {
          const res = await api.getVaultHealth();
          if (!mounted) return;
          setHealth(res);
          setError(null);
          return;
        }

        // Fallback (dev/demo): keep a safe default.
        if (!mounted) return;
        setHealth((h) => ({ ...h }));
        setError(null);
      } catch (e: any) {
        if (!mounted) return;
        setError(e?.message || 'Failed to query vault health');
      }
    };

    poll();
    const t = setInterval(poll, 10_000);
    return () => {
      mounted = false;
      clearInterval(t);
    };
  }, []);

  const status = useMemo(() => {
    if (error) return { label: 'Unknown', color: 'bg-slate-600' };
    if (!health.unlocked) return { label: 'Locked', color: 'bg-slate-700' };
    if (health.rotation_overdue) return { label: 'Rotate Due', color: 'bg-amber-600' };
    return { label: 'Healthy', color: 'bg-green-600' };
  }, [error, health.rotation_overdue, health.unlocked]);

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-lg text-primary">security</span>
          <span className="font-bold text-white">Vault Security</span>
        </div>
        <span className={`text-[10px] px-2 py-1 rounded-full text-white ${status.color}`}>{status.label}</span>
      </div>

      {error ? (
        <div className="text-xs text-red-300">{error}</div>
      ) : (
        <div className="space-y-2 text-sm">
          <div className="flex items-center justify-between">
            <span className="text-gray-400">Unlocked</span>
            <span className="text-white font-semibold">{health.unlocked ? 'Yes' : 'No'}</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-gray-400">Encrypted blobs</span>
            <span className="text-white font-semibold">{health.profiles_count}</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-gray-400">Last rotation</span>
            <span className="text-white font-semibold">{formatTs(health.last_rotation_ms)}</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-gray-400">Rotation due</span>
            <span className={`font-semibold ${health.rotation_overdue ? 'text-amber-300' : 'text-green-300'}`}>
              {health.rotation_overdue ? 'Yes' : 'No'}
            </span>
          </div>
        </div>
      )}

      <div className="mt-3 pt-3 border-t border-border-dark text-[10px] text-slate-500">
        Kill-switch triggers on 3 failed unlock attempts.
      </div>
    </div>
  );
}

