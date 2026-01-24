import React, { useState, useEffect } from 'react';

interface Agent {
  id: string;
  name: string;
  type: string;
  contextUsage: number;
  status: 'idle' | 'active' | 'paused' | 'error';
  lastActivity: Date;
  logs: string[];
}

interface LiveAgentsGridProps {
  updateInterval?: number;
}

const LiveAgentsGrid: React.FC<LiveAgentsGridProps> = ({
  updateInterval = 3000
}) => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Mock function to generate random log entries for demonstration
  const generateRandomLog = (agentType: string): string => {
    const actions = [
      "Analyzing request...",
      "Searching knowledge base...",
      "Optimizing solution...",
      "Processing data stream...",
      "Generating response...",
      "Executing subroutine...",
      "Verifying credentials...",
      "Indexing content...",
      "Training on new samples...",
      "Evaluating performance metrics..."
    ];
    
    const targets = [
      "user query", 
      "system parameters",
      "input data",
      "memory store",
      "knowledge vector",
      "API response",
      "external service",
      "authentication token",
      "neural weights",
      "runtime environment"
    ];

    const results = [
      "completed successfully",
      "partial match found",
      "completed with warnings",
      "optimized result",
      "found relevant information",
      "identified pattern",
      "established connection",
      "validated structure",
      "initiated process"
    ];

    // Customize log based on agent type
    let specificActions = [];
    
    if (agentType === 'KinkResearcher') {
      specificActions = [
        "Exploring relationship dynamics...",
        "Analyzing preference patterns...",
        "Cataloging interest compatibility...",
        "Evaluating trust boundaries..."
      ];
    } else if (agentType === 'CodeOptimizer') {
      specificActions = [
        "Refactoring nested loops...",
        "Optimizing memory allocation...",
        "Reducing cyclomatic complexity...",
        "Implementing caching layer..."
      ];
    } else if (agentType === 'CreativeWriter') {
      specificActions = [
        "Generating narrative arc...",
        "Developing character profile...",
        "Crafting dialogue options...",
        "Enriching descriptive passages..."
      ];
    }

    // Either use a generic or specific action
    const useSpecific = specificActions.length > 0 && Math.random() > 0.7;
    const action = useSpecific 
      ? specificActions[Math.floor(Math.random() * specificActions.length)]
      : actions[Math.floor(Math.random() * actions.length)];
      
    const target = targets[Math.floor(Math.random() * targets.length)];
    const result = results[Math.floor(Math.random() * results.length)];
    
    const timestamp = new Date().toLocaleTimeString();
    
    // Format log with timestamp
    return `[${timestamp}] ${action} ${Math.random() > 0.5 ? target + " " + result : ""}`;
  };

  // Mock function to get simulated agent data
  const fetchAgentData = () => {
    // For demo, create some sample agents
    const mockAgents: Agent[] = [
      {
        id: 'agent-1',
        name: 'KinkResearcher',
        type: 'Research',
        contextUsage: Math.floor(20 + Math.random() * 60),
        status: Math.random() > 0.2 ? 'active' : 'idle',
        lastActivity: new Date(),
        logs: Array(5).fill(null).map(() => generateRandomLog('KinkResearcher'))
      },
      {
        id: 'agent-2',
        name: 'CodeOptimizer',
        type: 'Development',
        contextUsage: Math.floor(40 + Math.random() * 50),
        status: Math.random() > 0.1 ? 'active' : 'paused',
        lastActivity: new Date(Date.now() - 1000 * 60 * Math.random() * 10),
        logs: Array(5).fill(null).map(() => generateRandomLog('CodeOptimizer'))
      },
      {
        id: 'agent-3',
        name: 'CreativeWriter',
        type: 'Content',
        contextUsage: Math.floor(10 + Math.random() * 40),
        status: Math.random() > 0.3 ? 'active' : 'idle',
        lastActivity: new Date(Date.now() - 1000 * 60 * Math.random() * 5),
        logs: Array(5).fill(null).map(() => generateRandomLog('CreativeWriter'))
      }
    ];

    setAgents(mockAgents);
    setLoading(false);
  };

  useEffect(() => {
    // Initial fetch
    fetchAgentData();

    // Set up interval for updates
    const interval = setInterval(() => {
      // Update existing agents with new data
      setAgents(prevAgents => {
        return prevAgents.map(agent => {
          // Only update active agents
          if (agent.status === 'active' || Math.random() > 0.7) {
            const newLog = generateRandomLog(agent.name);
            const updatedLogs = [newLog, ...agent.logs].slice(0, 5);
            const contextChange = Math.random() > 0.5 ? 
              Math.min(95, agent.contextUsage + Math.floor(Math.random() * 10)) : 
              Math.max(5, agent.contextUsage - Math.floor(Math.random() * 10));
            
            return {
              ...agent,
              contextUsage: contextChange,
              lastActivity: new Date(),
              logs: updatedLogs
            };
          }
          return agent;
        });
      });
    }, updateInterval);

    return () => clearInterval(interval);
  }, [updateInterval]);

  const getContextUsageColor = (usage: number): string => {
    if (usage < 40) return 'bg-green-500';
    if (usage < 70) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const getStatusColor = (status: Agent['status']): string => {
    switch (status) {
      case 'active': return 'bg-green-500';
      case 'idle': return 'bg-gray-400';
      case 'paused': return 'bg-yellow-500';
      case 'error': return 'bg-red-500';
      default: return 'bg-gray-400';
    }
  };

  const getStatusText = (status: Agent['status']): string => {
    switch (status) {
      case 'active': return 'Active';
      case 'idle': return 'Idle';
      case 'paused': return 'Paused';
      case 'error': return 'Error';
      default: return 'Unknown';
    }
  };

  if (loading) {
    return (
      <div className="bg-panel-dark border border-border-dark rounded-lg p-4 h-full">
        <div className="flex items-center gap-2 mb-4">
          <span className="material-symbols-outlined text-lg text-primary">smart_toy</span>
          <h2 className="font-bold text-white">Live Agents</h2>
        </div>
        <div className="flex justify-center items-center h-32">
          <div className="animate-pulse text-gray-400">Loading agents...</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-panel-dark border border-border-dark rounded-lg p-4 h-full">
        <div className="flex items-center gap-2 mb-4">
          <span className="material-symbols-outlined text-lg text-primary">smart_toy</span>
          <h2 className="font-bold text-white">Live Agents</h2>
        </div>
        <div className="text-red-400 flex items-center justify-center h-32">
          <span className="material-symbols-outlined mr-2">error</span>
          {error}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-4 h-full">
      {/* Header */}
      <div className="flex justify-between items-center mb-4">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-lg text-primary">smart_toy</span>
          <h2 className="font-bold text-white">Live Agents</h2>
        </div>
        <span className="text-xs text-gray-400">
          {agents.filter(a => a.status === 'active').length} active / {agents.length} total
        </span>
      </div>

      {/* Agents Grid */}
      <div className="grid grid-cols-1 gap-4">
        {agents.map(agent => (
          <div 
            key={agent.id}
            className="border border-border-dark rounded-lg p-3 bg-background-dark hover:bg-panel-dark transition-colors"
          >
            <div className="flex justify-between items-start mb-2">
              <div>
                <div className="flex items-center gap-2">
                  <div className={`size-2.5 rounded-full ${getStatusColor(agent.status)}`}></div>
                  <span className="font-semibold text-white">{agent.name}</span>
                  <span className="text-xs px-1.5 py-0.5 rounded bg-primary/20 text-primary">{agent.type}</span>
                </div>
                <div className="text-xs text-gray-400 mt-1">
                  Last activity: {agent.lastActivity.toLocaleTimeString()}
                </div>
              </div>
              <div className="text-right">
                <div className="text-xs text-gray-400">Context Window</div>
                <div className="flex items-center gap-2">
                  <div className="w-16 bg-gray-700 rounded-full h-1.5">
                    <div 
                      className={`h-1.5 rounded-full ${getContextUsageColor(agent.contextUsage)}`}
                      style={{ width: `${agent.contextUsage}%` }}
                    ></div>
                  </div>
                  <span className="text-xs font-mono">{agent.contextUsage}%</span>
                </div>
              </div>
            </div>
            
            {/* Activity Logs */}
            <div className="mt-3 border border-border-dark rounded-md p-2 bg-black bg-opacity-20 h-24 overflow-y-auto custom-scrollbar">
              <div className="space-y-1">
                {agent.logs.map((log, index) => (
                  <div key={index} className="text-xs font-mono text-gray-400">
                    {log}
                  </div>
                ))}
                {agent.logs.length === 0 && (
                  <div className="text-xs italic text-gray-500">No recent activity</div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default LiveAgentsGrid;