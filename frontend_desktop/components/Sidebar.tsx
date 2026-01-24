
import React, { useState } from 'react';
import { useAtom } from 'jotai';
import { modeAtom } from '../stores/modeStore';
import { Project, ChatHistoryItem } from '../types';

interface SidebarProps {
  onSettingsClick: () => void;
  onLogoClick: () => void;
  onAddProjectClick: () => void;
  onNewOrchestration: () => void;
  onViewChange: (view: 'chat' | 'scheduler' | 'professional') => void;
  currentView: 'chat' | 'scheduler' | 'professional';
  projects: Project[];
  activeProjectId: string | null;
  activeChatId: string | null;
  onSelectProject: (id: string) => void;
  onSelectChat: (id: string) => void;
  onRenameChat: (id: string, newTitle: string) => void;
  onDeleteChat: (id: string) => void;
  customLogo: string | null;
  customUserLogo: string | null;
  chatHistory: ChatHistoryItem[];
}

const Sidebar: React.FC<SidebarProps> = ({ 
  onSettingsClick, 
  onLogoClick,
  onAddProjectClick,
  onNewOrchestration,
  onViewChange,
  currentView,
  projects, 
  activeProjectId, 
  activeChatId,
  onSelectProject,
  onSelectChat,
  onRenameChat,
  onDeleteChat,
  customLogo,
  customUserLogo,
  chatHistory
}) => {
  const [editingChatId, setEditingChatId] = useState<string | null>(null);
  const [tempTitle, setTempTitle] = useState('');
  const [mode] = useAtom(modeAtom);

  const filteredHistory = chatHistory.filter(h => h.projectId === activeProjectId);

  const handleLogoKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onLogoClick();
    }
  };

  const startRename = (e: React.MouseEvent, item: ChatHistoryItem) => {
    e.stopPropagation();
    setEditingChatId(item.id);
    setTempTitle(item.title);
  };

  const submitRename = (id: string) => {
    if (tempTitle.trim()) {
      onRenameChat(id, tempTitle.trim());
    }
    setEditingChatId(null);
  };

  const handleRenameKeyDown = (e: React.KeyboardEvent, id: string) => {
    if (e.key === 'Enter') submitRename(id);
    if (e.key === 'Escape') setEditingChatId(null);
  };

  return (
    <aside className="w-72 border-r border-border-dark flex flex-col bg-background-dark hidden md:flex h-full shrink-0">
      <div 
        onClick={onLogoClick}
        onKeyDown={handleLogoKeyDown}
        role="button"
        tabIndex={0}
        className="p-6 pb-2 flex items-center gap-3 cursor-pointer group/branding hover:bg-panel-dark/30 transition-all active:scale-[0.98] outline-none focus-visible:ring-2 focus-visible:ring-primary/50 rounded-lg m-1"
        title="Open System Settings"
      >
        <div 
          className="size-8 rounded-lg bg-primary flex items-center justify-center text-white shrink-0 overflow-hidden border border-primary/20 shadow-lg group-hover/branding:shadow-primary/20 group-hover/branding:scale-105 transition-all"
        >
          {customLogo ? (
            <img src={customLogo} alt="Logo" className="w-full h-full object-cover" />
          ) : (
            <span className="material-symbols-outlined text-[20px] group-hover/branding:rotate-12 transition-transform">architecture</span>
          )}
        </div>
        <div className="flex flex-col overflow-hidden">
          <h1 className="text-sm font-bold tracking-tight text-white uppercase truncate group-hover/branding:text-primary transition-colors">Phoenix AGI</h1>
          <span className="text-[10px] text-primary font-mono uppercase tracking-widest">Sovereign OS</span>
        </div>
      </div>

      <div className="p-4">
        <button 
          onClick={onNewOrchestration}
          className="w-full flex items-center justify-center gap-2 px-4 py-2.5 bg-panel-dark hover:bg-slate-800 border border-border-dark rounded-xl text-sm font-medium text-white transition-all group shadow-sm active:scale-95"
        >
          <span className="material-symbols-outlined text-[20px] text-primary group-hover:scale-110 transition-transform">add_comment</span>
          New Chat
        </button>
      </div>

      <div className="flex-1 px-4 overflow-y-auto space-y-6">
        <section>
          <div className="space-y-1">
            {mode === 'Professional' ? (
              <>
                <button 
                  onClick={() => onViewChange('chat')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'chat' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">task</span>
                  <span className="text-xs font-bold uppercase tracking-wider">Tasks</span>
                </button>
                <button 
                  onClick={() => onViewChange('scheduler')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'scheduler' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">factory</span>
                  <span className="text-xs font-bold uppercase tracking-wider">Agent Factory</span>
                </button>
                <button 
                  onClick={() => onViewChange('professional')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'professional' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">monitoring</span>
                  <span className="text-xs font-bold uppercase tracking-wider">Professional Dashboard</span>
                </button>
              </>
            ) : (
              <>
                <button 
                  onClick={() => onViewChange('chat')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'chat' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">vault</span>
                  <span className="text-xs font-bold uppercase tracking-wider">The Vault</span>
                </button>
                <button 
                  onClick={() => onViewChange('scheduler')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'scheduler' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">timeline</span>
                  <span className="text-xs font-bold uppercase tracking-wider">Relationship Timeline</span>
                </button>
                <button 
                  onClick={() => onViewChange('professional')}
                  className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${currentView === 'professional' ? 'bg-primary/10 text-primary' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'}`}
                >
                  <span className="material-symbols-outlined text-[18px]">monitoring</span>
                  <span className="text-xs font-bold uppercase tracking-wider">Professional Dashboard</span>
                </button>
              </>
            )}
          </div>
        </section>

        <section>
          <div className="flex items-center justify-between px-2 mb-2">
            <h3 className="text-[10px] font-bold text-slate-500 uppercase tracking-[0.2em]">Live Projects</h3>
            <button 
              onClick={onAddProjectClick}
              className="material-symbols-outlined text-[16px] text-slate-500 cursor-pointer hover:text-white hover:rotate-90 transition-all p-1"
            >
              add
            </button>
          </div>
          <div className="space-y-1.5">
            {projects.map((project) => (
              <div 
                key={project.id}
                onClick={() => {
                  onSelectProject(project.id);
                  onViewChange('chat');
                }}
                className={`flex items-start gap-3 px-3 py-2.5 rounded-xl cursor-pointer transition-all group border ${
                  activeProjectId === project.id
                  ? 'bg-primary/10 border-primary/30 text-primary' 
                  : 'border-transparent hover:bg-panel-dark text-slate-400 hover:text-slate-200'
                }`}
              >
                <div className={`size-8 rounded-lg flex items-center justify-center shrink-0 border transition-colors ${
                  activeProjectId === project.id ? 'bg-primary border-primary text-white' : 'bg-slate-800/50 border-border-dark group-hover:border-slate-600'
                }`}>
                  <span className="material-symbols-outlined text-[18px]">
                    {project.icon}
                  </span>
                </div>
                <div className="flex flex-col overflow-hidden">
                  <span className="text-xs font-bold truncate leading-tight">{project.name}</span>
                  <span className="text-[9px] font-mono text-slate-500 truncate mt-0.5 opacity-60 group-hover:opacity-100 transition-opacity">
                    {project.location}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </section>

        <section>
          <div className="px-2 mb-2 flex items-center gap-2">
            <h3 className="text-[10px] font-bold text-slate-500 uppercase tracking-[0.2em]">Chat History</h3>
          </div>
          <div className="space-y-1">
            {filteredHistory.length === 0 ? (
              <p className="text-[10px] text-slate-600 px-3 italic uppercase">No logs for this context</p>
            ) : (
              filteredHistory.map((item) => (
                <div 
                  key={item.id}
                  onClick={() => {
                    if (editingChatId !== item.id) {
                      onSelectChat(item.id);
                      onViewChange('chat');
                    }
                  }}
                  className={`flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer transition-all group/history relative ${
                    activeChatId === item.id ? 'bg-panel-dark text-white' : 'text-slate-400 hover:text-slate-200 hover:bg-panel-dark'
                  }`}
                >
                  <span className={`material-symbols-outlined text-[18px] transition-colors shrink-0 ${
                    activeChatId === item.id ? 'text-primary' : 'text-slate-600 group-hover/history:text-primary'
                  }`}>history</span>
                  
                  {editingChatId === item.id ? (
                    <input
                      autoFocus
                      className="bg-background-dark border border-primary/50 rounded px-1.5 py-0.5 text-xs text-white w-full focus:ring-0 outline-none"
                      value={tempTitle}
                      onChange={(e) => setTempTitle(e.target.value)}
                      onBlur={() => submitRename(item.id)}
                      onKeyDown={(e) => handleRenameKeyDown(e, item.id)}
                      onClick={(e) => e.stopPropagation()}
                    />
                  ) : (
                    <span className="text-xs truncate flex-1 pr-12">{item.title}</span>
                  )}

                  {editingChatId !== item.id && (
                    <div className="absolute right-2 flex items-center gap-1 opacity-0 group-hover/history:opacity-100 transition-opacity">
                      <button 
                        onClick={(e) => startRename(e, item)}
                        className="p-1 hover:text-primary text-slate-500 transition-colors"
                        title="Rename Log"
                      >
                        <span className="material-symbols-outlined text-[16px]">edit</span>
                      </button>
                      <button 
                        onClick={(e) => { e.stopPropagation(); onDeleteChat(item.id); }}
                        className="p-1 hover:text-red-500 text-slate-500 transition-colors"
                        title="Delete Log"
                      >
                        <span className="material-symbols-outlined text-[16px]">delete</span>
                      </button>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        </section>
      </div>

      <div className="p-4 border-t border-border-dark mt-auto">
        <div className="flex items-center gap-3 p-2 rounded-xl bg-panel-dark/50 border border-border-dark">
          <div 
            className="size-8 rounded-full bg-cover bg-center border border-primary/30 overflow-hidden" 
            style={!customUserLogo ? { backgroundImage: 'url("https://picsum.photos/seed/phoenix/200/200")' } : {}}
          >
            {customUserLogo && <img src={customUserLogo} alt="User" className="w-full h-full object-cover" />}
          </div>
          <div className="flex flex-col overflow-hidden flex-1">
            <span className="text-xs font-bold truncate">User</span>
            <span className="text-[10px] text-slate-500 font-mono">ID: PHX-AGI-01</span>
          </div>
          <button 
            onClick={onSettingsClick}
            className="material-symbols-outlined text-sm text-slate-400 hover:text-white transition-colors p-1"
          >
            settings
          </button>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
