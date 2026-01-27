import React, { useMemo, useState } from 'react';
import { getPhoenixApiBase } from '../env';

type GriefStage = 'Denial' | 'Anger' | 'Bargaining' | 'Depression' | 'Acceptance';

const TAGS = ['Work', 'Social', 'Health', 'Internal', 'Partner'] as const;

export default function L9EntryAdvanced(props: { onLogged?: () => void }) {
  const { onLogged } = props;
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [stage, setStage] = useState<GriefStage>('Acceptance');
  const [intensity, setIntensity] = useState(55);
  const [energy, setEnergy] = useState(60);
  const [text, setText] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const toggleTag = (t: string) => {
    setTags((prev) => (prev.includes(t) ? prev.filter((x) => x !== t) : [...prev, t]));
  };

  const submit = async () => {
    try {
      setSaving(true);
      setError(null);
      const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/events`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          stage,
          intensity,
          energy_level: energy,
          context_tags: tags,
          text: text.trim() ? text.trim() : undefined,
        }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setText('');
      setTags([]);
      onLogged?.();
    } catch (e: any) {
      setError(e?.message || 'Failed to log entry');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/50 p-4">
      <div className="flex items-start justify-between gap-4">
        <div>
          <div className="text-[10px] text-slate-500 uppercase tracking-widest">L9 entry</div>
          <div className="text-sm text-slate-200 mt-1">High-resolution emotional check-in</div>
        </div>
        <button
          onClick={submit}
          disabled={saving}
          className="px-3 py-1.5 rounded-full bg-primary/15 border border-primary/30 text-primary text-[10px] font-bold uppercase tracking-widest hover:bg-primary/20 disabled:opacity-50"
        >
          {saving ? 'Saving…' : 'Log Entry'}
        </button>
      </div>

      <div className="mt-3 grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="space-y-2">
          <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400">Stage</label>
          <select
            value={stage}
            onChange={(e) => setStage(e.target.value as GriefStage)}
            className="w-full rounded-xl bg-black/30 border border-border-dark px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40"
          >
            <option value="Denial">Denial</option>
            <option value="Anger">Anger</option>
            <option value="Bargaining">Bargaining</option>
            <option value="Depression">Depression</option>
            <option value="Acceptance">Acceptance</option>
          </select>

          <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400 mt-3">Intensity</label>
          <div className="flex items-center gap-3">
            <input
              type="range"
              min={0}
              max={100}
              value={intensity}
              onChange={(e) => setIntensity(Number(e.target.value))}
              className="w-full"
            />
            <div className="w-12 text-right text-xs font-mono text-slate-300">{intensity}%</div>
          </div>
        </div>

        <div className="space-y-2">
          <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400">Energy</label>
          <div className="flex items-center gap-4">
            <div className="h-28 flex items-center">
              <input
                type="range"
                min={0}
                max={100}
                value={energy}
                onChange={(e) => setEnergy(Number(e.target.value))}
                className="h-28"
                // Use standards-based writing-mode value; 'bt-lr' is legacy and not in csstype's WritingMode.
                style={{ writingMode: 'vertical-rl', WebkitAppearance: 'slider-vertical' as any }}
              />
            </div>
            <div>
              <div className="text-xs font-mono text-slate-300">{energy}%</div>
              <div className="text-[10px] uppercase tracking-widest text-slate-500">capacity</div>
            </div>
          </div>

          <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400 mt-3">Context tags</label>
          <div className="flex flex-wrap gap-2">
            {TAGS.map((t) => {
              const on = tags.includes(t);
              return (
                <button
                  key={t}
                  onClick={() => toggleTag(t)}
                  className={`px-2 py-1 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
                    on
                      ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-200'
                      : 'bg-black/30 border-border-dark text-slate-400 hover:text-white'
                  }`}
                  type="button"
                >
                  {t}
                </button>
              );
            })}
          </div>
        </div>

        <div className="space-y-2">
          <label className="block text-[10px] font-bold uppercase tracking-widest text-slate-400">Notes (optional)</label>
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={6}
            className="w-full rounded-xl bg-black/30 border border-border-dark px-3 py-2 text-sm text-slate-200 placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/40"
            placeholder="What happened / what triggered this?"
          />
          {error && <div className="text-xs text-rose-300">{error}</div>}
        </div>
      </div>
    </div>
  );
}

