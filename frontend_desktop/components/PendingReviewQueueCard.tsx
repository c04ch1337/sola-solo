import React, { useEffect, useMemo, useState } from 'react';

type ReviewStatus = 'pending' | 'approved' | 'rejected';

type PendingReviewItem = {
  id: string;
  status: ReviewStatus;
  added_ms: number;
  candidate: {
    url: string;
    title: string;
    source_domain: string;
    resolution: string;
    relevance: number;
    mood_tags: string[];
    kink_mapping: string[];
  };
};

function fmt(ms: number) {
  try {
    return new Date(ms).toLocaleString();
  } catch {
    return String(ms);
  }
}

export default function PendingReviewQueueCard() {
  const [items, setItems] = useState<PendingReviewItem[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const poll = async () => {
      try {
        const api = (window as any).solaApp;
        if (api?.getReviewQueue) {
          const res = await api.getReviewQueue();
          if (!mounted) return;
          setItems(res || []);
          setError(null);
          return;
        }

        // Fallback in dev: keep empty.
        if (!mounted) return;
        setError(null);
      } catch (e: any) {
        if (!mounted) return;
        setError(e?.message || 'Failed to load review queue');
      }
    };

    poll();
    const t = setInterval(poll, 10_000);
    return () => {
      mounted = false;
      clearInterval(t);
    };
  }, []);

  const pending = useMemo(() => items.filter((i) => i.status === 'pending'), [items]);

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-lg text-primary">inbox</span>
          <span className="font-bold text-white">Pending Review</span>
        </div>
        <span className="text-[10px] px-2 py-1 rounded-full bg-primary/20 text-primary">
          {pending.length} pending
        </span>
      </div>

      {error ? (
        <div className="text-xs text-red-300">{error}</div>
      ) : pending.length === 0 ? (
        <div className="text-xs text-slate-400">No items awaiting review.</div>
      ) : (
        <div className="space-y-3">
          {pending.slice(0, 3).map((it) => (
            <div key={it.id} className="border border-border-dark rounded-lg p-3 bg-black/20">
              <div className="text-xs font-semibold text-white truncate" title={it.candidate.title}>
                {it.candidate.title}
              </div>
              <div className="mt-1 text-[10px] text-slate-500 font-mono flex items-center justify-between gap-2">
                <span className="truncate" title={it.candidate.url}>
                  {it.candidate.source_domain} • {it.candidate.resolution}
                </span>
                <span>rel={(it.candidate.relevance * 100).toFixed(0)}%</span>
              </div>
              <div className="mt-2 text-[10px] text-slate-400">
                Mood: {it.candidate.mood_tags.join(', ') || '—'}
              </div>
              {it.candidate.kink_mapping.length > 0 && (
                <div className="mt-1 text-[10px] text-slate-400">
                  Matches: {it.candidate.kink_mapping.join(', ')}
                </div>
              )}
              <div className="mt-2 text-[10px] text-slate-600">Added: {fmt(it.added_ms)}</div>
            </div>
          ))}
          {pending.length > 3 && (
            <div className="text-[10px] text-slate-500">+{pending.length - 3} more…</div>
          )}
        </div>
      )}
    </div>
  );
}

