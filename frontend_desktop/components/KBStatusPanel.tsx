/**
 * KBStatusPanel.tsx
 * 
 * Displays the Vector Knowledge Base status in the Recommendations panel.
 * Shows whether Sola is currently "Learning" (indexing) or "Recalling" (searching).
 * 
 * Part of Phase I: "The Cognitive Bridge" - Automated Context Injection
 */

import React, { useState, useEffect, useCallback } from 'react';

interface KBStatus {
  enabled: boolean;
  backend: 'qdrant' | 'sled';
  path: string;
  qdrant_url: string | null;
  collection: string;
  operation_status: 'idle' | 'learning' | 'recalling' | 'offline';
  operation_status_display: string;
  memory_count?: number;
  memory_count_display?: string;
  evolution_count?: number;
}

interface RecalledEvolution {
  id: string;
  text: string;
  score: number;
  file_path?: string;
  action?: string;
  status?: string;
  timestamp?: string;
  test_passed?: boolean;
}

interface MemoryContext {
  lessons_learned: RecalledEvolution[];
  known_regressions: RecalledEvolution[];
  kb_status: 'idle' | 'learning' | 'recalling' | 'offline';
}

interface KBStatusPanelProps {
  backendUrl: string;
  onClose?: () => void;
  className?: string;
}

const KBStatusPanel: React.FC<KBStatusPanelProps> = ({ backendUrl, onClose, className = '' }) => {
  const [status, setStatus] = useState<KBStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [recallQuery, setRecallQuery] = useState('');
  const [recallResult, setRecallResult] = useState<MemoryContext | null>(null);
  const [isRecalling, setIsRecalling] = useState(false);

  // Fetch KB status
  const fetchStatus = useCallback(async () => {
    try {
      const response = await fetch(`${backendUrl}/api/kb/status`);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const data = await response.json();
      setStatus(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch KB status');
    } finally {
      setLoading(false);
    }
  }, [backendUrl]);

  // Perform semantic recall
  const performRecall = async () => {
    if (!recallQuery.trim()) return;
    
    setIsRecalling(true);
    try {
      const response = await fetch(`${backendUrl}/api/kb/recall`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ query: recallQuery, top_k: 5 }),
      });
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      
      const data = await response.json();
      setRecallResult(data.context);
    } catch (err) {
      console.error('Recall failed:', err);
    } finally {
      setIsRecalling(false);
    }
  };

  // Poll status every 5 seconds
  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, [fetchStatus]);

  // Status indicator color
  const getStatusColor = (opStatus: string) => {
    switch (opStatus) {
      case 'idle': return 'text-green-400';
      case 'learning': return 'text-yellow-400 animate-pulse';
      case 'recalling': return 'text-blue-400 animate-pulse';
      case 'offline': return 'text-slate-500';
      default: return 'text-slate-400';
    }
  };

  // Status icon
  const getStatusIcon = (opStatus: string) => {
    switch (opStatus) {
      case 'idle': return 'psychology';
      case 'learning': return 'school';
      case 'recalling': return 'search';
      case 'offline': return 'cloud_off';
      default: return 'help';
    }
  };

  if (loading) {
    return (
      <div className={`bg-panel-dark border border-border-dark rounded-xl p-4 ${className}`}>
        <div className="flex items-center gap-2 text-slate-400">
          <span className="material-symbols-outlined animate-spin text-[18px]">sync</span>
          <span className="text-sm">Loading KB status...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className={`bg-panel-dark border border-border-dark rounded-xl p-4 ${className}`}>
        <div className="flex items-center gap-2 text-red-400">
          <span className="material-symbols-outlined text-[18px]">error</span>
          <span className="text-sm">KB Error: {error}</span>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-panel-dark border border-border-dark rounded-xl overflow-hidden ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border-dark bg-slate-800/30">
        <div className="flex items-center gap-2">
          <span className={`material-symbols-outlined text-[20px] ${getStatusColor(status?.operation_status || 'offline')}`}>
            {getStatusIcon(status?.operation_status || 'offline')}
          </span>
          <h3 className="text-sm font-bold text-white uppercase tracking-wider">Vector KB</h3>
          <span className={`text-xs px-2 py-0.5 rounded-full ${
            status?.enabled ? 'bg-green-500/20 text-green-400' : 'bg-slate-500/20 text-slate-400'
          }`}>
            {status?.enabled ? 'Online' : 'Offline'}
          </span>
        </div>
        {onClose && (
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-white transition-colors"
          >
            <span className="material-symbols-outlined text-[18px]">close</span>
          </button>
        )}
      </div>

      {/* Status Details */}
      <div className="p-4 space-y-4">
        {/* Operation Status */}
        <div className="flex items-center justify-between">
          <span className="text-xs text-slate-500 uppercase tracking-wider">Status</span>
          <span className={`text-sm font-medium ${getStatusColor(status?.operation_status || 'offline')}`}>
            {status?.operation_status_display || '⚫ Unknown'}
          </span>
        </div>

        {/* Backend Type */}
        <div className="flex items-center justify-between">
          <span className="text-xs text-slate-500 uppercase tracking-wider">Backend</span>
          <span className="text-sm text-slate-300 font-mono">
            {status?.backend === 'qdrant' ? '🚀 Qdrant' : '💾 Sled'}
          </span>
        </div>

        {/* Memory Count */}
        {status?.memory_count !== undefined && (
          <div className="flex items-center justify-between">
            <span className="text-xs text-slate-500 uppercase tracking-wider">Memories</span>
            <span className="text-sm text-slate-300">
              {status.memory_count_display || `${status.memory_count} indexed`}
            </span>
          </div>
        )}

        {/* Evolution Count */}
        {status?.evolution_count !== undefined && (
          <div className="flex items-center justify-between">
            <span className="text-xs text-slate-500 uppercase tracking-wider">Evolutions</span>
            <span className="text-sm text-primary font-medium">
              {status.evolution_count} recorded
            </span>
          </div>
        )}

        {/* Collection */}
        <div className="flex items-center justify-between">
          <span className="text-xs text-slate-500 uppercase tracking-wider">Collection</span>
          <span className="text-sm text-slate-400 font-mono truncate max-w-[150px]">
            {status?.collection || 'sola_history'}
          </span>
        </div>

        {/* Semantic Recall Test */}
        {status?.enabled && (
          <div className="pt-3 border-t border-border-dark">
            <label className="text-xs text-slate-500 uppercase tracking-wider block mb-2">
              Test Semantic Recall
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={recallQuery}
                onChange={(e) => setRecallQuery(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && performRecall()}
                placeholder="Enter task or file path..."
                className="flex-1 bg-slate-800/50 border border-border-dark rounded-lg px-3 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-primary/50"
              />
              <button
                onClick={performRecall}
                disabled={isRecalling || !recallQuery.trim()}
                className="px-3 py-2 bg-primary/20 hover:bg-primary/30 text-primary rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <span className="material-symbols-outlined text-[18px]">
                  {isRecalling ? 'sync' : 'search'}
                </span>
              </button>
            </div>

            {/* Recall Results */}
            {recallResult && (
              <div className="mt-3 space-y-2">
                {recallResult.lessons_learned.length > 0 && (
                  <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-3">
                    <h4 className="text-xs font-bold text-green-400 uppercase mb-2">
                      ✅ Lessons Learned ({recallResult.lessons_learned.length})
                    </h4>
                    {recallResult.lessons_learned.map((lesson, i) => (
                      <div key={lesson.id} className="text-xs text-slate-300 mb-1">
                        <span className="text-green-400">[{(lesson.score * 100).toFixed(0)}%]</span>{' '}
                        {lesson.text.substring(0, 100)}...
                      </div>
                    ))}
                  </div>
                )}

                {recallResult.known_regressions.length > 0 && (
                  <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-3">
                    <h4 className="text-xs font-bold text-red-400 uppercase mb-2">
                      ⚠️ Known Regressions ({recallResult.known_regressions.length})
                    </h4>
                    {recallResult.known_regressions.map((regression, i) => (
                      <div key={regression.id} className="text-xs text-slate-300 mb-1">
                        <span className="text-red-400">[{(regression.score * 100).toFixed(0)}%]</span>{' '}
                        {regression.text.substring(0, 100)}...
                      </div>
                    ))}
                  </div>
                )}

                {recallResult.lessons_learned.length === 0 && recallResult.known_regressions.length === 0 && (
                  <div className="text-xs text-slate-500 italic">
                    No relevant memories found for this query.
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="px-4 py-2 bg-slate-800/20 border-t border-border-dark">
        <div className="flex items-center justify-between text-[10px] text-slate-500">
          <span>Phase I: Cognitive Bridge</span>
          <span className="font-mono">{status?.path || './data/vector_db'}</span>
        </div>
      </div>
    </div>
  );
};

export default KBStatusPanel;
