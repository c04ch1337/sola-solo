import { useCallback, useEffect, useMemo, useRef, useState } from 'react'

type Role = 'user' | 'assistant'

export type ChatMessage = {
  role: Role
  content: string
}

type SseEvent =
  | { type: 'start' }
  | { type: 'delta'; text: string }
  | { type: 'done' }
  | { type: 'error'; message: string }

export function useStreamingChat({ apiBase }: { apiBase: string }) {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [assistantText, setAssistantText] = useState('')
  const [isStreaming, setIsStreaming] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const abortRef = useRef<AbortController | null>(null)

  // React 18 StrictMode mounts/unmounts components twice in dev.
  // Ensure any in-flight stream is aborted on unmount to prevent “double stream” ghosting.
  useEffect(() => {
    return () => {
      abortRef.current?.abort()
      abortRef.current = null
    }
  }, [])

  const endpoint = useMemo(() => `${apiBase.replace(/\/$/, '')}/api/chat`, [apiBase])

  const stop = useCallback(() => {
    abortRef.current?.abort()
    abortRef.current = null
    setIsStreaming(false)
  }, [])

  const clear = useCallback(() => {
    stop()
    setMessages([])
    setAssistantText('')
    setError(null)
  }, [stop])

  const send = useCallback(
    async (prompt: string, opts?: { system?: string }) => {
      setError(null)
      setIsStreaming(true)
      setAssistantText('')
      setMessages((prev) => [...prev, { role: 'user', content: prompt }])

      // Defensive: ensure we never have two concurrent streams.
      abortRef.current?.abort()

      const controller = new AbortController()
      abortRef.current = controller

      // Keep a local accumulator so we can commit the final assistant message reliably.
      let acc = ''

      try {
        const res = await fetch(endpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ message: prompt, system: opts?.system }),
          signal: controller.signal,
        })

        if (!res.ok) {
          const text = await res.text().catch(() => '')
          throw new Error(`HTTP ${res.status}: ${text || res.statusText}`)
        }

        if (!res.body) {
          throw new Error('No response body (streaming not supported?)')
        }

        const reader = res.body.getReader()
        const decoder = new TextDecoder('utf-8')
        let pending = ''

        const handleEvent = (ev: SseEvent) => {
          if (ev.type === 'delta') {
            acc += ev.text
            setAssistantText(acc)
          }
          if (ev.type === 'error') {
            setError(ev.message)
          }
        }

        while (true) {
          const { value, done } = await reader.read()
          if (done) break

          pending += decoder.decode(value, { stream: true })

          // SSE splits events by blank line.
          const parts = pending.split('\n\n')
          pending = parts.pop() ?? ''

          for (const part of parts) {
            const lines = part.split('\n')
            for (const line of lines) {
              const trimmed = line.trim()
              if (!trimmed.startsWith('data:')) continue
              const payload = trimmed.slice('data:'.length).trim()
              if (!payload) continue

              try {
                const ev = JSON.parse(payload) as SseEvent
                handleEvent(ev)
              } catch {
                // Ignore non-JSON events.
              }
            }
          }
        }

        if (acc) {
          setMessages((prev) => [...prev, { role: 'assistant', content: acc }])
        }
      } catch (e) {
        const name =
          e instanceof DOMException
            ? e.name
            : typeof e === 'object' && e !== null && 'name' in e
              ? String((e as { name?: unknown }).name)
              : ''

        if (name !== 'AbortError') {
          const message =
            e instanceof Error
              ? e.message
              : typeof e === 'string'
                ? e
                : 'Unknown error'
          setError(message)
        }
      } finally {
        abortRef.current = null
        setIsStreaming(false)
      }
    },
    [endpoint],
  )

  return { messages, assistantText, isStreaming, error, send, stop, clear }
}
