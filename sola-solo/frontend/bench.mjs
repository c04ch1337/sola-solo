// Phase H+ Deterministic micro-benchmark harness with JIT warmup & stability scoring.
// Runs entirely in Node (fast feedback, low noise).
//
// Usage:
//   node --expose-gc bench.mjs
//   node --expose-gc bench.mjs --iters 10000 --warmup 5000 --trials 5
//   BENCH_MODULE=./dist/some.js BENCH_EXPORT=fn node --expose-gc bench.mjs
//
// For TypeScript source files, use tsx:
//   npx tsx --expose-gc bench.mjs
//   BENCH_MODULE=./src/components/Aura.tsx BENCH_EXPORT=calculateAuraIntensity npx tsx --expose-gc bench.mjs

import { performance } from 'node:perf_hooks'

function parseArgs(argv) {
  // Defaults: 5 trials × 10,000 iterations with 5,000 warmup
  const out = { iters: 10_000, warmup: 5_000, trials: 5 }
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]
    if (a === '--iters') {
      const n = Number(argv[i + 1])
      if (Number.isFinite(n) && n > 0) out.iters = Math.floor(n)
      i++
    } else if (a === '--warmup') {
      const n = Number(argv[i + 1])
      if (Number.isFinite(n) && n >= 0) out.warmup = Math.floor(n)
      i++
    } else if (a === '--trials' || a === '--runs') {
      // Accept both --trials and --runs for backward compatibility
      const n = Number(argv[i + 1])
      if (Number.isFinite(n) && n >= 1) out.trials = Math.floor(n)
      i++
    }
  }
  return out
}

async function loadTarget() {
  const modPath = process.env.BENCH_MODULE
  const expName = process.env.BENCH_EXPORT

  if (!modPath || !expName) {
    // Default target: a tiny deterministic loop (LCG).
    return () => {
      let x = 0
      for (let i = 0; i < 50; i++) x = (x * 1664525 + 1013904223) >>> 0
      return x
    }
  }

  const mod = await import(modPath)
  const fn = mod?.[expName] ?? mod?.default?.[expName]
  if (typeof fn !== 'function') {
    throw new Error(`BENCH_EXPORT '${expName}' is not a function in ${modPath}`)
  }
  return fn
}

/**
 * Attempt to trigger garbage collection if --expose-gc was passed.
 * This reduces noise between runs.
 */
function tryGC() {
  if (typeof globalThis.gc === 'function') {
    globalThis.gc()
  }
}

/**
 * Run a single measurement pass.
 * @param {Function} fn - The function to benchmark.
 * @param {number} iters - Number of iterations.
 * @returns {{ totalMs: number, sink: number }}
 */
function measureOnce(fn, iters) {
  const start = performance.now()
  let sink = 0
  for (let i = 0; i < iters; i++) {
    // Prevent complete dead-code elimination
    sink ^= fn() | 0
  }
  const end = performance.now()
  return { totalMs: end - start, sink }
}

/**
 * Compute mean and standard deviation.
 * @param {number[]} arr
 * @returns {{ mean: number, stddev: number }}
 */
function stats(arr) {
  const n = arr.length
  if (n === 0) return { mean: 0, stddev: 0 }
  const mean = arr.reduce((a, b) => a + b, 0) / n
  const variance = arr.reduce((sum, x) => sum + (x - mean) ** 2, 0) / n
  return { mean, stddev: Math.sqrt(variance) }
}

/**
 * Compute median (filters out OS context-switching outliers).
 * @param {number[]} arr
 * @returns {number}
 */
function median(arr) {
  if (arr.length === 0) return 0
  const sorted = [...arr].sort((a, b) => a - b)
  const mid = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 0
    ? (sorted[mid - 1] + sorted[mid]) / 2
    : sorted[mid]
}

const { iters, warmup, trials } = parseArgs(process.argv.slice(2))

const fn = await loadTarget()

// ─────────────────────────────────────────────────────────────────────────────
// Phase 1: JIT Warm-up (trigger V8 TurboFan optimization)
// 5,000 iterations to ensure TurboFan has fully optimized the hot path.
// ─────────────────────────────────────────────────────────────────────────────
for (let i = 0; i < warmup; i++) fn()

// Clear GC between warmup and measurement for maximum stability
tryGC()

// ─────────────────────────────────────────────────────────────────────────────
// Phase 2: Trial-Based Measurement (5 trials × 10,000 iterations by default)
// Using median per_iter_ns to filter out OS context-switching noise.
// ─────────────────────────────────────────────────────────────────────────────
const trialResults = []
const perIterNsPerTrial = []
let finalSink = 0

for (let t = 0; t < trials; t++) {
  tryGC()
  const { totalMs, sink } = measureOnce(fn, iters)
  trialResults.push(totalMs)
  perIterNsPerTrial.push((totalMs * 1e6) / iters)
  finalSink ^= sink
}

const { mean: meanMs, stddev: stddevMs } = stats(trialResults)
const medianMs = median(trialResults)
const medianPerIterNs = median(perIterNsPerTrial)

// Coefficient of Variation (CV) as stability score: lower is better.
// CV < 5% = stable, 5-15% = moderate noise, >15% = high noise (inconclusive).
const cv = meanMs > 0 ? (stddevMs / meanMs) * 100 : 0
const stability = cv < 5 ? 'stable' : cv < 15 ? 'moderate_noise' : 'high_noise'

// ─────────────────────────────────────────────────────────────────────────────
// Emit JSON so the backend can parse it reliably.
// ─────────────────────────────────────────────────────────────────────────────
process.stdout.write(
  JSON.stringify(
    {
      ok: true,
      iters,
      warmup,
      trials,
      trial_times_ms: trialResults.map((t) => Number(t.toFixed(4))),
      mean_ms: Number(meanMs.toFixed(4)),
      median_ms: Number(medianMs.toFixed(4)),
      stddev_ms: Number(stddevMs.toFixed(4)),
      cv_pct: Number(cv.toFixed(2)),
      stability,
      // Use MEDIAN per_iter_ns to filter OS context-switching outliers
      per_iter_ns: Math.round(medianPerIterNs),
      mean_per_iter_ns: Math.round((meanMs * 1e6) / iters),
      sink: finalSink,
    },
    null,
    2,
  ),
)

