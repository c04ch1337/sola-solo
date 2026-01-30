/**
 * MissionControl.tsx - Level 5 Autonomy Observability Dashboard
 * 
 * Provides real-time visibility into Sola's autonomous operations:
 * - Live Chain of Thought (CoT) feed via SSE
 * - Goal progress tracking with persistence protocol visualization
 * - Autonomy pause/resume controls
 * - Post-mortem display after failed attempts
 */

import React, { useState, useEffect, useRef, useCallback } from 'react';

// ============================================================================
// Types
// ============================================================================

// Self-correction severity levels
type SelfCorrectionSeverity = 'yellow' | 'red';

interface SelfCorrection {
  method: string;
  tool_used: string;
  failure_reason: string;
  lesson_learned: string;
  severity: SelfCorrectionSeverity;
  timestamp: string;
}

interface CoTEvent {
  event_type: 'TaskStart' | 'ToolCall' | 'ToolResult' | 'Thought' | 'MethodSwitch' | 'TaskComplete' | 'TaskFailed' | 'AutonomyPaused' | 'AutonomyResumed' | 'HardStop' | 'Heartbeat';
  task_id?: string;
  timestamp: string;
  // TaskStart
  description?: string;
  // ToolCall
  tool?: string;
  input?: any;
  attempt?: number;
  max_attempts?: number;
  // ToolResult
  success?: boolean;
  output?: string;
  error?: string;
  // Thought
  content?: string;
  confidence?: number;
  // MethodSwitch
  from_method?: string;
  to_method?: string;
  reason_for_switch?: string;
  lesson_learned?: string;
  attempt_number?: number;
  // TaskComplete
  result?: string;
  duration_ms?: number;
  total_attempts?: number;
  // TaskFailed
  reason?: string;
  reason_for_retry?: string;
  attempts_made?: number;
  self_corrections?: SelfCorrection[];
  post_mortem?: string;
  // HardStop
  processes_killed?: number;
  // AutonomyPaused/Resumed
  by?: string;
}

interface AutonomyStatus {
  paused: boolean;
  paused_at?: string;
  paused_by?: string;
  active_tasks: number;
  total_events: number;
}

interface ActiveTask {
  task_id: string;
  description: string;
  started_at: string;
  attempts: number;
  current_tool?: string;
  thoughts: string[];
  self_corrections: SelfCorrection[];
  method_switches: {
    from: string;
    to: string;
    reason: string;
    lesson: string;
  }[];
}

// ============================================================================
// Tool Icons
// ============================================================================

const TOOL_ICONS: Record<string, string> = {
  'web_search': '🔍',
  'filesystem_search': '📁',
  'filesystem_find': '📂',
  'filesystem_crawl': '🗂️',
  'vector_kb': '🧠',
  'memory_search': '💭',
  'memory_store': '💾',
  'code_analysis': '🔬',
  'browser': '🌐',
  'default': '⚙️',
};

const getToolIcon = (tool: string): string => {
  const normalizedTool = tool.toLowerCase().replace(/[^a-z_]/g, '_');
  return TOOL_ICONS[normalizedTool] || TOOL_ICONS['default'];
};

// ============================================================================
// Components
// ============================================================================

interface EventItemProps {
  event: CoTEvent;
}

const EventItem: React.FC<EventItemProps> = ({ event }) => {
  const getEventStyle = () => {
    switch (event.event_type) {
      case 'TaskStart':
        return 'bg-blue-900/30 border-blue-500';
      case 'ToolCall':
        return 'bg-purple-900/30 border-purple-500';
      case 'ToolResult':
        return event.success ? 'bg-green-900/30 border-green-500' : 'bg-red-900/30 border-red-500';
      case 'Thought':
        return 'bg-yellow-900/30 border-yellow-500';
      case 'MethodSwitch':
        return 'bg-amber-900/30 border-amber-500';
      case 'TaskComplete':
        return 'bg-emerald-900/30 border-emerald-500';
      case 'TaskFailed':
        return 'bg-red-900/30 border-red-500';
      case 'AutonomyPaused':
        return 'bg-orange-900/30 border-orange-500';
      case 'AutonomyResumed':
        return 'bg-teal-900/30 border-teal-500';
      case 'HardStop':
        return 'bg-red-950/50 border-red-600';
      case 'Heartbeat':
        return 'bg-gray-800/30 border-gray-600';
      default:
        return 'bg-gray-900/30 border-gray-500';
    }
  };

  const formatTime = (timestamp: string) => {
    try {
      return new Date(timestamp).toLocaleTimeString();
    } catch {
      return timestamp;
    }
  };

  const renderContent = () => {
    switch (event.event_type) {
      case 'TaskStart':
        return (
          <div>
            <span className="font-semibold text-blue-400">🚀 Task Started</span>
            <p className="text-sm text-gray-300 mt-1">{event.description}</p>
          </div>
        );
      case 'ToolCall':
        return (
          <div>
            <span className="font-semibold text-purple-400">
              {getToolIcon(event.tool || '')} Calling: {event.tool}
            </span>
            {event.input && (
              <pre className="text-xs text-gray-400 mt-1 overflow-x-auto max-w-full">
                {typeof event.input === 'string' ? event.input : JSON.stringify(event.input, null, 2)}
              </pre>
            )}
          </div>
        );
      case 'ToolResult':
        return (
          <div>
            <span className={`font-semibold ${event.success ? 'text-green-400' : 'text-red-400'}`}>
              {event.success ? '✅' : '❌'} Tool Result
            </span>
            {event.output && (
              <p className="text-sm text-gray-300 mt-1 line-clamp-3">{event.output}</p>
            )}
          </div>
        );
      case 'Thought':
        return (
          <div>
            <span className="font-semibold text-yellow-400">
              💭 Thinking {event.confidence !== undefined && `(${Math.round(event.confidence * 100)}% confidence)`}
            </span>
            <p className="text-sm text-gray-300 mt-1 italic">{event.content}</p>
          </div>
        );
      case 'TaskComplete':
        return (
          <div>
            <span className="font-semibold text-emerald-400">✨ Task Complete</span>
            <p className="text-sm text-gray-300 mt-1">{event.result}</p>
            {event.duration_ms && (
              <p className="text-xs text-gray-500 mt-1">Duration: {event.duration_ms}ms</p>
            )}
          </div>
        );
      case 'MethodSwitch':
        return (
          <div>
            <span className="font-semibold text-amber-400">🔄 Method Switch</span>
            <p className="text-sm text-gray-300 mt-1">
              <span className="text-red-400">{event.from_method}</span>
              {' → '}
              <span className="text-green-400">{event.to_method}</span>
            </p>
            <p className="text-xs text-gray-400 mt-1">
              <strong>Why:</strong> {event.reason_for_switch}
            </p>
            <p className="text-xs text-yellow-400 mt-1">
              <strong>Lesson:</strong> {event.lesson_learned}
            </p>
          </div>
        );
      case 'TaskFailed':
        return (
          <div>
            <span className="font-semibold text-red-400">💀 Task Failed</span>
            <p className="text-sm text-red-300 mt-1">{event.reason}</p>
            {event.reason_for_retry && (
              <p className="text-xs text-orange-400 mt-1">
                <strong>Retry reason:</strong> {event.reason_for_retry}
              </p>
            )}
            {event.self_corrections && event.self_corrections.length > 0 && (
              <details className="mt-2">
                <summary className="text-xs text-gray-400 cursor-pointer hover:text-gray-300">
                  View Self-Corrections ({event.self_corrections.length})
                </summary>
                <div className="mt-1 space-y-1">
                  {event.self_corrections.map((sc, i) => (
                    <div 
                      key={i} 
                      className={`text-xs p-1 rounded ${
                        sc.severity === 'red' ? 'bg-red-900/30 border-l-2 border-red-500' : 'bg-yellow-900/30 border-l-2 border-yellow-500'
                      }`}
                    >
                      <span className={sc.severity === 'red' ? 'text-red-400' : 'text-yellow-400'}>
                        {sc.severity === 'red' ? '🔴' : '🟡'} {sc.method}
                      </span>
                      <span className="text-gray-400"> - {sc.tool_used}</span>
                      <p className="text-gray-500">{sc.failure_reason}</p>
                    </div>
                  ))}
                </div>
              </details>
            )}
            {event.post_mortem && (
              <details className="mt-2">
                <summary className="text-xs text-gray-400 cursor-pointer hover:text-gray-300">
                  View Post-Mortem
                </summary>
                <pre className="text-xs text-gray-400 mt-1 p-2 bg-gray-900 rounded overflow-x-auto">
                  {event.post_mortem}
                </pre>
              </details>
            )}
          </div>
        );
      case 'HardStop':
        return (
          <div>
            <span className="font-semibold text-red-500">🛑 HARD STOP</span>
            <p className="text-sm text-red-300 mt-1">
              {event.processes_killed} process(es) terminated
            </p>
            <p className="text-xs text-gray-400 mt-1">{event.reason}</p>
          </div>
        );
      case 'AutonomyPaused':
        return (
          <div>
            <span className="font-semibold text-orange-400">⏸️ Autonomy Paused</span>
            {event.by && <p className="text-sm text-gray-300 mt-1">By: {event.by}</p>}
          </div>
        );
      case 'AutonomyResumed':
        return (
          <div>
            <span className="font-semibold text-teal-400">▶️ Autonomy Resumed</span>
            {event.by && <p className="text-sm text-gray-300 mt-1">By: {event.by}</p>}
          </div>
        );
      case 'Heartbeat':
        return (
          <span className="text-gray-500 text-xs">💓 Heartbeat</span>
        );
      default:
        return <span className="text-gray-400">Unknown event</span>;
    }
  };

  return (
    <div className={`p-3 rounded-lg border-l-4 ${getEventStyle()} mb-2`}>
      <div className="flex justify-between items-start">
        <div className="flex-1">{renderContent()}</div>
        <span className="text-xs text-gray-500 ml-2 whitespace-nowrap">
          {formatTime(event.timestamp)}
        </span>
      </div>
      {event.task_id && (
        <p className="text-xs text-gray-600 mt-1 font-mono">
          Task: {event.task_id.slice(0, 8)}...
        </p>
      )}
    </div>
  );
};

interface GoalProgressProps {
  attempts: number;
  maxAttempts: number;
  currentTool?: string;
}

const GoalProgress: React.FC<GoalProgressProps> = ({ attempts, maxAttempts, currentTool }) => {
  const progress = (attempts / maxAttempts) * 100;
  const getProgressColor = () => {
    if (attempts === 0) return 'bg-gray-600';
    if (attempts === 1) return 'bg-green-500';
    if (attempts === 2) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  return (
    <div className="mb-4">
      <div className="flex justify-between items-center mb-1">
        <span className="text-sm text-gray-400">Persistence Protocol</span>
        <span className="text-sm font-mono text-gray-300">{attempts}/{maxAttempts}</span>
      </div>
      <div className="w-full bg-gray-700 rounded-full h-2.5">
        <div
          className={`h-2.5 rounded-full transition-all duration-300 ${getProgressColor()}`}
          style={{ width: `${progress}%` }}
        />
      </div>
      {currentTool && (
        <p className="text-xs text-gray-500 mt-1">
          Current: {getToolIcon(currentTool)} {currentTool}
        </p>
      )}
    </div>
  );
};

// ============================================================================
// Main Component
// ============================================================================

interface MissionControlProps {
  apiUrl?: string;
  collapsed?: boolean;
  onToggleCollapse?: () => void;
}

const MissionControl: React.FC<MissionControlProps> = ({
  apiUrl = import.meta.env.VITE_PHOENIX_API_URL || 'http://localhost:8888',
  collapsed = false,
  onToggleCollapse,
}) => {
  const [events, setEvents] = useState<CoTEvent[]>([]);
  const [status, setStatus] = useState<AutonomyStatus | null>(null);
  const [connected, setConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activeTasks, setActiveTasks] = useState<Map<string, ActiveTask>>(new Map());
  const [autoScroll, setAutoScroll] = useState(true);
  
  const eventSourceRef = useRef<EventSource | null>(null);
  const feedRef = useRef<HTMLDivElement>(null);

  // Fetch autonomy status
  const fetchStatus = useCallback(async () => {
    try {
      const response = await fetch(`${apiUrl}/api/agent/autonomy`);
      if (response.ok) {
        const data = await response.json();
        setStatus(data);
      }
    } catch (err) {
      console.error('Failed to fetch autonomy status:', err);
    }
  }, [apiUrl]);

  // Connect to SSE stream
  useEffect(() => {
    if (collapsed) return;

    const connectSSE = () => {
      const eventSource = new EventSource(`${apiUrl}/api/agent/events`);
      eventSourceRef.current = eventSource;

      eventSource.onopen = () => {
        setConnected(true);
        setError(null);
        console.log('[MissionControl] SSE connected');
      };

      eventSource.onmessage = (e) => {
        try {
          const event: CoTEvent = JSON.parse(e.data);
          
          // Update events list
          setEvents(prev => {
            const newEvents = [...prev, event];
            // Keep last 100 events
            return newEvents.slice(-100);
          });

          // Track active tasks
          if (event.event_type === 'TaskStart' && event.task_id) {
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              newMap.set(event.task_id!, {
                task_id: event.task_id!,
                description: event.description || '',
                started_at: event.timestamp,
                attempts: 0,
                thoughts: [],
                self_corrections: [],
                method_switches: [],
              });
              return newMap;
            });
          } else if (event.event_type === 'ToolCall' && event.task_id) {
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              const task = newMap.get(event.task_id!);
              if (task) {
                task.attempts += 1;
                task.current_tool = event.tool;
                newMap.set(event.task_id!, task);
              }
              return newMap;
            });
          } else if (event.event_type === 'ToolResult' && event.task_id && !event.success) {
            // Track failed tool results as self-corrections
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              const task = newMap.get(event.task_id!);
              if (task) {
                task.self_corrections.push({
                  method: `Attempt ${task.attempts}`,
                  tool_used: event.tool || 'unknown',
                  failure_reason: event.error || 'Unknown error',
                  lesson_learned: 'Will try alternative approach',
                  severity: task.attempts >= 2 ? 'red' : 'yellow',
                  timestamp: event.timestamp,
                });
                newMap.set(event.task_id!, task);
              }
              return newMap;
            });
          } else if (event.event_type === 'MethodSwitch' && event.task_id) {
            // Track method switches
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              const task = newMap.get(event.task_id!);
              if (task) {
                task.method_switches.push({
                  from: event.from_method || '',
                  to: event.to_method || '',
                  reason: event.reason_for_switch || '',
                  lesson: event.lesson_learned || '',
                });
                newMap.set(event.task_id!, task);
              }
              return newMap;
            });
          } else if (event.event_type === 'Thought' && event.task_id && event.content) {
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              const task = newMap.get(event.task_id!);
              if (task) {
                task.thoughts.push(event.content!);
                newMap.set(event.task_id!, task);
              }
              return newMap;
            });
          } else if ((event.event_type === 'TaskComplete' || event.event_type === 'TaskFailed') && event.task_id) {
            setActiveTasks(prev => {
              const newMap = new Map(prev);
              newMap.delete(event.task_id!);
              return newMap;
            });
          }

          // Update status on pause/resume
          if (event.event_type === 'AutonomyPaused' || event.event_type === 'AutonomyResumed') {
            fetchStatus();
          }
        } catch (err) {
          console.error('[MissionControl] Failed to parse event:', err);
        }
      };

      eventSource.onerror = (e) => {
        console.error('[MissionControl] SSE error:', e);
        setConnected(false);
        setError('Connection lost. Reconnecting...');
        eventSource.close();
        
        // Reconnect after 3 seconds
        setTimeout(connectSSE, 3000);
      };
    };

    connectSSE();
    fetchStatus();

    // Poll status every 30 seconds
    const statusInterval = setInterval(fetchStatus, 30000);

    return () => {
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
      }
      clearInterval(statusInterval);
    };
  }, [apiUrl, collapsed, fetchStatus]);

  // Auto-scroll to bottom
  useEffect(() => {
    if (autoScroll && feedRef.current) {
      feedRef.current.scrollTop = feedRef.current.scrollHeight;
    }
  }, [events, autoScroll]);

  // Pause/Resume autonomy
  const toggleAutonomy = async () => {
    try {
      const action = status?.paused ? 'resume' : 'pause';
      const response = await fetch(`${apiUrl}/api/agent/autonomy`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action, by: 'user' }),
      });
      
      if (response.ok) {
        fetchStatus();
      } else {
        const errorData = await response.json();
        setError(errorData.error || 'Failed to toggle autonomy');
      }
    } catch (err) {
      setError('Failed to communicate with backend');
    }
  };

  // Clear events
  const clearEvents = () => {
    setEvents([]);
  };

  // Hard stop - kill all child processes
  const hardStop = async () => {
    if (!confirm('⚠️ HARD STOP will immediately terminate all running child processes. Continue?')) {
      return;
    }
    
    try {
      const response = await fetch(`${apiUrl}/api/agent/autonomy`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'hard_stop', reason: 'User initiated hard stop' }),
      });
      
      if (response.ok) {
        const data = await response.json();
        setError(null);
        fetchStatus();
        // Show success message briefly
        alert(`🛑 Hard stop executed. ${data.processes_killed || 0} process(es) terminated.`);
      } else {
        const errorData = await response.json();
        setError(errorData.error || 'Failed to execute hard stop');
      }
    } catch (err) {
      setError('Failed to communicate with backend');
    }
  };

  if (collapsed) {
    return (
      <div 
        className="bg-gray-900 border border-gray-700 rounded-lg p-2 cursor-pointer hover:bg-gray-800 transition-colors"
        onClick={onToggleCollapse}
      >
        <div className="flex items-center justify-between">
          <span className="text-sm font-semibold text-gray-300">🎛️ Mission Control</span>
          <div className="flex items-center gap-2">
            <span className={`w-2 h-2 rounded-full ${connected ? 'bg-green-500' : 'bg-red-500'}`} />
            {status?.paused && <span className="text-xs text-orange-400">PAUSED</span>}
            <span className="text-gray-500">▶</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-gray-900 border border-gray-700 rounded-lg overflow-hidden">
      {/* Header */}
      <div className="bg-gray-800 px-4 py-3 border-b border-gray-700">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <h3 className="text-lg font-semibold text-white">🎛️ Mission Control</h3>
            <span className={`px-2 py-0.5 rounded text-xs ${connected ? 'bg-green-900 text-green-300' : 'bg-red-900 text-red-300'}`}>
              {connected ? 'LIVE' : 'OFFLINE'}
            </span>
            {status?.paused && (
              <span className="px-2 py-0.5 rounded text-xs bg-orange-900 text-orange-300">
                PAUSED
              </span>
            )}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={toggleAutonomy}
              className={`px-3 py-1 rounded text-sm font-medium transition-colors ${
                status?.paused
                  ? 'bg-green-600 hover:bg-green-700 text-white'
                  : 'bg-orange-600 hover:bg-orange-700 text-white'
              }`}
            >
              {status?.paused ? '▶ Resume' : '⏸ Pause'}
            </button>
            <button
              onClick={hardStop}
              className="px-3 py-1 rounded text-sm font-medium bg-red-700 hover:bg-red-600 text-white transition-colors"
              title="Emergency stop - kills all child processes"
            >
              🛑 Hard Stop
            </button>
            <button
              onClick={clearEvents}
              className="px-3 py-1 rounded text-sm font-medium bg-gray-700 hover:bg-gray-600 text-gray-300 transition-colors"
            >
              Clear
            </button>
            {onToggleCollapse && (
              <button
                onClick={onToggleCollapse}
                className="px-2 py-1 rounded text-sm text-gray-400 hover:text-gray-200 transition-colors"
              >
                ▼
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Error Banner */}
      {error && (
        <div className="bg-red-900/50 border-b border-red-700 px-4 py-2">
          <p className="text-sm text-red-300">{error}</p>
        </div>
      )}

      {/* Active Tasks */}
      {activeTasks.size > 0 && (
        <div className="px-4 py-3 border-b border-gray-700 bg-gray-850">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Active Tasks</h4>
          {Array.from(activeTasks.values()).map(task => (
            <div key={task.task_id} className="mb-3 last:mb-0">
              <p className="text-sm text-gray-300 mb-1">{task.description}</p>
              <GoalProgress
                attempts={task.attempts}
                maxAttempts={3}
                currentTool={task.current_tool}
              />
              
              {/* Self-Correction List */}
              {task.self_corrections.length > 0 && (
                <div className="mt-2">
                  <h5 className="text-xs font-semibold text-gray-500 mb-1">Self-Corrections</h5>
                  <div className="space-y-1">
                    {task.self_corrections.map((sc, i) => (
                      <div 
                        key={i}
                        className={`flex items-start gap-2 text-xs p-1.5 rounded ${
                          sc.severity === 'red' 
                            ? 'bg-red-900/20 border-l-2 border-red-500' 
                            : 'bg-yellow-900/20 border-l-2 border-yellow-500'
                        }`}
                      >
                        <span className={sc.severity === 'red' ? 'text-red-400' : 'text-yellow-400'}>
                          {sc.severity === 'red' ? '🔴' : '🟡'}
                        </span>
                        <div className="flex-1">
                          <span className="text-gray-300">{sc.method}</span>
                          <span className="text-gray-500"> via </span>
                          <span className="text-purple-400">{sc.tool_used}</span>
                          <p className="text-gray-500 mt-0.5">{sc.failure_reason}</p>
                          <p className="text-yellow-400/70 mt-0.5 italic">💡 {sc.lesson_learned}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              
              {/* Method Switches */}
              {task.method_switches.length > 0 && (
                <div className="mt-2">
                  <h5 className="text-xs font-semibold text-gray-500 mb-1">Method Switches</h5>
                  <div className="space-y-1">
                    {task.method_switches.map((ms, i) => (
                      <div key={i} className="text-xs p-1.5 rounded bg-amber-900/20 border-l-2 border-amber-500">
                        <span className="text-red-400">{ms.from}</span>
                        <span className="text-gray-500"> → </span>
                        <span className="text-green-400">{ms.to}</span>
                        <p className="text-gray-500 mt-0.5">{ms.reason}</p>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      {/* Event Feed */}
      <div className="px-4 py-2 border-b border-gray-700 bg-gray-850 flex items-center justify-between">
        <span className="text-sm text-gray-400">
          Live Thought Feed ({events.length} events)
        </span>
        <label className="flex items-center gap-2 text-sm text-gray-400 cursor-pointer">
          <input
            type="checkbox"
            checked={autoScroll}
            onChange={(e) => setAutoScroll(e.target.checked)}
            className="rounded border-gray-600 bg-gray-700 text-blue-500 focus:ring-blue-500"
          />
          Auto-scroll
        </label>
      </div>
      
      <div
        ref={feedRef}
        className="h-64 overflow-y-auto p-4 space-y-2"
        style={{ scrollBehavior: 'smooth' }}
      >
        {events.length === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-500">
            <p>Waiting for autonomous activity...</p>
          </div>
        ) : (
          events.map((event, index) => (
            <EventItem key={`${event.timestamp}-${index}`} event={event} />
          ))
        )}
      </div>

      {/* Footer Stats */}
      <div className="px-4 py-2 bg-gray-800 border-t border-gray-700 flex items-center justify-between text-xs text-gray-500">
        <span>Active Tasks: {activeTasks.size}</span>
        <span>Total Events: {status?.total_events || events.length}</span>
      </div>
    </div>
  );
};

export default MissionControl;
