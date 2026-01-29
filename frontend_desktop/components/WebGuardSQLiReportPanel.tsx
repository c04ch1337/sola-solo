import React, { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

// SQLi-specific report types
export interface SqliFinding {
  id: string;
  sqli_type: 'Error-based' | 'Boolean-blind' | 'Time-blind' | 'UNION-based';
  severity: 'Critical' | 'High' | 'Medium' | 'Low';
  payload: string;
  payload_description: string;
  poc_url: string;
  evidence: string;
  remediation: string;
  detected_database?: string;
}

export interface SqliTestReport {
  id: string;
  target_url: string;
  parameter: string;
  scan_time: string;
  duration_ms: number;
  payloads_tested: number;
  errors_detected: number;
  time_delays_detected: number;
  boolean_differences: number;
  findings: SqliFinding[];
  summary: {
    vulnerable: boolean;
    total_findings: number;
    critical_count: number;
    high_count: number;
    medium_count: number;
    overall_risk: string;
    detected_database?: string;
  };
  payload_results?: Array<{
    payload: string;
    payload_description: string;
    payload_type: string;
    error_detected: boolean;
    error_message?: string;
    response_time_ms: number;
    time_delay_detected: boolean;
    boolean_difference?: boolean;
    status_code: number;
  }>;
}

export interface WebGuardSQLiReportData {
  report: SqliTestReport | null;
  markdown: string;
  timestamp: number;
}

interface WebGuardSQLiReportPanelProps {
  isOpen: boolean;
  onClose: () => void;
  report: WebGuardSQLiReportData | null;
  onCommand: (command: string) => void;
}

const getSeverityColor = (severity: string): string => {
  switch (severity.toLowerCase()) {
    case 'critical': return 'text-red-500 bg-red-500/20 border-red-500/30';
    case 'high': return 'text-orange-500 bg-orange-500/20 border-orange-500/30';
    case 'medium': return 'text-yellow-500 bg-yellow-500/20 border-yellow-500/30';
    case 'low': return 'text-blue-500 bg-blue-500/20 border-blue-500/30';
    default: return 'text-slate-400 bg-slate-500/20 border-slate-500/30';
  }
};

const getSeverityEmoji = (severity: string): string => {
  switch (severity.toLowerCase()) {
    case 'critical': return '🔴';
    case 'high': return '🟠';
    case 'medium': return '🟡';
    case 'low': return '🔵';
    default: return '⚪';
  }
};

const getSQLiTypeColor = (type: string): string => {
  switch (type.toLowerCase()) {
    case 'error-based': return 'text-red-400 bg-red-500/10 border-red-500/20';
    case 'boolean-blind': return 'text-orange-400 bg-orange-500/10 border-orange-500/20';
    case 'time-blind': return 'text-purple-400 bg-purple-500/10 border-purple-500/20';
    case 'union-based': return 'text-pink-400 bg-pink-500/10 border-pink-500/20';
    default: return 'text-slate-400 bg-slate-500/10 border-slate-500/20';
  }
};

const SeverityBadge: React.FC<{ severity: string; count: number }> = ({ severity, count }) => {
  const colorClass = getSeverityColor(severity);
  const emoji = getSeverityEmoji(severity);
  
  return (
    <div className={`flex items-center gap-2 px-3 py-1.5 rounded-lg border ${colorClass}`}>
      <span className="text-sm">{emoji}</span>
      <span className="text-xs font-bold uppercase tracking-wider">{severity}</span>
      <span className="text-lg font-bold">{count}</span>
    </div>
  );
};

const SqliFindingCard: React.FC<{ finding: SqliFinding; index: number }> = ({ finding, index }) => {
  const [expanded, setExpanded] = useState(false);
  const severityClass = getSeverityColor(finding.severity);
  const typeClass = getSQLiTypeColor(finding.sqli_type);
  
  return (
    <div className={`bg-black/40 border rounded-lg overflow-hidden ${severityClass.split(' ').slice(1).join(' ')}`}>
      <div 
        className="flex items-center justify-between p-4 cursor-pointer hover:bg-white/5 transition-colors"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center gap-3 flex-1">
          <span className="text-lg">{getSeverityEmoji(finding.severity)}</span>
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <h4 className="font-semibold text-white truncate">{finding.id}</h4>
              <span className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider ${typeClass}`}>
                {finding.sqli_type}
              </span>
            </div>
            <p className="text-xs text-slate-400 truncate">{finding.payload_description}</p>
          </div>
        </div>
        <span className={`material-symbols-outlined transition-transform ${expanded ? 'rotate-180' : ''}`}>
          expand_more
        </span>
      </div>
      
      {expanded && (
        <div className="px-4 pb-4 space-y-3 border-t border-border-dark/50 pt-3">
          <div>
            <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Payload</span>
            <code className="text-xs bg-black/60 px-2 py-1 rounded text-red-400 font-mono block overflow-x-auto whitespace-pre-wrap break-all">
              {finding.payload}
            </code>
          </div>
          
          <div>
            <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Proof of Concept URL</span>
            <a 
              href={finding.poc_url} 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-xs bg-black/60 px-2 py-1 rounded text-cyan-400 font-mono block overflow-x-auto hover:text-cyan-300 break-all"
            >
              {finding.poc_url}
            </a>
          </div>
          
          <div>
            <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Evidence</span>
            <p className="text-sm text-slate-300 bg-black/40 px-3 py-2 rounded">{finding.evidence}</p>
          </div>
          
          {finding.detected_database && (
            <div className="flex items-center gap-2 px-3 py-2 bg-purple-500/10 border border-purple-500/20 rounded-lg">
              <span className="material-symbols-outlined text-purple-400 text-sm">database</span>
              <span className="text-sm text-purple-300">Detected Database: <strong>{finding.detected_database}</strong></span>
            </div>
          )}
          
          <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-3">
            <span className="text-[10px] uppercase tracking-wider text-green-400 flex items-center gap-1 mb-2">
              <span className="material-symbols-outlined text-sm">healing</span>
              Remediation
            </span>
            <div className="text-sm text-green-300 whitespace-pre-line">{finding.remediation}</div>
          </div>
        </div>
      )}
    </div>
  );
};

const WebGuardSQLiReportPanel: React.FC<WebGuardSQLiReportPanelProps> = ({ 
  isOpen, 
  onClose, 
  report, 
  onCommand 
}) => {
  if (!isOpen) return null;

  const handleExport = (format: 'json' | 'markdown') => {
    if (!report || !report.report) return;
    
    let content: string;
    let filename: string;
    let mimeType: string;
    
    if (format === 'json') {
      content = JSON.stringify(report.report, null, 2);
      filename = `webguard-sqli-${report.report.id}.json`;
      mimeType = 'application/json';
    } else {
      content = report.markdown;
      filename = `webguard-sqli-${report.report.id}.md`;
      mimeType = 'text/markdown';
    }
    
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-full max-w-5xl h-[85vh] bg-panel-dark border border-border-dark rounded-xl shadow-2xl flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border-dark bg-gradient-to-r from-red-900/30 to-orange-900/30">
          <div className="flex items-center gap-3">
            <span className="material-symbols-outlined text-3xl text-red-400">database</span>
            <div>
              <h2 className="text-xl font-bold text-white flex items-center gap-2">
                WebGuard SQLi Report
                <span className="text-[10px] px-2 py-0.5 bg-red-500/20 text-red-400 rounded uppercase tracking-widest">
                  SQL Injection Test
                </span>
              </h2>
              <p className="text-xs text-slate-400">SQL injection vulnerability analysis results</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => onCommand('webguard test-sqli ')}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors text-slate-400 hover:text-white"
              title="Run New SQLi Test"
            >
              <span className="material-symbols-outlined">add</span>
            </button>
            {report && report.report && (
              <>
                <button
                  onClick={() => handleExport('json')}
                  className="p-2 hover:bg-white/10 rounded-lg transition-colors text-blue-400 hover:text-blue-300"
                  title="Export as JSON"
                >
                  <span className="material-symbols-outlined">download</span>
                </button>
                <button
                  onClick={() => handleExport('markdown')}
                  className="p-2 hover:bg-white/10 rounded-lg transition-colors text-purple-400 hover:text-purple-300"
                  title="Export as Markdown"
                >
                  <span className="material-symbols-outlined">description</span>
                </button>
              </>
            )}
            <button
              onClick={onClose}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-slate-400">close</span>
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar">
          {!report || !report.report ? (
            <div className="flex flex-col items-center justify-center h-full text-slate-500 p-8">
              <span className="material-symbols-outlined text-6xl mb-4 opacity-30">database</span>
              <p className="text-lg font-medium">No SQLi test report available</p>
              <p className="text-sm mt-2 text-center max-w-md">
                Run a SQL injection test to see results here. Use{' '}
                <code className="bg-black/40 px-2 py-0.5 rounded">webguard test-sqli &lt;url&gt; &lt;param&gt;</code>
              </p>
              <button
                onClick={() => onCommand('webguard test-sqli ')}
                className="mt-4 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 border border-red-500/30 rounded-lg transition-colors text-red-300"
              >
                Run SQLi Test
              </button>
            </div>
          ) : (
            <div className="p-6 space-y-6">
              {/* Report Summary */}
              <div className="bg-black/30 border border-border-dark rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <div>
                    <h3 className="text-lg font-bold text-white flex items-center gap-2">
                      <span className="material-symbols-outlined text-red-400">database</span>
                      SQL Injection Test Results
                    </h3>
                    <p className="text-sm text-slate-400 mt-1 font-mono truncate max-w-2xl">
                      {report.report.target_url}
                    </p>
                    <p className="text-xs text-slate-500 mt-1">
                      Parameter: <code className="text-cyan-400">{report.report.parameter}</code>
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-xs text-slate-500">Scan ID</p>
                    <p className="text-xs font-mono text-slate-400">{report.report.id}</p>
                    <p className="text-xs text-slate-500 mt-2">
                      {new Date(report.timestamp).toLocaleString()}
                    </p>
                  </div>
                </div>

                {/* Severity Summary */}
                <div className="grid grid-cols-4 gap-3 mb-6">
                  <SeverityBadge severity="Critical" count={report.report.summary.critical_count} />
                  <SeverityBadge severity="High" count={report.report.summary.high_count} />
                  <SeverityBadge severity="Medium" count={report.report.summary.medium_count} />
                  <SeverityBadge severity="Low" count={0} />
                </div>

                {/* SQLi-specific stats */}
                <div className="grid grid-cols-4 gap-4 mb-4 p-4 bg-black/20 rounded-lg">
                  <div className="text-center">
                    <p className="text-2xl font-bold text-cyan-400">{report.report.payloads_tested}</p>
                    <p className="text-[10px] uppercase tracking-wider text-slate-400">Payloads Tested</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-orange-400">{report.report.errors_detected}</p>
                    <p className="text-[10px] uppercase tracking-wider text-slate-400">Errors Detected</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-purple-400">{report.report.time_delays_detected}</p>
                    <p className="text-[10px] uppercase tracking-wider text-slate-400">Time Delays</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-yellow-400">{report.report.boolean_differences}</p>
                    <p className="text-[10px] uppercase tracking-wider text-slate-400">Boolean Diffs</p>
                  </div>
                </div>

                {/* Vulnerability Status */}
                <div className={`p-4 rounded-lg border ${
                  report.report.summary.vulnerable 
                    ? 'bg-red-500/10 border-red-500/30' 
                    : 'bg-green-500/10 border-green-500/30'
                }`}>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <span className={`material-symbols-outlined text-2xl ${
                        report.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                      }`}>
                        {report.report.summary.vulnerable ? 'warning' : 'verified_user'}
                      </span>
                      <span className={`font-bold ${
                        report.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                      }`}>
                        {report.report.summary.vulnerable 
                          ? 'SQL Injection Vulnerabilities Detected!' 
                          : 'No SQL Injection Vulnerabilities Found'}
                      </span>
                    </div>
                    {report.report.summary.detected_database && (
                      <span className="px-3 py-1 bg-purple-500/20 border border-purple-500/30 rounded-lg text-purple-300 text-sm font-medium">
                        {report.report.summary.detected_database}
                      </span>
                    )}
                  </div>
                  {report.report.summary.overall_risk && (
                    <p className="text-sm text-slate-300 mt-2">
                      Overall Risk: <strong className="text-orange-400">{report.report.summary.overall_risk}</strong>
                    </p>
                  )}
                </div>

                {/* Scan Metadata */}
                <div className="flex items-center gap-6 text-xs text-slate-400 mt-4">
                  <span className="flex items-center gap-1">
                    <span className="material-symbols-outlined text-sm">schedule</span>
                    {report.report.duration_ms}ms
                  </span>
                  <span className="flex items-center gap-1">
                    <span className="material-symbols-outlined text-sm">database</span>
                    Parameter: {report.report.parameter}
                  </span>
                </div>
              </div>

              {/* Findings List */}
              {report.report.findings && report.report.findings.length > 0 && (
                <div>
                  <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                    <span className="material-symbols-outlined text-lg">report</span>
                    SQLi Findings ({report.report.findings.length})
                  </h3>
                  <div className="space-y-3">
                    {report.report.findings.map((finding, index) => (
                      <SqliFindingCard key={finding.id || index} finding={finding} index={index} />
                    ))}
                  </div>
                </div>
              )}

              {/* Full Markdown Report */}
              <div>
                <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                  <span className="material-symbols-outlined text-lg">description</span>
                  Full Report
                </h3>
                <div className="bg-black/30 rounded-lg border border-border-dark p-6 prose prose-invert max-w-none prose-sm">
                  <ReactMarkdown remarkPlugins={[remarkGfm]}>
                    {report.markdown}
                  </ReactMarkdown>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default WebGuardSQLiReportPanel;
