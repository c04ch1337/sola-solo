import React, { useMemo, useState } from 'react';
import { getPhoenixApiBase } from '../env';

type Persona = 'Secure' | 'Avoidant-Dismissive' | 'Anxious-Preoccupied';

type ResonanceResult = {
  resonance_score: number;
  persona: string;
  likely_response: string;
  flags: string[];
  strengths: string[];
  suggestions: string[];
};

export default function ResonanceSimulator(props: { script: string; tone?: 'gentle' | 'direct' }) {
  const { script, tone } = props;
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [persona, setPersona] = useState<Persona>('Secure');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<ResonanceResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const meterColor = (v: number) => {
    if (v >= 80) return 'bg-emerald-400/30 border-emerald-400/40';
    if (v >= 55) return 'bg-amber-400/20 border-amber-400/30';
    return 'bg-rose-400/20 border-rose-400/30';
  };

  const run = async () => {
    try {
      setLoading(true);
      setError(null);
      setResult(null);

      const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/resonate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ persona, script, tone }),
      });

      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = (await res.json()) as ResonanceResult;
      setResult(data);
    } catch (e: any) {
      setError(e?.message || 'Simulation failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="rounded-2xl border border-border-dark bg-black/20 p-3">
      <div className="flex items-center justify-between gap-3">
        <div>
          <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Relational Resonance</div>
          <div className="text-[10px] uppercase tracking-[0.22em] text-slate-500 mt-0.5">
            dry-run your script against a partner persona
          </div>
        </div>

        <div className="flex items-center gap-2">
          <select
            value={persona}
            onChange={(e) => setPersona(e.target.value as Persona)}
            className="rounded-xl bg-black/30 border border-border-dark px-2 py-1 text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
            title="Partner persona"
          >
            <option value="Secure">Secure</option>
            <option value="Avoidant-Dismissive">Avoidant-Dismissive</option>
            <option value="Anxious-Preoccupied">Anxious-Preoccupied</option>
          </select>

          <button
            onClick={run}
            disabled={loading || !script.trim()}
            className="px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-200 text-[10px] font-bold uppercase tracking-widest hover:bg-emerald-500/15 transition-colors disabled:opacity-50"
            title="Run simulation"
          >
            {loading ? 'Running…' : 'Run Simulation'}
          </button>
        </div>
      </div>

      {error && <div className="mt-2 text-xs text-rose-300">{error}</div>}

      {result && (
        <div className="mt-3 space-y-3">
          <div className={`rounded-xl border ${meterColor(result.resonance_score)} p-3`}>
            <div className="flex items-center justify-between">
              <div className="text-xs text-slate-200">
                Resonance Meter: <span className="font-bold">{result.resonance_score}%</span>
              </div>
              <div className="text-[10px] font-mono text-slate-500">{result.persona}</div>
            </div>
            <div className="mt-2 h-2 rounded bg-black/30 overflow-hidden">
              <div
                className="h-full bg-emerald-400/60"
                style={{ width: `${Math.max(0, Math.min(100, result.resonance_score))}%` }}
              />
            </div>
          </div>

          <div className="rounded-xl border border-border-dark bg-black/30 p-3">
            <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Likely response</div>
            <div className="mt-1 text-sm text-slate-200 leading-relaxed">{result.likely_response}</div>
          </div>

          {(result.flags?.length || result.strengths?.length || result.suggestions?.length) && (
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <div className="rounded-xl border border-border-dark bg-black/20 p-3">
                <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Flags</div>
                <ul className="mt-1 text-xs text-slate-300 list-disc pl-4 space-y-1">
                  {(result.flags || []).slice(0, 4).map((f, i) => (
                    <li key={i}>{f}</li>
                  ))}
                  {(result.flags || []).length === 0 && <li className="list-none text-slate-500">None detected</li>}
                </ul>
              </div>
              <div className="rounded-xl border border-border-dark bg-black/20 p-3">
                <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Strengths</div>
                <ul className="mt-1 text-xs text-slate-300 list-disc pl-4 space-y-1">
                  {(result.strengths || []).slice(0, 4).map((s, i) => (
                    <li key={i}>{s}</li>
                  ))}
                  {(result.strengths || []).length === 0 && <li className="list-none text-slate-500">—</li>}
                </ul>
              </div>
              <div className="rounded-xl border border-border-dark bg-black/20 p-3">
                <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">Suggestions</div>
                <ul className="mt-1 text-xs text-slate-300 list-disc pl-4 space-y-1">
                  {(result.suggestions || []).slice(0, 4).map((s, i) => (
                    <li key={i}>{s}</li>
                  ))}
                  {(result.suggestions || []).length === 0 && <li className="list-none text-slate-500">—</li>}
                </ul>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

