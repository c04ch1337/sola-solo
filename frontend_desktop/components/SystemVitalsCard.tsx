import React, { useState, useEffect, useRef } from 'react';
import { createChart, ColorType } from 'lightweight-charts';

interface GPUStats {
  gpu: number;
  vram: number;
  temperature: number;
  agentLatency?: number;
  timestamp: Date;
}

interface SystemVitalsCardProps {
  updateInterval?: number;
}

const SystemVitalsCard: React.FC<SystemVitalsCardProps> = ({ 
  updateInterval = 2000 
}) => {
  const [stats, setStats] = useState<GPUStats>({
    gpu: 0,
    vram: 0,
    temperature: 0,
    agentLatency: 0,
    timestamp: new Date()
  });
  const [history, setHistory] = useState<GPUStats[]>([]);
  const [error, setError] = useState<string | null>(null);

  // Chart refs
  const vramChartRef = useRef<HTMLDivElement>(null);
  const tempChartRef = useRef<HTMLDivElement>(null);
  const latencyChartRef = useRef<HTMLDivElement>(null);

  // Chart instances
  const vramChartInstance = useRef<any>(null);
  const tempChartInstance = useRef<any>(null);
  const latencyChartInstance = useRef<any>(null);
  
  // Series refs
  const vramSeries = useRef<any>(null);
  const tempSeries = useRef<any>(null);
  const latencySeries = useRef<any>(null);

  const maxHistory = 60; // 2 minutes of data with 2-second intervals

  useEffect(() => {
    const updateStats = async () => {
      try {
        if (window.solaApp?.getGpuUsage) {
          const newStats = await window.solaApp.getGpuUsage();
          const gpuStats: GPUStats = {
            ...newStats,
            // Simulate agent latency for now - this would come from a real endpoint in production
            agentLatency: Math.round(10 + Math.random() * 90),
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
            agentLatency: Math.round(10 + Math.random() * 90),
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

  // Initialize charts
  useEffect(() => {
    if (!vramChartRef.current || !tempChartRef.current || !latencyChartRef.current) return;
    
    // Initialize charts if they don't exist
    if (!vramChartInstance.current) {
      // VRAM Chart
      vramChartInstance.current = createChart(vramChartRef.current, {
        width: 96,
        height: 40,
        layout: {
          background: { type: ColorType.Solid, color: 'transparent' },
          textColor: 'transparent',
        },
        grid: {
          vertLines: { visible: false },
          horzLines: { visible: false },
        },
        rightPriceScale: { visible: false },
        timeScale: { visible: false },
        crosshair: { mode: 0 },
        handleScroll: false,
        handleScale: false,
      });
      
      // Add VRAM series
      vramSeries.current = vramChartInstance.current.addLineSeries({
        color: stats.vram < 50 ? '#4ade80' : stats.vram < 80 ? '#facc15' : '#ef4444',
        lineWidth: 2,
      });
    }
    
    if (!tempChartInstance.current) {
      // Temperature Chart
      tempChartInstance.current = createChart(tempChartRef.current, {
        width: 96,
        height: 40,
        layout: {
          background: { type: ColorType.Solid, color: 'transparent' },
          textColor: 'transparent',
        },
        grid: {
          vertLines: { visible: false },
          horzLines: { visible: false },
        },
        rightPriceScale: { visible: false },
        timeScale: { visible: false },
        crosshair: { mode: 0 },
        handleScroll: false,
        handleScale: false,
      });
      
      // Add Temperature series
      tempSeries.current = tempChartInstance.current.addLineSeries({
        color: stats.temperature < 50 ? '#4ade80' : stats
        .temperature < 70 ? '#facc15' : '#ef4444',
        lineWidth: 2,
      });
    }
    
    if (!latencyChartInstance.current) {
      // Latency Chart
      latencyChartInstance.current = createChart(latencyChartRef.current, {
        width: 96,
        height: 40,
        layout: {
          background: { type: ColorType.Solid, color: 'transparent' },
          textColor: 'transparent',
        },
        grid: {
          vertLines: { visible: false },
          horzLines: { visible: false },
        },
        rightPriceScale: { visible: false },
        timeScale: { visible: false },
        crosshair: { mode: 0 },
        handleScroll: false,
        handleScale: false,
      });
      
      // Add Latency series
      latencySeries.current = latencyChartInstance.current.addLineSeries({
        color: (stats.agentLatency || 0) < 30 ? '#4ade80' : 
               (stats.agentLatency || 0) < 70 ? '#facc15' : '#ef4444',
        lineWidth: 2,
      });
    }
    
    // Cleanup function
    return () => {
      if (vramChartInstance.current) {
        vramChartInstance.current.remove();
        vramChartInstance.current = null;
      }
      if (tempChartInstance.current) {
        tempChartInstance.current.remove();
        tempChartInstance.current = null;
      }
      if (latencyChartInstance.current) {
        latencyChartInstance.current.remove();
        latencyChartInstance.current = null;
      }
    };
  }, []);

  // Update chart data when history changes
  useEffect(() => {
    if (history.length < 2) return;

    // Format data for charts
    const chartData = history.map((item, index) => ({
      time: index,
      value: item.vram
    }));
    
    const tempChartData = history.map((item, index) => ({
      time: index,
      value: item.temperature
    }));
    
    const latencyChartData = history.map((item, index) => ({
      time: index,
      value: item.agentLatency || 0
    }));
    
    // Update series data
    if (vramSeries.current) {
      vramSeries.current.setData(chartData);
      vramSeries.current.applyOptions({
        color: stats.vram < 50 ? '#4ade80' : stats.vram < 80 ? '#facc15' : '#ef4444',
      });
    }
    
    if (tempSeries.current) {
      tempSeries.current.setData(tempChartData);
      tempSeries.current.applyOptions({
        color: stats.temperature < 50 ? '#4ade80' : stats.temperature < 70 ? '#facc15' : '#ef4444',
      });
    }
    
    if (latencySeries.current) {
      latencySeries.current.setData(latencyChartData);
      latencySeries.current.applyOptions({
        color: (stats.agentLatency || 0) < 30 ? '#4ade80' : 
               (stats.agentLatency || 0) < 70 ? '#facc15' : '#ef4444',
      });
    }
  }, [history, stats]);

  const getUsageColor = (percentage: number): string => {
    if (percentage < 50) return 'text-green-400';
    if (percentage < 80) return 'text-yellow-400';
    return 'text-red-400';
  };

  const getUsageBgColor = (percentage: number): string => {
    if (percentage < 50) return 'bg-green-500';
    if (percentage < 80) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const getTemperatureColor = (temp: number): string => {
    if (temp < 50) return 'text-green-400';
    if (temp < 70) return 'text-yellow-400';
    return 'text-red-400';
  };

  const getStatusLevel = (usage: number): string => {
    if (usage < 30) return 'Idle';
    if (usage < 60) return 'Moderate';
    if (usage < 85) return 'High';
    return 'Critical';
  };

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-4 shadow-lg">
      {/* Header */}
      <div className="flex justify-between items-center mb-3">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-lg text-primary">memory</span>
          <span className="font-bold text-white">System Vitals</span>
        </div>
        <div className="text-xs text-gray-400">
          {stats.timestamp.toLocaleTimeString()}
        </div>
      </div>

      {error ? (
        <div className="text-red-400 text-sm">{error}</div>
      ) : (
        <div className="space-y-4">
          {/* VRAM Usage with Sparkline */}
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span className="text-gray-400">VRAM Usage</span>
              <span className={getUsageColor(stats.vram)}>
                {Math.round(stats.vram)}% - {getStatusLevel(stats.vram)}
              </span>
            </div>
            <div className="flex gap-3 items-center">
              <div className="w-full bg-gray-700 rounded-full h-2 flex-grow">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${getUsageBgColor(stats.vram)}`}
                  style={{ width: `${stats.vram}%` }}
                />
              </div>
              {history.length > 5 && (
                <div className="w-24 h-10" ref={vramChartRef}></div>
              )}
            </div>
          </div>

          {/* GPU Temperature with Sparkline */}
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span className="text-gray-400">GPU Temperature</span>
              <span className={getTemperatureColor(stats.temperature)}>
                {stats.temperature}°C
              </span>
            </div>
            <div className="flex gap-3 items-center">
              <div className="w-full bg-gray-700 rounded-full h-2 flex-grow">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    stats.temperature < 50 ? 'bg-green-500' : 
                    stats.temperature < 70 ? 'bg-yellow-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${Math.min(stats.temperature, 80) / 80 * 100}%` }}
                />
              </div>
              {history.length > 5 && (
                <div className="w-24 h-10" ref={tempChartRef}></div>
              )}
            </div>
          </div>

          {/* Agent Latency with Sparkline */}
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span className="text-gray-400">Agent Latency</span>
              <span className={getUsageColor(stats.agentLatency || 0)}>
                {stats.agentLatency || 0}ms
              </span>
            </div>
            <div className="flex gap-3 items-center">
              <div className="w-full bg-gray-700 rounded-full h-2 flex-grow">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    (stats.agentLatency || 0) < 30 ? 'bg-green-500' : 
                    (stats.agentLatency || 0) < 70 ? 'bg-yellow-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${Math.min((stats.agentLatency || 0), 100)}%` }}
                />
              </div>
              {history.length > 5 && (
                <div className="w-24 h-10" ref={latencyChartRef}></div>
              )}
            </div>
          </div>

          {/* Critical Warnings */}
          {stats.gpu > 90 && (
            <div className="text-xs text-red-300 mt-2 flex items-center gap-1">
              <span>🔥</span>
              <span>Critical GPU usage detected</span>
            </div>
          )}
          
          {stats.temperature > 75 && (
            <div className="text-xs text-red-300 mt-2 flex items-center gap-1">
              <span>🔥</span>
              <span>GPU temperature critical</span>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default SystemVitalsCard;