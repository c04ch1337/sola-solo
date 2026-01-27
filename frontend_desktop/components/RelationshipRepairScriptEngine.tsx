import React, { useMemo, useState } from 'react';
import ResonanceSimulator from './ResonanceSimulator';
import ReadinessCheck from './ReadinessCheck';
import { fmtCountdown, useRegulatoryBrake } from '../hooks/useRegulatoryBrake';
import { getPhoenixApiBase } from '../env';

type NvcScript = {
  observation: string;
  feeling: string;
  need: string;
  request: string;
};

function sanitizeLine(s: string) {
  return s.replace(/\s+/g, ' ').trim();
}

function guessFeeling(stress: string) {
  const t = stress.toLowerCase();
  if (/(overwhelm|overwhelmed|too much|burnout)/.test(t)) return 'overwhelmed';
  if (/(anxious|anxiety|worry|worried)/.test(t)) return 'anxious';
  if (/(angry|mad|frustrat)/.test(t)) return 'frustrated';
  if (/(sad|down|depress)/.test(t)) return 'sad';
  if (/(tired|exhaust)/.test(t)) return 'tired';
  return 'stressed';
}

function guessNeed(goal: string) {
  const t = goal.toLowerCase();
  if (/(time|schedule|space|alone)/.test(t)) return 'space and predictability';
  if (/(support|help|team|together)/.test(t)) return 'support and partnership';
  if (/(trust|honest|transparen)/.test(t)) return 'trust and clarity';
  if (/(affection|intimacy|close)/.test(t)) return 'connection and closeness';
  if (/(calm|peace|quiet)/.test(t)) return 'calm and reassurance';
  return 'connection and understanding';
}

function buildNvcScript(params: {
  stressLog: string;
  partnerGoal: string;
  tone: 'gentle' | 'direct';
}): NvcScript {
  const stress = sanitizeLine(params.stressLog);
  const goal = sanitizeLine(params.partnerGoal);
  const feeling = guessFeeling(stress);
  const need = guessNeed(goal);

  const observation = params.tone === 'direct'
    ? `When I notice ${stress || 'the stress from work showing up for me today'}`
    : `When I notice ${stress || 'my work stress showing up today'}`;

  const request = params.tone === 'direct'
    ? `Would you be willing to talk with me for 10 minutes tonight about ${goal || 'how we can stay connected'} and decide on one small next step?`
    : `Would you be open to a 10-minute check-in tonight so we can support ${goal || 'our connection'}?`;

  return {
    observation,
    feeling: `I feel ${feeling}`,
    need: `because I need ${need}`,
    request,
  };
}

export default function RelationshipRepairScriptEngine() {
  const [stressLog, setStressLog] = useState('a heavy workload and deadlines piling up');
  const [partnerGoal, setPartnerGoal] = useState('more connection and less tension after work');
  const [tone, setTone] = useState<'gentle' | 'direct'>('gentle');
  const [copied, setCopied] = useState(false);
  const [readinessOpen, setReadinessOpen] = useState(false);
  const [lastResonanceScore, setLastResonanceScore] = useState<number | null>(null);
  const brake = useRegulatoryBrake();

  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const script = useMemo(() => buildNvcScript({ stressLog, partnerGoal, tone }), [stressLog, partnerGoal, tone]);
  const formatted = useMemo(
    () => `${script.observation} -> ${script.feeling} -> ${script.need} -> ${script.request}`,
    [script]
  );

  const doCopy = async () => {
    try {
      await navigator.clipboard.writeText(formatted);

      // Persist to counselor backend (best-effort; never blocks clipboard UX).
      fetch(`${PHOENIX_API_BASE}/api/counselor/scripts`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          observation: script.observation,
          feeling: script.feeling,
          need: script.need,
          request: script.request,
          formatted,
        }),
      }).catch(() => {
        // no-op
      });

      setCopied(true);
      setTimeout(() => setCopied(false), 1600);
    } catch {
      // no-op
    }
  };

  const copy = async () => {
    // Pre-flight interlock: open readiness modal first.
    setReadinessOpen(true);
  };

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/70 p-4">
      <div className="flex items-start justify-between gap-4 mb-3">
        <div>
          <h2 className="text-sm font-bold text-white uppercase tracking-wider">Relationship Repair Script Engine</h2>
          <p className="text-[10px] text-slate-500 uppercase tracking-widest">
            Generate NVC scripts • Professional stress logs + Personal partner goals
          </p>
        </div>

        <div className="flex items-center gap-2">
          <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">Tone</div>
          <button
            onClick={() => setTone((t) => (t === 'gentle' ? 'direct' : 'gentle'))}
            className={`px-3 py-1 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
              tone === 'gentle'
                ? 'bg-primary/15 border-primary/30 text-primary'
                : 'bg-emerald-500/10 border-emerald-500/30 text-emerald-300'
            }`}
            title="Switch tone"
          >
            {tone}
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div className="space-y-3">
          <div>
            <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-1">
              Professional: stress log
            </label>
            <textarea
              value={stressLog}
              onChange={(e) => setStressLog(e.target.value)}
              rows={3}
              className="w-full rounded-xl bg-black/30 border border-border-dark px-3 py-2 text-sm text-slate-200 placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/40"
              placeholder="e.g., nonstop meetings and a deadline tomorrow"
            />
          </div>

          <div>
            <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400 mb-1">
              Personal: partner goal
            </label>
            <textarea
              value={partnerGoal}
              onChange={(e) => setPartnerGoal(e.target.value)}
              rows={3}
              className="w-full rounded-xl bg-black/30 border border-border-dark px-3 py-2 text-sm text-slate-200 placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/40"
              placeholder="e.g., feel close and understood this week"
            />
          </div>
        </div>

        <div className="space-y-3">
          <div className="rounded-xl border border-border-dark bg-black/30 p-3">
            <div className="flex items-center justify-between gap-3">
              <div className="text-[10px] font-bold uppercase tracking-widest text-slate-400">NVC output</div>
              <button
                onClick={copy}
                className={`px-3 py-1 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
                  brake.blocked
                    ? 'bg-rose-500/10 border-rose-500/30 text-rose-200 animate-pulse'
                    : 'bg-primary/15 border-primary/30 text-primary hover:bg-primary/20'
                }`}
                title="Copy script"
              >
                {copied ? 'Copied' : brake.blocked ? `Brake ${fmtCountdown(brake.secondsLeft)}` : 'Copy'}
              </button>
            </div>

            <div className="mt-2 text-sm text-slate-200 leading-relaxed">
              <div className="mb-2">
                <span className="text-slate-500 font-mono text-[11px]">[Observation]</span>
                <div className="mt-0.5">{script.observation}</div>
              </div>
              <div className="mb-2">
                <span className="text-slate-500 font-mono text-[11px]">[Feeling]</span>
                <div className="mt-0.5">{script.feeling}</div>
              </div>
              <div className="mb-2">
                <span className="text-slate-500 font-mono text-[11px]">[Need]</span>
                <div className="mt-0.5">{script.need}</div>
              </div>
              <div>
                <span className="text-slate-500 font-mono text-[11px]">[Request]</span>
                <div className="mt-0.5">{script.request}</div>
              </div>
            </div>
          </div>

          <ResonanceSimulator script={formatted} tone={tone} />

          <ReadinessCheck
            open={readinessOpen}
            onClose={() => setReadinessOpen(false)}
            onProceed={async () => {
              setReadinessOpen(false);
              await doCopy();
            }}
            stressLog={stressLog}
            resonanceScore={lastResonanceScore}
          />

          <div className="text-[10px] text-slate-500 uppercase tracking-widest">
            Format: [Observation] -&gt; [Feeling] -&gt; [Need] -&gt; [Request]
          </div>
        </div>
      </div>
    </div>
  );
}


