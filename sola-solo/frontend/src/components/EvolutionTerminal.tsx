import type { BenchmarkReport, CmdResult } from '../hooks/useEvolution'
import type { EvolutionEntry } from '../hooks/useEvolution'

function FitnessBadge({ delta }: { delta: number }) {
  const isFaster = delta < 0 // Negative delta means lower time (faster)
  const color = isFaster ? 'text-emerald-400' : 'text-rose-400'
  const icon = isFaster ? '↓' : '↑'

  return (
    <div className={`flex items-center gap-2 font-mono ${color}`}>
      <span>Fitness:</span>
      <span className="font-bold">{Math.abs(delta).toFixed(2)}% {icon}</span>
      <span className="text-xs opacity-70">({isFaster ? 'Optimized' : 'Regression'})</span>
    </div>
  )
}

function StabilityBadge({ stability, cvPct }: { stability: string | null; cvPct: number | null }) {
  if (!stability) return null

  const colorMap: Record<string, string> = {
    stable: 'text-emerald-400 bg-emerald-500/10 ring-emerald-400/20',
    moderate_noise: 'text-amber-400 bg-amber-500/10 ring-amber-400/20',
    high_noise: 'text-rose-400 bg-rose-500/10 ring-rose-400/20',
  }
  const labelMap: Record<string, string> = {
    stable: 'Stable',
    moderate_noise: 'Moderate Noise',
    high_noise: 'High Noise (Inconclusive)',
  }

  const color = colorMap[stability] ?? 'text-zinc-400 bg-zinc-500/10 ring-zinc-400/20'
  const label = labelMap[stability] ?? stability

  return (
    <div className={`flex items-center gap-2 rounded-full px-2 py-1 text-[11px] ring-1 ${color}`}>
      <span>{label}</span>
      {cvPct != null && <span className="opacity-70">(CV: {cvPct.toFixed(1)}%)</span>}
    </div>
  )
}

function Block({ title, text }: { title: string; text: string }) {
  if (!text.trim()) return null
  return (
    <div className="mt-3">
      <div className="mb-1 text-[11px] uppercase tracking-widest text-zinc-400">{title}</div>
      <pre className="max-h-[28vh] overflow-auto rounded-2xl border border-white/10 bg-black/40 p-4 text-xs text-zinc-100">
        {text}
      </pre>
    </div>
  )
}

export function EvolutionTerminal({
  result,
  benchmarkReport,
  benchmarkPending,
  benchmarkError,
  repairAttempt,
  repairMaxAttempts,
  manualInterventionRequired,
  canRollback,
  isRollingBack,
  rollbackResult,
  rollbackError,
  onRollback,
  diffPatch,
  history,
  historyError,
  onRefreshHistory,
  onRestore,
}: {
  result: CmdResult | null
  benchmarkReport: BenchmarkReport | null
  benchmarkPending: boolean
  benchmarkError: string | null
  repairAttempt: number
  repairMaxAttempts: number
  manualInterventionRequired: boolean
  canRollback: boolean
  isRollingBack: boolean
  rollbackResult: CmdResult | null
  rollbackError: string | null
  onRollback: () => void
  diffPatch: string
  history: EvolutionEntry[]
  historyError: string | null
  onRefreshHistory: () => void
  onRestore: (entry: EvolutionEntry) => void
}) {
  if (!result) {
    return (
      <div className="rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-xl">
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-widest text-zinc-400">Evolution</div>
          <button
            className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-200 hover:bg-white/10 disabled:cursor-not-allowed disabled:opacity-50"
            onClick={onRollback}
            disabled={!canRollback || isRollingBack}
            type="button"
            title={!canRollback ? 'No evolution snapshot created this session yet.' : 'Hard revert to previous snapshot'}
          >
            {isRollingBack ? 'Reverting…' : 'Revert Evolution'}
          </button>
        </div>
        <div className="mt-2 text-sm text-zinc-300">No evolution run yet.</div>
        {rollbackError ? (
          <div className="mt-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
            {rollbackError}
          </div>
        ) : null}
      </div>
    )
  }

  return (
    <div className="rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-xl">
      <div className="flex items-center justify-between">
        <div className="text-xs uppercase tracking-widest text-zinc-400">Evolution</div>
        <div className="flex items-center gap-2">
          <button
            className={
              'rounded-full border px-3 py-1 text-xs disabled:cursor-not-allowed disabled:opacity-50 ' +
              (canRollback
                ? 'border-orange-400/25 bg-orange-500/10 text-orange-100 hover:bg-orange-500/15'
                : 'border-white/10 bg-white/5 text-zinc-200 hover:bg-white/10')
            }
            onClick={onRollback}
            disabled={!canRollback || isRollingBack}
            type="button"
            title={!canRollback ? 'No evolution snapshot created this session yet.' : 'Hard revert to previous snapshot'}
          >
            {isRollingBack ? 'Reverting…' : 'Revert Evolution'}
          </button>

          <div
            className={
              'rounded-full px-2 py-1 text-[11px] ' +
              (result.ok
                ? 'bg-emerald-500/15 text-emerald-200 ring-1 ring-emerald-400/20'
                : manualInterventionRequired
                  ? 'bg-red-500/15 text-red-200 ring-1 ring-red-400/20'
                  : 'bg-amber-500/15 text-amber-200 ring-1 ring-amber-400/20')
            }
          >
            {result.ok
              ? 'Build OK'
              : manualInterventionRequired
                ? 'Manual Intervention Required'
                : 'Build Failed'}{' '}
            (exit {result.status}, {result.duration_ms}ms)
          </div>
        </div>
      </div>

      <div className="mt-3 flex flex-col gap-2 rounded-2xl border border-white/10 bg-black/20 px-4 py-3 text-xs text-zinc-200">
        <div className="flex items-center justify-between gap-3">
          <div className="text-zinc-300">
            Repair attempts: <span className="text-zinc-100">{repairAttempt}</span> /{' '}
            <span className="text-zinc-100">{repairMaxAttempts}</span>
          </div>
          {!result.ok && manualInterventionRequired ? (
            <div className="text-red-200">Autonomous repair halted</div>
          ) : null}
        </div>

        <div className="flex items-center justify-between gap-3">
          <div className="text-zinc-300">Benchmark:</div>
          {benchmarkPending ? (
            <div className="text-cyan-200">Evaluating fitness…</div>
          ) : benchmarkReport?.delta_pct != null ? (
            <FitnessBadge delta={benchmarkReport.delta_pct} />
          ) : benchmarkError ? (
            <div className="text-red-200">Fitness evaluation failed</div>
          ) : (
            <div className="text-zinc-400">Not evaluated</div>
          )}
        </div>

        {benchmarkReport?.mutation_stability ? (
          <div className="flex items-center justify-between gap-3">
            <div className="text-zinc-300">Stability:</div>
            <StabilityBadge
              stability={benchmarkReport.mutation_stability}
              cvPct={benchmarkReport.mutation_cv_pct}
            />
          </div>
        ) : null}
      </div>

      {benchmarkError ? (
        <div className="mt-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
          {benchmarkError}
        </div>
      ) : null}

      <Block title="stdout" text={result.stdout} />
      <Block title="stderr" text={result.stderr} />

      <Block title="mutation diff" text={diffPatch} />

      {benchmarkReport ? (
        <details className="mt-4 rounded-2xl border border-white/10 bg-black/20 p-4">
          <summary className="cursor-pointer select-none text-[11px] uppercase tracking-widest text-zinc-400">
            Benchmark Details
            {benchmarkReport.low_confidence && (
              <span className="ml-2 rounded-full bg-amber-500/20 px-2 py-0.5 text-[10px] font-medium text-amber-400 ring-1 ring-amber-400/30">
                ⚠ Low Confidence
              </span>
            )}
          </summary>

          {benchmarkReport.low_confidence && (
            <div className="mt-3 rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-xs text-amber-300">
              <strong>Warning:</strong> Variance between trials exceeds 15%. Results may be unreliable due to OS
              context-switching or background processes. Consider re-running the benchmark.
            </div>
          )}

          <div className="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2">
            <div className="rounded-2xl border border-white/10 bg-black/30 p-3">
              <div className="flex items-center justify-between">
                <div className="text-[11px] uppercase tracking-widest text-zinc-400">Baseline</div>
                <StabilityBadge
                  stability={benchmarkReport.baseline_stability}
                  cvPct={benchmarkReport.baseline_cv_pct}
                />
              </div>
              <div className="mt-2 text-xs text-zinc-200">
                median_ms:{' '}
                <span className="font-mono text-zinc-100">
                  {benchmarkReport.baseline_median_ms != null
                    ? benchmarkReport.baseline_median_ms.toFixed(3)
                    : 'n/a'}
                </span>
              </div>
              <Block title="stdout" text={benchmarkReport.baseline.stdout} />
              <Block title="stderr" text={benchmarkReport.baseline.stderr} />
            </div>

            <div className="rounded-2xl border border-white/10 bg-black/30 p-3">
              <div className="flex items-center justify-between">
                <div className="text-[11px] uppercase tracking-widest text-zinc-400">Mutation (Sandbox)</div>
                <StabilityBadge
                  stability={benchmarkReport.mutation_stability}
                  cvPct={benchmarkReport.mutation_cv_pct}
                />
              </div>
              <div className="mt-2 text-xs text-zinc-200">
                median_ms:{' '}
                <span className="font-mono text-zinc-100">
                  {benchmarkReport.mutation_median_ms != null
                    ? benchmarkReport.mutation_median_ms.toFixed(3)
                    : 'n/a'}
                </span>
              </div>
              <Block title="stdout" text={benchmarkReport.mutation.stdout} />
              <Block title="stderr" text={benchmarkReport.mutation.stderr} />
            </div>
          </div>
        </details>
      ) : null}

      <div className="mt-4 rounded-2xl border border-white/10 bg-black/20 p-4">
        <div className="mb-2 flex items-center justify-between">
          <div className="text-[11px] uppercase tracking-widest text-zinc-400">Evolution Log</div>
          <button
            className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-200 hover:bg-white/10"
            onClick={onRefreshHistory}
            type="button"
          >
            Refresh
          </button>
        </div>

        {historyError ? (
          <div className="mb-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
            {historyError}
          </div>
        ) : null}

        {history.length ? (
          <div className="max-h-[26vh] overflow-auto">
            <div className="space-y-2">
              {[...history]
                .slice()
                .sort((a, b) => (b.timestamp_ms ?? 0) - (a.timestamp_ms ?? 0))
                .slice(0, 25)
                .map((e) => (
                  <div
                    key={`${e.timestamp_ms}-${e.snapshot_commit}-${e.path}`}
                    className="flex items-center justify-between gap-3 rounded-xl border border-white/10 bg-black/20 px-3 py-2"
                  >
                    <div className="min-w-0">
                      <div className="truncate text-xs text-zinc-100">{e.path}</div>
                      <div className="mt-1 flex flex-wrap gap-2 text-[11px] text-zinc-400">
                        <span className="rounded-full bg-white/5 px-2 py-0.5">{e.status}</span>
                        <span className="rounded-full bg-white/5 px-2 py-0.5">
                          {String(e.snapshot_commit).slice(0, 10)}
                        </span>
                        {e.note ? (
                          <span className="truncate rounded-full bg-white/5 px-2 py-0.5">{e.note}</span>
                        ) : null}
                      </div>
                    </div>

                    <button
                      className="shrink-0 rounded-full border border-orange-400/25 bg-orange-500/10 px-3 py-1 text-xs text-orange-100 hover:bg-orange-500/15"
                      onClick={() => onRestore(e)}
                      type="button"
                      title="Restore this file from the snapshot commit"
                    >
                      Restore
                    </button>
                  </div>
                ))}
            </div>
          </div>
        ) : (
          <div className="text-xs text-zinc-400">No manifest entries yet.</div>
        )}
      </div>

      {rollbackResult ? (
        <div className="mt-4 rounded-2xl border border-white/10 bg-black/20 p-4">
          <div className="mb-2 flex items-center justify-between">
            <div className="text-[11px] uppercase tracking-widest text-zinc-400">Rollback</div>
            <div
              className={
                'rounded-full px-2 py-1 text-[11px] ' +
                (rollbackResult.ok
                  ? 'bg-orange-500/15 text-orange-100 ring-1 ring-orange-400/20'
                  : 'bg-red-500/15 text-red-200 ring-1 ring-red-400/20')
              }
            >
              {rollbackResult.ok ? 'Reverted' : 'Rollback Failed'} (exit {rollbackResult.status},{' '}
              {rollbackResult.duration_ms}ms)
            </div>
          </div>
          <Block title="rollback stdout" text={rollbackResult.stdout} />
          <Block title="rollback stderr" text={rollbackResult.stderr} />
        </div>
      ) : null}

      {rollbackError ? (
        <div className="mt-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
          {rollbackError}
        </div>
      ) : null}
    </div>
  )
}

