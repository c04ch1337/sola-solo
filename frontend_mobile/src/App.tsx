import React, { useEffect, useState } from 'react';
import './App.css';
import MobileEcho from './MobileEcho';
import {
  type ContextTag,
  type L9Stage,
  enqueueLog,
  peekQueue,
  startBackgroundSync,
  subscribeSyncState,
  type SyncState,
} from './syncStore';
import { getPhoenixApiBase } from './env';

const STAGES: L9Stage[] = ['Denial', 'Anger', 'Bargaining', 'Depression', 'Acceptance'];
const TAGS: ContextTag[] = ['Home', 'Transit', 'Work', 'Other'];

function clamp0to100(n: number) {
  return Math.max(0, Math.min(100, Math.round(n)));
}

export default function App() {
  const [stage, setStage] = useState<L9Stage>('Acceptance');
  const [energy, setEnergy] = useState(60);
  const [intensity, setIntensity] = useState(55);
  const [tag, setTag] = useState<ContextTag>('Home');
  const [text, setText] = useState('');

  const [sync, setSync] = useState<SyncState>({ status: 'offline', retryingInSec: 0 });
  const [lastSavedId, setLastSavedId] = useState<string | null>(null);
  const [showSafety, setShowSafety] = useState(false);
  // Queue size can be derived directly; renders are already triggered by state updates.
  // Avoid useMemo() here to keep eslint react-hooks/exhaustive-deps happy.
  const queuedCount = peekQueue().length;

  useEffect(() => {
    startBackgroundSync(4000);
    return subscribeSyncState(setSync);
  }, []);

  const apiBase = getPhoenixApiBase();
  const port = import.meta.env.VITE_MOBILE_PORT || '3000';

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const ePct = clamp0to100(energy);
    const iPct = clamp0to100(intensity);

    const entry = enqueueLog({
      stage,
      energy_level: ePct,
      intensity: iPct,
      tag,
      text: text.trim(),
    });
    setLastSavedId(entry.id);
    setText('');

    // Immediate feedback: Flooding Quadrant
    if (iPct > 75 && ePct < 30) {
      setShowSafety(true);
    }
  };

  const syncStatus = sync.status;
  const retryMsg = sync.status === 'offline' && sync.retryingInSec > 0 ? `Retrying in ${sync.retryingInSec}s…` : '';

  return (
    <div className="page">
      <header className="header">
        <div className="title">L9 Mobile Bridge</div>
        <div className="subtitle">Offline-first log capture • auto-sync when available</div>
      </header>

      <section className="statusRow">
        <div className={`statusPill status-${syncStatus} ${syncStatus === 'syncing' ? 'isPulsing' : ''}`}>
          <span className="statusDot" />
          <span className="statusText">
            {syncStatus === 'syncing' ? 'Syncing…' : syncStatus === 'online' ? 'Cloud: Online' : 'Cloud: Offline'}
          </span>
        </div>
        <div className="queueMeta">
          Queued: {queuedCount}
          {retryMsg ? <span className="retryMsg"> • {retryMsg}</span> : null}
        </div>
      </section>

      <form className="card" onSubmit={onSubmit}>
        <div className="sectionTitle">Stage</div>
        <div className="stageGrid">
          {STAGES.map((s) => (
            <button
              key={s}
              type="button"
              className={`stageBtn ${stage === s ? 'active' : ''}`}
              onClick={() => setStage(s)}
            >
              {s}
            </button>
          ))}
        </div>

        <div className="sliderBlock">
          <div className="sliderHeader">
            <div className="sectionTitle">Energy</div>
            <div className="valueChip">{clamp0to100(energy)}%</div>
          </div>
          <input
            className="slider"
            type="range"
            min={0}
            max={100}
            value={energy}
            onChange={(e) => setEnergy(Number(e.target.value))}
          />
        </div>

        <div className="sliderBlock">
          <div className="sliderHeader">
            <div className="sectionTitle">Intensity</div>
            <div className="valueChip">{clamp0to100(intensity)}%</div>
          </div>
          <input
            className="slider"
            type="range"
            min={0}
            max={100}
            value={intensity}
            onChange={(e) => setIntensity(Number(e.target.value))}
          />
        </div>

        <div className="sectionTitle">Context</div>
        <div className="tagRow">
          {TAGS.map((t) => (
            <button
              key={t}
              type="button"
              className={`tagBtn ${tag === t ? 'active' : ''}`}
              onClick={() => setTag(t)}
            >
              {t}
            </button>
          ))}
        </div>

        <label className="sectionTitle" htmlFor="note">
          Notes
        </label>
        <textarea
          id="note"
          className="textarea"
          rows={5}
          placeholder="Optional: what’s happening / what triggered this?"
          value={text}
          onChange={(e) => setText(e.target.value)}
        />

        <button className="submitBtn" type="submit">
          Save Log (Offline First)
        </button>

        <div className="footnote">
          <div>Backend: {String(apiBase)}</div>
          <div>Mobile: port {String(port)}</div>
          {lastSavedId ? <div>Last saved: {lastSavedId}</div> : null}
        </div>
      </form>

      <MobileEcho apiBase={String(apiBase)} refreshKey={lastSavedId || ''} />

      {showSafety ? (
        <div className="modalOverlay" role="dialog" aria-modal="true">
          <div className="modalCard">
            <div className="modalTitle">Safety Alert</div>
            <div className="modalBody">
              You are in the 'Flooding Quadrant.' Pause for 5 minutes before engaging others.
            </div>
            <button className="modalBtn" type="button" onClick={() => setShowSafety(false)}>
              I will pause
            </button>
          </div>
        </div>
      ) : null}
    </div>
  );
}
