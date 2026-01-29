import React, { useEffect, useMemo, useState } from 'react';

type MissionStatus = 'queued' | 'running' | 'completed' | 'failed';

type ScoutMission = {
  mission_id: string;
  title: string;
  query: string;
  status: MissionStatus;
  started_ms: number;
  finished_ms?: number | null;
  enqueued_count: number;
  error?: string | null;
};

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
    thumbnail_url?: string | null;
    transcript_text?: string | null;
  };
};

function fmt(ms?: number | null) {
  if (!ms) return '—';
  try {
    return new Date(ms).toLocaleString();
  } catch {
    return String(ms);
  }
}

async function invokeTauri<T>(cmd: string, args: any): Promise<T> {
  const t = (window as any).__TAURI__?.tauri;
  if (!t?.invoke) throw new Error('Tauri invoke not available');
  return t.invoke(cmd, args);
}

export default function MissionControl() {
  const [missions, setMissions] = useState<ScoutMission[]>([]);
  const [queue, setQueue] = useState<PendingReviewItem[]>([]);
  const [query, setQuery] = useState('4K Cyberpunk aesthetics');
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const pending = useMemo(() => queue.filter((i) => i.status === 'pending'), [queue]);

  const refresh = async () => {
    try {
      setError(null);

      // Missions
      const m = await invokeTauri<ScoutMission[]>('list_scout_missions', {});
      setMissions(m || []);

      // Review Queue
      const q = (window as any).solaApp?.getReviewQueue
        ? await (window as any).solaApp.getReviewQueue()
        : await invokeTauri<PendingReviewItem[]>('get_review_queue', {});
      setQueue(q || []);
    } catch (e: any) {
      setError(e?.message || 'Failed to refresh');
    }
  };

  useEffect(() => {
    refresh();

    // Listen for backend mission/review updates if running under Tauri.
    const ev = (window as any).__TAURI__?.event;
    if (!ev?.listen) return;

    let unlisten1: any;
    let unlisten2: any;
    let unlisten3: any;

    (async () => {
      unlisten1 = await ev.listen('mission_update', () => refresh());
      unlisten2 = await ev.listen('mission_finished', () => refresh());
      unlisten3 = await ev.listen('review_queue_updated', () => refresh());
    })();

    return () => {
      try { unlisten1?.(); } catch {}
      try { unlisten2?.(); } catch {}
      try { unlisten3?.(); } catch {}
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const startMission = async () => {
    setRunning(true);
    try {
      setError(null);
      // apply_filters(resolution, relevance)
      await invokeTauri('apply_filters', { resolution: '1080p', relevance: 0.8 });
      // search_media(query, mode, user_kinks)
      await invokeTauri('search_media', { query, mode: 'Professional', userKinks: [] });
      await refresh();
    } catch (e: any) {
      setError(e?.message || 'Mission failed');
    } finally {
      setRunning(false);
    }
  };

  const accept = async (id: string) => {
    try {
      setError(null);
      await invokeTauri('accept_review_item', { id });
      await refresh();
    } catch (e: any) {
      setError(e?.message || 'Accept failed');
    }
  };

  const discard = async (id: string) => {
    try {
      setError(null);
      await invokeTauri('discard_review_item', { id });
      await refresh();
    } catch (e: any) {
      setError(e?.message || 'Discard failed');
    }
  };

  return (
    <div className="h-full flex flex-col gap-4">
      <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
        <div className="flex items-center justify-between gap-3">
          <div>
            <div className="text-white font-bold flex items-center gap-2">
              <span className="material-symbols-outlined text-primary">science</span>
              Mission Control
            </div>
            <div className="text-xs text-slate-400 mt-1">Scout Agent: media research + review gating</div>
          </div>

          <div className="flex items-center gap-2">
            <button
              className="px-3 py-2 rounded-lg bg-black/30 border border-border-dark text-slate-200 text-xs hover:bg-black/40"
              onClick={refresh}
            >
              Refresh
            </button>
          </div>
        </div>

        <div className="mt-4 flex flex-col md:flex-row gap-2">
          <input
            className="flex-1 bg-black/20 border border-border-dark rounded-lg px-3 py-2 text-xs text-white outline-none"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Scouting query (e.g., 4K Cyberpunk Aesthetics)"
          />
          <button
            className={`px-4 py-2 rounded-lg text-xs font-bold border transition-colors ${
              running ? 'bg-slate-700/40 border-border-dark text-slate-400' : 'bg-primary/20 border-primary/30 text-primary hover:bg-primary/30'
            }`}
            disabled={running}
            onClick={startMission}
          >
            {running ? 'Running…' : 'Start Mission'}
          </button>
        </div>
        {error && <div className="mt-3 text-xs text-rose-300">{error}</div>}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <span className="material-symbols-outlined text-primary">rocket_launch</span>
              <span className="font-bold text-white">Active Missions</span>
            </div>
            <span className="text-[10px] px-2 py-1 rounded-full bg-primary/20 text-primary">
              {missions.length} total
            </span>
          </div>

          {missions.length === 0 ? (
            <div className="text-xs text-slate-400">No missions yet.</div>
          ) : (
            <div className="space-y-3">
              {missions.slice(0, 8).map((m) => (
                <div key={m.mission_id} className="border border-border-dark rounded-lg p-3 bg-black/20">
                  <div className="flex items-center justify-between gap-2">
                    <div className="text-xs font-semibold text-white truncate" title={m.title}>
                      {m.title}
                    </div>
                    <span className={`text-[10px] px-2 py-1 rounded-full border ${
                      m.status === 'running'
                        ? 'border-blue-500/30 text-blue-400 bg-blue-500/10'
                        : m.status === 'completed'
                        ? 'border-emerald-500/30 text-emerald-400 bg-emerald-500/10'
                        : m.status === 'failed'
                        ? 'border-rose-500/30 text-rose-400 bg-rose-500/10'
                        : 'border-slate-500/30 text-slate-400 bg-slate-500/10'
                    }`}
                    >
                      {m.status}
                    </span>
                  </div>
                  <div className="mt-1 text-[10px] text-slate-500 font-mono truncate" title={m.query}>
                    {m.query}
                  </div>
                  <div className="mt-2 text-[10px] text-slate-600 flex items-center justify-between">
                    <span>Start: {fmt(m.started_ms)}</span>
                    <span>Finish: {fmt(m.finished_ms || null)}</span>
                  </div>
                  {m.status === 'completed' && (
                    <div className="mt-2 text-[10px] text-slate-400">Enqueued: {m.enqueued_count}</div>
                  )}
                  {m.error && <div className="mt-2 text-[10px] text-rose-300">{m.error}</div>}
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <span className="material-symbols-outlined text-primary">inbox</span>
              <span className="font-bold text-white">Review Queue</span>
            </div>
            <span className="text-[10px] px-2 py-1 rounded-full bg-primary/20 text-primary">
              {pending.length} pending
            </span>
          </div>

          {pending.length === 0 ? (
            <div className="text-xs text-slate-400">No findings awaiting review.</div>
          ) : (
            <div className="space-y-3">
              {pending.slice(0, 6).map((it) => (
                <div key={it.id} className="border border-border-dark rounded-lg p-3 bg-black/20">
                  <div className="flex items-center justify-between gap-2">
                    <div className="text-xs font-semibold text-white truncate" title={it.candidate.title}>
                      {it.candidate.title}
                    </div>
                    <span className="text-[10px] text-slate-500 font-mono">rel={(it.candidate.relevance * 100).toFixed(0)}%</span>
                  </div>
                  <div className="mt-1 text-[10px] text-slate-500 font-mono truncate" title={it.candidate.url}>
                    {it.candidate.source_domain} • {it.candidate.resolution}
                  </div>
                  {it.candidate.transcript_text && (
                    <div className="mt-2 text-[10px] text-slate-400 line-clamp-3 whitespace-pre-wrap">
                      {it.candidate.transcript_text.slice(0, 320)}
                      {it.candidate.transcript_text.length > 320 ? '…' : ''}
                    </div>
                  )}
                  <div className="mt-3 flex items-center gap-2">
                    <button
                      className="px-3 py-1.5 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-[10px] font-bold hover:bg-emerald-500/20"
                      onClick={() => accept(it.id)}
                    >
                      Approve for Vault
                    </button>
                    <button
                      className="px-3 py-1.5 rounded-lg bg-rose-500/10 border border-rose-500/30 text-rose-300 text-[10px] font-bold hover:bg-rose-500/20"
                      onClick={() => discard(it.id)}
                    >
                      Discard
                    </button>
                    <span className="ml-auto text-[10px] text-slate-600">Added: {fmt(it.added_ms)}</span>
                  </div>
                </div>
              ))}
              {pending.length > 6 && (
                <div className="text-[10px] text-slate-500">+{pending.length - 6} more…</div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

