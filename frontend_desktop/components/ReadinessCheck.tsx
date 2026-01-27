import React, { useEffect, useMemo, useState } from 'react';
import { fmtCountdown, useIntervention, useRegulatoryBrake } from '../hooks/useRegulatoryBrake';
import { getPhoenixApiBase } from '../env';

type ReadinessResponse = {
  ready: boolean;
  readiness_score: number;
  window_status: 'Green' | 'Yellow' | 'Red' | string;
  reasons: string[];
  cooldown_seconds: number;
  evaluated_at_ms: number;
};

function fmtTime(sec: number) {
  const s = Math.max(0, Math.floor(sec));
  const m = Math.floor(s / 60);
  const r = s % 60;
  if (m <= 0) return `${r}s`;
  return `${m}m ${r}s`;
}

export default function ReadinessCheck(props: {
  open: boolean;
  onClose: () => void;
  onProceed: () => void;
  stressLog: string;
  resonanceScore?: number | null;
}) {
  const { open, onClose, onProceed, stressLog, resonanceScore } = props;
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [answers, setAnswers] = useState({ hungry: false, angry: false, lonely: false, tired: false });
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<ReadinessResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [cooldownLeft, setCooldownLeft] = useState(0);
  const [riskScore, setRiskScore] = useState<number | null>(null);
  const brake = useRegulatoryBrake();
  const intervention = useIntervention(riskScore);

  useEffect(() => {
    if (!open) return;
    setAnswers({ hungry: false, angry: false, lonely: false, tired: false });
    setData(null);
    setError(null);
    setCooldownLeft(0);
    setRiskScore(null);
  }, [open]);

  useEffect(() => {
    if (cooldownLeft <= 0) return;
    const t = setInterval(() => setCooldownLeft((s) => Math.max(0, s - 1)), 1000);
    return () => clearInterval(t);
  }, [cooldownLeft]);

  const windowBadge = (w: string) => {
    if (w === 'Green') return 'bg-emerald-500/10 border-emerald-500/30 text-emerald-200';
    if (w === 'Yellow') return 'bg-amber-500/10 border-amber-500/30 text-amber-200';
    return 'bg-rose-500/10 border-rose-500/30 text-rose-200';
  };

  const compute = async () => {
    try {
      setLoading(true);
      setError(null);

      // Encode the 3 rapid-fire questions into a single stress-log augmentation.
      const haltSuffix = ` HALT(self-check): hungry=${answers.hungry} angry=${answers.angry} lonely=${answers.lonely} tired=${answers.tired}`;
      const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/readiness`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ stress_log: `${stressLog}${haltSuffix}` }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const json = (await res.json()) as ReadinessResponse;
      setData(json);

      // Convert readiness → risk score (simple inverse mapping).
      const r = Math.min(100, Math.max(0, 100 - Math.round(json.readiness_score || 0)));
      setRiskScore(r);

      if (!json.ready && json.cooldown_seconds) {
        setCooldownLeft(json.cooldown_seconds);
      }

      // Regulatory Brake:
      // If the user is out of their window of tolerance, enforce a 20-minute pause
      // (or the backend cooldown, whichever is larger) and surface a grounding exercise.
      if (!json.ready) {
        const enforced = json.window_status === 'Red' ? 20 * 60 : 0;
        const total = Math.max(enforced, json.cooldown_seconds || 0);
        if (total > 0) brake.startBrake(total);
      }
    } catch (e: any) {
      setError(e?.message || 'Failed to run readiness check');
    } finally {
      setLoading(false);
    }
  };

  if (!open) return null;

  const resonanceLowReadiness =
    typeof resonanceScore === 'number' && resonanceScore >= 80 && data && !data.ready;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div className="w-full max-w-xl rounded-2xl border border-border-dark bg-panel-dark/90 shadow-2xl">
        <div className="p-4 border-b border-border-dark flex items-start justify-between gap-4">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider">Pre-Flight Readiness Check</div>
            <div className="text-[10px] uppercase tracking-[0.22em] text-slate-500 mt-0.5">
              HALT + flooding screen (offline heuristic)
            </div>
          </div>
          <button
            onClick={onClose}
            className="px-3 py-1 rounded-full bg-black/30 border border-border-dark text-slate-300 text-[10px] font-bold uppercase tracking-widest hover:bg-black/40"
          >
            Close
          </button>
        </div>

        <div className="p-4 space-y-4">
          <div className="rounded-xl border border-border-dark bg-black/20 p-3">
            <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">3 rapid-fire questions</div>
            <div className="mt-2 grid grid-cols-2 gap-2 text-xs text-slate-200">
              {(
                [
                  ['hungry', 'Hungry'],
                  ['angry', 'Angry'],
                  ['lonely', 'Lonely'],
                  ['tired', 'Tired'],
                ] as const
              ).map(([k, label]) => (
                <label key={k} className="flex items-center gap-2 rounded-lg border border-border-dark bg-black/20 px-3 py-2">
                  <input
                    type="checkbox"
                    checked={answers[k]}
                    onChange={(e) => setAnswers((a) => ({ ...a, [k]: e.target.checked }))}
                  />
                  <span>{label}</span>
                </label>
              ))}
            </div>
            <div className="mt-3 flex items-center gap-2">
              <button
                onClick={compute}
                disabled={loading}
                className="px-3 py-1 rounded-full bg-primary/15 border border-primary/30 text-primary text-[10px] font-bold uppercase tracking-widest hover:bg-primary/20 disabled:opacity-50"
              >
                {loading ? 'Checking…' : 'Run Check'}
              </button>
              {cooldownLeft > 0 && (
                <div className="text-[10px] font-mono text-slate-400">
                  Cool-down: {fmtTime(cooldownLeft)}
                </div>
              )}
            </div>
          </div>

          {error && <div className="text-xs text-rose-300">{error}</div>}

          {data && (
            <div className="rounded-xl border border-border-dark bg-black/20 p-3">
              <div className="flex items-center justify-between gap-3">
                <div className="text-xs text-slate-200">
                  Readiness: <span className="font-bold">{data.readiness_score}%</span>
                </div>
                <div className={`px-3 py-1 rounded-full border text-[10px] font-bold uppercase tracking-widest ${windowBadge(data.window_status)}`}>
                  {data.window_status}
                </div>
              </div>
              <ul className="mt-2 text-xs text-slate-300 list-disc pl-4 space-y-1">
                {data.reasons.slice(0, 4).map((r, i) => (
                  <li key={i}>{r}</li>
                ))}
              </ul>

              {resonanceLowReadiness && (
                <div className="mt-3 rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-xs text-amber-100">
                  Your script is strong (high Resonance), but your internal battery is low. Consider sending this tomorrow
                  morning or after the cool-down window.
                </div>
              )}

              {brake.blocked && (
                <div className="mt-3 rounded-xl border border-rose-500/30 bg-rose-500/10 p-3 text-xs text-rose-100">
                  <div className="flex items-center justify-between gap-2">
                    <div className="font-bold uppercase tracking-widest text-[10px]">Regulatory Brake • flooding pause</div>
                    <div className="font-mono text-[10px] text-rose-200/90">{fmtCountdown(brake.secondsLeft)}</div>
                  </div>
                  <div className="mt-1 text-rose-100/90">
                    This is a protective pause to avoid sending messages while emotionally flooded.
                  </div>

                  {intervention.loading ? (
                    <div className="mt-2 text-rose-100/70">Loading grounding exercise…</div>
                  ) : intervention.error ? (
                    <div className="mt-2 text-rose-100/70">Grounding exercise unavailable ({intervention.error}).</div>
                  ) : intervention.data ? (
                    <div className="mt-2 rounded-lg border border-rose-500/20 bg-black/20 p-3">
                      <div className="text-[10px] font-bold uppercase tracking-widest text-rose-200">{intervention.data.title}</div>
                      <div className="mt-1 text-xs text-rose-50/90 leading-relaxed">{intervention.data.exercise}</div>
                      <div className="mt-2 text-[10px] font-mono text-rose-200/80">
                        Suggested: {fmtCountdown(intervention.data.recommended_seconds)}
                      </div>
                    </div>
                  ) : null}
                </div>
              )}
            </div>
          )}
        </div>

        <div className="p-4 border-t border-border-dark flex items-center justify-end gap-2">
          <button
            onClick={onClose}
            className="px-3 py-1 rounded-full bg-black/30 border border-border-dark text-slate-300 text-[10px] font-bold uppercase tracking-widest hover:bg-black/40"
          >
            Cancel
          </button>
          <button
            onClick={onProceed}
            disabled={cooldownLeft > 0 || brake.blocked}
            className="px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-200 text-[10px] font-bold uppercase tracking-widest hover:bg-emerald-500/15 disabled:opacity-50"
          >
            Proceed to Copy
          </button>
        </div>
      </div>
    </div>
  );
}


