import React, { useEffect, useMemo, useRef, useState } from 'react';
import { getPhoenixApiBase } from '../env';

type NotesGetResponse = {
  success: boolean;
  key: string;
  note: string;
};

type NotesPostResponse = {
  success: boolean;
  key: string;
  note: string;
};

export default function SemanticScratchpad(props: { onSettled?: () => void }) {
  const { onSettled } = props;
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [note, setNote] = useState('');
  const [lastSavedAt, setLastSavedAt] = useState<number | null>(null);

  const saveTimerRef = useRef<number | null>(null);
  const lastSavedNoteRef = useRef<string>('');
  const settledTimerRef = useRef<number | null>(null);
  const didHydrateRef = useRef<boolean>(false);

  const saveNow = async (nextNote: string) => {
    // Avoid noisy writes.
    if (nextNote === lastSavedNoteRef.current) return;

    try {
      setSaving(true);
      setError(null);

      const res = await fetch(`${PHOENIX_API_BASE}/api/memory/notes`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ note: nextNote }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);

      const data = (await res.json()) as NotesPostResponse;
      if (!data?.success) throw new Error('Save failed');

      lastSavedNoteRef.current = data.note ?? nextNote;
      setLastSavedAt(Date.now());
    } catch (e: any) {
      setError(e?.message || 'Failed to save');
    } finally {
      setSaving(false);
    }
  };

  const scheduleSave = (nextNote: string) => {
    if (saveTimerRef.current) {
      window.clearTimeout(saveTimerRef.current);
    }
    saveTimerRef.current = window.setTimeout(() => {
      saveNow(nextNote).catch(() => {});
    }, 800);
  };

  useEffect(() => {
    let alive = true;
    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await fetch(`${PHOENIX_API_BASE}/api/memory/notes`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const data = (await res.json()) as NotesGetResponse;
        if (!alive) return;
        const initial = data?.note || '';
        setNote(initial);
        lastSavedNoteRef.current = initial;
        didHydrateRef.current = true;
      } catch (e: any) {
        if (!alive) return;
        setError(e?.message || 'Failed to load');
      } finally {
        if (!alive) return;
        setLoading(false);
      }
    })();

    return () => {
      alive = false;
      if (saveTimerRef.current) {
        window.clearTimeout(saveTimerRef.current);
      }
      if (settledTimerRef.current) {
        window.clearTimeout(settledTimerRef.current);
      }
    };
  }, [PHOENIX_API_BASE]);

  // Debounced "settled" callback: when the user stops typing, ask the Echo layer
  // to re-fetch so it incorporates the updated semantic context.
  useEffect(() => {
    if (!didHydrateRef.current) return;
    if (!onSettled) return;

    if (settledTimerRef.current) {
      window.clearTimeout(settledTimerRef.current);
    }
    settledTimerRef.current = window.setTimeout(() => {
      onSettled();
    }, 1500);

    return () => {
      if (settledTimerRef.current) {
        window.clearTimeout(settledTimerRef.current);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [note]);

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/50 p-4">
      <div className="flex items-start justify-between gap-4">
        <div>
          <div className="text-[10px] font-bold uppercase tracking-widest text-slate-300">
            AGI Long-Term Context
          </div>
          <div className="text-[10px] uppercase tracking-[0.22em] text-slate-500 mt-0.5">
            Current life chapter • persistent semantic memory
          </div>
        </div>
        <div className="text-[10px] font-mono text-slate-500">
          {loading ? 'Loading…' : saving ? 'Saving…' : error ? 'Offline' : lastSavedAt ? 'Saved' : 'Ready'}
        </div>
      </div>

      {error && (
        <div className="mt-2 rounded-xl border border-amber-500/30 bg-amber-500/10 p-2 text-[11px] text-amber-100">
          {error}
        </div>
      )}

      <textarea
        value={note}
        onChange={(e) => {
          const next = e.target.value;
          setNote(next);
          scheduleSave(next);
        }}
        onBlur={() => {
          // Ensure last edit is persisted.
          if (saveTimerRef.current) {
            window.clearTimeout(saveTimerRef.current);
            saveTimerRef.current = null;
          }
          saveNow(note).catch(() => {});
        }}
        placeholder={
          'Examples:\n' +
          '- 3-week project migration (stressful; tight deadlines)\n' +
          '- Sleeping ~6h; trying to stabilize routine\n' +
          '- Priority: repair conversation with partner on Sunday\n'
        }
        className="mt-3 w-full min-h-[220px] resize-y rounded-xl border border-border-dark bg-black/30 p-3 text-sm text-slate-200 placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/40 font-mono leading-relaxed"
      />

      <div className="mt-2 text-[10px] text-slate-500 uppercase tracking-[0.22em]">
        Auto-saves as you type • Stored in Soul Vault key: <span className="font-mono">vault:global_context</span>
      </div>
    </div>
  );
}
