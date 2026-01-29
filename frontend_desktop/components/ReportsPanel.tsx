import React, { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

// Types for Vulnerability Reports
export interface VulnerabilityReport {
  id: string;
  title: string;
  generated_at: string;
  severity: 'Info' | 'Low' | 'Medium' | 'High' | 'Critical';
  risk_score: number;
  executive_summary: string;
  findings: Finding[];
  affected_assets: string[];
  proof_of_concept?: string;
  remediation?: RemediationPlan;
  mitre_mapping?: MitreMapping[];
  tags: string[];
  references: string[];
  markdown: string;
}

export interface Finding {
  id: string;
  title: string;
  severity: 'Info' | 'Low' | 'Medium' | 'High' | 'Critical';
  description: string;
  evidence: string[];
  cve_ids: string[];
  cvss_score?: number;
}

export interface RemediationPlan {
  priority: 'Info' | 'Low' | 'Medium' | 'High' | 'Critical';
  estimated_effort: string;
  steps: RemediationStep[];
  validation: string[];
}

export interface RemediationStep {
  order: number;
  action: string;
  details: string;
  tools: string[];
}

export interface MitreMapping {
  technique_id: string;
  technique_name: string;
  tactic: string;
  description: string;
}

interface ReportsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  reports: VulnerabilityReport[];
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
    case 'low': return '🟢';
    case 'info': return 'ℹ️';
    default: return '⚪';
  }
};

const ReportsPanel: React.FC<ReportsPanelProps> = ({ isOpen, onClose, reports, onCommand }) => {
  const [selectedTab, setSelectedTab] = useState<'latest' | 'history'>('latest');
  const [selectedReportIndex, setSelectedReportIndex] = useState(0);
  const [viewMode, setViewMode] = useState<'rendered' | 'markdown'>('rendered');

  if (!isOpen) return null;

  const latestReport = reports[0] || null;
  const currentReport = selectedTab === 'latest' ? latestReport : reports[selectedReportIndex];

  const exportReport = (format: 'json' | 'markdown') => {
    if (!currentReport) return;

    let content: string;
    let filename: string;
    let mimeType: string;

    if (format === 'json') {
      content = JSON.stringify(currentReport, null, 2);
      filename = `report-${currentReport.id}.json`;
      mimeType = 'application/json';
    } else {
      content = currentReport.markdown;
      filename = `report-${currentReport.id}.md`;
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
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="w-full max-w-6xl h-[90vh] bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 border border-border-dark rounded-2xl shadow-2xl flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-border-dark bg-black/30">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-3">
              <span className="material-symbols-outlined text-4xl text-cyan-400">description</span>
              <h2 className="text-2xl font-bold text-white">
                Vulnerability Reports
              </h2>
              <p className="text-xs text-slate-400">Professional security analysis and remediation</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-700/50 rounded-lg transition-colors"
            aria-label="Close"
          >
            <span className="material-symbols-outlined text-slate-400">close</span>
          </button>
        </div>

        {/* Quick Actions */}
        <div className="flex items-center gap-2 p-4 border-b border-border-dark bg-black/20">
          <button
            onClick={() => onCommand('report last scan')}
            className="flex items-center gap-2 px-4 py-2 bg-cyan-500/20 hover:bg-cyan-500/30 border border-cyan-500/30 rounded-lg transition-colors"
          >
            <span className="material-symbols-outlined text-sm text-cyan-400">refresh</span>
            <span className="text-sm font-medium text-cyan-300">Generate from Last Scan</span>
          </button>
          <button
            onClick={() => onCommand('report list')}
            className="flex items-center gap-2 px-4 py-2 bg-slate-500/20 hover:bg-slate-500/30 border border-slate-500/30 rounded-lg transition-colors"
          >
            <span className="material-symbols-outlined text-sm text-slate-400">list</span>
            <span className="text-sm font-medium text-slate-300">Refresh List</span>
          </button>
          {currentReport && (
            <>
              <button
                onClick={() => exportReport('markdown')}
                className="flex items-center gap-2 px-4 py-2 bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/30 rounded-lg transition-colors ml-auto"
              >
                <span className="material-symbols-outlined text-sm text-purple-400">download</span>
                <span className="text-sm font-medium text-purple-300">Export MD</span>
              </button>
              <button
                onClick={() => exportReport('json')}
                className="flex items-center gap-2 px-4 py-2 bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/30 rounded-lg transition-colors"
              >
                <span className="material-symbols-outlined text-sm text-purple-400">download</span>
                <span className="text-sm font-medium text-purple-300">Export JSON</span>
              </button>
            </>
          )}
        </div>

        {/* Tabs */}
        <div className="flex gap-2 p-4 border-b border-border-dark bg-black/10">
          <button
            onClick={() => setSelectedTab('latest')}
            className={`px-4 py-2 rounded-lg font-medium transition-colors ${
              selectedTab === 'latest'
                ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                : 'text-slate-400 hover:bg-slate-700/30'
            }`}
          >
            Latest Report
          </button>
          <button
            onClick={() => setSelectedTab('history')}
            className={`px-4 py-2 rounded-lg font-medium transition-colors ${
              selectedTab === 'history'
                ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                : 'text-slate-400 hover:bg-slate-700/30'
            }`}
          >
            History ({reports.length})
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {!currentReport ? (
            <div className="flex flex-col items-center justify-center h-full text-slate-400">
              <span className="material-symbols-outlined text-6xl mb-4 opacity-30">description</span>
              <p className="text-lg font-medium">No reports yet</p>
              <p className="text-sm mt-2 text-center max-w-md">
                Generate reports from security scans using <code className="px-2 py-1 bg-slate-700/50 rounded">report last scan</code>
              </p>
            </div>
          ) : selectedTab === 'history' ? (
            <div className="p-6 space-y-4">
              {reports.map((report, index) => (
                <div
                  key={report.id}
                  onClick={() => setSelectedReportIndex(index)}
                  className={`p-4 rounded-xl border cursor-pointer transition-all ${
                    index === selectedReportIndex
                      ? 'bg-cyan-500/10 border-cyan-500/30'
                      : 'bg-black/20 border-border-dark hover:bg-black/30'
                  }`}
                >
                  <div className="flex items-start gap-4">
                    <span className="text-3xl">{getSeverityEmoji(report.severity)}</span>
                    <div className="flex-1">
                      <h3 className="font-semibold text-white truncate max-w-md">
                        {report.title}
                      </h3>
                      <p className="text-xs text-slate-400 mt-1">
                        {new Date(report.generated_at).toLocaleString()} • Risk Score: {report.risk_score.toFixed(1)}/10.0
                      </p>
                      <div className="flex items-center gap-2 mt-2">
                        <span className={`px-2 py-1 rounded text-xs font-medium ${getSeverityColor(report.severity)}`}>
                          {report.severity.toUpperCase()}
                        </span>
                        <span className="text-xs text-slate-500">{report.findings.length} findings</span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div>
              {/* Report Header */}
              <div className="bg-black/30 border border-border-dark rounded-xl p-6 mb-6">
                <div className="flex items-start gap-4">
                  <span className="text-5xl">{getSeverityEmoji(currentReport.severity)}</span>
                  <div className="flex-1">
                    <h2 className="text-2xl font-bold text-white mb-2">{currentReport.title}</h2>
                    <p className="text-sm text-slate-400 font-mono mb-4">Report ID: {currentReport.id}</p>
                    <div className="flex items-center gap-4 text-sm text-slate-400">
                      <span>Generated: {new Date(currentReport.generated_at).toLocaleString()}</span>
                      <span>•</span>
                      <span className={`px-3 py-1 rounded font-medium ${getSeverityColor(currentReport.severity)}`}>
                        {currentReport.severity.toUpperCase()}
                      </span>
                      <span>•</span>
                      <span>Risk Score: <span className="text-white font-bold">{currentReport.risk_score.toFixed(1)}/10.0</span></span>
                    </div>
                  </div>
                </div>
              </div>

              {/* View Mode Toggle */}
              <div className="flex gap-2 mb-4">
                <button
                  onClick={() => setViewMode('rendered')}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    viewMode === 'rendered'
                      ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                      : 'text-slate-400 hover:bg-slate-700/30'
                  }`}
                >
                  Rendered
                </button>
                <button
                  onClick={() => setViewMode('markdown')}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    viewMode === 'markdown'
                      ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                      : 'text-slate-400 hover:bg-slate-700/30'
                  }`}
                >
                  Markdown
                </button>
              </div>

              {/* Report Content */}
              {viewMode === 'rendered' ? (
                <div className="prose prose-invert prose-slate max-w-none">
                  <ReactMarkdown remarkPlugins={[remarkGfm]}>
                    {currentReport.markdown}
                  </ReactMarkdown>
                </div>
              ) : (
                <pre className="bg-black/50 border border-border-dark rounded-xl p-6 text-sm text-slate-300 overflow-x-auto">
                  {currentReport.markdown}
                </pre>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ReportsPanel;
