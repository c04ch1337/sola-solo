import React, { useEffect, useMemo, useState } from 'react';
import { getPhoenixApiBase } from '../env';

type NarrativeResponse = {
  success: boolean;
  narrative: string;
  window_days: number;
  stage_counts?: Record<string, number>;
};

type CorrelationsResponse = {
  success: boolean;
  window_days: number;
  total_events: number;
  top_trigger?: {
    tag: string;
    frequency: number;
    avg_intensity: number;
    avg_energy: number;
    corr_energy_intensity: number;
    impact: string;
    risk_score: number;
  } | null;
};

export default function CounselorEcho(props: { refreshKey?: number }) {
  const { refreshKey } = props;
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [narrative, setNarrative] = useState<string>('');
  const [trigger, setTrigger] = useState<CorrelationsResponse['top_trigger']>(null);

  useEffect(() => {
    let alive = true;
    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/narrative?days=7`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const data = (await res.json()) as NarrativeResponse;
        if (!alive) return;
        setNarrative(data.narrative || '');

        // Trigger correlations (best-effort; don't fail the card)
        fetch(`${PHOENIX_API_BASE}/api/counselor/analytics/correlations?days=14`)
          .then((r) => (r.ok ? r.json() : null))
          .then((j) => {
            if (!alive || !j) return;
            const c = j as CorrelationsResponse;
            setTrigger(c.top_trigger || null);
          })
          .catch(() => {
            // ignore
          });
      } catch (e: any) {
        if (!alive) return;
        setError(e?.message || 'Failed to load narrative');
      } finally {
        if (!alive) return;
        setLoading(false);
      }
    })();
    return () => {
      alive = false;
    };
  }, [PHOENIX_API_BASE, refreshKey]);

  return (
    <div className="rounded-2xl border border-emerald-500/20 bg-emerald-500/5 p-4">
      <div className="flex items-start justify-between gap-4">
        <div>
          <div className="text-[10px] font-bold uppercase tracking-widest text-emerald-200/90">
            Echo • weekly reflection
          </div>
          <div className="text-[10px] uppercase tracking-[0.22em] text-slate-500 mt-0.5">
            7-day synthesis (supportive heuristic)
          </div>
        </div>
        <div className="text-[10px] font-mono text-slate-500">
          {loading ? 'Loading…' : error ? 'Offline' : 'Ready'}
        </div>
      </div>

      <div className="mt-2 text-sm text-slate-200 leading-relaxed">
        {loading ? (
          <div className="opacity-70">
            Generating reflection from recent grief-map signals…
          </div>
        ) : error ? (
          <div className="text-slate-400">
            Narrative unavailable ({error}).
          </div>
        ) : (
          <div>
            <div>{narrative}</div>
            {trigger && trigger.risk_score >= 70 && (
              <div className="mt-3 rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-xs text-amber-100">
                <div className="font-bold uppercase tracking-widest text-[10px]">Trigger Alert</div>
                <div className="mt-1">
                  <span className="font-bold">#{trigger.tag}</span> is showing a repeated “{trigger.impact}” pattern
                  (risk {trigger.risk_score}%, n={trigger.frequency}). Consider running a Readiness cool-down before
                  high-stakes conversations in this context.
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}


