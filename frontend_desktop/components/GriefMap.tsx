import React, { useMemo } from 'react';
import {
  CartesianGrid,
  ResponsiveContainer,
  Scatter,
  ScatterChart,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

export type GriefStage = 'Denial' | 'Anger' | 'Bargaining' | 'Depression' | 'Acceptance';

export type L9TherapeuticLog = {
  ts: string; // ISO timestamp
  text: string;
  /** -1..1 (optional). When absent, we assume mid intensity. */
  mood?: number;
};

export type GriefAggregate = {
  day: string; // YYYY-MM-DD
  stage: GriefStage | string;
  count: number;
  average_intensity: number; // 0..100
  average_energy: number; // 0..100
};

const STAGES: GriefStage[] = ['Denial', 'Anger', 'Bargaining', 'Depression', 'Acceptance'];

function clamp01(n: number) {
  return Math.max(0, Math.min(1, n));
}

function isoDay(ts: string) {
  const d = new Date(ts);
  if (Number.isNaN(d.getTime())) return 'Invalid Date';
  return d.toISOString().slice(0, 10);
}

function stageSignals(text: string): Record<GriefStage, number> {
  const t = text.toLowerCase();

  // NOTE: This is intentionally heuristic.
  // When L9 therapeutic memory exposes structured stage markers, swap these out.
  const denial = /(can'?t be|this isn'?t real|no way|impossible|i don'?t believe|in denial)/.test(t) ? 1 : 0;
  const anger = /(angry|mad|furious|resent|hate|unfair|rage|annoyed)/.test(t) ? 1 : 0;
  const bargaining = /(if only|what if|maybe i could|i should have|deal|promise|bargain)/.test(t) ? 1 : 0;
  const depression = /(hopeless|empty|tired|sad|depressed|numb|can'?t get out of bed|no energy)/.test(t) ? 1 : 0;
  const acceptance = /(accept|okay with|i can live with|moving forward|i understand|it is what it is)/.test(t) ? 1 : 0;

  return {
    Denial: denial,
    Anger: anger,
    Bargaining: bargaining,
    Depression: depression,
    Acceptance: acceptance,
  };
}

function intensityFromMood(mood?: number) {
  if (typeof mood !== 'number') return 0.6;
  // Convert -1..1 → 0..1 (bias toward mid so we still see data).
  return clamp01((mood + 1) / 2 * 0.8 + 0.1);
}

type HeatCell = {
  x: number;
  y: number;
  day: string;
  stage: GriefStage;
  value: number; // 0..1 (intensity)
  energy: number; // 0..1
  samples: string[];
};

function heatColor(value: number) {
  // Lavender → Sage ramp (safe-space aesthetic)
  const v = clamp01(value);
  const lavender = { r: 167, g: 139, b: 250 }; // purple-400-ish
  const sage = { r: 127, g: 191, b: 154 };
  const r = Math.round(lavender.r + (sage.r - lavender.r) * v);
  const g = Math.round(lavender.g + (sage.g - lavender.g) * v);
  const b = Math.round(lavender.b + (sage.b - lavender.b) * v);
  return `rgb(${r}, ${g}, ${b})`;
}

function dayLabel(day: string) {
  // YYYY-MM-DD → MM/DD
  if (day.length === 10) return `${day.slice(5, 7)}/${day.slice(8, 10)}`;
  return day;
}

const HeatTooltip = ({ active, payload }: any) => {
  if (!active || !payload?.length) return null;
  const p: HeatCell = payload[0].payload;

  return (
    <div className="rounded-xl border border-border-dark bg-black/80 backdrop-blur px-3 py-2 shadow-xl max-w-[360px]">
      <div className="flex items-center justify-between gap-4">
        <div className="text-xs font-bold text-white uppercase tracking-wider">{p.stage}</div>
        <div className="text-[10px] font-mono text-slate-400">{p.day}</div>
      </div>
      <div className="mt-1 flex items-center gap-2">
        <div
          className="size-3 rounded"
          style={{ backgroundColor: heatColor(p.value) }}
        />
        <div className="text-xs text-slate-200">
          Intensity: <span className="font-bold">{Math.round(p.value * 100)}%</span>
        </div>
      </div>
      {p.samples.length > 0 && (
        <div className="mt-2 text-[11px] text-slate-300 leading-snug">
          <div className="text-[10px] uppercase tracking-widest text-slate-500 mb-1">Signals</div>
          <ul className="list-disc pl-4 space-y-1">
            {p.samples.slice(0, 3).map((s, i) => (
              <li key={i} className="truncate">{s}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

export default function GriefMap({
  logs,
  stats,
}: {
  logs: L9TherapeuticLog[];
  stats?: GriefAggregate[];
}) {
  const { cells, days } = useMemo(() => {
    // If backend aggregates are provided, render those directly.
    if (stats && stats.length > 0) {
      const days = Array.from(new Set(stats.map((s) => s.day))).sort();
      const dayIdx = new Map(days.map((d, i) => [d, i] as const));
      const cells: HeatCell[] = [];
      for (const s of stats) {
        const stage = (STAGES.includes(s.stage as any) ? (s.stage as GriefStage) : 'Depression');
        cells.push({
          x: dayIdx.get(s.day) ?? 0,
          y: STAGES.indexOf(stage),
          day: s.day,
          stage,
          value: clamp01((s.average_intensity ?? 0) / 100),
          energy: clamp01((s.average_energy ?? 0) / 100),
          samples: [],
        });
      }
      return { cells, days };
    }

    const dayToStageAcc = new Map<string, Map<GriefStage, { sum: number; n: number; samples: string[] }>>();

    const ensure = (day: string, stage: GriefStage) => {
      if (!dayToStageAcc.has(day)) dayToStageAcc.set(day, new Map());
      const stageMap = dayToStageAcc.get(day)!;
      if (!stageMap.has(stage)) stageMap.set(stage, { sum: 0, n: 0, samples: [] });
      return stageMap.get(stage)!;
    };

    for (const log of logs) {
      const day = isoDay(log.ts);
      const signals = stageSignals(log.text);
      const intensity = intensityFromMood(log.mood);

      // If no stage triggers, still treat as low ambient “processing” signal.
      const anySignal = STAGES.some((s) => signals[s] > 0);
      for (const stage of STAGES) {
        const raw = anySignal ? signals[stage] : (stage === 'Depression' ? 0.25 : 0.15);
        const value = clamp01(raw * intensity);
        const acc = ensure(day, stage);
        acc.sum += value;
        acc.n += 1;
        if (raw > 0) acc.samples.push(log.text);
      }
    }

    const days = Array.from(dayToStageAcc.keys()).sort();
    const cells: HeatCell[] = [];

    for (let x = 0; x < days.length; x++) {
      const day = days[x];
      for (let y = 0; y < STAGES.length; y++) {
        const stage = STAGES[y];
        const acc = dayToStageAcc.get(day)?.get(stage);
        const avg = acc && acc.n > 0 ? clamp01(acc.sum / acc.n) : 0;
        cells.push({
          x,
          y,
          day,
          stage,
          value: avg,
          energy: 0.6,
          samples: acc?.samples ?? [],
        });
      }
    }

    return { cells, days };
  }, [logs]);

  return (
    <div className="rounded-2xl border border-border-dark bg-panel-dark/70 p-4">
      <div className="flex items-start justify-between gap-4 mb-3">
        <div>
          <h2 className="text-sm font-bold text-white uppercase tracking-wider">Grief Map</h2>
          <p className="text-[10px] text-slate-500 uppercase tracking-widest">
            Mood heatmap • L9 therapeutic memory signals → 5 stages
          </p>
        </div>
        <div className="text-[10px] font-mono text-slate-500">
          {days.length === 0 ? 'No logs yet' : `${days.length} days`}
        </div>
      </div>

      <div className="h-[260px]">
        <ResponsiveContainer width="100%" height="100%">
          <ScatterChart margin={{ top: 10, right: 10, bottom: 30, left: 80 }}>
            <CartesianGrid stroke="rgba(148, 163, 184, 0.12)" vertical horizontal />
            <XAxis
              type="number"
              dataKey="x"
              allowDecimals={false}
              domain={[-0.5, Math.max(0, days.length - 0.5)]}
              tickFormatter={(v) => {
                const idx = typeof v === 'number' ? v : Number(v);
                const day = days[idx];
                return day ? dayLabel(day) : '';
              }}
              tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
              axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              interval={Math.max(0, Math.ceil(days.length / 8) - 1)}
            />
            <YAxis
              type="number"
              dataKey="y"
              allowDecimals={false}
              domain={[-0.5, 4.5]}
              tickFormatter={(v) => {
                const idx = typeof v === 'number' ? v : Number(v);
                return STAGES[idx] ?? '';
              }}
              tick={{ fill: '#94a3b8', fontSize: 10, fontFamily: 'JetBrains Mono' }}
              axisLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              tickLine={{ stroke: 'rgba(148, 163, 184, 0.2)' }}
              width={90}
            />
            <Tooltip content={<HeatTooltip />} cursor={{ stroke: 'rgba(167, 139, 250, 0.35)' }} />
            <Scatter
              data={cells}
              shape={(props: any) => {
                const { cx, cy, payload } = props;
                const p: HeatCell = payload;
                const size = Math.round(10 + (p.energy ?? 0.6) * 14);
                const fill = heatColor(p.value);
                const stroke = p.value > 0.66 ? 'rgba(255,255,255,0.35)' : 'rgba(0,0,0,0.0)';
                return (
                  <rect
                    x={cx - size / 2}
                    y={cy - size / 2}
                    width={size}
                    height={size}
                    rx={4}
                    fill={fill}
                    fillOpacity={0.25 + p.value * 0.7}
                    stroke={stroke}
                  />
                );
              }}
            />
          </ScatterChart>
        </ResponsiveContainer>
      </div>

      <div className="mt-3 flex items-center justify-between gap-4">
        <div className="text-[10px] text-slate-500 uppercase tracking-widest">
          lower intensity
        </div>
        <div className="flex items-center gap-2">
          {[0, 0.25, 0.5, 0.75, 1].map((v) => (
            <div
              key={v}
              className="size-4 rounded"
              style={{ backgroundColor: heatColor(v), opacity: 0.25 + v * 0.7 }}
              title={`${Math.round(v * 100)}%`}
            />
          ))}
        </div>
        <div className="text-[10px] text-slate-500 uppercase tracking-widest">
          higher intensity
        </div>
      </div>
    </div>
  );
}

