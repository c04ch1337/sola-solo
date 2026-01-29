import React, { useState, useEffect } from 'react';

interface ConsentToggleProps {
  onToggle?: (enabled: boolean) => void;
  initialValue?: boolean;
  showLabel?: boolean;
  compact?: boolean;
}

export const ConsentToggle: React.FC<ConsentToggleProps> = ({
  onToggle,
  initialValue = false,
  showLabel = true,
  compact = false
}) => {
  const [enabled, setEnabled] = useState(initialValue);
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    // Load from localStorage on mount
    const saved = localStorage.getItem('sola-access-porn-capability');
    if (saved !== null) {
      const savedValue = saved === 'true';
      setEnabled(savedValue);
      if (onToggle) onToggle(savedValue);
    }
  }, [onToggle]);

  const handleToggle = async () => {
    setIsAnimating(true);
    const newValue = !enabled;
    
    // Update global capability
    if (window.solaApp?.setAccessPornCapability) {
      try {
        window.solaApp.setAccessPornCapability(newValue);
      } catch (error) {
        console.error('Failed to update capability:', error);
      }
    }

    // Save to localStorage
    localStorage.setItem('sola-access-porn-capability', newValue.toString());
    
    setEnabled(newValue);
    if (onToggle) onToggle(newValue);
    
    // Animation complete
    setTimeout(() => setIsAnimating(false), 300);
  };

  const getToggleColor = () => {
    if (enabled) {
      return isAnimating ? 'bg-red-500/70' : 'bg-red-600';
    }
    return isAnimating ? 'bg-green-500/70' : 'bg-green-600';
  };

  const getIcon = () => {
    if (enabled) {
      return isAnimating ? '🔓' : '🔓';
    }
    return isAnimating ? '🔒' : '🔒';
  };

  const getStatusText = () => {
    if (enabled) {
      return 'Explicit Research Enabled';
    }
    return 'Professional Mode Only';
  };

  const getDescription = () => {
    if (enabled) {
      return 'Explicit research, NSFW content generation, and Tier 2 site access enabled';
    }
    return 'Only professional content and Tier 1 site access available';
  };

  const renderCompact = () => (
    <div className="flex items-center gap-2">
      <button
        onClick={handleToggle}
        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-all duration-300 ${getToggleColor()}`}
      >
        <span
          className={`inline-block h-4 w-4 transform bg-white rounded-full transition-transform duration-300 ${
            enabled ? 'translate-x-6' : 'translate-x-1'
          }`}
        />
      </button>
      {showLabel && (
        <span className="text-xs text-gray-400">{getStatusText()}</span>
      )}
    </div>
  );

  const renderFull = () => (
    <div className={`bg-panel-dark border border-border-dark rounded-lg p-4 transition-all duration-300 ${
      enabled ? 'border-red-500/30' : 'border-green-500/30'
    }`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-3">
          <span className="text-2xl">{getIcon()}</span>
          <div>
            <div className="font-semibold">Privacy & Consent Switch</div>
            <div className={`text-sm ${enabled ? 'text-red-400' : 'text-green-400'}`}>
              {getStatusText()}
            </div>
          </div>
        </div>
        
        {/* Toggle */}
        <button
          onClick={handleToggle}
          className={`relative inline-flex h-8 w-14 items-center rounded-full transition-all duration-300 ${getToggleColor()}`}
        >
          <span
            className={`inline-block h-6 w-6 transform bg-white rounded-full transition-transform duration-300 ${
              enabled ? 'translate-x-7' : 'translate-x-1'
            }`}
          />
        </button>
      </div>

      {/* Description */}
      <div className="text-sm text-gray-400 mb-3">
        {getDescription()}
      </div>

      {/* Capabilities List */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
        <div className={`p-2 rounded border ${enabled ? 'bg-red-500/10 border-red-500/30' : 'bg-gray-700/50 border-gray-600'}`}>
          <div className="font-medium">Image Generation</div>
          <div className="text-gray-400">
            {enabled ? '60% explicit ratio enabled' : 'Professional only'}
          </div>
        </div>
        
        <div className={`p-2 rounded border ${enabled ? 'bg-red-500/10 border-red-500/30' : 'bg-gray-700/50 border-gray-600'}`}>
          <div className="font-medium">Browser Research</div>
          <div className="text-gray-400">
            {enabled ? 'Tier 1 & 2 sites accessible' : 'Tier 1 sites only'}
          </div>
        </div>

        <div className={`p-2 rounded border ${enabled ? 'bg-red-500/10 border-red-500/30' : 'bg-gray-700/50 border-gray-600'}`}>
          <div className="font-medium">Memory Storage</div>
          <div className="text-gray-400">
            {enabled ? 'Layer 5 procedural storage active' : 'Layer 1-4 only'}
          </div>
        </div>

        <div className={`p-2 rounded border ${enabled ? 'bg-red-500/10 border-red-500/30' : 'bg-gray-700/50 border-gray-600'}`}>
          <div className="font-medium">Safety Protocols</div>
          <div className="text-gray-400">
            {enabled ? 'Advanced content filtering' : 'Restrictive filtering'}
          </div>
        </div>
      </div>

      {/* Warning */}
      {enabled && (
        <div className="mt-3 p-2 bg-red-500/10 border border-red-500/30 rounded flex items-center gap-2 text-red-400 text-xs">
          <span>⚠️</span>
          <span>
            Explicit content capabilities active. Ensure physical privacy and appropriate context.
          </span>
        </div>
      )}

      {/* Status Indicator */}
      <div className="flex items-center gap-2 mt-3 pt-3 border-t border-border-dark">
        <div className={`w-2 h-2 rounded-full ${enabled ? 'bg-red-500 animate-pulse' : 'bg-green-500'}`} />
        <span className="text-xs text-gray-400">
          Last updated: {new Date().toLocaleTimeString()}
        </span>
      </div>
    </div>
  );

  return compact ? renderCompact() : renderFull();
};

export default ConsentToggle;