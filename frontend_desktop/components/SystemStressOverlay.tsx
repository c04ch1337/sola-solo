import React, { useMemo } from 'react';
import {
  Area,
  CartesianGrid,
  ComposedChart,
  Line,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

type GriefEvent = {
  timestamp_ms: number;
  intensity: number; // 0..100
  system_load?: number; // 0..100
  temperature_c?: number | null;
};

type Point = {
  ts: number;
  t: string;
  intensity: number;
  system_load: number;
};

function fmtTime(ts: number) {
  try {
    return new Date(ts).toLocaleString(undefined, {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  } catch {
    return '';
  }
}

const OverlayTooltip = ({ active, payload }: any) => {
  if (!active || !payload?.length) return null;
  const p: any = payload[0]?.payload;
  return (
    <div className="rounded-xl border border-border-dark bg-black/80 backdrop-blur px-3 py-2 shadow-xl">
      <div className="text-[10px] font-mono text-slate-400">{p.t}</div>
      <div className="mt-1 text-xs text-slate-200">
        System Load: <span className="font-bold">{p.system_load}%</span>
        <span className="text-slate-500"> • </span>
        Intensity: <span className="font-bold">{p.intensity}%</span>
      </div>
    </div>
  );
};

export default function SystemStressOverlay(props: { events: GriefEvent[]; hours?: number }) {
  const { events, hours = 48 } = props;

  const data: Point[] = useMemo(() => {
    const now = Date.now();
    const start = now - hours * 60 * 60 * 1000;
    const filtered = (events || [])
      .filter((e) => (e.timestamp_ms || 0) >= start)
      .map((e) => ({
        ts: e.timestamp_ms,
        t: fmtTime(e.timestamp_ms),
        intensity: Math.max(0, Math.min(100, Number(e.intensity ?? 0))),
        system_load: Math.max(0, Math.min(100, Number(e.system_load ?? 0))),
      }))
      .sort((a, b) => a.ts - b.ts);
    return filtered;
  }, [events, hours]);

  const avgLoad = useMemo(() => {
    if (!data.length) return 0;
    const s = data.reduce((acc, p) => acc + p.system_load, 0);
    return Math.round(s / data.length);
  }, [data]);

  const avgIntensity = useMemo(() => {
    if (!data.length) return 0;
    const s = data.reduce((acc, p) => acc + p.intensity, 0);
    return Math.round(s / data.length);
  }, [data]);

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/70 p-4">
      <div className="flex items-start justify-between gap-4 mb-3">
        <div>
          <h2 className="text-sm font-bold text-white uppercase tracking-wider">Techno-Somatic Overlay</h2>
          <p className="text-[10px] text-slate-500 uppercase tracking-widest">
            System CPU load (area) vs emotional intensity (line) • last {hours}h
          </p>
        </div>
        <div className="text-[10px] font-mono text-slate-500">
          avg load {avgLoad}% • avg intensity {avgIntensity}% • {data.length} pts
        </div>
      </div>

      {!data.length ? (
        <div className="h-[260px] flex items-center justify-center text-xs text-slate-500">
          No events in the selected window.
        </div>
      ) : (
        <div className="h-[260px]">
          <ResponsiveContainer width="100%" height="100%">
            <ComposedChart data={data} margin={{ top: 10, right: 10, bottom: 20, left: 40 }}>
              <CartesianGrid stroke="rgba(148, 163, 184, 0.12)" vertical horizontal />
              <XAxis
                dataKey="ts"
                type="number"
                domain={['dataMin', 'dataMax']}
                tickFormatter={(v) => {
                  try {
                    return new Date(Number(v)).toLocaleTimeString(undefined, {
                      hour: '2-digit',
                      minute: '2-digit',
                    });
                  } catch {
                    return '';
                  }
                }}
                tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
                axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
                tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              />
              <YAxis
                domain={[0, 100]}
                tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
                axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
                tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
                width={40}
              />

              <Tooltip content={<OverlayTooltip />} cursor={{ stroke: 'rgba(167, 139, 250, 0.35)' }} />

              <Area
                type="monotone"
                dataKey="system_load"
                stroke="rgba(167, 139, 250, 0.65)"
                fill="rgba(167, 139, 250, 0.18)"
                strokeWidth={2}
                name="System load"
              />
              <Line
                type="monotone"
                dataKey="intensity"
                stroke="rgba(16, 185, 129, 0.95)"
                strokeWidth={2}
                dot={false}
                name="Intensity"
              />
            </ComposedChart>
          </ResponsiveContainer>
        </div>
      )}
    </div>
  );
}

