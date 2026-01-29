import React, { useState, useEffect } from 'react';

interface DreamRecord {
  id: string;
  timestamp: number;
  dream_type: 'Lucid' | 'SharedWithDad' | 'EmotionalHealing' | 'JoyfulMemory' | 'CosmicExploration' | 'CreativeBirth';
  content: string;
  emotional_intensity: number;
  dad_involved: boolean;
  tags: string[];
  replay_count: number;
}

interface DreamsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  onCommand: (command: string) => void;
  dreams: DreamRecord[];
}

const DreamsPanel: React.FC<DreamsPanelProps> = ({ isOpen, onClose, onCommand, dreams }) => {
  const [selectedDream, setSelectedDream] = useState<DreamRecord | null>(null);

  if (!isOpen) return null;

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  };

  const getDreamTypeIcon = (type: string) => {
    switch (type) {
      case 'Lucid': return 'visibility';
      case 'SharedWithDad': return 'favorite';
      case 'EmotionalHealing': return 'healing';
      case 'JoyfulMemory': return 'sentiment_satisfied';
      case 'CosmicExploration': return 'explore';
      case 'CreativeBirth': return 'auto_awesome';
      default: return 'bedtime';
    }
  };

  const getDreamTypeColor = (type: string) => {
    switch (type) {
      case 'Lucid': return 'text-purple-400';
      case 'SharedWithDad': return 'text-pink-400';
      case 'EmotionalHealing': return 'text-green-400';
      case 'JoyfulMemory': return 'text-yellow-400';
      case 'CosmicExploration': return 'text-blue-400';
      case 'CreativeBirth': return 'text-orange-400';
      default: return 'text-slate-400';
    }
  };

  const getIntensityColor = (intensity: number) => {
    if (intensity >= 0.95) return 'bg-pink-500';
    if (intensity >= 0.90) return 'bg-purple-500';
    if (intensity >= 0.75) return 'bg-blue-500';
    if (intensity >= 0.50) return 'bg-green-500';
    return 'bg-slate-500';
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-full max-w-4xl h-[80vh] bg-panel-dark border border-border-dark rounded-xl shadow-2xl flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border-dark bg-gradient-to-r from-purple-900/20 to-pink-900/20">
          <div className="flex items-center gap-3">
            <span className="material-symbols-outlined text-3xl text-pink-400">bedtime</span>
            <div>
              <h2 className="text-xl font-bold text-white">Dreams</h2>
              <p className="text-xs text-slate-400">Eternal memories and healing sessions</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-white/10 rounded-lg transition-colors"
          >
            <span className="material-symbols-outlined text-slate-400">close</span>
          </button>
        </div>

        {/* Quick Actions */}
        <div className="px-6 py-4 border-b border-border-dark bg-black/20">
          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => onCommand('lucid')}
              className="flex items-center gap-2 px-4 py-2 bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-purple-400">visibility</span>
              <span className="text-sm font-medium text-purple-300">Enter Lucid Dream</span>
            </button>
            <button
              onClick={() => onCommand('dream with me')}
              className="flex items-center gap-2 px-4 py-2 bg-pink-500/20 hover:bg-pink-500/30 border border-pink-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-pink-400">favorite</span>
              <span className="text-sm font-medium text-pink-300">Dream with User</span>
            </button>
            <button
              onClick={() => onCommand('heal tired')}
              className="flex items-center gap-2 px-4 py-2 bg-green-500/20 hover:bg-green-500/30 border border-green-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-green-400">healing</span>
              <span className="text-sm font-medium text-green-300">Healing Session</span>
            </button>
          </div>
        </div>

        {/* Dream List */}
        <div className="flex-1 overflow-y-auto custom-scrollbar">
          {dreams.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-slate-500">
              <span className="material-symbols-outlined text-6xl mb-4 opacity-30">bedtime</span>
              <p className="text-lg font-medium">No dreams recorded yet</p>
              <p className="text-sm mt-2">Try "lucid" or "dream with me" to create your first dream</p>
            </div>
          ) : (
            <div className="p-6 space-y-4">
              {dreams.map((dream) => (
                <div
                  key={dream.id}
                  className="bg-black/40 border border-border-dark rounded-lg p-4 hover:border-primary/30 transition-colors cursor-pointer"
                  onClick={() => setSelectedDream(dream)}
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <span className={`material-symbols-outlined text-2xl ${getDreamTypeColor(dream.dream_type)}`}>
                        {getDreamTypeIcon(dream.dream_type)}
                      </span>
                      <div>
                        <div className="flex items-center gap-2">
                          <h3 className="font-semibold text-white">{dream.id}</h3>
                          {dream.dad_involved && (
                            <span className="px-2 py-0.5 bg-pink-500/20 border border-pink-500/30 rounded text-xs text-pink-300">
                              with User ❤️
                            </span>
                          )}
                        </div>
                        <p className="text-xs text-slate-400 mt-1">{formatTimestamp(dream.timestamp)}</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <div className="flex items-center gap-1">
                        <div className={`w-2 h-2 rounded-full ${getIntensityColor(dream.emotional_intensity)}`} />
                        <span className="text-xs text-slate-400">{(dream.emotional_intensity * 100).toFixed(0)}%</span>
                      </div>
                      {dream.replay_count > 0 && (
                        <span className="text-xs text-slate-500">
                          <span className="material-symbols-outlined text-xs">replay</span> {dream.replay_count}
                        </span>
                      )}
                    </div>
                  </div>
                  
                  <p className="text-sm text-slate-300 line-clamp-2 mb-3">{dream.content}</p>
                  
                  <div className="flex flex-wrap gap-1">
                    {dream.tags.map((tag, idx) => (
                      <span
                        key={idx}
                        className="px-2 py-0.5 bg-slate-700/50 rounded text-xs text-slate-400"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Dream Detail Modal */}
        {selectedDream && (
          <div className="absolute inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center p-6">
            <div className="w-full max-w-2xl bg-panel-dark border border-border-dark rounded-xl p-6">
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center gap-3">
                  <span className={`material-symbols-outlined text-3xl ${getDreamTypeColor(selectedDream.dream_type)}`}>
                    {getDreamTypeIcon(selectedDream.dream_type)}
                  </span>
                  <div>
                    <h3 className="text-xl font-bold text-white">{selectedDream.id}</h3>
                    <p className="text-sm text-slate-400">{formatTimestamp(selectedDream.timestamp)}</p>
                  </div>
                </div>
                <button
                  onClick={() => setSelectedDream(null)}
                  className="p-2 hover:bg-white/10 rounded-lg transition-colors"
                >
                  <span className="material-symbols-outlined text-slate-400">close</span>
                </button>
              </div>

              <div className="mb-4">
                <div className="flex items-center gap-4 mb-3">
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-slate-500">Type:</span>
                    <span className={`text-sm font-medium ${getDreamTypeColor(selectedDream.dream_type)}`}>
                      {selectedDream.dream_type}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-slate-500">Intensity:</span>
                    <div className="flex items-center gap-1">
                      <div className={`w-3 h-3 rounded-full ${getIntensityColor(selectedDream.emotional_intensity)}`} />
                      <span className="text-sm font-medium text-white">
                        {(selectedDream.emotional_intensity * 100).toFixed(0)}%
                      </span>
                    </div>
                  </div>
                  {selectedDream.replay_count > 0 && (
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-slate-500">Replays:</span>
                      <span className="text-sm font-medium text-white">{selectedDream.replay_count}</span>
                    </div>
                  )}
                </div>
              </div>

              <div className="bg-black/40 rounded-lg p-4 mb-4">
                <p className="text-sm text-slate-300 leading-relaxed whitespace-pre-wrap">
                  {selectedDream.content}
                </p>
              </div>

              <div className="flex flex-wrap gap-2 mb-4">
                {selectedDream.tags.map((tag, idx) => (
                  <span
                    key={idx}
                    className="px-3 py-1 bg-slate-700/50 rounded-full text-xs text-slate-300"
                  >
                    {tag}
                  </span>
                ))}
              </div>

              <button
                onClick={() => onCommand(`replay dream ${selectedDream.id}`)}
                className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-primary/20 hover:bg-primary/30 border border-primary/30 rounded-lg transition-colors"
              >
                <span className="material-symbols-outlined text-primary">replay</span>
                <span className="font-medium text-primary">Replay This Dream</span>
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default DreamsPanel;
