import { useEffect, useMemo, useState } from 'react';
import { getPhoenixApiBase } from './env';

type NarrativeResponse = {
  success?: boolean;
  narrative?: string;
};

export default function MobileEcho(props: { apiBase: string; refreshKey?: string | number }) {
  const { apiBase, refreshKey } = props;

  const url = useMemo(() => {
    const base = String(apiBase || getPhoenixApiBase()).replace(/\/$/, '');
    return `${base}/api/counselor/narrative?days=1`;
  }, [apiBase]);

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [narrative, setNarrative] = useState<string>('');

  useEffect(() => {
    let alive = true;

    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await fetch(url);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const json = (await res.json()) as NarrativeResponse;
        const text = String(json?.narrative || '').trim();
        if (alive) setNarrative(text);
      } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : 'Failed to load';
        if (alive) setError(msg);
      } finally {
        if (alive) setLoading(false);
      }
    })();

    return () => {
      alive = false;
    };
  }, [url, refreshKey]);

  return (
    <section className="sageCard">
      <div className="sageTitle">Mobile Echo (last 24h)</div>
      {loading ? <div className="sageBody">Listening…</div> : null}
      {!loading && error ? <div className="sageBody sageError">{error}</div> : null}
      {!loading && !error ? (
        <div className="sageBody">
          {narrative || 'No narrative yet. Log a check-in to generate a 1-day summary.'}
        </div>
      ) : null}
    </section>
  );
}

