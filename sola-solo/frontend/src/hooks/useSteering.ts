import { useCallback, useEffect, useMemo, useState } from 'react'

export type AuditHotspot = {
  path: string
  total: number
  build_failed: number
  error: number
  ok: number
}

export type AuditReport = {
  total_evolutions: number
  ok: number
  build_failed: number
  error: number
  success_rate_pct: number
  total_build_duration_ms: number
  avg_build_duration_ms: number
  hotspots: AuditHotspot[]
}

function fmtMs(ms: number) {
  if (!Number.isFinite(ms)) return '—'
  if (ms < 1000) return `${ms}ms`
  return `${(ms / 1000).toFixed(2)}s`
}

export function useSteering({ apiBase }: { apiBase: string }) {
  const auditEndpoint = useMemo(() => `${apiBase.replace(/\/$/, '')}/api/audit`, [apiBase])

  const [audit, setAudit] = useState<AuditReport | null>(null)
  const [auditError, setAuditError] = useState<string | null>(null)
  const [isLoadingAudit, setIsLoadingAudit] = useState(false)

  const fetchAudit = useCallback(async (): Promise<AuditReport> => {
    setIsLoadingAudit(true)
    setAuditError(null)
    try {
      const res = await fetch(auditEndpoint, { method: 'GET' })
      const ct = res.headers.get('content-type') ?? ''
      if (!res.ok || !ct.includes('application/json')) {
        const text = await res.text().catch(() => '')
        throw new Error(`Audit HTTP ${res.status}: ${text || res.statusText}`)
      }
      const json = (await res.json()) as AuditReport
      setAudit(json)
      return json
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Unknown audit error'
      setAuditError(message)
      throw e
    } finally {
      setIsLoadingAudit(false)
    }
  }, [auditEndpoint])

  useEffect(() => {
    void fetchAudit().catch(() => {})
  }, [fetchAudit])

  const recommendations = useMemo(() => {
    if (!audit?.hotspots?.length) return [] as string[]
    const top = [...audit.hotspots]
      .slice()
      .sort((a, b) => b.build_failed + b.error - (a.build_failed + a.error))
      .slice(0, 3)

    return top.map((h) => {
      const fail = h.build_failed + h.error
      const pct = h.total ? Math.round((fail / h.total) * 100) : 0
      return `Hotspot: ${h.path} failed ${fail}/${h.total} (${pct}%). Consider refactor for stability.`
    })
  }, [audit])

  const auditContext = useMemo(() => {
    if (!audit) return ''
    const top = [...(audit.hotspots ?? [])]
      .slice()
      .sort((a, b) => b.build_failed + b.error - (a.build_failed + a.error))
      .slice(0, 5)

    const hotspotLines = top.map((h) => {
      const fail = h.build_failed + h.error
      return `- ${h.path}: ok=${h.ok}, failed=${h.build_failed}, error=${h.error}, total=${h.total} (fail=${fail})`
    })

    return [
      'AUDIT_CONTEXT (use this to steer safer evolutions):',
      `- total_evolutions=${audit.total_evolutions}, success_rate=${audit.success_rate_pct.toFixed(1)}%`,
      `- build_time_total=${fmtMs(audit.total_build_duration_ms)}, build_time_avg=${fmtMs(audit.avg_build_duration_ms)}`,
      hotspotLines.length ? 'Hotspots:' : 'Hotspots: (none)',
      ...hotspotLines,
    ].join('\n')
  }, [audit])

  return {
    audit,
    auditError,
    isLoadingAudit,
    fetchAudit,
    recommendations,
    auditContext,
  }
}

