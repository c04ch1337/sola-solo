import React, { useMemo } from 'react';
import { CartesianGrid, ResponsiveContainer, Scatter, ScatterChart, Tooltip, XAxis, YAxis } from 'recharts';

type GriefEvent = {
  timestamp_ms: number;
  stage: string;
  intensity: number; // 0..100
  energy_level: number; // 0..100
  context_tags: string[];
  text?: string | null;
  system_load?: number; // 0..100
  temperature_c?: number | null;
};

function stageColor(stage: string) {
  const s = stage.toLowerCase();
  if (s.includes('denial')) return 'rgba(167,139,250,0.75)';
  if (s.includes('anger')) return 'rgba(244,63,94,0.75)';
  if (s.includes('bargain')) return 'rgba(251,191,36,0.75)';
  if (s.includes('depress')) return 'rgba(148,163,184,0.75)';
  return 'rgba(16,185,129,0.75)'; // acceptance
}

const PointTooltip = ({ active, payload }: any) => {
  if (!active || !payload?.length) return null;
  const p: any = payload[0].payload;
  return (
    <div className="rounded-xl border border-border-dark bg-black/80 backdrop-blur px-3 py-2 shadow-xl max-w-[360px]">
      <div className="text-xs font-bold text-white uppercase tracking-wider">{p.stage}</div>
      <div className="mt-1 text-xs text-slate-200">
        Energy: <span className="font-bold">{p.energy_level}%</span> • Intensity:{' '}
        <span className="font-bold">{p.intensity}%</span>
      </div>
      <div className="mt-1 text-[11px] text-slate-400">
        Tags: {(p.context_tags || []).join(', ') || '—'}
      </div>
      {p.text && <div className="mt-1 text-[11px] text-slate-300 truncate">{p.text}</div>}
    </div>
  );
};

export default function CorrelationChart(props: { events: GriefEvent[] }) {
  const { events } = props;
  const data = useMemo(() => {
    return (events || []).map((e) => ({
      ...e,
      fill: stageColor(e.stage),
    }));
  }, [events]);

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/70 p-4">
      <div className="flex items-start justify-between gap-4 mb-3">
        <div>
          <h2 className="text-sm font-bold text-white uppercase tracking-wider">Correlation Scatter</h2>
          <p className="text-[10px] text-slate-500 uppercase tracking-widest">
            Energy (X) vs Intensity (Y) • colored by stage
          </p>
        </div>
        <div className="text-[10px] font-mono text-slate-500">{data.length} pts</div>
      </div>

      <div className="h-[260px]">
        <ResponsiveContainer width="100%" height="100%">
          <ScatterChart margin={{ top: 10, right: 10, bottom: 30, left: 40 }}>
            <CartesianGrid stroke="rgba(148, 163, 184, 0.12)" vertical horizontal />
            <XAxis
              type="number"
              dataKey="energy_level"
              domain={[0, 100]}
              tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
              axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              label={{ value: 'Energy', position: 'insideBottom', offset: -10, fill: '#94a3b8', fontSize: 10 }}
            />
            <YAxis
              type="number"
              dataKey="intensity"
              domain={[0, 100]}
              tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
              axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              label={{ value: 'Intensity', angle: -90, position: 'insideLeft', fill: '#94a3b8', fontSize: 10 }}
              width={40}
            />
            <Tooltip content={<PointTooltip />} cursor={{ stroke: 'rgba(167, 139, 250, 0.35)' }} />
            <Scatter
              data={data}
              shape={(props: any) => {
                const { cx, cy, payload } = props;
                const r = Math.max(4, Math.min(11, 4 + (payload.intensity / 100) * 7));
                return <circle cx={cx} cy={cy} r={r} fill={payload.fill} fillOpacity={0.7} />;
              }}
            />
          </ScatterChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

