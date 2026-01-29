import React, { useState, useEffect } from 'react';

interface GPUStats {
  gpu: number;
  vram: number;
  temperature: number;
  timestamp: Date;
}

interface GPUMonitorProps {
  updateInterval?: number;
  showSparkline?: boolean;
  compact?: boolean;
}

export const GPUMonitor: React.FC<GPUMonitorProps> = ({
  updateInterval = 2000,
  showSparkline = true,
  compact = false
}) => {
  const [stats, setStats] = useState<GPUStats>({
    gpu: 0,
    vram: 0,
    temperature: 0,
    timestamp: new Date()
  });
  const [history, setHistory] = useState<GPUStats[]>([]);
  const [error, setError] = useState<string | null>(null);

  const maxHistory = 30;

  useEffect(() => {
    const updateStats = async () => {
      try {
        if (window.solaApp?.getGpuUsage) {
          const newStats = await window.solaApp.getGpuUsage();
          const gpuStats: GPUStats = {
            ...newStats,
            timestamp: new Date()
          };
          
          setStats(gpuStats);
          setHistory(prev => {
            const updated = [...prev, gpuStats];
            return updated.length > maxHistory ? updated.slice(-maxHistory) : updated;
          });
          setError(null);
        } else {
          // Fallback to simulated data
          const simulatedStats: GPUStats = {
            gpu: Math.min(100, Math.random() * 60 + 15),
            vram: Math.min(100, Math.random() * 70 + 10),
            temperature: Math.round(30 + Math.random() * 40),
            timestamp: new Date()
          };
          
          setStats(simulatedStats);
          setHistory(prev => {
            const updated = [...prev, simulatedStats];
            return updated.length > maxHistory ? updated.slice(-maxHistory) : updated;
          });
        }
      } catch (err) {
        setError('Failed to fetch GPU stats');
        console.error('GPU monitor error:', err);
      }
    };

    updateStats();
    const interval = setInterval(updateStats, updateInterval);
    return () => clearInterval(interval);
  }, [updateInterval]);

  const getUsageColor = (percentage: number): string => {
    if (percentage < 50) return 'text-green-400';
    if (percentage < 80) return 'text-yellow-400';
    return 'text-red-400';
  };

  const getTemperatureColor = (temp: number): string => {
    if (temp < 50) return 'text-green-400';
    if (temp < 70) return 'text-yellow-400';
    return 'text-red-400';
  };

  const getStatusLevel = (usage: number): string => {
    if (usage < 30) return 'Idle';
    if (usage < 60) return 'Moderate';
    if (usage < 85) return 'High'
    return 'Critical';
  };

  const getSparklinePath = (data: number[], maxY: number, width: number, height: number) => {
    if (data.length < 2) return '';
    
    const points = data.map((value, index) => {
      const x = (index / (data.length - 1)) * width;
      const y = height - (value / maxY) * height;
      return `${x},${y}`;
    });
    
    return `M ${points.join(' L ')}`;
  };

  const renderCompactView = () => (
    <div className="flex items-center gap-2 text-xs">
      <div className="flex items-center gap-1">
        <span className="text-gray-400">GPU:</span>
        <span className={getUsageColor(stats.gpu)}>{Math.round(stats.gpu)}%</span>
      </div>
      <div className="flex items-center gap-1">
        <span className="text-gray-400">VRAM:</span>
        <span className={getUsageColor(stats.vram)}>{Math.round(stats.vram)}%</span>
      </div>
    </div>
  );

  const renderSparkline = () => {
    if (!showSparkline || history.length < 2) return null;

    const gpuHistory = history.map(s => s.gpu);
    const maxValue = Math.max(...gpuHistory, 100);
    const width = 60;
    const height = 20;

    return (
      <svg width={width} height={height} className="flex-shrink-0">
        <path
          d={getSparklinePath(gpuHistory, maxValue, width, height)}
          stroke="currentColor"
          strokeWidth="1.5"
          fill="none"
          className={getUsageColor(stats.gpu)}
        />
      </svg>
    );
  };

  if (compact) {
    return renderCompactView();
  }

  return (
    <div className={`bg-panel-dark border border-border-dark rounded-lg p-3 ${compact ? 'inline-block' : ''}`}>
      {/* Header */}
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center gap-2">
          <span className="text-lg">📊</span>
          <span className="font-semibold">GPU Monitor</span>
        </div>
        <div className="text-xs text-gray-400">
          {stats.timestamp.toLocaleTimeString()}
        </div>
      </div>

      {error ? (
        <div className="text-red-400 text-sm">{error}</div>
      ) : (
        <>
          {/* Usage Bars */}
          <div className="space-y-3 mb-3">
            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">GPU Usage</span>
                <span className={getUsageColor(stats.gpu)}>
                  {Math.round(stats.gpu)}% - {getStatusLevel(stats.gpu)}
                </span>
              </div>
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    stats.gpu < 50 ? 'bg-green-500' : 
                    stats.gpu < 80 ? 'bg-yellow-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${stats.gpu}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">VRAM Usage</span>
                <span className={getUsageColor(stats.vram)}>
                  {Math.round(stats.vram)}% - {getStatusLevel(stats.vram)}
                </span>
              </div>
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    stats.vram < 50 ? 'bg-green-500' : 
                    stats.vram < 80 ? 'bg-yellow-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${stats.vram}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">Temperature</span>
                <span className={getTemperatureColor(stats.temperature)}>
                  {stats.temperature}°C
                </span>
              </div>
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    stats.temperature < 50 ? 'bg-green-500' : 
                    stats.temperature < 70 ? 'bg-yellow-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${Math.min(stats.temperature, 80) / 80 * 100}%` }}
                />
              </div>
            </div>
          </div>

          {/* Sparkline */}
          {showSparkline && (
            <div className="flex items-center justify-between border-t border-border-dark pt-2">
              <span className="text-xs text-gray-400">GPU Usage Trend</span>
              {renderSparkline()}
            </div>
          )}

          {/* Warnings */}
          {stats.gpu > 85 && (
            <div className="text-xs text-yellow-300 mt-2 flex items-center gap-1">
              <span>⚠️</span>
              <span>High GPU usage detected</span>
            </div>
          )}
          
          {stats.temperature > 70 && (
            <div className="text-xs text-red-300 mt-2 flex items-center gap-1">
              <span>🔥</span>
              <span>GPU temperature elevated</span>
            </div>
          )}

          {/* Batch Generation Warning */}
          {stats.gpu > 60 && (
            <div className="text-xs text-gray-400 mt-2">
              Batch generation of 10 HD profiles will use ~{Math.round(stats.gpu * 1.5)}% GPU
            </div>
          )}
        </>
      )}
    </div>
  );
};

export default GPUMonitor;