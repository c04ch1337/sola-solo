import React, { useState } from 'react';
import SystemVitalsCard from './SystemVitalsCard';
import LiveAgentsGrid from './LiveAgentsGrid';
import TaskOrchestrator from './TaskOrchestrator';

interface ProfessionalDashboardProps {
  // Add any props required
}

const ProfessionalDashboard: React.FC<ProfessionalDashboardProps> = () => {
  const [activeTab, setActiveTab] = useState<'overview' | 'agents' | 'tasks'>('overview');

  const renderTabContent = () => {
    switch (activeTab) {
      case 'overview':
        return (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
            <div className="">
              <SystemVitalsCard />
            </div>
            <div className="lg:col-span-2">
              <LiveAgentsGrid />
            </div>
            <div className="lg:col-span-3">
              <TaskOrchestrator />
            </div>
          </div>
        );
      case 'agents':
        return <LiveAgentsGrid />;
      case 'tasks':
        return <TaskOrchestrator />;
      default:
        return null;
    }
  };

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="border-b border-border-dark p-3 flex items-center gap-4">
        <h1 className="text-lg font-bold text-white flex items-center gap-2">
          <span className="material-symbols-outlined text-primary">workspaces</span>
          Professional Dashboard
        </h1>
        
        <div className="ml-auto flex items-center gap-2 text-xs">
          <button 
            className={`px-3 py-1.5 rounded-full ${
              activeTab === 'overview' 
                ? 'bg-primary/20 text-primary' 
                : 'text-gray-400 hover:bg-panel-dark hover:text-white'
            }`}
            onClick={() => setActiveTab('overview')}
          >
            Overview
          </button>
          <button 
            className={`px-3 py-1.5 rounded-full ${
              activeTab === 'agents' 
                ? 'bg-primary/20 text-primary' 
                : 'text-gray-400 hover:bg-panel-dark hover:text-white'
            }`}
            onClick={() => setActiveTab('agents')}
          >
            Agent Factory
          </button>
          <button 
            className={`px-3 py-1.5 rounded-full ${
              activeTab === 'tasks' 
                ? 'bg-primary/20 text-primary' 
                : 'text-gray-400 hover:bg-panel-dark hover:text-white'
            }`}
            onClick={() => setActiveTab('tasks')}
          >
            Task Orchestrator
          </button>
        </div>
        
        <div className="flex items-center gap-1">
          <button className="p-1.5 rounded text-gray-400 hover:text-white hover:bg-panel-dark">
            <span className="material-symbols-outlined text-[18px]">refresh</span>
          </button>
          <button className="p-1.5 rounded text-gray-400 hover:text-white hover:bg-panel-dark">
            <span className="material-symbols-outlined text-[18px]">tune</span>
          </button>
          <button className="p-1.5 rounded text-gray-400 hover:text-white hover:bg-panel-dark">
            <span className="material-symbols-outlined text-[18px]">more_vert</span>
          </button>
        </div>
      </div>
      
      <div className="flex-1 overflow-auto p-4">
        {renderTabContent()}
      </div>
      
      <div className="border-t border-border-dark px-3 py-2 text-xs text-gray-500 flex items-center justify-between">
        <span>L5 Procedural Memory: Active</span>
        <div className="flex items-center gap-2">
          <span>Last updated: {new Date().toLocaleTimeString()}</span>
          <span className="size-2 rounded-full bg-green-500"></span>
        </div>
      </div>
    </div>
  );
};

export default ProfessionalDashboard;