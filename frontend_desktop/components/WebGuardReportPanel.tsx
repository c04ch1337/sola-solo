import React, { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

// Types for WebGuard scan results
export interface WebGuardFinding {
  category: string;
  name: string;
  description: string;
  severity: 'Critical' | 'High' | 'Medium' | 'Low' | 'Info';
  remediation?: string;
  current_value?: string;
  expected_value?: string;
  poc?: string;
}

export interface WebGuardSummary {
  total_findings: number;
  critical_count: number;
  high_count: number;
  medium_count: number;
  low_count: number;
  info_count: number;
}

export interface PassiveScanReport {
  id: string;
  target_url: string;
  scan_time: string;
  duration_ms: number;
  status_code?: number;
  findings: WebGuardFinding[];
  summary: WebGuardSummary;
  security_headers?: Record<string, string>;
  technologies_detected?: string[];
}

export interface XssScanReport {
  id: string;
  target_url: string;
  parameter: string;
  scan_time: string;
  duration_ms: number;
  payloads_tested: number;
  payloads_reflected: number;
  payloads_executed: number;
  findings: WebGuardFinding[];
  summary: {
    vulnerable: boolean;
    total_findings: number;
    critical_count: number;
    high_count: number;
    medium_count: number;
    low_count: number;
    info_count: number;
  };
}

export interface SqliScanReport {
  id: string;
  target_url: string;
  parameter: string;
  scan_time: string;
  duration_ms: number;
  payloads_tested: number;
  errors_detected: number;
  time_delays_detected: number;
  boolean_differences: number;
  findings: WebGuardFinding[];
  summary: {
    vulnerable: boolean;
    total_findings: number;
    critical_count: number;
    high_count: number;
    medium_count: number;
    overall_risk: string;
    detected_database?: string;
  };
}

export interface RedirectScanReport {
  id: string;
  target_url: string;
  parameter: string;
  scan_time: string;
  duration_ms: number;
  payloads_tested: number;
  redirects_detected: number;
  external_redirects: number;
  javascript_redirects: number;
  findings: WebGuardFinding[];
  summary: {
    vulnerable: boolean;
    total_findings: number;
    critical_count: number;
    high_count: number;
    medium_count: number;
    overall_risk: string;
  };
}

export interface CmdInjScanReport {
  id: string;
  target_url: string;
  parameter: string;
  scan_time: string;
  duration_ms: number;
  payloads_tested: number;
  injections_detected: number;
  findings: WebGuardFinding[];
  summary: {
    vulnerable: boolean;
    total_findings: number;
    critical_count: number;
    high_count: number;
    medium_count: number;
    overall_risk: string;
  };
}

export interface WebGuardReportData {
  type: 'passive' | 'xss' | 'sqli' | 'redirect' | 'cmdinj';
  report: PassiveScanReport | XssScanReport | SqliScanReport | RedirectScanReport | CmdInjScanReport | null;
  markdown: string;
  timestamp: number;
}

interface WebGuardReportPanelProps {
  isOpen: boolean;
  onClose: () => void;
  reports: WebGuardReportData[];
  onCommand: (command: string) => void;
}

const getSeverityColor = (severity: string): string => {
  switch (severity.toLowerCase()) {
    case 'critical': return 'text-red-500 bg-red-500/20 border-red-500/30';
    case 'high': return 'text-orange-500 bg-orange-500/20 border-orange-500/30';
    case 'medium': return 'text-yellow-500 bg-yellow-500/20 border-yellow-500/30';
    case 'low': return 'text-blue-500 bg-blue-500/20 border-blue-500/30';
    case 'info': return 'text-slate-400 bg-slate-500/20 border-slate-500/30';
    default: return 'text-slate-400 bg-slate-500/20 border-slate-500/30';
  }
};

const getSeverityEmoji = (severity: string): string => {
  switch (severity.toLowerCase()) {
    case 'critical': return '🔴';
    case 'high': return '🟠';
    case 'medium': return '🟡';
    case 'low': return '🔵';
    case 'info': return '⚪';
    default: return '⚪';
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

const FindingCard: React.FC<{ finding: WebGuardFinding; index: number }> = ({ finding, index }) => {
  const [expanded, setExpanded] = useState(false);
  const severityClass = getSeverityColor(finding.severity);
  
  return (
    <div className={`bg-black/40 border rounded-lg overflow-hidden ${severityClass.split(' ').slice(1).join(' ')}`}>
      <div 
        className="flex items-center justify-between p-4 cursor-pointer hover:bg-white/5 transition-colors"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center gap-3">
          <span className="text-lg">{getSeverityEmoji(finding.severity)}</span>
          <div>
            <h4 className="font-semibold text-white">{finding.name}</h4>
            <span className="text-[10px] uppercase tracking-wider text-slate-400">{finding.category}</span>
          </div>
        </div>
        <span className={`material-symbols-outlined transition-transform ${expanded ? 'rotate-180' : ''}`}>
          expand_more
        </span>
      </div>
      
      {expanded && (
        <div className="px-4 pb-4 space-y-3 border-t border-border-dark/50 pt-3">
          <p className="text-sm text-slate-300">{finding.description}</p>
          
          {finding.current_value && (
            <div>
              <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Current Value</span>
              <code className="text-xs bg-black/60 px-2 py-1 rounded text-red-400 font-mono block overflow-x-auto">
                {finding.current_value}
              </code>
            </div>
          )}
          
          {finding.expected_value && (
            <div>
              <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Expected Value</span>
              <code className="text-xs bg-black/60 px-2 py-1 rounded text-green-400 font-mono block overflow-x-auto">
                {finding.expected_value}
              </code>
            </div>
          )}
          
          {finding.poc && (
            <div>
              <span className="text-[10px] uppercase tracking-wider text-slate-500 block mb-1">Proof of Concept</span>
              <code className="text-xs bg-black/60 px-2 py-1 rounded text-orange-400 font-mono block overflow-x-auto whitespace-pre-wrap">
                {finding.poc}
              </code>
            </div>
          )}
          
          {finding.remediation && (
            <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-3">
              <span className="text-[10px] uppercase tracking-wider text-green-400 flex items-center gap-1 mb-2">
                <span className="material-symbols-outlined text-sm">healing</span>
                Remediation
              </span>
              <p className="text-sm text-green-300">{finding.remediation}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

const WebGuardReportPanel: React.FC<WebGuardReportPanelProps> = ({ isOpen, onClose, reports, onCommand }) => {
  const [selectedTab, setSelectedTab] = useState<'latest' | 'history'>('latest');
  const [selectedReportIndex, setSelectedReportIndex] = useState(0);

  if (!isOpen) return null;

  const latestReport = reports[0] || null;
  const currentReport = selectedTab === 'latest' ? latestReport : reports[selectedReportIndex];

  const handleExport = (format: 'json' | 'markdown') => {
    if (!currentReport) return;
    
    let content: string;
    let filename: string;
    let mimeType: string;
    
    if (format === 'json') {
      content = JSON.stringify(currentReport.report, null, 2);
      filename = `webguard-report-${currentReport.report?.id || 'unknown'}.json`;
      mimeType = 'application/json';
    } else {
      content = currentReport.markdown;
      filename = `webguard-report-${currentReport.report?.id || 'unknown'}.md`;
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

  // Type guard for passive scan report
  const isPassiveReport = (report: any): report is PassiveScanReport => {
    return report && ('security_headers' in report || !('parameter' in report));
  };

  // Type guard for XSS scan report
  const isXssReport = (report: any): report is XssScanReport => {
    return report && 'parameter' in report && 'payloads_reflected' in report;
  };

  // Type guard for SQLi scan report
  const isSqliReport = (report: any): report is SqliScanReport => {
    return report && 'parameter' in report && 'errors_detected' in report;
  };

  // Type guard for redirect scan report
  const isRedirectReport = (report: any): report is RedirectScanReport => {
    return report && 'parameter' in report && 'redirects_detected' in report;
  };

  // Type guard for command injection scan report
  const isCmdInjReport = (report: any): report is CmdInjScanReport => {
    return report && 'parameter' in report && 'injections_detected' in report;
  };

  // Get unified statistics across all reports
  const getUnifiedStats = () => {
    const stats = {
      totalScans: reports.length,
      totalFindings: 0,
      critical: 0,
      high: 0,
      medium: 0,
      low: 0,
      info: 0,
      vulnerable: 0,
      scanTypes: {
        passive: 0,
        xss: 0,
        sqli: 0,
        redirect: 0,
        cmdinj: 0,
      }
    };

    reports.forEach(r => {
      stats.scanTypes[r.type] = (stats.scanTypes[r.type] || 0) + 1;
      
      if (r.report?.summary) {
        const summary = r.report.summary as any;
        stats.totalFindings += summary.total_findings || 0;
        stats.critical += summary.critical_count || 0;
        stats.high += summary.high_count || 0;
        stats.medium += summary.medium_count || 0;
        stats.low += summary.low_count || 0;
        stats.info += summary.info_count || 0;
        
        if (summary.vulnerable) {
          stats.vulnerable += 1;
        }
      }
    });

    return stats;
  };

  const unifiedStats = getUnifiedStats();

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-full max-w-5xl h-[85vh] bg-panel-dark border border-border-dark rounded-xl shadow-2xl flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border-dark bg-gradient-to-r from-cyan-900/30 to-blue-900/30">
          <div className="flex items-center gap-3">
            <span className="material-symbols-outlined text-3xl text-cyan-400">shield</span>
            <div>
              <h2 className="text-xl font-bold text-white flex items-center gap-2">
                WebGuard Unified Reports
                <span className="text-[10px] px-2 py-0.5 bg-cyan-500/20 text-cyan-400 rounded uppercase tracking-widest">
                  Security Scanner
                </span>
              </h2>
              <p className="text-xs text-slate-400">
                Unified web vulnerability analysis • {unifiedStats.totalScans} scans • {unifiedStats.totalFindings} findings
                {unifiedStats.vulnerable > 0 && (
                  <span className="ml-2 text-red-400 font-semibold">• {unifiedStats.vulnerable} vulnerable</span>
                )}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => onCommand('webguard help')}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors text-slate-400 hover:text-white"
              title="WebGuard Help"
            >
              <span className="material-symbols-outlined">help</span>
            </button>
            <button
              onClick={onClose}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-slate-400">close</span>
            </button>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="px-6 py-4 border-b border-border-dark bg-black/20">
          <div className="flex flex-wrap items-center gap-3">
            <button
              onClick={() => onCommand('webguard scan ')}
              className="flex items-center gap-2 px-4 py-2 bg-cyan-500/20 hover:bg-cyan-500/30 border border-cyan-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-cyan-400">radar</span>
              <span className="text-sm font-medium text-cyan-300">New Passive Scan</span>
            </button>
            <button
              onClick={() => onCommand('webguard test-xss ')}
              className="flex items-center gap-2 px-4 py-2 bg-orange-500/20 hover:bg-orange-500/30 border border-orange-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-orange-400">bug_report</span>
              <span className="text-sm font-medium text-orange-300">XSS Test</span>
            </button>
            <button
              onClick={() => onCommand('webguard test-sqli ')}
              className="flex items-center gap-2 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 border border-red-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-red-400">database</span>
              <span className="text-sm font-medium text-red-300">SQLi Test</span>
            </button>
            <button
              onClick={() => onCommand('webguard test-redirect ')}
              className="flex items-center gap-2 px-4 py-2 bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-purple-400">open_in_new</span>
              <span className="text-sm font-medium text-purple-300">Redirect Test</span>
            </button>
            <button
              onClick={() => onCommand('webguard test-cmdinj ')}
              className="flex items-center gap-2 px-4 py-2 bg-pink-500/20 hover:bg-pink-500/30 border border-pink-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-pink-400">terminal</span>
              <span className="text-sm font-medium text-pink-300">CmdInj Test</span>
            </button>
            <button
              onClick={() => onCommand('webguard report last')}
              className="flex items-center gap-2 px-4 py-2 bg-slate-500/20 hover:bg-slate-500/30 border border-slate-500/30 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-sm text-slate-400">history</span>
              <span className="text-sm font-medium text-slate-300">Last Report</span>
            </button>
            
            {currentReport && (
              <div className="ml-auto flex items-center gap-2">
                <button
                  onClick={() => handleExport('json')}
                  className="flex items-center gap-1 px-3 py-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/20 rounded-lg transition-colors text-blue-400 text-sm"
                >
                  <span className="material-symbols-outlined text-sm">download</span>
                  JSON
                </button>
                <button
                  onClick={() => handleExport('markdown')}
                  className="flex items-center gap-1 px-3 py-1.5 bg-purple-500/10 hover:bg-purple-500/20 border border-purple-500/20 rounded-lg transition-colors text-purple-400 text-sm"
                >
                  <span className="material-symbols-outlined text-sm">download</span>
                  Markdown
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Unified Statistics Banner */}
        {reports.length > 0 && (
          <div className="px-6 py-4 border-b border-border-dark bg-gradient-to-r from-slate-900/50 to-slate-800/50">
            <div className="grid grid-cols-6 gap-4">
              <div className="text-center">
                <p className="text-2xl font-bold text-cyan-400">{unifiedStats.totalScans}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">Total Scans</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-red-400">{unifiedStats.critical}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">Critical</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-orange-400">{unifiedStats.high}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">High</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-yellow-400">{unifiedStats.medium}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">Medium</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-blue-400">{unifiedStats.low}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">Low</p>
              </div>
              <div className="text-center">
                <p className="text-2xl font-bold text-slate-400">{unifiedStats.totalFindings}</p>
                <p className="text-[10px] uppercase tracking-wider text-slate-400">Total Findings</p>
              </div>
            </div>
            <div className="mt-3 flex items-center justify-center gap-4 text-xs">
              <span className="px-2 py-1 bg-cyan-500/20 text-cyan-400 rounded">Passive: {unifiedStats.scanTypes.passive}</span>
              <span className="px-2 py-1 bg-orange-500/20 text-orange-400 rounded">XSS: {unifiedStats.scanTypes.xss}</span>
              <span className="px-2 py-1 bg-red-500/20 text-red-400 rounded">SQLi: {unifiedStats.scanTypes.sqli}</span>
              <span className="px-2 py-1 bg-purple-500/20 text-purple-400 rounded">Redirect: {unifiedStats.scanTypes.redirect}</span>
              <span className="px-2 py-1 bg-pink-500/20 text-pink-400 rounded">CmdInj: {unifiedStats.scanTypes.cmdinj}</span>
            </div>
          </div>
        )}

        {/* Tabs */}
        <div className="flex border-b border-border-dark bg-black/10">
          <button
            onClick={() => setSelectedTab('latest')}
            className={`px-6 py-3 text-sm font-medium transition-colors ${
              selectedTab === 'latest' 
                ? 'text-cyan-400 border-b-2 border-cyan-400 bg-cyan-500/5' 
                : 'text-slate-400 hover:text-white hover:bg-white/5'
            }`}
          >
            Latest Report
          </button>
          <button
            onClick={() => setSelectedTab('history')}
            className={`px-6 py-3 text-sm font-medium transition-colors ${
              selectedTab === 'history' 
                ? 'text-cyan-400 border-b-2 border-cyan-400 bg-cyan-500/5' 
                : 'text-slate-400 hover:text-white hover:bg-white/5'
            }`}
          >
            History ({reports.length})
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar">
          {!currentReport ? (
            <div className="flex flex-col items-center justify-center h-full text-slate-500 p-8">
              <span className="material-symbols-outlined text-6xl mb-4 opacity-30">shield</span>
              <p className="text-lg font-medium">No scan reports yet</p>
              <p className="text-sm mt-2 text-center max-w-md">
                Run a security scan to see results here. Use <code className="bg-black/40 px-2 py-0.5 rounded">webguard scan &lt;url&gt;</code> or the buttons above.
              </p>
            </div>
          ) : selectedTab === 'history' ? (
            <div className="p-6 space-y-4">
              {reports.map((report, index) => (
                <div
                  key={index}
                  className={`bg-black/40 border rounded-lg p-4 hover:border-cyan-500/30 transition-colors cursor-pointer ${
                    selectedReportIndex === index ? 'border-cyan-500/50 ring-1 ring-cyan-500/30' : 'border-border-dark'
                  }`}
                  onClick={() => {
                    setSelectedReportIndex(index);
                    setSelectedTab('latest');
                  }}
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-3">
                      <span className={`material-symbols-outlined text-2xl ${
                        report.type === 'xss' ? 'text-orange-400' : 
                        report.type === 'sqli' ? 'text-red-400' : 
                        report.type === 'redirect' ? 'text-purple-400' :
                        report.type === 'cmdinj' ? 'text-pink-400' :
                        'text-cyan-400'
                      }`}>
                        {report.type === 'xss' ? 'bug_report' : 
                         report.type === 'sqli' ? 'database' : 
                         report.type === 'redirect' ? 'open_in_new' :
                         report.type === 'cmdinj' ? 'terminal' :
                         'radar'}
                      </span>
                      <div>
                        <h3 className="font-semibold text-white truncate max-w-md">
                          {report.report?.target_url || 'Unknown Target'}
                        </h3>
                        <p className="text-xs text-slate-400">
                          {new Date(report.timestamp).toLocaleString()} • {
                            report.type === 'xss' ? 'XSS Test' : 
                            report.type === 'sqli' ? 'SQLi Test' : 
                            report.type === 'redirect' ? 'Redirect Test' :
                            report.type === 'cmdinj' ? 'CmdInj Test' :
                            'Passive Scan'
                          }
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      {report.report?.summary && (
                        <>
                          {(report.report.summary as WebGuardSummary).critical_count > 0 && (
                            <span className="text-red-500">🔴 {(report.report.summary as WebGuardSummary).critical_count}</span>
                          )}
                          {(report.report.summary as WebGuardSummary).high_count > 0 && (
                            <span className="text-orange-500">🟠 {(report.report.summary as WebGuardSummary).high_count}</span>
                          )}
                        </>
                      )}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="p-6 space-y-6">
              {/* Report Summary */}
              {currentReport.report && (
                <div className="bg-black/30 border border-border-dark rounded-xl p-6">
                  <div className="flex items-center justify-between mb-4">
                    <div>
                      <h3 className="text-lg font-bold text-white flex items-center gap-2">
                        <span className={`material-symbols-outlined ${
                          currentReport.type === 'xss' ? 'text-orange-400' : 
                          currentReport.type === 'sqli' ? 'text-red-400' : 
                          currentReport.type === 'redirect' ? 'text-purple-400' :
                          currentReport.type === 'cmdinj' ? 'text-pink-400' :
                          'text-cyan-400'
                        }`}>
                          {currentReport.type === 'xss' ? 'bug_report' : 
                           currentReport.type === 'sqli' ? 'database' : 
                           currentReport.type === 'redirect' ? 'open_in_new' :
                           currentReport.type === 'cmdinj' ? 'terminal' :
                           'shield'}
                        </span>
                        {currentReport.type === 'xss' ? 'XSS Vulnerability Test' : 
                         currentReport.type === 'sqli' ? 'SQL Injection Test' : 
                         currentReport.type === 'redirect' ? 'Open Redirect Test' :
                         currentReport.type === 'cmdinj' ? 'Command Injection Test' :
                         'Passive Security Scan'}
                      </h3>
                      <p className="text-sm text-slate-400 mt-1 font-mono truncate max-w-2xl">
                        {currentReport.report.target_url}
                      </p>
                    </div>
                    <div className="text-right">
                      <p className="text-xs text-slate-500">Scan ID</p>
                      <p className="text-xs font-mono text-slate-400">{currentReport.report.id}</p>
                    </div>
                  </div>

                  {/* Severity Summary */}
                  <div className="grid grid-cols-5 gap-3 mb-6">
                    <SeverityBadge severity="Critical" count={currentReport.report.summary.critical_count} />
                    <SeverityBadge severity="High" count={currentReport.report.summary.high_count} />
                    <SeverityBadge severity="Medium" count={(currentReport.report.summary as WebGuardSummary).medium_count || 0} />
                    <SeverityBadge severity="Low" count={(currentReport.report.summary as WebGuardSummary).low_count || 0} />
                    <SeverityBadge severity="Info" count={(currentReport.report.summary as WebGuardSummary).info_count || 0} />
                  </div>

                  {/* XSS-specific stats */}
                  {isXssReport(currentReport.report) && (
                    <div className="grid grid-cols-3 gap-4 mb-4 p-4 bg-black/20 rounded-lg">
                      <div className="text-center">
                        <p className="text-2xl font-bold text-cyan-400">{currentReport.report.payloads_tested}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Payloads Tested</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-yellow-400">{currentReport.report.payloads_reflected}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Reflected</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-red-400">{currentReport.report.payloads_executed}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Executed</p>
                      </div>
                    </div>
                  )}

                  {/* Vulnerability Status - XSS */}
                  {isXssReport(currentReport.report) && (
                    <div className={`p-4 rounded-lg border ${
                      currentReport.report.summary.vulnerable 
                        ? 'bg-red-500/10 border-red-500/30' 
                        : 'bg-green-500/10 border-green-500/30'
                    }`}>
                      <div className="flex items-center gap-2">
                        <span className={`material-symbols-outlined text-2xl ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable ? 'warning' : 'verified_user'}
                        </span>
                        <span className={`font-bold ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable 
                            ? 'XSS Vulnerabilities Detected!' 
                            : 'No XSS Vulnerabilities Found'}
                        </span>
                      </div>
                    </div>
                  )}

                  {/* SQLi-specific stats */}
                  {isSqliReport(currentReport.report) && (
                    <div className="grid grid-cols-4 gap-4 mb-4 p-4 bg-black/20 rounded-lg">
                      <div className="text-center">
                        <p className="text-2xl font-bold text-cyan-400">{currentReport.report.payloads_tested}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Payloads Tested</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-orange-400">{currentReport.report.errors_detected}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Errors Detected</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-purple-400">{currentReport.report.time_delays_detected}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Time Delays</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-yellow-400">{currentReport.report.boolean_differences}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Boolean Diffs</p>
                      </div>
                    </div>
                  )}

                  {/* Vulnerability Status - SQLi */}
                  {isSqliReport(currentReport.report) && (
                    <div className={`p-4 rounded-lg border ${
                      currentReport.report.summary.vulnerable 
                        ? 'bg-red-500/10 border-red-500/30' 
                        : 'bg-green-500/10 border-green-500/30'
                    }`}>
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <span className={`material-symbols-outlined text-2xl ${
                            currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                          }`}>
                            {currentReport.report.summary.vulnerable ? 'warning' : 'verified_user'}
                          </span>
                          <span className={`font-bold ${
                            currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                          }`}>
                            {currentReport.report.summary.vulnerable 
                              ? 'SQL Injection Vulnerabilities Detected!' 
                              : 'No SQL Injection Vulnerabilities Found'}
                          </span>
                        </div>
                        {currentReport.report.summary.detected_database && (
                          <span className="px-3 py-1 bg-purple-500/20 border border-purple-500/30 rounded-lg text-purple-300 text-sm font-medium">
                            {currentReport.report.summary.detected_database}
                          </span>
                        )}
                      </div>
                    </div>
                  )}

                  {/* Redirect-specific stats */}
                  {isRedirectReport(currentReport.report) && (
                    <div className="grid grid-cols-4 gap-4 mb-4 p-4 bg-black/20 rounded-lg">
                      <div className="text-center">
                        <p className="text-2xl font-bold text-cyan-400">{currentReport.report.payloads_tested}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Payloads Tested</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-orange-400">{currentReport.report.redirects_detected}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Redirects Detected</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-purple-400">{currentReport.report.external_redirects}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">External</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-yellow-400">{currentReport.report.javascript_redirects}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">JavaScript</p>
                      </div>
                    </div>
                  )}

                  {/* Vulnerability Status - Redirect */}
                  {isRedirectReport(currentReport.report) && (
                    <div className={`p-4 rounded-lg border ${
                      currentReport.report.summary.vulnerable 
                        ? 'bg-red-500/10 border-red-500/30' 
                        : 'bg-green-500/10 border-green-500/30'
                    }`}>
                      <div className="flex items-center gap-2">
                        <span className={`material-symbols-outlined text-2xl ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable ? 'warning' : 'verified_user'}
                        </span>
                        <span className={`font-bold ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable 
                            ? 'Open Redirect Vulnerabilities Detected!' 
                            : 'No Open Redirect Vulnerabilities Found'}
                        </span>
                      </div>
                    </div>
                  )}

                  {/* CmdInj-specific stats */}
                  {isCmdInjReport(currentReport.report) && (
                    <div className="grid grid-cols-3 gap-4 mb-4 p-4 bg-black/20 rounded-lg">
                      <div className="text-center">
                        <p className="text-2xl font-bold text-cyan-400">{currentReport.report.payloads_tested}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Payloads Tested</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-red-400">{currentReport.report.injections_detected}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Injections Detected</p>
                      </div>
                      <div className="text-center">
                        <p className="text-2xl font-bold text-orange-400">{currentReport.report.summary.total_findings}</p>
                        <p className="text-[10px] uppercase tracking-wider text-slate-400">Total Findings</p>
                      </div>
                    </div>
                  )}

                  {/* Vulnerability Status - CmdInj */}
                  {isCmdInjReport(currentReport.report) && (
                    <div className={`p-4 rounded-lg border ${
                      currentReport.report.summary.vulnerable 
                        ? 'bg-red-500/10 border-red-500/30' 
                        : 'bg-green-500/10 border-green-500/30'
                    }`}>
                      <div className="flex items-center gap-2">
                        <span className={`material-symbols-outlined text-2xl ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable ? 'warning' : 'verified_user'}
                        </span>
                        <span className={`font-bold ${
                          currentReport.report.summary.vulnerable ? 'text-red-400' : 'text-green-400'
                        }`}>
                          {currentReport.report.summary.vulnerable 
                            ? 'Command Injection Vulnerabilities Detected!' 
                            : 'No Command Injection Vulnerabilities Found'}
                        </span>
                      </div>
                    </div>
                  )}

                  {/* Scan Metadata */}
                  <div className="flex items-center gap-6 text-xs text-slate-400 mt-4">
                    <span className="flex items-center gap-1">
                      <span className="material-symbols-outlined text-sm">schedule</span>
                      {currentReport.report.duration_ms}ms
                    </span>
                    {isPassiveReport(currentReport.report) && currentReport.report.status_code && (
                      <span className="flex items-center gap-1">
                        <span className="material-symbols-outlined text-sm">http</span>
                        Status {currentReport.report.status_code}
                      </span>
                    )}
                    {isXssReport(currentReport.report) && (
                      <span className="flex items-center gap-1">
                        <span className="material-symbols-outlined text-sm">code</span>
                        Param: {currentReport.report.parameter}
                      </span>
                    )}
                    {isSqliReport(currentReport.report) && (
                      <span className="flex items-center gap-1">
                        <span className="material-symbols-outlined text-sm">database</span>
                        Param: {currentReport.report.parameter}
                      </span>
                    )}
                    {isRedirectReport(currentReport.report) && (
                      <span className="flex items-center gap-1">
                        <span className="material-symbols-outlined text-sm">open_in_new</span>
                        Param: {currentReport.report.parameter}
                      </span>
                    )}
                    {isCmdInjReport(currentReport.report) && (
                      <span className="flex items-center gap-1">
                        <span className="material-symbols-outlined text-sm">terminal</span>
                        Param: {currentReport.report.parameter}
                      </span>
                    )}
                  </div>
                </div>
              )}

              {/* Findings List */}
              {currentReport.report?.findings && currentReport.report.findings.length > 0 && (
                <div>
                  <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                    <span className="material-symbols-outlined text-lg">report</span>
                    Findings ({currentReport.report.findings.length})
                  </h3>
                  <div className="space-y-3">
                    {currentReport.report.findings.map((finding, index) => (
                      <FindingCard key={index} finding={finding} index={index} />
                    ))}
                  </div>
                </div>
              )}

              {/* Security Headers Table (Passive Scan Only) */}
              {isPassiveReport(currentReport.report) && currentReport.report?.security_headers && 
               Object.keys(currentReport.report.security_headers).length > 0 && (
                <div>
                  <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                    <span className="material-symbols-outlined text-lg">lock</span>
                    Security Headers
                  </h3>
                  <div className="bg-black/30 rounded-lg border border-border-dark overflow-hidden">
                    <table className="w-full text-sm">
                      <thead className="bg-black/40">
                        <tr>
                          <th className="text-left px-4 py-3 text-[10px] uppercase tracking-widest text-slate-400">Header</th>
                          <th className="text-left px-4 py-3 text-[10px] uppercase tracking-widest text-slate-400">Value</th>
                        </tr>
                      </thead>
                      <tbody>
                        {Object.entries(currentReport.report.security_headers).map(([header, value], index) => (
                          <tr key={header} className={index % 2 === 0 ? 'bg-black/10' : ''}>
                            <td className="px-4 py-2 text-cyan-400 font-mono">{header}</td>
                            <td className="px-4 py-2 text-slate-300 font-mono text-xs break-all">{value}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                </div>
              )}

              {/* Detected Technologies (Passive Scan Only) */}
              {isPassiveReport(currentReport.report) && currentReport.report?.technologies_detected && 
               currentReport.report.technologies_detected.length > 0 && (
                <div>
                  <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                    <span className="material-symbols-outlined text-lg">dns</span>
                    Detected Technologies
                  </h3>
                  <div className="flex flex-wrap gap-2">
                    {currentReport.report.technologies_detected.map((tech, index) => (
                      <span 
                        key={index}
                        className="px-3 py-1.5 bg-slate-500/20 border border-slate-500/30 rounded-lg text-sm text-slate-300"
                      >
                        {tech}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {/* Raw Markdown Report */}
              <div>
                <h3 className="text-sm font-bold uppercase tracking-widest text-slate-400 mb-4 flex items-center gap-2">
                  <span className="material-symbols-outlined text-lg">description</span>
                  Full Report
                </h3>
                <div className="bg-black/30 rounded-lg border border-border-dark p-6 prose prose-invert max-w-none prose-sm">
                  <ReactMarkdown remarkPlugins={[remarkGfm]}>
                    {currentReport.markdown}
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

export default WebGuardReportPanel;
