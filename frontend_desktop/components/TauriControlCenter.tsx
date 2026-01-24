import React, { useState } from 'react';
import GPUMonitor from './GpuMonitor';
import ConsentToggle from './ConsentToggle';
import PhotoVault from './PhotoVault';

interface TauriControlCenterProps {
  minimized?: boolean;
}

export const TauriControlCenter: React.FC<TauriControlCenterProps> = ({ 
  minimized = false 
}) => {
  const [activeTab, setActiveTab] = useState<'monitor' | 'privacy' | 'photos'>('monitor');
  const [expanded, setExpanded] = useState(!minimized);

  const handleTabChange = (tab: 'monitor' | 'privacy' | 'photos') => {
    setActiveTab(tab);
    setExpanded(true);
  };

  const toggleExpanded = () => {
    setExpanded(!expanded);
  };

  const renderMinimizedView = () => (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-3 flex gap-3 items-center">
      <GPUMonitor compact />
      <div className="h-6 border-r border-border-dark"></div>
      <ConsentToggle compact showLabel={false} />
      <button
        className="ml-auto text-blue-400 hover:text-blue-300 text-xs flex items-center gap-1"
        onClick={toggleExpanded}
      >
        <span>Expand</span>
        <span>↗️</span>
      </button>
    </div>
  );

  if (!expanded) {
    return renderMinimizedView();
  }

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg overflow-hidden">
      {/* Header & Tabs */}
      <div className="p-4 border-b border-border-dark">
        <div className="flex justify-between items-center mb-4">
          <div className="flex items-center gap-2">
            <span className="text-2xl">🎛️</span>
            <h2 className="text-lg font-semibold">Sola Control Center</h2>
          </div>
          <button
            className="text-gray-400 hover:text-white"
            onClick={toggleExpanded}
          >
            ↘️
          </button>
        </div>
        
        <div className="flex space-x-1">
          <button
            className={`py-2 px-3 rounded-t text-sm font-medium transition-colors
              ${activeTab === 'monitor' 
                ? 'bg-blue-500 text-white' 
                : 'hover:bg-gray-700 text-gray-300'}`}
            onClick={() => handleTabChange('monitor')}
          >
            <span className="flex items-center gap-1">
              <span>📊</span>
              <span>GPU Monitor</span>
            </span>
          </button>
          
          <button
            className={`py-2 px-3 rounded-t text-sm font-medium transition-colors
              ${activeTab === 'privacy' 
                ? 'bg-blue-500 text-white' 
                : 'hover:bg-gray-700 text-gray-300'}`}
            onClick={() => handleTabChange('privacy')}
          >
            <span className="flex items-center gap-1">
              <span>🔒</span>
              <span>Privacy</span>
            </span>
          </button>
          
          <button
            className={`py-2 px-3 rounded-t text-sm font-medium transition-colors
              ${activeTab === 'photos' 
                ? 'bg-blue-500 text-white' 
                : 'hover:bg-gray-700 text-gray-300'}`}
            onClick={() => handleTabChange('photos')}
          >
            <span className="flex items-center gap-1">
              <span>📁</span>
              <span>Photo Vault</span>
            </span>
          </button>
        </div>
      </div>
      
      {/* Content Area */}
      <div className="p-4">
        {activeTab === 'monitor' && (
          <div>
            <GPUMonitor />
          </div>
        )}
        
        {activeTab === 'privacy' && (
          <div>
            <ConsentToggle />
          </div>
        )}
        
        {activeTab === 'photos' && (
          <div>
            <PhotoVault />
          </div>
        )}
      </div>
      
      {/* Status Footer */}
      <div className="border-t border-border-dark p-2 flex justify-between text-xs text-gray-400">
        <div>
          {window.solaApp?.getVersion 
            ? `Sola v${window.solaApp.getVersion}` 
            : 'Sola Desktop Control Center'}
        </div>
        <div>
          GPU Passthrough {activeTab === 'monitor' ? 'Active' : 'Ready'}
        </div>
      </div>
    </div>
  );
};

export default TauriControlCenter;