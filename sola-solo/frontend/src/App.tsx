import { useEffect, useMemo, useState } from 'react'
import { createTwoFilesPatch } from 'diff'
import { Aura } from './components/Aura'
import { EvolutionTerminal } from './components/EvolutionTerminal'
import { useStreamingChat } from './hooks/useStreamingChat'
import { useEvolution } from './hooks/useEvolution'
import { useSteering } from './hooks/useSteering'

export default function App() {
  const apiBase = useMemo(
    () => import.meta.env.VITE_API_BASE ?? 'http://127.0.0.1:8888',
    [],
  )

  const [prompt, setPrompt] = useState('')
  const { isStreaming, error, messages, assistantText, send, stop, clear } = useStreamingChat({
    apiBase,
  })

  const {
    audit,
    auditError,
    isLoadingAudit,
    fetchAudit,
    recommendations,
    auditContext,
  } = useSteering({ apiBase })

  // Lightweight token/cost tracking (heuristic): ~4 chars per token.
  const [usdPer1MTokens, setUsdPer1MTokens] = useState(3)
  const estimatedChatTokens = useMemo(() => {
    const all = [...messages.map((m) => m.content), assistantText].join('')
    const chars = all.length
    return Math.ceil(chars / 4)
  }, [messages, assistantText])
  const estimatedChatCostUsd = useMemo(() => {
    return (estimatedChatTokens / 1_000_000) * usdPer1MTokens
  }, [estimatedChatTokens, usdPer1MTokens])

  const [evolvePath, setEvolvePath] = useState('frontend/src/components/Aura.tsx')
  const [evolveCode, setEvolveCode] = useState('')
  const {
    isEvolving,
    lastResult,
    error: evolutionError,
    evolve,

    isSimulating,
    simulateResult,
    simulateError,
    simulate,

    benchmarkPending,
    benchmarkReport,
    benchmarkError,
    repairAttempt,
    repairMaxAttempts,
    manualInterventionRequired,

    canRollback,
    isRollingBack,
    rollbackResult,
    rollbackError,
    rollback,
    readFile,

    manifest,
    manifestError,
    fetchManifest,
    restoreFile,
  } = useEvolution({ apiBase })

  const rollbackFailed = Boolean(rollbackError || (rollbackResult && !rollbackResult.ok))

  const simulationFailed = Boolean(simulateError || (simulateResult && !simulateResult.ok))
  const lastSimulationOk = Boolean(simulateResult?.ok)

  const deltaPct = benchmarkReport?.delta_pct ?? null
  const hasRegressionWarning = typeof deltaPct === 'number' && deltaPct > 10
  const mergeEnabled = Boolean(lastSimulationOk && benchmarkReport && !benchmarkPending)

  const [diffPatch, setDiffPatch] = useState<string>('')

  useEffect(() => {
    void fetchManifest().catch(() => {})
  }, [fetchManifest])

  const onSend = async () => {
    const trimmed = prompt.trim()
    if (!trimmed || isStreaming) return
    setPrompt('')
    await send(trimmed, { system: auditContext })
  }

  return (
    <div className="min-h-full bg-zinc-950 text-zinc-100">
      <div className="pointer-events-none fixed inset-0">
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(99,102,241,0.15),transparent_60%),radial-gradient(ellipse_at_bottom,rgba(34,197,94,0.12),transparent_55%)]" />
      </div>

      <div className="relative mx-auto flex min-h-full max-w-3xl flex-col px-5 py-10">
        <header className="mb-8 flex items-center justify-between">
          <div className="text-sm tracking-wide text-zinc-300">Sola Solo</div>
          <div className="flex items-center gap-2">
            <button
              className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-200 hover:bg-white/10"
              onClick={clear}
              type="button"
            >
              Clear
            </button>
            {isStreaming ? (
              <button
                className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-200 hover:bg-white/10"
                onClick={stop}
                type="button"
              >
                Stop
              </button>
            ) : null}
          </div>
        </header>

        <main className="flex flex-1 flex-col items-center justify-center">
          <Aura
            isThinking={isStreaming}
            isEvolving={isEvolving}
            isRollingBack={isRollingBack}
            rollbackFailed={rollbackFailed}
          />

          <div className="mt-2 text-xs text-zinc-400">
            {isSimulating
              ? 'Simulation running (sandbox)…'
              : benchmarkPending
                ? 'Evaluating fitness (benchmarking)…'
              : simulationFailed
                ? 'Simulation failed (virtual mutation rejected)'
                : lastSimulationOk
                  ? benchmarkReport
                    ? 'Simulation OK + Fitness evaluated (ready to merge into reality)'
                    : benchmarkError
                      ? 'Simulation OK, but fitness evaluation failed (merge gated)'
                      : 'Simulation OK (awaiting fitness evaluation…)'
                  : ''}
          </div>

          <div className="mt-6 w-full rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-xl">
            <div className="flex items-center justify-between">
              <div className="text-xs uppercase tracking-widest text-zinc-400">Sola’s Recommendations</div>
              <button
                className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-200 hover:bg-white/10 disabled:cursor-not-allowed disabled:opacity-50"
                onClick={() => void fetchAudit()}
                disabled={isLoadingAudit}
                type="button"
              >
                {isLoadingAudit ? 'Auditing…' : 'Refresh Audit'}
              </button>
            </div>

            {auditError ? (
              <div className="mt-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                {auditError}
              </div>
            ) : null}

            {audit ? (
              <div className="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2">
                <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                  <div className="text-[11px] uppercase tracking-widest text-zinc-400">Health</div>
                  <div className="mt-2 text-sm text-zinc-200">
                    Success rate:{' '}
                    <span className="text-zinc-100">{audit.success_rate_pct.toFixed(1)}%</span>
                  </div>
                  <div className="mt-1 text-xs text-zinc-400">
                    Total evolutions: {audit.total_evolutions} · OK: {audit.ok} · Failed: {audit.build_failed} ·
                    Error: {audit.error}
                  </div>

                  <div className="mt-3 rounded-2xl border border-white/10 bg-black/30 p-3">
                    <div className="text-[11px] uppercase tracking-widest text-zinc-400">
                      Token efficiency (estimate)
                    </div>
                    <div className="mt-2 flex items-center justify-between gap-3">
                      <div className="text-xs text-zinc-200">
                        Estimated chat tokens: <span className="text-zinc-100">{estimatedChatTokens}</span>
                      </div>
                      <div className="text-xs text-zinc-200">
                        ≈ $<span className="text-zinc-100">{estimatedChatCostUsd.toFixed(4)}</span>
                      </div>
                    </div>

                    <div className="mt-2 flex items-center gap-2">
                      <div className="text-[11px] text-zinc-400">$/1M tokens</div>
                      <input
                        className="w-24 rounded-xl border border-white/10 bg-black/20 px-2 py-1 text-xs text-zinc-100 outline-none"
                        type="number"
                        step={0.5}
                        min={0}
                        value={usdPer1MTokens}
                        onChange={(e) => setUsdPer1MTokens(Number(e.target.value) || 0)}
                      />
                      <div className="text-[11px] text-zinc-500">(heuristic)</div>
                    </div>
                  </div>
                </div>

                <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                  <div className="text-[11px] uppercase tracking-widest text-zinc-400">Top hotspots</div>
                  <div className="mt-2 space-y-2">
                    {recommendations.length ? (
                      recommendations.map((rec) => (
                        <div key={rec} className="flex items-start justify-between gap-3">
                          <div className="text-xs text-zinc-200">{rec}</div>
                          <button
                            className="shrink-0 rounded-full border border-indigo-400/20 bg-indigo-500/10 px-3 py-1 text-xs text-indigo-100 hover:bg-indigo-500/15"
                            onClick={() => {
                              setPrompt(rec)
                            }}
                            type="button"
                          >
                            Ask
                          </button>
                        </div>
                      ))
                    ) : (
                      <div className="text-xs text-zinc-400">No hotspots yet.</div>
                    )}
                  </div>
                </div>
              </div>
            ) : (
              <div className="mt-3 text-xs text-zinc-400">No audit report yet.</div>
            )}

            <div className="mt-4 rounded-2xl border border-white/10 bg-black/20 p-4">
              <div className="text-[11px] uppercase tracking-widest text-zinc-400">Audit context (auto-injected)</div>
              <pre className="mt-2 max-h-[22vh] overflow-auto whitespace-pre-wrap text-xs text-zinc-200">
                {auditContext || '—'}
              </pre>
            </div>
          </div>

          <div className="mt-8 w-full rounded-3xl border border-white/10 bg-white/5 p-5 shadow-[0_20px_80px_-40px_rgba(0,0,0,0.8)] backdrop-blur-xl">
            <div className="mb-3 text-xs uppercase tracking-widest text-zinc-400">
              Chat
            </div>

            <div className="max-h-[38vh] space-y-3 overflow-auto pr-1">
              {messages.map((m, idx) => (
                <div
                  key={idx}
                  className={
                    m.role === 'user'
                      ? 'ml-auto max-w-[85%] rounded-2xl bg-indigo-500/15 px-4 py-3 text-sm text-zinc-100 ring-1 ring-indigo-400/15'
                      : 'mr-auto max-w-[85%] rounded-2xl bg-white/7 px-4 py-3 text-sm text-zinc-100 ring-1 ring-white/10'
                  }
                >
                  {m.content}
                </div>
              ))}

              {assistantText ? (
                <div className="mr-auto max-w-[85%] rounded-2xl bg-white/7 px-4 py-3 text-sm text-zinc-100 ring-1 ring-white/10">
                  {assistantText}
                </div>
              ) : null}
            </div>

            {error ? (
              <div className="mt-3 rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                {error}
              </div>
            ) : null}

            <div className="mt-4 flex items-center gap-3">
              <input
                className="w-full rounded-2xl border border-white/10 bg-black/20 px-4 py-3 text-sm text-zinc-100 outline-none placeholder:text-zinc-500 focus:border-indigo-400/30"
                placeholder={isStreaming ? 'Listening…' : 'Say something…'}
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault()
                    void onSend()
                  }
                }}
                disabled={isStreaming}
              />
              <button
                className="rounded-2xl border border-white/10 bg-white/10 px-4 py-3 text-sm text-zinc-100 hover:bg-white/15 disabled:cursor-not-allowed disabled:opacity-50"
                onClick={() => void onSend()}
                disabled={isStreaming || !prompt.trim()}
                type="button"
              >
                Send
              </button>
            </div>
          </div>

          <div className="mt-6 w-full space-y-4">
            <div className="rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-xl">
              <div className="mb-3 text-xs uppercase tracking-widest text-zinc-400">
                Evolution (atomic rewrite + build validation)
              </div>

              <div className="flex flex-col gap-3">
                <input
                  className="w-full rounded-2xl border border-white/10 bg-black/20 px-4 py-3 text-sm text-zinc-100 outline-none placeholder:text-zinc-500 focus:border-indigo-400/30"
                  value={evolvePath}
                  onChange={(e) => setEvolvePath(e.target.value)}
                  placeholder="frontend/src/... or backend/src/..."
                  disabled={isEvolving}
                />

                <textarea
                  className="min-h-[140px] w-full resize-y rounded-2xl border border-white/10 bg-black/20 px-4 py-3 font-mono text-xs text-zinc-100 outline-none placeholder:text-zinc-500 focus:border-indigo-400/30"
                  value={evolveCode}
                  onChange={(e) => setEvolveCode(e.target.value)}
                  placeholder="Paste the full file contents to atomically rewrite and validate…"
                  disabled={isEvolving}
                />

                <div className="flex items-center justify-end gap-2">
                  <button
                    className="rounded-2xl border border-white/10 bg-cyan-500/10 px-4 py-3 text-sm text-cyan-100 hover:bg-cyan-500/15 disabled:cursor-not-allowed disabled:opacity-50"
                    onClick={() =>
                      void (async () => {
                        await simulate({ path: evolvePath, code: evolveCode })
                      })()
                    }
                    disabled={isEvolving || isSimulating || !evolvePath.trim() || !evolveCode.trim()}
                    type="button"
                    title="Run build in an isolated sandbox (no writes to src/)"
                  >
                    {isSimulating ? 'Simulating…' : 'Simulate Mutation'}
                  </button>

                  <button
                    className="rounded-2xl border border-white/10 bg-white/10 px-4 py-3 text-sm text-zinc-100 hover:bg-white/15 disabled:cursor-not-allowed disabled:opacity-50"
                    onClick={() =>
                      void (async () => {
                        // Capture pre-mutation file state for diff visualization.
                        const before = await readFile(evolvePath)

                        const out = await evolve({ path: evolvePath, code: evolveCode })
                        setEvolveCode(out.finalCode)

                        // Capture post-mutation file state and render a unified diff.
                        const after = await readFile(evolvePath)
                        setDiffPatch(
                          createTwoFilesPatch(evolvePath, evolvePath, before, after, 'before', 'after', {
                            context: 3,
                          }),
                        )

                        // Refresh history manifest after an evolution attempt.
                        await fetchManifest().catch(() => {})
                      })()
                    }
                    disabled={isEvolving || !evolvePath.trim() || !evolveCode.trim()}
                    type="button"
                  >
                    {isEvolving ? 'Evolving…' : 'Evolve'}
                  </button>

                  <button
                    className="rounded-2xl border border-emerald-400/20 bg-emerald-500/10 px-4 py-3 text-sm text-emerald-100 hover:bg-emerald-500/15 disabled:cursor-not-allowed disabled:opacity-50"
                    onClick={() =>
                      void (async () => {
                        const out = await evolve({ path: evolvePath, code: evolveCode })
                        setEvolveCode(out.finalCode)
                        await fetchManifest().catch(() => {})
                      })()
                    }
                    disabled={!mergeEnabled || isEvolving || isSimulating}
                    type="button"
                    title={
                      !lastSimulationOk
                        ? 'Run a successful simulation first'
                        : benchmarkPending
                          ? 'Fitness evaluation in progress…'
                          : !benchmarkReport
                            ? benchmarkError
                              ? 'Benchmark failed (merge gated). Re-run simulation to retry.'
                              : 'Awaiting benchmark report (merge gated)'
                            : hasRegressionWarning
                              ? `Performance regression warning: +${deltaPct?.toFixed(2)}% slower`
                              : 'Apply this mutation to the real workspace'
                    }
                  >
                    Merge into Reality
                  </button>
                </div>

                {benchmarkError ? (
                  <div className="rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                    {benchmarkError}
                  </div>
                ) : null}

                {simulateError ? (
                  <div className="rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                    {simulateError}
                  </div>
                ) : null}

                {simulateResult ? (
                  <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                    <div className="mb-2 flex items-center justify-between">
                      <div className="text-[11px] uppercase tracking-widest text-zinc-400">Simulation</div>
                      <div
                        className={
                          'rounded-full px-2 py-1 text-[11px] ' +
                          (simulateResult.ok
                            ? 'bg-cyan-500/15 text-cyan-100 ring-1 ring-cyan-400/20'
                            : 'bg-red-500/15 text-red-200 ring-1 ring-red-400/20')
                        }
                      >
                        {simulateResult.ok ? 'Virtual Build OK' : 'Virtual Build Failed'} (exit{' '}
                        {simulateResult.status}, {simulateResult.duration_ms}ms)
                      </div>
                    </div>

                    <pre className="max-h-[22vh] overflow-auto rounded-2xl border border-white/10 bg-black/40 p-4 text-xs text-zinc-100">
                      {simulateResult.stderr || simulateResult.stdout || '(no output)'}
                    </pre>
                  </div>
                ) : null}

                {evolutionError ? (
                  <div className="rounded-xl border border-red-500/25 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                    {evolutionError}
                  </div>
                ) : null}
              </div>
            </div>

            <EvolutionTerminal
              result={lastResult}
              benchmarkPending={benchmarkPending}
              benchmarkReport={benchmarkReport}
              benchmarkError={benchmarkError}
              repairAttempt={repairAttempt}
              repairMaxAttempts={repairMaxAttempts}
              manualInterventionRequired={manualInterventionRequired}
              canRollback={canRollback}
              isRollingBack={isRollingBack}
              rollbackResult={rollbackResult}
              rollbackError={rollbackError}
              onRollback={() =>
                void (async () => {
                  if (isEvolving || isRollingBack) return
                  await rollback()
                  // Refresh local editor state to reflect reverted file contents.
                  const code = await readFile(evolvePath)
                  setEvolveCode(code)

                  // After rollback, clear stale diff and refresh manifest.
                  setDiffPatch('')
                  await fetchManifest().catch(() => {})
                })()
              }
              diffPatch={diffPatch}
              history={manifest}
              historyError={manifestError}
              onRefreshHistory={() => void fetchManifest()}
              onRestore={(entry) =>
                void (async () => {
                  if (!entry?.snapshot_commit || !entry?.path) return
                  // Restore file content from the chosen snapshot commit.
                  await restoreFile(entry.snapshot_commit, entry.path)

                  // Sync editor path + code with restored filesystem.
                  setEvolvePath(entry.path)
                  const code = await readFile(entry.path)
                  setEvolveCode(code)

                  // Clear diff (it no longer describes the current state).
                  setDiffPatch('')
                })()
              }
            />
          </div>
        </main>

        <footer className="mt-8 text-center text-xs text-zinc-500">
          Local gateway: <span className="text-zinc-400">{apiBase}</span>
        </footer>
      </div>
    </div>
  )
}
