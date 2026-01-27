import React, { useEffect, useMemo, useRef, useState } from 'react';
import GriefMap, { L9TherapeuticLog } from './GriefMap';
import RelationshipRepairScriptEngine from './RelationshipRepairScriptEngine';
import RelationalGhost from './RelationalGhost';
import CounselorEcho from './CounselorEcho';
import CounselorExport from './CounselorExport';
import L9EntryAdvanced from './L9EntryAdvanced';
import CorrelationChart from './CorrelationChart';
import SemanticScratchpad from './SemanticScratchpad';
import SystemStressOverlay from './SystemStressOverlay';
import NarrativeReframer from './NarrativeReframer';
import { fmtCountdown, useRegulatoryBrake } from '../hooks/useRegulatoryBrake';
import { useAtom } from 'jotai';
import { coolingStateAtom, setCoolingStateAtom } from '../stores/modeStore';
import { getPhoenixApiBase } from '../env';

type GriefAggregate = {
  day: string;
  stage: string;
  count: number;
  average_intensity: number;
  average_energy: number;
};

type GriefStatsResponse = {
  success: boolean;
  window_days: number;
  tag?: string | null;
  stage_counts: Record<string, number>;
  aggregates: GriefAggregate[];
  events?: any[];
};

function defaultMockLogs(): L9TherapeuticLog[] {
  const now = Date.now();
  const day = 24 * 60 * 60 * 1000;
  return [
    { ts: new Date(now - 8 * day).toISOString(), text: "This can't be happening. No way this is real.", mood: -0.4 },
    { ts: new Date(now - 7 * day).toISOString(), text: 'I am so angry that this feels unfair.', mood: -0.2 },
    { ts: new Date(now - 6 * day).toISOString(), text: 'If only I had done things differently… maybe I could have fixed it.', mood: -0.3 },
    { ts: new Date(now - 5 * day).toISOString(), text: 'I feel numb and tired. I cannot get out of bed.', mood: -0.7 },
    { ts: new Date(now - 4 * day).toISOString(), text: 'I feel sad, but I understand what happened.', mood: -0.2 },
    { ts: new Date(now - 3 * day).toISOString(), text: 'I accept that this is part of my story, and I can move forward slowly.', mood: 0.2 },
    { ts: new Date(now - 2 * day).toISOString(), text: 'A little overwhelmed at work, but I want to reconnect tonight.', mood: -0.1 },
    { ts: new Date(now - 1 * day).toISOString(), text: 'I feel calmer. It is what it is. I can live with it.', mood: 0.4 },
  ];
}

export default function CounselorDashboard() {
  const [useMock, setUseMock] = useState(true);
  const [tagFilter, setTagFilter] = useState<string>('');
  const [stats, setStats] = useState<GriefStatsResponse | null>(null);
  const [analysisMode, setAnalysisMode] = useState<'map' | 'analysis'>('map');
  const [echoRefreshKey, setEchoRefreshKey] = useState(0);
  const brake = useRegulatoryBrake();

  // Phase 13: Predictive Cooling (Deep Calm)
  const [coolingState] = useAtom(coolingStateAtom);
  const [, setCoolingState] = useAtom(setCoolingStateAtom);
  const [coolingToast, setCoolingToast] = useState<null | { title: string; body: string }>(null);
  const coolingDebounceRef = useRef<number | null>(null);

  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  // Phase: Mobile-Pairing helper (best-effort URL for the LAN device)
  const mobileBridgeUrl = useMemo(() => {
    // If the desktop UI is already being served via a LAN hostname/IP, reuse it.
    // Otherwise, fall back to a clear placeholder.
    const host = typeof window !== 'undefined' ? window.location.hostname : 'localhost';
    const isLocal = host === 'localhost' || host === '127.0.0.1' || host === '0.0.0.0';
    const pairingHost = isLocal ? '<YOUR-LAN-IP>' : host;
    return `http://${pairingHost}:3000`;
  }, []);

  // In Phase 1 we ship with mock logs.
  // Next step: pull L9 therapeutic memory logs via Phoenix backend.
  const logs = useMemo(() => (useMock ? defaultMockLogs() : []), [useMock]);

  const refreshStats = async () => {
    const qs = new URLSearchParams();
    qs.set('days', '14');
    if (tagFilter) qs.set('tag', tagFilter);
    const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/grief-stats?${qs.toString()}`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = (await res.json()) as GriefStatsResponse;
    setStats(data);
  };

  useEffect(() => {
    if (useMock) return;
    refreshStats().catch(() => {
      setStats(null);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [useMock, tagFilter, PHOENIX_API_BASE]);

  /**
   * Cooling Trigger:
   * Monitor the *latest* event’s `system_load` and `intensity`.
   * If (system_load > 80 && intensity > 75) → enable Deep Calm.
   */
  const latestLoadAndIntensity = useMemo(() => {
    const events = ((stats?.events as any[]) || []).filter(Boolean);
    if (!events.length) return { system_load: 0, intensity: 0 };

    const latest = events.reduce((acc, e) => {
      const ts = Number(e?.timestamp_ms ?? 0);
      const accTs = Number(acc?.timestamp_ms ?? 0);
      return ts >= accTs ? e : acc;
    }, events[0]);

    return {
      system_load: Math.max(0, Math.min(100, Number(latest?.system_load ?? 0))),
      intensity: Math.max(0, Math.min(100, Number(latest?.intensity ?? 0))),
    };
  }, [stats?.events]);

  useEffect(() => {
    const { system_load, intensity } = latestLoadAndIntensity;
    const shouldCool = system_load > 80 && intensity > 75;
    const next = shouldCool ? 'auto_cooling' : 'off';

    // Debounce to avoid rapid toggling around the threshold (prevents flicker/flash).
    if (coolingDebounceRef.current) {
      window.clearTimeout(coolingDebounceRef.current);
      coolingDebounceRef.current = null;
    }

    const delayMs = next === 'auto_cooling' ? 700 : 1200;
    coolingDebounceRef.current = window.setTimeout(() => {
      if (next === coolingState) return;

      // Toggle the root class via store (and defensively, directly).
      setCoolingState(next);
      document.documentElement.classList.toggle('mode-deep-calm', next === 'auto_cooling');

      if (next === 'auto_cooling') {
        setCoolingToast({
          title: 'Cooling Active',
          body: 'Digital friction detected. Switching to Low-Friction Mode. Consider closing non-essential apps.',
        });
      } else {
        setCoolingToast(null);
      }
    }, delayMs);
  }, [coolingState, latestLoadAndIntensity, setCoolingState]);

  useEffect(() => {
    if (!coolingToast) return;
    const t = window.setTimeout(() => setCoolingToast(null), 7000);
    return () => window.clearTimeout(t);
  }, [coolingToast]);

  return (
    <div className="h-full flex flex-col overflow-hidden">
      {coolingToast ? (
        <div className="pointer-events-none fixed top-20 right-4 z-[9999]">
          <div className="pointer-events-auto w-[300px] max-w-[calc(100vw-2rem)] rounded-2xl border border-border-dark bg-black/80 backdrop-blur px-4 py-3 shadow-2xl">
            <div className="flex items-start gap-3">
              <span className="material-symbols-outlined text-[18px] text-slate-200">ac_unit</span>
              <div className="min-w-0">
                <div className="text-xs font-bold text-white uppercase tracking-wider">{coolingToast.title}</div>
                <div className="mt-1 text-xs text-slate-300 leading-relaxed">{coolingToast.body}</div>
              </div>
            </div>
          </div>
        </div>
      ) : null}

      <div className="border-b border-border-dark p-4 flex items-center gap-4 bg-background-dark/60 backdrop-blur">
        <div>
          <h1 className="text-lg font-bold text-white flex items-center gap-2">
            <span className="material-symbols-outlined text-primary">spa</span>
            Counselor Dashboard
          </h1>
          <p className="text-[10px] text-slate-500 uppercase tracking-[0.22em]">
            Safe-space tools • grief mapping • relationship repair scripting
          </p>
        </div>

        <div className="ml-auto flex items-center gap-2">
          <div
            className={`px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest ${
              brake.blocked
                ? 'bg-rose-500/10 border-rose-500/30 text-rose-200 animate-pulse'
                : 'bg-emerald-500/10 border-emerald-500/30 text-emerald-200'
            }`}
            title={brake.blocked ? 'Regulatory brake active (flooding pause)' : 'Within window of tolerance'}
          >
            {brake.blocked ? `Flooded • ${fmtCountdown(brake.secondsLeft)}` : 'Regulated'}
          </div>

          <CounselorExport />

          <button
            onClick={() => setAnalysisMode((m) => (m === 'map' ? 'analysis' : 'map'))}
            className={`px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
              analysisMode === 'analysis'
                ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-200'
                : 'bg-black/30 border-border-dark text-slate-400 hover:text-white hover:bg-panel-dark'
            }`}
            title="Toggle analysis view"
          >
            {analysisMode === 'analysis' ? 'Analysis: ON' : 'Analysis: OFF'}
          </button>

          <select
            value={tagFilter}
            onChange={(e) => setTagFilter(e.target.value)}
            className="px-2 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest bg-black/30 border-border-dark text-slate-300 focus:outline-none focus:ring-2 focus:ring-primary/40"
            title="Filter grief stats by tag"
          >
            <option value="">All tags</option>
            <option value="Work">Work</option>
            <option value="Social">Social</option>
            <option value="Health">Health</option>
            <option value="Internal">Internal</option>
            <option value="Partner">Partner</option>
          </select>

          <button
            onClick={() => setUseMock((v) => !v)}
            className={`px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors ${
              useMock
                ? 'bg-primary/15 border-primary/30 text-primary'
                : 'bg-black/30 border-border-dark text-slate-400 hover:text-white hover:bg-panel-dark'
            }`}
            title="Toggle mock data"
          >
            {useMock ? 'Mock L9 Logs: ON' : 'Mock L9 Logs: OFF'}
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-auto p-4">
        <div className="max-w-7xl mx-auto grid grid-cols-1 xl:grid-cols-[1fr_360px] gap-4">
          <div className="space-y-4">
            <CounselorEcho refreshKey={echoRefreshKey} />

            <NarrativeReframer />

            <L9EntryAdvanced
              onLogged={() => {
                if (!useMock) refreshStats().catch(() => {});
              }}
            />
            <div className="rounded-2xl border border-border-dark bg-panel-dark/50 p-4">
              <div className="text-[10px] text-slate-500 uppercase tracking-widest">
                Counselor note
              </div>
              <div className="text-sm text-slate-200 mt-1 leading-relaxed">
                This dashboard is designed as a calm, non-judgmental workspace. The grief map is a visualization aid
                (not a diagnosis). Use it to spot trends, triggers, and moments worth revisiting in session.
              </div>
            </div>

          <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
              {analysisMode === 'analysis' && !useMock ? (
                <div className="space-y-4">
                  <SystemStressOverlay events={(stats?.events as any[]) || []} hours={48} />
                  <CorrelationChart events={(stats?.events as any[]) || []} />
                </div>
              ) : (
                <GriefMap logs={logs} stats={useMock ? undefined : stats?.aggregates} />
              )}

              <div className="space-y-4">
                <RelationshipRepairScriptEngine />
                <RelationalGhost />
              </div>
            </div>
          </div>

          <div className="xl:sticky xl:top-4 h-fit">
            <SemanticScratchpad onSettled={() => setEchoRefreshKey((k) => k + 1)} />
          </div>
        </div>

        <div className="max-w-7xl mx-auto mt-4">
          <div className="rounded-2xl border border-border-dark bg-panel-dark/40 px-4 py-3 flex items-center justify-between gap-3">
            <div>
              <div className="text-[10px] text-slate-500 uppercase tracking-widest">Mobile pairing</div>
              <div className="text-xs text-slate-200 mt-1">
                Access Mobile Bridge at{' '}
                <a
                  className="text-primary underline underline-offset-2"
                  href={mobileBridgeUrl}
                  target="_blank"
                  rel="noreferrer"
                >
                  {mobileBridgeUrl}
                </a>
              </div>
              <div className="text-[10px] text-slate-500 mt-1">
                Phoenix API: <span className="font-mono">{PHOENIX_API_BASE}</span>
              </div>
            </div>
            <div className="text-[10px] text-slate-500">Tip: replace &lt;YOUR-LAN-IP&gt; with your PC’s LAN IP</div>
          </div>
        </div>
      </div>
    </div>
  );
}

