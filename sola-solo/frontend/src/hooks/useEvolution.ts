import { useCallback, useMemo, useState } from 'react'

export type CmdResult = {
  ok: boolean
  status: number
  stdout: string
  stderr: string
  duration_ms: number
}

export type SimulateResponse = {
  sandbox_rel_path: string
  result: CmdResult
}

export type BenchmarkReport = {
  baseline: CmdResult
  mutation: CmdResult
  /** Median total time in ms (filters OS context-switching outliers) */
  baseline_median_ms: number | null
  mutation_median_ms: number | null
  delta_pct: number | null
  /** Stability classification: "stable", "moderate_noise", or "high_noise" */
  baseline_stability: string | null
  mutation_stability: string | null
  /** Coefficient of variation (%) for baseline and mutation */
  baseline_cv_pct: number | null
  mutation_cv_pct: number | null
  /** True if either baseline or mutation has CV > 15% (high variance = unreliable) */
  low_confidence: boolean
}

export type BenchmarkRequest = {
  iters?: number
  warmup?: number
  /** Number of measurement trials (default: 5) */
  trials?: number
  /** Module path to benchmark (e.g., "./src/components/Aura.tsx") */
  bench_module?: string
  /** Export name to benchmark (e.g., "calculateAuraIntensity") */
  bench_export?: string
}

export type EvolutionEntry = {
  timestamp_ms: number
  path: string
  snapshot_commit: string
  status: string
  build_status?: number | null
  build_duration_ms?: number | null
  build_stderr_excerpt?: string | null
  note?: string | null
}

export type EvolveRequest = {
  path: string
  code: string
  note?: string
}

export type ReadFileResponse = {
  path: string
  code: string
}

type RepairRequest = {
  path: string
  code: string
  stderr: string
}

type RepairResponse = {
  code: string
}

export type EvolutionOutcome = {
  result: CmdResult | null
  finalCode: string
  attempts: number
  manualInterventionRequired: boolean
}

export function useEvolution({ apiBase }: { apiBase: string }) {
  const endpoint = useMemo(() => `${apiBase.replace(/\/$/, '')}/api/evolve`, [apiBase])
  const repairEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/repair`,
    [apiBase],
  )
  const rollbackEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/rollback`,
    [apiBase],
  )
  const fileEndpoint = useMemo(() => `${apiBase.replace(/\/$/, '')}/api/file`, [apiBase])
  const manifestEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/manifest`,
    [apiBase],
  )
  const restoreEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/restore`,
    [apiBase],
  )
  const simulateEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/simulate`,
    [apiBase],
  )

  const benchmarkEndpoint = useMemo(
    () => `${apiBase.replace(/\/$/, '')}/api/benchmark`,
    [apiBase],
  )

  const [isEvolving, setIsEvolving] = useState(false)
  const [lastResult, setLastResult] = useState<CmdResult | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [repairAttempt, setRepairAttempt] = useState(0)
  const [repairMaxAttempts, setRepairMaxAttempts] = useState(3)
  const [manualInterventionRequired, setManualInterventionRequired] = useState(false)

  const [canRollback, setCanRollback] = useState(false)
  const [isRollingBack, setIsRollingBack] = useState(false)
  const [rollbackResult, setRollbackResult] = useState<CmdResult | null>(null)
  const [rollbackError, setRollbackError] = useState<string | null>(null)

  const [manifest, setManifest] = useState<EvolutionEntry[]>([])
  const [manifestError, setManifestError] = useState<string | null>(null)

  const [isSimulating, setIsSimulating] = useState(false)
  const [simulateResult, setSimulateResult] = useState<CmdResult | null>(null)
  const [simulateError, setSimulateError] = useState<string | null>(null)

  const [benchmarkPending, setBenchmarkPending] = useState(false)
  const [benchmarkReport, setBenchmarkReport] = useState<BenchmarkReport | null>(null)
  const [benchmarkError, setBenchmarkError] = useState<string | null>(null)

  const benchmark = useCallback(
    async (req?: BenchmarkRequest): Promise<BenchmarkReport> => {
      setBenchmarkPending(true)
      setBenchmarkError(null)
      setBenchmarkReport(null)

      try {
        const res = await fetch(benchmarkEndpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ iters: req?.iters }),
        })

        const ct = res.headers.get('content-type') ?? ''
        if (!res.ok || !ct.includes('application/json')) {
          const text = await res.text().catch(() => '')
          throw new Error(`Benchmark HTTP ${res.status}: ${text || res.statusText}`)
        }

        const json = (await res.json()) as BenchmarkReport
        setBenchmarkReport(json)
        return json
      } catch (e) {
        const message = e instanceof Error ? e.message : 'Unknown benchmark error'
        setBenchmarkError(message)
        throw e
      } finally {
        setBenchmarkPending(false)
      }
    },
    [benchmarkEndpoint],
  )

  const evolve = useCallback(
    async (
      req: EvolveRequest,
      opts?: {
        autoRepair?: boolean
        maxAttempts?: number
      },
    ): Promise<EvolutionOutcome> => {
      setIsEvolving(true)
      setError(null)
      setLastResult(null)
      setManualInterventionRequired(false)

      const autoRepair = opts?.autoRepair ?? true
      const maxAttempts = opts?.maxAttempts ?? 3
      setRepairMaxAttempts(maxAttempts)
      setRepairAttempt(0)

      const runEvolveOnce = async (payload: EvolveRequest) => {
        const res = await fetch(endpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload),
        })

        // `422` is expected for compile failures; backend still returns CmdResult JSON.
        const ct = res.headers.get('content-type') ?? ''
        if (ct.includes('application/json')) {
          const json = (await res.json()) as CmdResult
          // If the backend returned JSON, a snapshot commit was attempted; enable rollback.
          setCanRollback(true)
          return json
        }

        const text = await res.text().catch(() => '')
        throw new Error(`HTTP ${res.status}: ${text || res.statusText}`)
      }

      const requestRepair = async (payload: RepairRequest) => {
        const res = await fetch(repairEndpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload),
        })

        const ct = res.headers.get('content-type') ?? ''
        if (!res.ok || !ct.includes('application/json')) {
          const text = await res.text().catch(() => '')
          throw new Error(`Repair HTTP ${res.status}: ${text || res.statusText}`)
        }

        const json = (await res.json()) as RepairResponse
        if (!json.code || !json.code.trim()) {
          throw new Error('Repair returned empty code')
        }
        return json.code
      }

      let attempt = 0
      let code = req.code
      let last: CmdResult | null = null

      try {
        
        while (attempt < maxAttempts) {
          attempt += 1
          setRepairAttempt(attempt)

          last = await runEvolveOnce({ path: req.path, code })
          setLastResult(last)

          if (last.ok) {
            return {
              result: last,
              finalCode: code,
              attempts: attempt,
              manualInterventionRequired: false,
            }
          }

          if (!autoRepair || attempt >= maxAttempts) {
            break
          }

          // Ask OpenRouter (via backend) to produce corrected file contents.
          code = await requestRepair({ path: req.path, code, stderr: last.stderr })
        }

        setManualInterventionRequired(true)
        return {
          result: last,
          finalCode: code,
          attempts: Math.max(attempt, 1),
          manualInterventionRequired: true,
        }
      } catch (e) {
        const message = e instanceof Error ? e.message : 'Unknown error'
        setError(message)
        setManualInterventionRequired(true)
        return {
          result: null,
          finalCode: req.code,
          attempts: Math.max(attempt, 1),
          manualInterventionRequired: true,
        }
      } finally {
        setIsEvolving(false)
      }
    },
    [endpoint, repairEndpoint],
  )

  const rollback = useCallback(async (): Promise<CmdResult> => {
    setIsRollingBack(true)
    setRollbackError(null)
    setRollbackResult(null)
    try {
      const res = await fetch(rollbackEndpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      })

      const ct = res.headers.get('content-type') ?? ''
      if (ct.includes('application/json')) {
        const json = (await res.json()) as CmdResult
        setRollbackResult(json)
        return json
      }

      const text = await res.text().catch(() => '')
      throw new Error(`HTTP ${res.status}: ${text || res.statusText}`)
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Unknown rollback error'
      setRollbackError(message)
      throw e
    } finally {
      setIsRollingBack(false)
    }
  }, [rollbackEndpoint])

  const readFile = useCallback(
    async (path: string): Promise<string> => {
      const url = `${fileEndpoint}?path=${encodeURIComponent(path)}`
      const res = await fetch(url, { method: 'GET' })
      const ct = res.headers.get('content-type') ?? ''
      if (!res.ok || !ct.includes('application/json')) {
        const text = await res.text().catch(() => '')
        throw new Error(`Read file HTTP ${res.status}: ${text || res.statusText}`)
      }
      const json = (await res.json()) as ReadFileResponse
      if (typeof json.code !== 'string') {
        throw new Error('Read file returned invalid payload')
      }
      return json.code
    },
    [fileEndpoint],
  )

  const fetchManifest = useCallback(async (): Promise<EvolutionEntry[]> => {
    setManifestError(null)
    const res = await fetch(manifestEndpoint, { method: 'GET' })
    const ct = res.headers.get('content-type') ?? ''
    if (!res.ok || !ct.includes('application/json')) {
      const text = await res.text().catch(() => '')
      const msg = `Manifest HTTP ${res.status}: ${text || res.statusText}`
      setManifestError(msg)
      throw new Error(msg)
    }
    const json = (await res.json()) as EvolutionEntry[]
    setManifest(Array.isArray(json) ? json : [])
    return json
  }, [manifestEndpoint])

  const restoreFile = useCallback(
    async (commit_hash: string, path: string): Promise<CmdResult> => {
      const res = await fetch(restoreEndpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ commit_hash, path }),
      })
      const ct = res.headers.get('content-type') ?? ''
      if (ct.includes('application/json')) {
        return (await res.json()) as CmdResult
      }
      const text = await res.text().catch(() => '')
      throw new Error(`Restore HTTP ${res.status}: ${text || res.statusText}`)
    },
    [restoreEndpoint],
  )

  const simulate = useCallback(
    async (req: EvolveRequest): Promise<CmdResult> => {
      setIsSimulating(true)
      setSimulateError(null)
      setSimulateResult(null)

      // Any new simulation invalidates prior fitness evaluation.
      setBenchmarkError(null)
      setBenchmarkReport(null)
      try {
        const res = await fetch(simulateEndpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ path: req.path, code: req.code }),
        })

        const ct = res.headers.get('content-type') ?? ''
        if (!res.ok || !ct.includes('application/json')) {
          const text = await res.text().catch(() => '')
          throw new Error(`Simulate HTTP ${res.status}: ${text || res.statusText}`)
        }

        const json = (await res.json()) as SimulateResponse
        if (!json?.result) throw new Error('Simulate returned invalid payload')
        setSimulateResult(json.result)

        // Phase H: AUTO-RUN BENCHMARK (selection pressure) after a successful sandbox build.
        if (json.result.ok) {
          // Benchmark depends on the sandbox state created by `/api/simulate`.
          // If it fails, keep simulation result but gate merge.
          void benchmark().catch(() => {})
        }

        return json.result
      } catch (e) {
        const message = e instanceof Error ? e.message : 'Unknown simulation error'
        setSimulateError(message)
        throw e
      } finally {
        setIsSimulating(false)
      }
    },
    [benchmark, simulateEndpoint],
  )

  return {
    isEvolving,
    lastResult,
    error,
    evolve,
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

    isSimulating,
    simulateResult,
    simulateError,
    simulate,

    benchmarkPending,
    benchmarkReport,
    benchmarkError,
    benchmark,
  }
}

