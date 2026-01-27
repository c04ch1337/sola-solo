import React, { useEffect, useMemo, useState } from 'react';
import { useSetAtom } from 'jotai';
import { getPhoenixApiBase } from '../env';
import { reframingAvailableAtom } from '../stores/counselorStore';

type ReframeResponse = {
  success: boolean;
  fixed_belief: string;
  growth_reframe: string;
  evidence?: string[];
  lessons_used?: number;
};

export default function NarrativeReframer() {
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);
  const setReframingAvailable = useSetAtom(reframingAvailableAtom);

  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [data, setData] = useState<ReframeResponse | null>(null);
  const [adopted, setAdopted] = useState(false);

  const load = async () => {
    setError(null);
    setLoading(true);
    try {
      const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/narrative/reframe`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const json = (await res.json()) as ReframeResponse;
      setData(json);

      // User has now "seen" the available reframe.
      if (json?.success) setReframingAvailable(false);
    } catch (e: any) {
      setError(e?.message || 'Failed to load reframe');
      setData(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [PHOENIX_API_BASE]);

  const adopt = async () => {
    if (!data?.growth_reframe) return;
    setSaving(true);
    setError(null);
    try {
      // Phase 19: Update scratchpad via POST /api/memory/reconstruct (manual note mode).
      const note = `Adopted Growth Reframe:\n${data.growth_reframe.trim()}`;
      const res = await fetch(`${PHOENIX_API_BASE}/api/memory/reconstruct`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ note }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setAdopted(true);
    } catch (e: any) {
      setError(e?.message || 'Failed to adopt reframe');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/50 p-4">
      <div className="flex items-start justify-between gap-3">
        <div>
          <div className="text-[10px] text-slate-500 uppercase tracking-widest">Suggested Action</div>
          <h2 className="text-sm font-bold text-white uppercase tracking-wider mt-1 flex items-center gap-2">
            <span className="material-symbols-outlined text-[18px] text-amber-200">replay</span>
            Narrative Reframer
          </h2>
          <div className="text-[10px] text-slate-500 uppercase tracking-widest mt-1">
            Fixed Belief → Growth Reframe
            {typeof data?.lessons_used === 'number' ? (
              <span className="ml-2 text-slate-600 font-mono normal-case tracking-normal">
                evidence={data.lessons_used}
              </span>
            ) : null}
          </div>
        </div>

        <button
          onClick={load}
          disabled={loading}
          className={`px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
            loading
              ? 'bg-black/30 border-border-dark text-slate-500'
              : 'bg-black/30 border-border-dark text-slate-300 hover:text-white hover:bg-panel-dark'
          }`}
          title="Refresh reframe"
        >
          {loading ? 'Loading…' : 'Refresh'}
        </button>
      </div>

      {error ? <div className="mt-2 text-xs text-rose-300">{error}</div> : null}

      {data?.success ? (
        <div className="mt-3 space-y-3">
          {/* Split-view: Old Narrative vs New Narrative */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-3">
            <div className="rounded-xl border border-rose-500/30 bg-rose-500/10 p-3">
              <div className="text-[10px] font-bold uppercase tracking-widest text-rose-200">Old Narrative</div>
              <div className="mt-1 text-sm text-rose-100 leading-relaxed line-through decoration-rose-300/80">
                {data.fixed_belief || '—'}
              </div>
            </div>

            <div className="rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-3 shadow-[0_0_28px_rgba(16,185,129,0.18)]">
              <div className="text-[10px] font-bold uppercase tracking-widest text-emerald-200">New Narrative</div>
              <div className="mt-1 text-sm text-emerald-100 leading-relaxed">{data.growth_reframe || '—'}</div>
            </div>
          </div>

          {Array.isArray(data.evidence) && data.evidence.length ? (
            <div className="rounded-xl border border-border-dark bg-black/20 p-3">
              <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Evidence</div>
              <div className="mt-2 space-y-1">
                {data.evidence.slice(0, 4).map((e, idx) => (
                  <div key={idx} className="text-xs text-slate-200 leading-relaxed">
                    • {e}
                  </div>
                ))}
              </div>
            </div>
          ) : null}

          <div className="flex items-center justify-between gap-3">
            <button
              onClick={adopt}
              disabled={saving || adopted || !data.growth_reframe}
              className={`px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
                saving || adopted
                  ? 'bg-black/30 border-border-dark text-slate-500'
                  : 'bg-emerald-500/15 border-emerald-500/30 text-emerald-200 hover:bg-emerald-500/20'
              }`}
              title="Prepend this reframe into vault:global_context"
            >
              {adopted ? 'Adopted' : saving ? 'Saving…' : 'Adopt Reframe'}
            </button>

            <div className="text-[10px] text-slate-500">
              Writes to <span className="font-mono">vault:global_context</span>
            </div>
          </div>
        </div>
      ) : (
        <div className="mt-3 text-xs text-slate-500">{loading ? 'Generating…' : 'No reframe available yet.'}</div>
      )}
    </div>
  );
}

