
import React, { useState, useMemo, useRef, useEffect } from 'react';
import { ScheduledTask, Project, AgentType, TaskPriority, TaskStatus, RecurrencePattern } from '../types';

interface SchedulerViewProps {
  tasks: ScheduledTask[];
  projects: Project[];
  onAddTask: (task: Omit<ScheduledTask, 'id' | 'status'>) => void;
  onUpdateTask: (task: ScheduledTask) => void;
  onDeleteTask: (id: string) => void;
}

const AVAILABLE_TOOLS = [
  'Zscaler_API', 
  'Rapid7_Insight', 
  'Proofpoint_TAP', 
  'Okta_SystemLog', 
  'CrowdStrike_Falcon', 
  'AWS_CloudWatch', 
  'Sovereign_Scanner', 
  'Phoenix_LogParser', 
  'Zscaler_Tunnels', 
  'Rapid7_Nexpose',
  'Rust_Compiler_V2',
  'Vector_KB_Sync',
  'Digital_Twin_Mirror'
];

const TOOL_DETAILS: Record<string, { icon: string; desc: string; category: 'AI' | 'Data Processing' | 'Security' | 'System' | 'Network'; color: string; clearance: 'L1' | 'L2' | 'L3' }> = {
  'Zscaler_API': { 
    icon: 'cloud_sync', 
    desc: 'Secure network egress gateway for filtering AI model traffic and protecting external API calls.', 
    category: 'Network',
    color: 'text-cyan-400',
    clearance: 'L2'
  },
  'Rapid7_Insight': { 
    icon: 'bug_report', 
    desc: 'Security vulnerability analysis module for host-level agent environments and containers.', 
    category: 'Security',
    color: 'text-rose-400',
    clearance: 'L3'
  },
  'Proofpoint_TAP': { 
    icon: 'mail', 
    desc: 'Advanced threat protection for agent-to-human communications and secure payload delivery.', 
    category: 'Security',
    color: 'text-rose-400',
    clearance: 'L2'
  },
  'Okta_SystemLog': { 
    icon: 'fingerprint', 
    desc: 'Identity and access management logs for multi-agent authentication and secure session sequencing.', 
    category: 'Security',
    color: 'text-amber-400',
    clearance: 'L3'
  },
  'CrowdStrike_Falcon': { 
    icon: 'shield_moon', 
    desc: 'Endpoint detection and response (EDR) module for monitoring rogue or compromised agent processes.', 
    category: 'Security',
    color: 'text-rose-400',
    clearance: 'L3'
  },
  'AWS_CloudWatch': { 
    icon: 'cloud', 
    desc: 'System resource monitoring and real-time telemetry aggregation for cloud-based agent clusters.', 
    category: 'System',
    color: 'text-blue-400',
    clearance: 'L1'
  },
  'Sovereign_Scanner': { 
    icon: 'hub', 
    desc: 'Autonomous network topology mapping for internal bare-metal agent clusters and meshes.', 
    category: 'Network',
    color: 'text-cyan-400',
    clearance: 'L2'
  },
  'Phoenix_LogParser': { 
    icon: 'bolt', 
    desc: 'High-performance data pipeline manager for orchestrator telemetry and execution log analysis.', 
    category: 'Data Processing',
    color: 'text-primary',
    clearance: 'L1'
  },
  'Zscaler_Tunnels': { 
    icon: 'settings_input_component', 
    desc: 'Secure private tunnel connectivity for distributed agent mesh networks and remote execution.', 
    category: 'Network',
    color: 'text-cyan-400',
    clearance: 'L2'
  },
  'Rapid7_Nexpose': { 
    icon: 'troubleshoot', 
    desc: 'Vulnerability management engine for environment hardening and compliance verification.', 
    category: 'Security',
    color: 'text-rose-400',
    clearance: 'L3'
  },
  'Rust_Compiler_V2': { 
    icon: 'terminal', 
    desc: 'Bare-metal system binary compilation engine for custom agent capabilities and Rust plugins.', 
    category: 'System',
    color: 'text-slate-400',
    clearance: 'L3'
  },
  'Vector_KB_Sync': { 
    icon: 'database', 
    desc: 'AI knowledge base synchronization module for RAG and semantic context retrieval.', 
    category: 'AI',
    color: 'text-emerald-400',
    clearance: 'L2'
  },
  'Digital_Twin_Mirror': { 
    icon: 'clone', 
    desc: 'High-fidelity AI system state replication for predictive modeling and failover testing.', 
    category: 'AI',
    color: 'text-emerald-400',
    clearance: 'L3'
  }
};

const getToolIcon = (name: string) => TOOL_DETAILS[name]?.icon || 'build';
const getToolDesc = (name: string) => TOOL_DETAILS[name]?.desc || 'Standard system utility for mission execution.';
const getToolCategory = (name: string) => TOOL_DETAILS[name]?.category || 'General';
const getToolColor = (name: string) => TOOL_DETAILS[name]?.color || 'text-slate-400';
const getToolClearance = (name: string) => TOOL_DETAILS[name]?.clearance || 'L1';

const getToolStatus = (name: string, taskStatus: TaskStatus) => {
  if (taskStatus === 'RUNNING') return { label: 'ACTIVE', color: 'text-primary' };
  if (taskStatus === 'COMPLETED') return { label: 'OPTIMAL', color: 'text-emerald-400' };
  if (taskStatus === 'FAILED') return { label: 'ERROR', color: 'text-rose-500' };
  return { label: 'READY', color: 'text-blue-400' };
};

type SortKey = 'TIME' | 'PRIORITY' | 'TITLE';
type SortOrder = 'ASC' | 'DESC';

const SchedulerView: React.FC<SchedulerViewProps> = ({ tasks, projects, onAddTask, onUpdateTask, onDeleteTask }) => {
  const [isAdding, setIsAdding] = useState(false);
  const [expandedTaskId, setExpandedTaskId] = useState<string | null>(null);
  const [editingStatusTaskId, setEditingStatusTaskId] = useState<string | null>(null);
  const [isToolFilterOpen, setIsToolFilterOpen] = useState(false);
  const [isStatusFilterOpen, setIsStatusFilterOpen] = useState(false);
  const toolFilterRef = useRef<HTMLDivElement>(null);
  const statusFilterRef = useRef<HTMLDivElement>(null);
  const statusEditorRef = useRef<HTMLDivElement>(null);
  
  const [filterStatus, setFilterStatus] = useState<TaskStatus | 'ALL'>('ALL');
  const [filterProject, setFilterProject] = useState<string | 'ALL'>('ALL');
  const [filterAgent, setFilterAgent] = useState<AgentType | 'ALL'>('ALL');
  const [filterRecurrence, setFilterRecurrence] = useState<RecurrencePattern | 'ALL'>('ALL');
  const [filterTools, setFilterTools] = useState<string[]>([]);
  const [filterToolSearch, setFilterToolSearch] = useState('');
  const [sortBy, setSortBy] = useState<SortKey>('TIME');
  const [sortOrder, setSortOrder] = useState<SortOrder>('ASC');

  const [newTask, setNewTask] = useState<Omit<ScheduledTask, 'id' | 'status'>>({
    title: '',
    description: '',
    projectId: projects[0]?.id || '',
    targetAgent: 'Orchestrator',
    priority: 'MEDIUM',
    scheduledTime: new Date().toISOString().slice(0, 16),
    tools: [],
    recurring: 'NONE'
  });
  const [toolSearch, setToolSearch] = useState('');

  const filteredAndSortedTasks = useMemo(() => {
    return [...tasks]
      .filter(task => {
        if (filterStatus !== 'ALL' && task.status !== filterStatus) return false;
        if (filterProject !== 'ALL' && task.projectId !== filterProject) return false;
        if (filterAgent !== 'ALL' && task.targetAgent !== filterAgent) return false;
        if (filterRecurrence !== 'ALL' && task.recurring !== filterRecurrence) return false;
        if (filterTools.length > 0) {
          if (!filterTools.every(tool => task.tools.includes(tool))) return false;
        }
        return true;
      })
      .sort((a, b) => {
        let comparison = 0;
        if (sortBy === 'TIME') {
          comparison = new Date(a.scheduledTime).getTime() - new Date(b.scheduledTime).getTime();
        } else if (sortBy === 'PRIORITY') {
          const priorityWeight = { 'CRITICAL': 4, 'HIGH': 3, 'MEDIUM': 2, 'LOW': 1 };
          comparison = priorityWeight[a.priority] - priorityWeight[b.priority];
        } else if (sortBy === 'TITLE') {
          comparison = a.title.localeCompare(b.title);
        }
        return sortOrder === 'ASC' ? comparison : -comparison;
      });
  }, [tasks, filterStatus, filterProject, filterAgent, filterRecurrence, filterTools, sortBy, sortOrder]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onAddTask(newTask);
    setIsAdding(false);
    setNewTask({
      title: '',
      description: '',
      projectId: projects[0]?.id || '',
      targetAgent: 'Orchestrator',
      priority: 'MEDIUM',
      scheduledTime: new Date().toISOString().slice(0, 16),
      tools: [],
      recurring: 'NONE'
    });
  };

  const toggleToolInNew = (toolName: string) => {
    setNewTask(prev => ({
      ...prev,
      tools: prev.tools.includes(toolName) 
        ? prev.tools.filter(t => t !== toolName)
        : [...prev.tools, toolName]
    }));
  };

  const updateTaskTools = (task: ScheduledTask, toolName: string, action: 'add' | 'remove') => {
    let updatedTools = [...task.tools];
    if (action === 'add' && !updatedTools.includes(toolName)) {
      updatedTools.push(toolName);
    } else if (action === 'remove') {
      updatedTools = updatedTools.filter(t => t !== toolName);
    }
    onUpdateTask({ ...task, tools: updatedTools });
  };

  const getPriorityColor = (p: TaskPriority) => {
    switch (p) {
      case 'CRITICAL': return 'text-red-500 bg-red-500/10 border-red-500/20 shadow-red-500/5';
      case 'HIGH': return 'text-amber-500 bg-amber-500/10 border-amber-500/20 shadow-amber-500/5';
      case 'MEDIUM': return 'text-blue-500 bg-blue-500/10 border-blue-500/20 shadow-blue-500/5';
      case 'LOW': return 'text-slate-500 bg-slate-500/10 border-slate-500/20';
    }
  };

  const getStatusStyles = (status: TaskStatus) => {
    switch (status) {
      case 'PENDING':
        return {
          container: 'bg-amber-500/10 text-amber-500 border-amber-500/30 shadow-inner hover:bg-amber-500/20',
          dot: 'bg-amber-500',
          icon: 'pending',
          iconClass: 'text-amber-500'
        };
      case 'RUNNING':
        return {
          container: 'bg-blue-500/10 text-blue-400 border-blue-500/40 shadow-[0_0_15px_rgba(59,130,246,0.15)] ring-1 ring-blue-500/20 hover:bg-blue-500/20',
          dot: 'bg-blue-500 animate-pulse',
          icon: 'motion_photos_on',
          iconClass: 'animate-spin-slow text-blue-500'
        };
      case 'COMPLETED':
        return {
          container: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/40 shadow-emerald-500/5 hover:bg-emerald-500/20',
          dot: 'bg-emerald-500',
          icon: 'verified',
          iconClass: 'text-emerald-400'
        };
      case 'FAILED':
        return {
          container: 'bg-rose-500/10 text-rose-400 border-rose-500/40 shadow-rose-500/5 hover:bg-rose-500/20',
          dot: 'bg-rose-500',
          icon: 'dangerous',
          iconClass: 'text-rose-500'
        };
    }
  };

  const getRecurrenceLabel = (recurring: string) => {
    switch (recurring) {
      case 'NONE': return 'Single Run';
      case 'HOURLY': return 'Hourly';
      case 'THREE_HOURLY': return 'Every 3 Hours';
      case 'EVERY_SIX_HOURS': return 'Every 6 Hours';
      case 'TWELVE_HOURLY': return 'Every 12 Hours';
      case 'DAILY': return 'Daily';
      case 'WEEKLY': return 'Weekly';
      case 'WEEKLY_MON': return 'Weekly (Mon)';
      case 'WEEKLY_WED': return 'Weekly (Wed)';
      case 'WEEKLY_FRI': return 'Weekly (Fri)';
      case 'BI_WEEKLY': return 'Bi-weekly';
      case 'WEEKDAYS': return 'Weekdays (Mon-Fri)';
      case 'WEEKENDS': return 'Weekends (Sat-Sun)';
      case 'MONTHLY': return 'Monthly';
      case 'BI_MONTHLY': return 'Bi-monthly';
      case 'LAST_DAY_OF_MONTH': return 'Last Day of Month';
      case 'QUARTERLY': return 'Quarterly';
      case 'SEMI_ANNUALLY': return 'Semi-annually';
      case 'ANNUALLY': return 'Annually';
      default: return recurring;
    }
  };

  const getRecurrenceColor = (recurring: string) => {
    if (recurring === 'NONE') return 'text-slate-400';
    if (recurring.includes('HOURLY') || recurring.includes('SIX')) return 'text-primary';
    if (recurring === 'DAILY' || recurring === 'WEEKDAYS') return 'text-amber-500';
    if (recurring.includes('WEEKLY') || recurring.includes('BI_WEEKLY')) return 'text-blue-400';
    return 'text-emerald-400';
  };

  const handleToolFilterToggle = (tool: string) => {
    setFilterTools(prev => 
      prev.includes(tool) ? prev.filter(t => t !== tool) : [...prev, tool]
    );
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (toolFilterRef.current && !toolFilterRef.current.contains(event.target as Node)) {
        setIsToolFilterOpen(false);
      }
      if (statusFilterRef.current && !statusFilterRef.current.contains(event.target as Node)) {
        setIsStatusFilterOpen(false);
      }
      if (statusEditorRef.current && !statusEditorRef.current.contains(event.target as Node)) {
        setEditingStatusTaskId(null);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const RECURRENCE_OPTIONS: { label: string, value: RecurrencePattern, group: string }[] = [
    { label: 'Once (Single)', value: 'NONE', group: 'Simple Intervals' },
    { label: 'Every Hour', value: 'HOURLY', group: 'Simple Intervals' },
    { label: 'Every 3 Hours', value: 'THREE_HOURLY', group: 'Simple Intervals' },
    { label: 'Every 6 Hours', value: 'EVERY_SIX_HOURS', group: 'Simple Intervals' },
    { label: 'Every 12 Hours', value: 'TWELVE_HOURLY', group: 'Simple Intervals' },
    { label: 'Every Day', value: 'DAILY', group: 'Simple Intervals' },
    { label: 'Weekly', value: 'WEEKLY', group: 'Weekly Patterns' },
    { label: 'Bi-weekly', value: 'BI_WEEKLY', group: 'Weekly Patterns' },
    { label: 'Weekdays (Mon-Fri)', value: 'WEEKDAYS', group: 'Weekly Patterns' },
    { label: 'Weekends (Sat-Sun)', value: 'WEEKENDS', group: 'Weekly Patterns' },
    { label: 'Every Monday', value: 'WEEKLY_MON', group: 'Weekly Patterns' },
    { label: 'Every Wednesday', value: 'WEEKLY_WED', group: 'Weekly Patterns' },
    { label: 'Every Friday', value: 'WEEKLY_FRI', group: 'Weekly Patterns' },
    { label: 'Monthly', value: 'MONTHLY', group: 'Advanced Cycles' },
    { label: 'Bi-monthly', value: 'BI_MONTHLY', group: 'Advanced Cycles' },
    { label: 'Last Day of Month', value: 'LAST_DAY_OF_MONTH', group: 'Advanced Cycles' },
    { label: 'Quarterly', value: 'QUARTERLY', group: 'Advanced Cycles' },
    { label: 'Semi-annually', value: 'SEMI_ANNUALLY', group: 'Advanced Cycles' },
    { label: 'Annually', value: 'ANNUALLY', group: 'Advanced Cycles' },
  ];

  const groups = Array.from(new Set(RECURRENCE_OPTIONS.map(opt => opt.group)));

  const filteredToolsInMenu = useMemo(() => {
    return AVAILABLE_TOOLS.filter(t => 
      t.toLowerCase().includes(filterToolSearch.toLowerCase())
    );
  }, [filterToolSearch]);

  const clearAllFilters = () => {
    setFilterStatus('ALL');
    setFilterProject('ALL');
    setFilterAgent('ALL');
    setFilterTools([]);
    setFilterRecurrence('ALL');
    setFilterToolSearch('');
  };

  return (
    <div className="flex-1 overflow-y-auto p-8 space-y-8 animate-in fade-in duration-500">
      <style>{`
        @keyframes spin-slow {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
        .animate-spin-slow {
          animation: spin-slow 2s linear infinite;
        }
        .custom-scrollbar::-webkit-scrollbar {
          width: 4px;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: #334155;
          border-radius: 10px;
        }
        .no-scrollbar::-webkit-scrollbar {
          display: none;
        }
        .no-scrollbar {
          -ms-overflow-style: none;
          scrollbar-width: none;
        }
      `}</style>
      <div className="max-w-6xl mx-auto space-y-8">
        <div className="flex items-center justify-between">
          <div className="space-y-1">
            <h2 className="text-2xl font-bold text-white tracking-tight">Advanced Task Scheduler</h2>
            <p className="text-sm text-slate-500">Automate Orchestrator cycles and delegate missions to sub-agents.</p>
          </div>
          <div className="flex items-center gap-3">
            <button 
              onClick={() => setIsAdding(true)}
              className="flex items-center gap-2 px-4 py-2.5 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-border-dark rounded-xl text-sm font-bold uppercase tracking-widest transition-all active:scale-95"
            >
              <span className="material-symbols-outlined text-[20px]">post_add</span>
              Quick Mission
            </button>
            <button 
              onClick={() => setIsAdding(true)}
              className="flex items-center gap-2 px-6 py-2.5 bg-primary hover:bg-primary/90 text-white rounded-xl text-sm font-bold uppercase tracking-widest shadow-lg shadow-primary/20 transition-all active:scale-95"
            >
              <span className="material-symbols-outlined text-[20px]">add_task</span>
              Schedule Mission
            </button>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-4 gap-4">
          {[
            { label: 'Active Tasks', value: tasks.filter(t => t.status === 'RUNNING').length, icon: 'bolt', color: 'text-primary' },
            { label: 'Pending Cycle', value: tasks.filter(t => t.status === 'PENDING').length, icon: 'schedule', color: 'text-blue-400' },
            { label: 'Agent Load', value: '42%', icon: 'neurology', color: 'text-amber-400' },
            { label: 'Uptime', value: '99.98%', icon: 'security', color: 'text-green-400' },
          ].map((stat, i) => (
            <div key={i} className="bg-panel-dark border border-border-dark p-4 rounded-2xl flex items-center gap-4 shadow-sm hover:border-slate-600 transition-colors">
              <div className={`size-10 rounded-xl bg-slate-800/50 border border-border-dark flex items-center justify-center ${stat.color}`}>
                <span className="material-symbols-outlined">{stat.icon}</span>
              </div>
              <div className="flex-1 flex flex-col">
                <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">{stat.label}</span>
                <span className="text-lg font-bold text-white font-mono">{stat.value}</span>
              </div>
            </div>
          ))}
        </div>

        {/* Filter Bar */}
        <div className="bg-panel-dark border border-border-dark rounded-2xl p-4 flex flex-wrap items-center gap-4 shadow-xl">
          <div className="relative" ref={statusFilterRef}>
            <div 
              onClick={() => setIsStatusFilterOpen(!isStatusFilterOpen)}
              className={`flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border rounded-xl cursor-pointer transition-all ${filterStatus !== 'ALL' ? 'border-primary/50 text-primary' : 'border-border-dark text-slate-500 hover:border-slate-600'}`}
            >
              <span className="material-symbols-outlined text-[18px]">
                {filterStatus === 'ALL' ? 'filter_list' : getStatusStyles(filterStatus).icon}
              </span>
              <span className="text-[11px] font-bold uppercase select-none">
                {filterStatus === 'ALL' ? 'All Statuses' : filterStatus}
              </span>
              <span className={`material-symbols-outlined text-[18px] transition-transform ${isStatusFilterOpen ? 'rotate-180' : ''}`}>expand_more</span>
            </div>

            {isStatusFilterOpen && (
              <div className="absolute top-full left-0 mt-2 w-48 bg-panel-dark border border-border-dark rounded-xl shadow-2xl z-50 overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200">
                <div 
                  onClick={() => { setFilterStatus('ALL'); setIsStatusFilterOpen(false); }}
                  className={`flex items-center gap-3 px-4 py-3 cursor-pointer transition-colors hover:bg-slate-800 ${filterStatus === 'ALL' ? 'bg-primary/5 text-primary' : 'text-slate-400'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">filter_list</span>
                  <span className="text-[11px] font-bold uppercase">All Statuses</span>
                </div>
                {(['PENDING', 'RUNNING', 'COMPLETED', 'FAILED'] as TaskStatus[]).map((s) => {
                  const sStyles = getStatusStyles(s);
                  return (
                    <div 
                      key={s}
                      onClick={() => { setFilterStatus(s); setIsStatusFilterOpen(false); }}
                      className={`flex items-center gap-3 px-4 py-3 cursor-pointer transition-colors hover:bg-slate-800 ${filterStatus === s ? 'bg-primary/5 text-primary' : 'text-slate-400'}`}
                    >
                      <span className={`material-symbols-outlined text-[18px] ${sStyles.iconClass}`}>{sStyles.icon}</span>
                      <span className="text-[11px] font-bold uppercase">{s}</span>
                    </div>
                  );
                })}
              </div>
            )}
          </div>

          <div className="flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border border-border-dark rounded-xl">
            <span className="material-symbols-outlined text-slate-500 text-[18px]">workspaces</span>
            <select 
              className="bg-transparent border-none text-[11px] font-bold text-slate-300 uppercase focus:ring-0 cursor-pointer"
              value={filterProject}
              onChange={e => setFilterProject(e.target.value)}
            >
              <option value="ALL">All Projects</option>
              {projects.map(p => <option key={p.id} value={p.id}>{p.name}</option>)}
            </select>
          </div>

          <div className="flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border border-border-dark rounded-xl">
            <span className="material-symbols-outlined text-slate-500 text-[18px]">repeat</span>
            <select 
              className="bg-transparent border-none text-[11px] font-bold text-slate-300 uppercase focus:ring-0 cursor-pointer"
              value={filterRecurrence}
              onChange={e => setFilterRecurrence(e.target.value as any)}
            >
              <option value="ALL">All Cycles</option>
              {groups.map(group => (
                <optgroup key={group} label={group}>
                  {RECURRENCE_OPTIONS.filter(opt => opt.group === group).map(opt => (
                    <option key={opt.value} value={opt.value}>{opt.label}</option>
                  ))}
                </optgroup>
              ))}
            </select>
          </div>

          <div className="flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border border-border-dark rounded-xl">
            <span className="material-symbols-outlined text-slate-500 text-[18px]">engineering</span>
            <select 
              className="bg-transparent border-none text-[11px] font-bold text-slate-300 uppercase focus:ring-0 cursor-pointer"
              value={filterAgent}
              onChange={e => setFilterAgent(e.target.value as any)}
            >
              <option value="ALL">All Agents</option>
              <option value="Orchestrator">Orchestrator</option>
              <option value="RedTeamSupervisor">Red Team</option>
              <option value="BlueTeamSupervisor">Blue Team</option>
            </select>
          </div>

          <div className="relative" ref={toolFilterRef}>
            <div 
              onClick={() => setIsToolFilterOpen(!isToolFilterOpen)}
              className={`flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border rounded-xl cursor-pointer transition-all ${filterTools.length > 0 ? 'border-primary/50 text-primary' : 'border-border-dark text-slate-500 hover:border-slate-600'}`}
            >
              <span className="material-symbols-outlined text-[18px]">handyman</span>
              <span className="text-[11px] font-bold uppercase select-none">
                {filterTools.length === 0 ? 'All Tools' : `${filterTools.length} Tool${filterTools.length > 1 ? 's' : ''} Active`}
              </span>
              <span className={`material-symbols-outlined text-[18px] transition-transform ${isToolFilterOpen ? 'rotate-180' : ''}`}>expand_more</span>
            </div>

            {isToolFilterOpen && (
              <div className="absolute top-full left-0 mt-2 w-72 bg-panel-dark border border-border-dark rounded-xl shadow-2xl z-50 overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200">
                <div className="p-3 border-b border-border-dark bg-background-dark/50 space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">Select Capabilities</span>
                    {filterTools.length > 0 && (
                      <button 
                        onClick={(e) => { e.stopPropagation(); setFilterTools([]); }}
                        className="text-[9px] font-bold text-primary hover:underline uppercase"
                      >
                        Reset
                      </button>
                    )}
                  </div>
                  <div className="relative">
                    <span className="material-symbols-outlined absolute left-2 top-1/2 -translate-y-1/2 text-slate-500 text-[16px]">search</span>
                    <input 
                      autoFocus
                      type="text"
                      className="w-full bg-slate-900 border border-border-dark rounded-lg pl-8 pr-3 py-1.5 text-[11px] text-slate-300 focus:ring-primary/50 focus:border-primary/50 outline-none transition-all"
                      placeholder="Search associated tools..."
                      value={filterToolSearch}
                      onChange={(e) => setFilterToolSearch(e.target.value)}
                    />
                  </div>
                </div>
                <div className="max-h-64 overflow-y-auto custom-scrollbar p-1">
                  {filteredToolsInMenu.length > 0 ? filteredToolsInMenu.map((tool) => {
                    const isSelected = filterTools.includes(tool);
                    return (
                      <div 
                        key={tool}
                        onClick={() => handleToolFilterToggle(tool)}
                        className={`flex items-center gap-3 px-3 py-2 cursor-pointer transition-colors hover:bg-slate-800 rounded-lg group ${isSelected ? 'bg-primary/5' : ''}`}
                      >
                        <div className={`size-4 rounded border flex items-center justify-center transition-colors ${isSelected ? 'bg-primary border-primary' : 'border-border-dark group-hover:border-slate-600'}`}>
                          {isSelected && <span className="material-symbols-outlined text-white text-[12px] font-black">check</span>}
                        </div>
                        <span className={`material-symbols-outlined text-[16px] ${isSelected ? 'text-primary' : 'text-slate-600'}`}>{getToolIcon(tool)}</span>
                        <div className="flex flex-col min-w-0">
                           <span className={`text-[11px] font-mono truncate ${isSelected ? 'text-primary font-bold' : 'text-slate-400 group-hover:text-slate-200'}`}>
                            {tool.replace(/_/g, ' ')}
                          </span>
                          <span className="text-[8px] text-slate-600 font-mono tracking-tighter uppercase leading-none">{getToolCategory(tool)}</span>
                        </div>
                      </div>
                    );
                  }) : (
                    <div className="p-4 text-center">
                      <p className="text-[10px] text-slate-600 italic uppercase">No tools match your query.</p>
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>

          <div className="h-6 w-px bg-border-dark hidden md:block" />

          <div className="flex items-center gap-2 px-3 py-1.5 bg-background-dark/50 border border-border-dark rounded-xl">
            <span className="material-symbols-outlined text-slate-500 text-[18px]">sort</span>
            <select 
              className="bg-transparent border-none text-[11px] font-bold text-slate-300 uppercase focus:ring-0 cursor-pointer"
              value={sortBy}
              onChange={e => setSortBy(e.target.value as SortKey)}
            >
              <option value="TIME">Scheduled Time</option>
              <option value="PRIORITY">Priority Level</option>
              <option value="TITLE">Mission Title</option>
            </select>
            <button 
              onClick={() => setSortOrder(prev => prev === 'ASC' ? 'DESC' : 'ASC')}
              className="p-1 hover:bg-slate-700 rounded transition-colors text-primary"
              title="Toggle Sort Order"
            >
              <span className="material-symbols-outlined text-[18px]">
                {sortOrder === 'ASC' ? 'expand_less' : 'expand_more'}
              </span>
            </button>
          </div>
          
          <div className="flex-1 text-right">
            <span className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">
              Showing {filteredAndSortedTasks.length} missions
            </span>
          </div>
        </div>

        {/* List Body */}
        <div className="bg-panel-dark border border-border-dark rounded-2xl overflow-hidden shadow-xl">
          <div className="px-6 py-4 border-b border-border-dark bg-background-dark/30 flex items-center justify-between">
            <h3 className="text-xs font-bold uppercase tracking-[0.2em] text-slate-400">Scheduled Operations</h3>
            <div className="flex gap-2">
               <span className="px-2 py-0.5 rounded bg-green-500/10 text-green-500 text-[10px] font-mono border border-green-500/20">AGENT_POOL: NOMINAL</span>
            </div>
          </div>
          <div className="divide-y divide-border-dark">
            {filteredAndSortedTasks.length === 0 ? (
              <div className="p-12 text-center space-y-3">
                <span className="material-symbols-outlined text-4xl text-slate-700">event_busy</span>
                <p className="text-slate-500 text-sm">No missions match your current filter criteria.</p>
                <button 
                  onClick={clearAllFilters}
                  className="text-xs text-primary font-bold uppercase hover:underline"
                >
                  Clear all filters & resets view
                </button>
              </div>
            ) : (
              filteredAndSortedTasks.map((task) => {
                const statusStyles = getStatusStyles(task.status);
                const isExpanded = expandedTaskId === task.id;
                const isEditingStatus = editingStatusTaskId === task.id;

                return (
                  <div key={task.id} className={`hover:bg-slate-800/30 transition-all group relative overflow-hidden border-l-4 ${
                    task.status === 'RUNNING' ? 'border-blue-500 shadow-[inset_0_0_10px_rgba(59,130,246,0.15)]' : 
                    task.status === 'COMPLETED' ? 'border-emerald-500' :
                    task.status === 'FAILED' ? 'border-rose-500' : 
                    task.status === 'PENDING' ? 'border-amber-500' : 'border-slate-600'
                  }`}>
                    <div 
                      className="p-6 flex items-start gap-6 cursor-pointer select-none"
                      onClick={() => setExpandedTaskId(isExpanded ? null : task.id)}
                    >
                      <div className={`size-12 rounded-2xl border flex items-center justify-center shrink-0 shadow-inner relative transition-transform ${getPriorityColor(task.priority)} ${isExpanded ? 'scale-105' : ''}`}>
                        <span className="material-symbols-outlined text-[24px]">
                          {task.targetAgent === 'RedTeamSupervisor' ? 'security_update_warning' : task.targetAgent === 'BlueTeamSupervisor' ? 'verified_user' : 'smart_toy'}
                        </span>
                        <div className={`absolute -bottom-1.5 -right-1.5 size-6 rounded-full border-2 border-panel-dark flex items-center justify-center bg-panel-dark transition-all ${statusStyles.iconClass}`}>
                           <span className="material-symbols-outlined text-[14px] font-black">{statusStyles.icon}</span>
                        </div>
                      </div>
                      <div className="flex-1 min-w-0 space-y-1">
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-3">
                            {isExpanded ? (
                              <input 
                                autoFocus
                                className="bg-transparent border-b border-primary/50 text-slate-200 font-bold focus:ring-0 focus:outline-none min-w-[200px]"
                                value={task.title}
                                onChange={(e) => onUpdateTask({ ...task, title: e.target.value })}
                                onClick={(e) => e.stopPropagation()}
                              />
                            ) : (
                              <h4 className="font-bold text-slate-200 group-hover:text-white transition-colors truncate">{task.title}</h4>
                            )}
                            <span className={`text-[9px] font-bold px-1.5 py-0.5 rounded border uppercase tracking-tighter shrink-0 ${getPriorityColor(task.priority)}`}>
                              {task.priority}
                            </span>
                          </div>
                          {!isExpanded && task.tools.length > 0 && (
                            <div className="flex items-center gap-1">
                              {task.tools.slice(0, 3).map((t, i) => (
                                <div key={i} className="size-6 rounded bg-slate-800/50 border border-border-dark flex items-center justify-center" title={t}>
                                  <span className={`material-symbols-outlined text-[14px] ${getToolColor(t)}`}>{getToolIcon(t)}</span>
                                </div>
                              ))}
                              {task.tools.length > 3 && <span className="text-[9px] text-slate-600 font-mono ml-1">+{task.tools.length - 3}</span>}
                            </div>
                          )}
                        </div>
                        {isExpanded ? (
                          <textarea 
                            className="w-full bg-slate-900/50 border border-border-dark rounded-lg p-2 text-xs text-slate-400 focus:ring-primary/30 mt-1"
                            value={task.description}
                            onChange={(e) => onUpdateTask({ ...task, description: e.target.value })}
                            onClick={(e) => e.stopPropagation()}
                            rows={2}
                          />
                        ) : (
                          <p className="text-xs text-slate-500 line-clamp-1">{task.description}</p>
                        )}
                        <div className="flex items-center gap-4 pt-2 overflow-x-auto no-scrollbar">
                          <div className="flex items-center gap-1.5 text-[10px] text-slate-400 font-mono shrink-0">
                            <span className="material-symbols-outlined text-sm">folder</span>
                            {projects.find(p => p.id === task.projectId)?.name}
                          </div>
                          <div className="flex items-center gap-1.5 text-[10px] text-slate-400 font-mono shrink-0">
                            <span className="material-symbols-outlined text-sm">schedule</span>
                            {new Date(task.scheduledTime).toLocaleString()}
                          </div>
                          <div className={`flex items-center gap-1.5 text-[10px] font-mono font-bold shrink-0 ${getRecurrenceColor(task.recurring)}`}>
                            <span className="material-symbols-outlined text-sm">repeat</span>
                            {getRecurrenceLabel(task.recurring)}
                          </div>
                          <div className="flex items-center gap-1.5 text-[10px] text-primary font-mono font-bold shrink-0">
                            <span className="material-symbols-outlined text-sm">engineering</span>
                            {task.targetAgent}
                          </div>
                        </div>
                      </div>
                      <div className="flex flex-col items-end gap-3 shrink-0">
                        {/* Status Editor Dropdown */}
                        <div className="relative" ref={editingStatusTaskId === task.id ? statusEditorRef : null}>
                          <div 
                            className={`flex items-center gap-2 px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-[0.1em] border transition-all ${statusStyles.container} cursor-pointer min-w-[140px] justify-center group/status-pill active:scale-95`}
                            onClick={(e) => {
                              e.stopPropagation();
                              setEditingStatusTaskId(isEditingStatus ? null : task.id);
                            }}
                          >
                            <span className={`material-symbols-outlined text-[20px] ${statusStyles.iconClass} transition-transform group-hover/status-pill:scale-125`}>
                              {statusStyles.icon}
                            </span>
                            <span className="font-mono">{task.status}</span>
                            <span className="material-symbols-outlined text-[14px] opacity-40 transition-transform group-hover/status-pill:translate-y-0.5">arrow_drop_down</span>
                          </div>

                          {isEditingStatus && (
                            <div 
                              className="absolute top-full right-0 mt-2 w-48 bg-panel-dark border border-border-dark rounded-2xl shadow-[0_15px_40px_rgba(0,0,0,0.6)] z-[70] overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200 backdrop-blur-md"
                              onClick={(e) => e.stopPropagation()}
                            >
                              {(['PENDING', 'RUNNING', 'COMPLETED', 'FAILED'] as TaskStatus[]).map((s) => {
                                const sStyles = getStatusStyles(s);
                                return (
                                  <button
                                    key={s}
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      onUpdateTask({ ...task, status: s });
                                      setEditingStatusTaskId(null);
                                    }}
                                    className={`w-full flex items-center gap-4 px-5 py-3.5 hover:bg-slate-800 transition-all text-left group/item ${task.status === s ? 'bg-primary/5' : ''}`}
                                  >
                                    <span className={`material-symbols-outlined text-[22px] transition-transform group-hover/item:scale-110 ${sStyles.iconClass}`}>
                                      {sStyles.icon}
                                    </span>
                                    <div className="flex flex-col">
                                      <span className={`text-[11px] font-black uppercase tracking-widest ${sStyles.iconClass}`}>{s}</span>
                                      {task.status === s && <span className="text-[8px] text-slate-600 font-mono tracking-tighter uppercase">Current State</span>}
                                    </div>
                                    {task.status === s && (
                                      <span className="material-symbols-outlined text-[18px] text-primary ml-auto">check</span>
                                    )}
                                  </button>
                                );
                              })}
                            </div>
                          )}
                        </div>
                        <div className="flex items-center gap-2">
                          <button 
                            onClick={(e) => { e.stopPropagation(); onDeleteTask(task.id); }}
                            className="p-2 text-slate-600 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-all opacity-0 group-hover:opacity-100 active:scale-90"
                            title="Abort Mission"
                          >
                            <span className="material-symbols-outlined text-[18px]">delete</span>
                          </button>
                          <span className={`material-symbols-outlined text-slate-500 transition-transform duration-300 ${isExpanded ? 'rotate-180 text-primary' : ''}`}>
                            expand_more
                          </span>
                        </div>
                      </div>
                    </div>
                    
                    {isExpanded && (
                      <div className="px-6 pb-6 pt-0 border-t border-border-dark/30 animate-in slide-in-from-top-2 duration-300">
                        <div className="bg-background-dark/40 rounded-xl p-6 mt-4 space-y-10">
                          {/* OPERATIONAL TOOLCHAIN INVENTORY SECTION */}
                          <div className="space-y-6">
                            <div className="flex items-center justify-between border-b border-border-dark/30 pb-3">
                              <div className="flex items-center gap-3">
                                <div className="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shadow-lg shadow-primary/10">
                                  <span className="material-symbols-outlined text-[20px]">construction</span>
                                </div>
                                <div className="flex flex-col">
                                  <h4 className="text-[11px] font-bold text-slate-200 uppercase tracking-[0.2em]">Operational Toolchain Inventory</h4>
                                  <p className="text-[9px] text-slate-500 font-mono uppercase tracking-tight">Active Module Mapping & Capabilities</p>
                                </div>
                              </div>
                              <div className="flex items-center gap-4">
                                <div className="hidden sm:flex items-center gap-2">
                                   <span className="text-[9px] font-mono text-slate-500 uppercase">Deployed Capacity:</span>
                                   <span className="text-[10px] font-bold text-primary bg-primary/10 px-2 py-0.5 rounded border border-primary/20">
                                     {task.tools.length} MODULES
                                   </span>
                                </div>
                                <div className="h-4 w-px bg-border-dark hidden sm:block"></div>
                                <span className="text-[10px] font-mono text-slate-600 uppercase">State: SYNCED</span>
                              </div>
                            </div>

                            {/* Detailed Tools Grid */}
                            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                              {task.tools.length > 0 ? task.tools.map((tool, idx) => {
                                const toolStatus = getToolStatus(tool, task.status);
                                const colorClass = getToolColor(tool);
                                const icon = getToolIcon(tool);
                                const category = getToolCategory(tool);
                                const clearance = getToolClearance(tool);
                                
                                // Random simulated metrics for aesthetic realism
                                const latency = Math.floor(Math.random() * 85) + 12;
                                const health = Math.floor(Math.random() * 20) + 80;

                                return (
                                  <div 
                                    key={idx} 
                                    className="group/toolcard relative p-5 bg-slate-900/60 border border-border-dark rounded-2xl flex flex-col gap-4 hover:bg-slate-800/80 hover:border-primary/40 transition-all shadow-xl overflow-hidden animate-in fade-in slide-in-from-top-4"
                                    style={{ animationDelay: `${idx * 0.05}s` }}
                                  >
                                    <div className="flex items-start justify-between relative z-10">
                                      <div className="flex items-center gap-3">
                                        <div className={`size-11 rounded-xl bg-background-dark border border-border-dark flex items-center justify-center ${colorClass} group-hover/toolcard:scale-110 transition-transform shadow-inner`}>
                                          <span className="material-symbols-outlined text-[22px]">{icon}</span>
                                        </div>
                                        <div className="flex flex-col min-w-0">
                                          <span className="text-[11px] font-bold text-slate-200 group-hover/toolcard:text-primary transition-colors truncate block uppercase tracking-wide">
                                            {tool.replace(/_/g, ' ')}
                                          </span>
                                          <div className="flex items-center gap-2 mt-0.5">
                                            <span className={`text-[8px] font-black px-1.5 py-0.5 rounded border ${colorClass} bg-current/5 uppercase tracking-tighter`}>
                                              {category}
                                            </span>
                                            <span className="text-[8px] font-mono text-slate-600 bg-slate-800/50 px-1.5 py-0.5 rounded uppercase tracking-widest border border-border-dark/30">
                                              SEC: {clearance}
                                            </span>
                                          </div>
                                        </div>
                                      </div>
                                      <div className={`flex flex-col items-end gap-1 shrink-0 ${toolStatus.color}`}>
                                        <span className="text-[9px] font-black tracking-[0.15em] leading-none uppercase">{toolStatus.label}</span>
                                        {task.status === 'RUNNING' && <div className="size-1 bg-current rounded-full animate-pulse shadow-[0_0_8px_rgba(255,87,51,0.5)]"></div>}
                                      </div>
                                    </div>
                                    
                                    <div className="space-y-4 relative z-10">
                                      <p className="text-[10px] text-slate-500 leading-relaxed line-clamp-2 min-h-[30px] font-medium italic opacity-80 group-hover/toolcard:opacity-100 transition-opacity">
                                        "{getToolDesc(tool)}"
                                      </p>
                                      
                                      <div className="space-y-3 pt-3 border-t border-border-dark/40">
                                        {/* Telemetry Progress Bars */}
                                        <div className="space-y-1.5">
                                          <div className="flex justify-between items-center px-0.5">
                                            <span className="text-[8px] font-bold text-slate-600 uppercase tracking-widest">Capability Health</span>
                                            <span className="text-[9px] font-mono text-emerald-400">{health}%</span>
                                          </div>
                                          <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                                            <div className="h-full bg-emerald-500 transition-all duration-1000" style={{ width: `${health}%` }}></div>
                                          </div>
                                        </div>

                                        <div className="flex items-center justify-between">
                                          <div className="grid grid-cols-2 gap-4 flex-1">
                                            <div className="flex flex-col">
                                              <span className="text-[7px] text-slate-600 font-bold uppercase tracking-tighter">Response Time</span>
                                              <span className="text-[10px] font-mono text-slate-400 group-hover/toolcard:text-slate-200 transition-colors">{latency}ms</span>
                                            </div>
                                            <div className="flex flex-col">
                                              <span className="text-[7px] text-slate-600 font-bold uppercase tracking-tighter">Sync Index</span>
                                              <span className="text-[10px] font-mono text-slate-400 group-hover/toolcard:text-slate-200 transition-colors">NOMINAL</span>
                                            </div>
                                          </div>
                                          <button 
                                            onClick={(e) => { e.stopPropagation(); updateTaskTools(task, tool, 'remove'); }}
                                            className="opacity-0 group-hover/toolcard:opacity-100 p-2 text-slate-600 hover:text-rose-500 hover:bg-rose-500/10 rounded-xl transition-all ml-2 active:scale-90"
                                            title="De-authorize Module"
                                          >
                                            <span className="material-symbols-outlined text-[18px]">link_off</span>
                                          </button>
                                        </div>
                                      </div>
                                    </div>
                                    <div className="absolute -top-12 -right-12 w-32 h-32 bg-primary/5 rounded-full blur-[40px] group-hover/toolcard:bg-primary/15 transition-colors pointer-events-none"></div>
                                  </div>
                                );
                              }) : (
                                <div className="col-span-full py-20 flex flex-col items-center justify-center gap-5 border border-dashed border-border-dark rounded-3xl bg-black/10 transition-colors hover:bg-black/20 group/empty">
                                  <div className="size-14 rounded-2xl bg-slate-800/40 flex items-center justify-center text-slate-700 group-hover/empty:text-primary transition-all duration-500 group-hover/empty:scale-110">
                                    <span className="material-symbols-outlined text-4xl">extension_off</span>
                                  </div>
                                  <div className="text-center space-y-1.5">
                                    <p className="text-xs text-slate-500 font-bold uppercase tracking-[0.3em]">No Mapped Intelligence Capabilities</p>
                                    <p className="text-[10px] text-slate-600 font-mono uppercase tracking-tight italic max-w-sm mx-auto leading-relaxed">
                                      This mission is currently executing on baseline kernel instructions without specialized module augmentation. Add tools to expand the agent context.
                                    </p>
                                  </div>
                                </div>
                              )}
                            </div>
                          </div>

                          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                            <div className="space-y-5">
                              <div className="flex items-center gap-3 border-b border-border-dark/30 pb-3">
                                <span className="material-symbols-outlined text-emerald-500 text-lg">add_circle</span>
                                <span className="text-[11px] font-bold text-slate-200 uppercase tracking-[0.2em]">Map New Intelligence Capability</span>
                              </div>
                              
                              {/* Multi-select Tool Repository in expanded view */}
                              <div className="relative group">
                                <span className="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-600 text-[18px] group-focus-within:text-primary transition-colors">search</span>
                                <input 
                                  type="text"
                                  placeholder="Filter available tools..."
                                  className="w-full bg-slate-900/50 border border-border-dark rounded-xl pl-10 pr-4 py-3 text-[11px] text-slate-300 focus:ring-1 focus:ring-primary/50 transition-all outline-none shadow-inner"
                                  value={toolSearch}
                                  onChange={(e) => setToolSearch(e.target.value)}
                                  onClick={(e) => e.stopPropagation()}
                                />
                              </div>
                              <div className="flex flex-wrap gap-2 max-h-[180px] overflow-y-auto custom-scrollbar p-1">
                                {AVAILABLE_TOOLS.filter(t => t.toLowerCase().includes(toolSearch.toLowerCase())).map((tool) => {
                                   const isSelected = task.tools.includes(tool);
                                   const color = getToolColor(tool);
                                   return (
                                    <button
                                      key={tool}
                                      onClick={(e) => { 
                                        e.stopPropagation(); 
                                        updateTaskTools(task, tool, isSelected ? 'remove' : 'add'); 
                                      }}
                                      className={`px-4 py-2.5 rounded-xl border transition-all uppercase tracking-tighter flex items-center gap-3 group/tool shadow-sm active:scale-95 ${
                                        isSelected 
                                          ? 'bg-primary/20 border-primary text-primary shadow-[0_0_15px_rgba(255,87,51,0.1)]' 
                                          : 'bg-slate-900/50 border-border-dark text-slate-500 hover:text-slate-300 hover:border-slate-600'
                                      }`}
                                    >
                                      <span className={`material-symbols-outlined text-[16px] transition-transform ${isSelected ? 'scale-110' : 'group-hover:rotate-12'} ${isSelected ? 'text-primary' : color}`}>{getToolIcon(tool)}</span>
                                      <span className="text-[10px] font-mono">{tool.replace(/_/g, ' ')}</span>
                                      {isSelected && <span className="material-symbols-outlined text-xs">check</span>}
                                    </button>
                                   );
                                })}
                              </div>
                            </div>

                            <div className="space-y-5">
                              <div className="flex items-center gap-3 border-b border-border-dark/30 pb-3">
                                <span className="material-symbols-outlined text-primary text-lg">analytics</span>
                                <span className="text-[11px] font-bold text-slate-200 uppercase tracking-[0.2em]">Agent Assignment Context</span>
                              </div>
                              <div className="grid grid-cols-2 gap-4">
                                 <div className="p-5 bg-black/20 rounded-2xl border border-border-dark space-y-2.5 group hover:border-primary/20 transition-colors shadow-inner">
                                   <div className="flex items-center gap-2">
                                     <span className="material-symbols-outlined text-[14px] text-slate-600">fingerprint</span>
                                     <span className="text-[9px] font-bold text-slate-600 uppercase tracking-[0.1em]">Session UUID</span>
                                   </div>
                                   <p className="text-[10px] font-mono text-slate-400 truncate pl-5 select-all">{task.id}</p>
                                 </div>
                                 <div className="p-5 bg-black/20 rounded-2xl border border-border-dark space-y-2.5 shadow-inner">
                                   <div className="flex items-center gap-2">
                                     <span className="material-symbols-outlined text-[14px] text-slate-600">hub</span>
                                     <span className="text-[9px] font-bold text-slate-600 uppercase tracking-[0.1em]">Responsible Sub-Agent</span>
                                   </div>
                                   <div className="flex items-center gap-2 pl-5">
                                      <span className="size-1.5 rounded-full bg-primary animate-pulse"></span>
                                      <p className="text-[10px] font-mono text-slate-400 font-bold">{task.targetAgent}</p>
                                   </div>
                                 </div>
                              </div>
                              <div className="p-5 bg-slate-800/20 rounded-2xl border border-border-dark flex items-center justify-between group cursor-help transition-all hover:bg-slate-800/40">
                                 <div className="flex items-center gap-3">
                                    <div className="size-8 rounded-lg bg-slate-900 border border-border-dark flex items-center justify-center text-slate-500 group-hover:text-primary transition-colors">
                                       <span className="material-symbols-outlined text-sm">history</span>
                                    </div>
                                    <div className="flex flex-col">
                                       <span className="text-[9px] font-bold text-slate-500 uppercase tracking-widest">Operational Uptime</span>
                                       <span className="text-[10px] font-mono text-slate-300">Mission Clock: 00:42:12:04</span>
                                    </div>
                                 </div>
                                 <span className="material-symbols-outlined text-slate-700 text-lg">arrow_right_alt</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                );
              })
            )}
          </div>
        </div>
      </div>

      {isAdding && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-md p-6">
          <div className="w-full max-w-2xl bg-panel-dark border border-border-dark rounded-3xl overflow-hidden shadow-2xl animate-in zoom-in-95 duration-200">
            <header className="px-8 py-6 border-b border-border-dark bg-background-dark/50 flex items-center justify-between">
              <div className="flex items-center gap-3">
                <span className="material-symbols-outlined text-primary">event_note</span>
                <h2 className="text-lg font-bold text-white uppercase tracking-wider">Configure Mission Cycle</h2>
              </div>
              <button onClick={() => setIsAdding(false)} className="text-slate-500 hover:text-white transition-colors">
                <span className="material-symbols-outlined">close</span>
              </button>
            </header>
            
            <form onSubmit={handleSubmit} className="p-8 space-y-8 max-h-[75vh] overflow-y-auto custom-scrollbar">
              <div className="grid grid-cols-2 gap-6">
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Mission Title</label>
                  <input 
                    required
                    className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 text-slate-200 shadow-inner outline-none transition-all"
                    placeholder="e.g. Daily Zscaler Log Sync"
                    value={newTask.title}
                    onChange={e => setNewTask({...newTask, title: e.target.value})}
                  />
                </div>
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Target Agent</label>
                  <select 
                    className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 text-slate-200 cursor-pointer outline-none transition-all"
                    value={newTask.targetAgent}
                    onChange={e => setNewTask({...newTask, targetAgent: e.target.value as AgentType})}
                  >
                    <option value="Orchestrator">Core Orchestrator</option>
                    <option value="RedTeamSupervisor">Red Team Supervisor</option>
                    <option value="BlueTeamSupervisor">Blue Team Supervisor</option>
                  </select>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-6">
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Mission Priority</label>
                  <select 
                    className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 text-slate-200 cursor-pointer outline-none transition-all"
                    value={newTask.priority}
                    onChange={e => setNewTask({...newTask, priority: e.target.value as TaskPriority})}
                  >
                    <option value="LOW">Low - Background Routine</option>
                    <option value="MEDIUM">Medium - Active Sync</option>
                    <option value="HIGH">High - Critical Analysis</option>
                    <option value="CRITICAL">Critical - System Lockdown</option>
                  </select>
                </div>
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Recurrence Cycle</label>
                  <select 
                    className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 text-slate-200 cursor-pointer outline-none transition-all"
                    value={newTask.recurring}
                    onChange={e => setNewTask({...newTask, recurring: e.target.value as RecurrencePattern})}
                  >
                    {groups.map(group => (
                      <optgroup key={group} label={group}>
                        {RECURRENCE_OPTIONS.filter(opt => opt.group === group).map(opt => (
                          <option key={opt.value} value={opt.value}>{opt.label}</option>
                        ))}
                      </optgroup>
                    ))}
                  </select>
                </div>
              </div>

              <div className="space-y-2">
                <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Execution Timestamp</label>
                <input 
                  type="datetime-local"
                  required
                  className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 text-slate-200 shadow-inner outline-none transition-all"
                  value={newTask.scheduledTime}
                  onChange={e => setNewTask({...newTask, scheduledTime: e.target.value})}
                />
              </div>

              <div className="space-y-2">
                <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Mission Parameters (Description)</label>
                <textarea 
                  required
                  className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary focus:border-primary px-4 py-3 h-24 resize-none text-slate-200 shadow-inner outline-none transition-all"
                  placeholder="Describe what the agent should accomplish..."
                  value={newTask.description}
                  onChange={e => setNewTask({...newTask, description: e.target.value})}
                />
              </div>

              {/* Enhanced Tool Multi-Select in Add Mission Modal */}
              <div className="space-y-4">
                <div className="flex items-center justify-between border-b border-border-dark/50 pb-2">
                  <div className="flex items-center gap-2">
                    <span className="material-symbols-outlined text-primary text-lg">handyman</span>
                    <label className="text-[10px] font-bold text-slate-200 uppercase tracking-widest">Capability Orchestration (Multi-Select)</label>
                  </div>
                  <span className="text-[9px] font-mono text-slate-500 uppercase tracking-widest">{newTask.tools.length} Modules Mapped</span>
                </div>
                
                <div className="space-y-4">
                  {/* Selected Chips Area */}
                  {newTask.tools.length > 0 ? (
                    <div className="flex flex-wrap gap-2 p-3 bg-black/30 border border-border-dark rounded-xl min-h-[50px] items-center">
                      {newTask.tools.map(tool => (
                        <div key={tool} className="flex items-center gap-2 px-3 py-1.5 bg-primary/20 border border-primary/40 rounded-full text-primary animate-in zoom-in-95 duration-200">
                          <span className="material-symbols-outlined text-sm">{getToolIcon(tool)}</span>
                          <span className="text-[10px] font-bold uppercase tracking-tight">{tool.replace(/_/g, ' ')}</span>
                          <button 
                            type="button"
                            onClick={() => toggleToolInNew(tool)}
                            className="size-4 hover:bg-primary/20 rounded-full flex items-center justify-center transition-colors"
                          >
                            <span className="material-symbols-outlined text-xs">close</span>
                          </button>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="p-4 border border-dashed border-border-dark rounded-xl text-center">
                      <p className="text-[10px] text-slate-600 uppercase tracking-widest italic">No capabilities mapped yet. Select from the repository below.</p>
                    </div>
                  )}

                  {/* Searchable Repository List */}
                  <div className="space-y-3">
                    <div className="relative group/search">
                      <span className="material-symbols-outlined absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 text-lg group-focus-within/search:text-primary transition-colors">search</span>
                      <input 
                        type="text"
                        className="w-full bg-background-dark border-border-dark rounded-xl text-sm focus:ring-primary/50 pl-11 pr-4 py-3 text-slate-200 font-mono shadow-inner outline-none transition-all"
                        placeholder="Filter tool repository..."
                        value={toolSearch}
                        onChange={e => setToolSearch(e.target.value)}
                      />
                    </div>
                    
                    <div className="flex flex-wrap gap-2 max-h-[160px] overflow-y-auto p-2 bg-black/20 rounded-2xl border border-border-dark custom-scrollbar">
                      {AVAILABLE_TOOLS.filter(t => t.toLowerCase().includes(toolSearch.toLowerCase())).map((tool) => {
                        const isSelected = newTask.tools.includes(tool);
                        const color = getToolColor(tool);
                        return (
                          <button
                            key={tool}
                            type="button"
                            onClick={() => toggleToolInNew(tool)}
                            className={`px-3 py-2 rounded-xl text-[10px] font-mono flex items-center gap-2 transition-all border ${
                              isSelected 
                                ? 'bg-primary border-primary text-white shadow-lg' 
                                : 'bg-slate-900/50 border-border-dark text-slate-500 hover:text-slate-300 hover:border-slate-600'
                            }`}
                          >
                            <span className={`material-symbols-outlined text-[16px] ${isSelected ? 'text-white' : color}`}>{getToolIcon(tool)}</span>
                            {tool.replace(/_/g, ' ')}
                            {isSelected && <span className="material-symbols-outlined text-xs">check</span>}
                          </button>
                        );
                      })}
                    </div>
                  </div>
                </div>
              </div>

              <div className="flex justify-end gap-4 pt-6 sticky bottom-0 bg-panel-dark/95 backdrop-blur-sm border-t border-border-dark/50 -mx-8 px-8 py-4">
                <button 
                  type="button" 
                  onClick={() => setIsAdding(false)} 
                  className="px-6 py-2.5 rounded-xl text-xs font-bold uppercase tracking-widest text-slate-500 hover:text-white transition-colors"
                >
                  Cancel
                </button>
                <button 
                  type="submit" 
                  className="px-10 py-3 bg-primary text-white rounded-xl text-xs font-bold uppercase tracking-widest shadow-xl shadow-primary/30 hover:scale-105 active:scale-95 transition-all"
                >
                  Confirm Schedule
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default SchedulerView;
