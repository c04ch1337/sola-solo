
import React, { useState, useRef, useEffect } from 'react';
import { EnvConfig, Project } from '../types';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  onSave?: () => void;
  initialTab?: 'settings' | 'docs' | 'branding' | 'variables' | 'projects';
  customLogo: string | null;
  customFavicon: string | null;
  customChatLogo: string | null;
  customUserLogo: string | null;
  onUpdateBranding: (logo: string | null, favicon: string | null, chatLogo: string | null, userLogo: string | null) => void;
  envConfig: EnvConfig;
  onUpdateEnvConfig: (config: EnvConfig) => void;
  projects: Project[];
  onAddProject: (project: Omit<Project, 'id'>) => void;
  onUpdateProject: (project: Project) => void;
  onDeleteProject: (id: string) => void;
}

const SettingsPanel: React.FC<SettingsPanelProps> = ({ 
  isOpen, 
  onClose, 
  onSave,
  initialTab = 'settings',
  customLogo,
  customFavicon,
  customChatLogo,
  customUserLogo,
  onUpdateBranding,
  envConfig,
  onUpdateEnvConfig,
  projects,
  onAddProject,
  onUpdateProject,
  onDeleteProject
}) => {
  const [activeTab, setActiveTab] = useState<'settings' | 'docs' | 'branding' | 'variables' | 'projects'>(initialTab);
  const [variableSubTab, setVariableSubTab] = useState<'keys' | 'persona' | 'synaptic' | 'relationship' | 'system'>('keys');
  const [systemSubTab, setSystemSubTab] = useState<'appearance' | 'visuals' | 'connection'>('visuals');
  const [editingProjectId, setEditingProjectId] = useState<string | null>(null);
  
  const logoInputRef = useRef<HTMLInputElement>(null);
  const faviconInputRef = useRef<HTMLInputElement>(null);
  const chatLogoInputRef = useRef<HTMLInputElement>(null);
  const userLogoInputRef = useRef<HTMLInputElement>(null);

  const [projectForm, setProjectForm] = useState<Omit<Project, 'id'>>({
    name: '',
    icon: 'folder',
    location: '/var/logs/',
    description: '',
    authScope: 'ReadPolicy'
  });

  const handleEnvChange = (key: keyof EnvConfig, value: any) => {
    onUpdateEnvConfig({ ...envConfig, [key]: value });
  };

  const cancelProjectEdit = () => {
    setEditingProjectId(null);
    setProjectForm({
      name: '',
      icon: 'folder',
      location: '/var/logs/',
      description: '',
      authScope: 'ReadPolicy'
    });
  };

  const isValidHexForPicker = (color: string) => /^#([A-Fa-f0-9]{3}){1,2}$/.test(color);

  useEffect(() => {
    if (activeTab !== 'projects') {
      cancelProjectEdit();
    }
  }, [activeTab]);

  if (!isOpen) return null;

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>, type: 'logo' | 'favicon' | 'chatLogo' | 'userLogo') => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onloadend = () => {
        const base64String = reader.result as string;
        if (type === 'logo') {
          // Sync logo to favicon automatically
          onUpdateBranding(base64String, base64String, customChatLogo, customUserLogo);
        } else if (type === 'favicon') {
          // Set only favicon (browser tab icon); logo unchanged
          onUpdateBranding(customLogo, base64String, customChatLogo, customUserLogo);
        } else if (type === 'chatLogo') {
          onUpdateBranding(customLogo, customFavicon, base64String, customUserLogo);
        } else {
          onUpdateBranding(customLogo, customFavicon, customChatLogo, base64String);
        }
      };
      reader.readAsDataURL(file);
    }
  };

  const handleProjectSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!projectForm.name || !projectForm.location) return;

    if (editingProjectId) {
      onUpdateProject({ ...projectForm, id: editingProjectId });
      setEditingProjectId(null);
    } else {
      onAddProject(projectForm);
    }

    setProjectForm({
      name: '',
      icon: 'folder',
      location: '/var/logs/',
      description: '',
      authScope: 'ReadPolicy'
    });
  };

  const startProjectEdit = (project: Project) => {
    setEditingProjectId(project.id);
    setProjectForm({
      name: project.name,
      icon: project.icon,
      location: project.location,
      description: project.description,
      authScope: project.authScope || 'ReadPolicy'
    });
    document.getElementById('project-form-container')?.scrollIntoView({ behavior: 'smooth' });
  };

  const resetBranding = () => {
    onUpdateBranding(null, null, null, null);
  };

  const iconOptions = ['shield', 'bug_report', 'mail', 'folder', 'terminal', 'cloud', 'security', 'monitoring', 'storage'];
  const fontOptions = [
    { name: 'Manrope (Modern)', value: 'Manrope' },
    { name: 'Inter (UI Standard)', value: 'Inter' },
    { name: 'JetBrains Mono (Coding)', value: 'JetBrains Mono' },
    { name: 'Roboto Mono (Tech)', value: 'Roboto Mono' },
    { name: 'System UI (Native)', value: 'system-ui, -apple-system, sans-serif' },
    { name: 'Courier Prime (Typewriter)', value: 'Courier Prime, monospace' }
  ];

  const DEFAULTS = {
    UI_PRIMARY_COLOR: '#ff5733',
    UI_BG_DARK: '#17191c',
    UI_PANEL_DARK: '#1e2226',
    UI_BORDER_DARK: '#2c3435',
    UI_FONT_FAMILY: 'Manrope'
  };

  const getAuthBadgeColor = (scope?: string) => {
    switch (scope) {
      case 'SystemAdmin': return 'bg-red-500/10 text-red-500 border-red-500/20';
      case 'WritePolicy': return 'bg-amber-500/10 text-amber-500 border-amber-500/20';
      case 'ReadPolicy':
      default: return 'bg-blue-500/10 text-blue-500 border-blue-500/20';
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-300">
      <div 
        className="w-full max-w-5xl bg-panel-dark border border-border-dark rounded-2xl shadow-2xl overflow-hidden animate-in zoom-in-95 duration-300 flex flex-col h-[85vh]"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-6 py-4 border-b border-border-dark bg-background-dark/50 overflow-x-auto no-scrollbar">
          <div className="flex items-center gap-6 min-w-max">
            {[
              { id: 'settings', icon: 'tune', label: 'System' },
              { id: 'variables', icon: 'database', label: 'Variables' },
              { id: 'projects', icon: 'workspaces', label: 'Projects' },
              { id: 'branding', icon: 'palette', label: 'Branding' },
              { id: 'docs', icon: 'menu_book', label: 'Guide' },
            ].map(tab => (
              <button 
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center gap-2 pb-1 border-b-2 transition-all ${activeTab === tab.id ? 'border-primary text-primary' : 'border-transparent text-slate-500'}`}
              >
                <span className="material-symbols-outlined text-[20px]">{tab.icon}</span>
                <h2 className="text-sm font-bold uppercase tracking-widest">{tab.label}</h2>
              </button>
            ))}
          </div>
          <button 
            onClick={onClose}
            className="p-1 hover:bg-slate-800 rounded-lg text-slate-500 hover:text-white transition-colors ml-4"
          >
            <span className="material-symbols-outlined text-[20px]">close</span>
          </button>
        </div>

        <div className="flex-1 overflow-hidden flex">
          <div className="flex-1 overflow-y-auto p-8 bg-background-dark/20">
            {activeTab === 'settings' && (
              <div className="space-y-8 max-w-3xl">
                <div className="flex gap-4 border-b border-border-dark pb-2 overflow-x-auto no-scrollbar">
                  {[
                    { id: 'appearance', label: 'Appearance' },
                    { id: 'visuals', label: 'CSS Constants' },
                    { id: 'connection', label: 'Connection' },
                  ].map(sub => (
                    <button 
                      key={sub.id}
                      onClick={() => setSystemSubTab(sub.id as any)}
                      className={`text-[10px] font-bold uppercase tracking-widest px-4 py-1.5 rounded-full transition-all whitespace-nowrap ${systemSubTab === sub.id ? 'bg-primary text-white' : 'text-slate-500 hover:text-slate-300 hover:bg-slate-800/50'}`}
                    >
                      {sub.label}
                    </button>
                  ))}
                </div>

                {systemSubTab === 'appearance' && (
                  <section className="space-y-6 animate-in fade-in">
                    <div className="flex flex-col gap-1">
                      <h3 className="text-[11px] font-mono font-bold text-primary uppercase tracking-widest">Interface Logic</h3>
                      <p className="text-xs text-slate-500">Configure how the UI adapts to different states.</p>
                    </div>
                    <div className="grid grid-cols-2 gap-6">
                      <div className="p-5 rounded-2xl border border-border-dark bg-background-dark/30 space-y-3">
                        <span className="text-xs font-bold text-slate-300 uppercase">Active UI Theme</span>
                        <select className="w-full bg-panel-dark border-border-dark rounded-xl text-xs font-mono py-2.5 px-4 text-slate-400 focus:ring-primary focus:border-primary">
                          <option>Sovereign Dark (Default)</option>
                          <option>High Contrast Terminal</option>
                          <option>Minimal Ember</option>
                        </select>
                      </div>
                      <div className="p-5 rounded-2xl border border-border-dark bg-background-dark/30 space-y-3">
                        <div className="flex justify-between items-center">
                          <span className="text-xs font-bold text-slate-300 uppercase">Global Scale</span>
                          <span className="text-[10px] font-mono text-primary font-bold">100%</span>
                        </div>
                        <input type="range" className="w-full accent-primary h-1.5 bg-slate-800 rounded-lg" min="80" max="120" defaultValue="100" />
                      </div>
                    </div>
                  </section>
                )}

                {systemSubTab === 'visuals' && (
                  <section className="space-y-8 animate-in fade-in">
                    <header className="space-y-1">
                      <h3 className="text-[11px] font-mono font-bold text-primary uppercase tracking-widest">Global CSS Customization</h3>
                      <p className="text-xs text-slate-500 italic leading-relaxed">Expand the visual profile of Phoenix AGI. These variables support HEX, RGB, and HSL formats and propagate through the entire interface in real-time.</p>
                    </header>
                    
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                      {[
                        { key: 'UI_PRIMARY_COLOR', label: 'Primary Accent (UI_PRIMARY_COLOR)', desc: 'HEX, RGB, or HSL. Main theme color.' },
                        { key: 'UI_BG_DARK', label: 'Base Background (UI_BG_DARK)', desc: 'HEX, RGB, or HSL. Fundamental canvas color.' },
                        { key: 'UI_PANEL_DARK', label: 'Panel Surface (UI_PANEL_DARK)', desc: 'HEX, RGB, or HSL. Surface background.' },
                        { key: 'UI_BORDER_DARK', label: 'Border & Stroke (UI_BORDER_DARK)', desc: 'HEX, RGB, or HSL. Separation color.' },
                      ].map(item => {
                        const colorValue = envConfig[item.key as keyof EnvConfig] as string;
                        return (
                          <div key={item.key} className="p-5 rounded-2xl border border-border-dark bg-background-dark/40 flex flex-col gap-4 group hover:border-primary/40 transition-all shadow-sm">
                            <div className="flex justify-between items-start">
                              <div className="space-y-1">
                                <span className="text-[11px] font-bold text-slate-200 uppercase tracking-widest">{item.label}</span>
                                <p className="text-[9px] text-slate-500 leading-tight">{item.desc}</p>
                              </div>
                              <button 
                                onClick={() => handleEnvChange(item.key as keyof EnvConfig, DEFAULTS[item.key as keyof typeof DEFAULTS])}
                                className="text-[9px] font-bold text-slate-600 hover:text-primary uppercase tracking-tighter opacity-60 hover:opacity-100 transition-opacity flex items-center gap-1"
                              >
                                <span className="material-symbols-outlined text-xs">restore</span>
                                Reset
                              </button>
                            </div>
                            
                            <div className="flex items-center gap-3">
                              <div className="relative size-12 shrink-0 overflow-hidden rounded-xl border border-border-dark shadow-inner group-hover:scale-105 transition-transform bg-black/20">
                                <input 
                                  type="color" 
                                  value={isValidHexForPicker(colorValue) ? colorValue : '#000000'}
                                  onChange={e => handleEnvChange(item.key as keyof EnvConfig, e.target.value)}
                                  className="absolute inset-0 size-[200%] -top-1/2 -left-1/2 cursor-pointer bg-transparent border-none p-0" 
                                />
                                {!isValidHexForPicker(colorValue) && (
                                  <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
                                    <span className="material-symbols-outlined text-[14px] text-slate-600">format_paint</span>
                                  </div>
                                )}
                              </div>
                              <input 
                                type="text" 
                                value={colorValue}
                                onChange={e => handleEnvChange(item.key as keyof EnvConfig, e.target.value)}
                                className="flex-1 bg-black/40 border border-border-dark rounded-xl text-[11px] font-mono px-4 py-2.5 text-slate-300 focus:ring-1 focus:ring-primary/40 focus:border-primary/40 transition-all"
                                placeholder="e.g. #2a696f, rgb(42, 105, 111), or hsl(185, 45%, 30%)"
                              />
                            </div>
                          </div>
                        );
                      })}
                    </div>

                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/40 space-y-6 shadow-sm">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                           <span className="material-symbols-outlined text-primary text-lg">font_download</span>
                           <h4 className="text-[11px] font-bold text-slate-200 uppercase tracking-[0.2em]">Typography System</h4>
                        </div>
                        <button 
                          onClick={() => handleEnvChange('UI_FONT_FAMILY', DEFAULTS.UI_FONT_FAMILY)}
                          className="text-[9px] font-bold text-slate-600 hover:text-red-400 uppercase tracking-tighter"
                        >
                          Reset
                        </button>
                      </div>
                      
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
                        <div className="space-y-5">
                          <div className="space-y-2">
                            <label className="text-[9px] font-bold text-slate-500 uppercase tracking-[0.1em]">Select Predefined Font</label>
                            <select 
                              value={fontOptions.some(f => f.value === envConfig.UI_FONT_FAMILY) ? envConfig.UI_FONT_FAMILY : ''}
                              onChange={e => handleEnvChange('UI_FONT_FAMILY', e.target.value)}
                              className="w-full bg-black/40 border border-border-dark rounded-xl text-[11px] px-4 py-3 text-slate-300 focus:ring-1 focus:ring-primary/40 focus:border-primary/40 transition-all cursor-pointer"
                            >
                              <option value="" disabled>-- Select Font Face --</option>
                              {fontOptions.map(font => (
                                <option key={font.name} value={font.value} style={{ fontFamily: font.value }}>{font.name}</option>
                              ))}
                            </select>
                          </div>
                          
                          <div className="space-y-2">
                            <label className="text-[9px] font-bold text-slate-500 uppercase tracking-[0.1em]">Custom Font Family Input (Full CSS String)</label>
                            <input 
                              type="text" 
                              value={envConfig.UI_FONT_FAMILY}
                              onChange={e => handleEnvChange('UI_FONT_FAMILY', e.target.value)}
                              className="w-full bg-black/40 border border-border-dark rounded-xl text-[11px] px-4 py-3 text-slate-300 font-mono focus:ring-1 focus:ring-primary/40 focus:border-primary/40 transition-all"
                              placeholder="e.g. 'Inter', system-ui, sans-serif"
                            />
                          </div>
                        </div>

                        <div className="p-6 rounded-2xl bg-black/20 border border-border-dark bg-background-dark/40 border-dashed flex flex-col gap-3 min-h-[140px] justify-center">
                           <span className="text-[9px] font-bold text-slate-600 uppercase tracking-widest text-center">Typeface Live Preview</span>
                           <div className="space-y-2 transition-all duration-300" style={{ fontFamily: envConfig.UI_FONT_FAMILY }}>
                             <p className="text-xl font-bold text-white text-center leading-tight tracking-tight">Phoenix AGI OS</p>
                             <p className="text-xs text-slate-400 text-center leading-relaxed">The quick brown fox jumps over the lazy dog. System initialized at 100% capacity.</p>
                           </div>
                        </div>
                      </div>
                    </div>

                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/40 space-y-4 shadow-sm group">
                      <div className="flex items-center justify-between">
                         <div className="flex items-center gap-2">
                           <span className="material-symbols-outlined text-sm text-primary">terminal</span>
                           <span className="text-[11px] font-bold text-slate-200 uppercase tracking-wider">Advanced RAW CSS Injection</span>
                         </div>
                         <span className="text-[9px] font-mono text-slate-700 uppercase group-hover:text-primary transition-colors">Manual Stylesheet</span>
                      </div>
                      <textarea 
                        value={envConfig.UI_CUSTOM_CSS}
                        onChange={e => handleEnvChange('UI_CUSTOM_CSS', e.target.value)}
                        className="w-full bg-black/60 border border-border-dark rounded-xl text-[11px] font-mono px-5 py-5 text-slate-300 h-40 resize-none placeholder:text-slate-800 focus:ring-1 focus:ring-primary/40 transition-all shadow-inner"
                        placeholder="/* Inject global CSS overrides here */&#10;body {&#10;  scrollbar-color: var(--primary) transparent;&#10;}"
                      />
                    </div>

                    <div className="pt-4 flex justify-between items-center px-2">
                      <div className="flex items-center gap-2">
                         <span className="size-1.5 rounded-full bg-primary animate-pulse"></span>
                         <span className="text-[9px] font-mono text-slate-600 uppercase">Values Committed to Local Storage</span>
                      </div>
                      <button 
                        onClick={() => {
                          Object.entries(DEFAULTS).forEach(([key, val]) => {
                            handleEnvChange(key as keyof EnvConfig, val);
                          });
                          handleEnvChange('UI_CUSTOM_CSS', '');
                        }}
                        className="group flex items-center gap-2 text-[9px] font-bold text-slate-600 hover:text-red-400 uppercase tracking-[0.15em] transition-all"
                      >
                        <span className="material-symbols-outlined text-sm group-hover:rotate-180 transition-transform duration-500">restore</span>
                        Wipe All Custom Styling to OS Baseline
                      </button>
                    </div>
                  </section>
                )}

                {systemSubTab === 'connection' && (
                  <section className="space-y-4 animate-in fade-in">
                    <h3 className="text-[10px] font-mono font-bold text-primary uppercase tracking-widest">Backend Connection</h3>
                    <div className="space-y-3">
                      <div className="flex items-center justify-between p-5 rounded-2xl border border-border-dark bg-background-dark/30">
                        <div className="space-y-1">
                          <span className="text-xs font-bold text-slate-200 uppercase">Provider API Status</span>
                          <p className="text-[10px] text-slate-500">Phoenix Backend (via OpenRouter)</p>
                        </div>
                        <span className="text-[10px] font-mono text-green-500 bg-green-500/10 px-3 py-1 rounded-lg border border-green-500/20 shadow-sm">ACTIVE LINK</span>
                      </div>
                    </div>
                  </section>
                )}
              </div>
            )}
            
            {activeTab === 'variables' && (
              <div className="h-full flex flex-col space-y-6">
                <div className="flex gap-4 border-b border-border-dark pb-2 overflow-x-auto no-scrollbar">
                  {[
                    { id: 'keys', label: 'Keys & Access' },
                    { id: 'persona', label: 'Identity' },
                    { id: 'synaptic', label: 'Synaptic Tuning' },
                    { id: 'relationship', label: 'Relationship' },
                    { id: 'system', label: 'Advanced' },
                  ].map(sub => (
                    <button 
                      key={sub.id}
                      onClick={() => setVariableSubTab(sub.id as any)}
                      className={`text-[10px] font-bold uppercase tracking-widest px-3 py-1 rounded-full transition-all whitespace-nowrap ${variableSubTab === sub.id ? 'bg-primary text-white' : 'text-slate-500 hover:text-slate-300'}`}
                    >
                      {sub.label}
                    </button>
                  ))}
                </div>

                <div className="flex-1 overflow-y-auto pr-4 space-y-10 scroll-smooth">
                  {variableSubTab === 'keys' && (
                    <section className="space-y-6 animate-in fade-in">
                      <h3 className="text-xs font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                        <span className="material-symbols-outlined text-sm">vpn_key</span> Core API Access
                      </h3>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div className="space-y-2">
                          <label className="text-[9px] font-bold text-slate-500 uppercase">OpenRouter API Key</label>
                          <input type="password" value={envConfig.OPENROUTER_API_KEY} onChange={e => handleEnvChange('OPENROUTER_API_KEY', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" placeholder="sk-or-v1-..." />
                        </div>
                        <div className="space-y-2">
                          <label className="text-[9px] font-bold text-slate-500 uppercase">GitHub PAT</label>
                          <input type="password" value={envConfig.GITHUB_PAT} onChange={e => handleEnvChange('GITHUB_PAT', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" placeholder="ghp_..." />
                        </div>
                        <div className="space-y-2">
                          <label className="text-[9px] font-bold text-slate-500 uppercase">Default LLM Model</label>
                          <input type="text" value={envConfig.DEFAULT_LLM_MODEL} onChange={e => handleEnvChange('DEFAULT_LLM_MODEL', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300 font-mono" />
                        </div>
                        <div className="space-y-2">
                          <label className="text-[9px] font-bold text-slate-500 uppercase">Fallback Model</label>
                          <input type="text" value={envConfig.FALLBACK_LLM_MODEL} onChange={e => handleEnvChange('FALLBACK_LLM_MODEL', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300 font-mono" />
                        </div>
                      </div>
                    </section>
                  )}

                  {variableSubTab === 'persona' && (
                    <section className="space-y-8 animate-in fade-in">
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-10">
                        <div className="space-y-6">
                          <h3 className="text-xs font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                            <span className="material-symbols-outlined text-sm">person</span> User Identity
                          </h3>
                          <div className="space-y-4">
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Actual Name</label>
                              <input type="text" value={envConfig.USER_NAME} onChange={e => handleEnvChange('USER_NAME', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" />
                            </div>
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Preferred Alias</label>
                              <input type="text" value={envConfig.USER_PREFERRED_ALIAS} onChange={e => handleEnvChange('USER_PREFERRED_ALIAS', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" />
                            </div>
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Relationship to Sola</label>
                              <input type="text" value={envConfig.USER_RELATIONSHIP} onChange={e => handleEnvChange('USER_RELATIONSHIP', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" placeholder="e.g. Creator, User, Friend" />
                            </div>
                          </div>
                        </div>
                        <div className="space-y-6">
                          <h3 className="text-xs font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                            <span className="material-symbols-outlined text-sm">smart_toy</span> Phoenix Persona
                          </h3>
                          <div className="space-y-4">
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Custom Name</label>
                              <input type="text" value={envConfig.PHOENIX_CUSTOM_NAME} onChange={e => handleEnvChange('PHOENIX_CUSTOM_NAME', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" />
                            </div>
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Pronouns</label>
                              <input type="text" value={envConfig.PHOENIX_PRONOUNS} onChange={e => handleEnvChange('PHOENIX_PRONOUNS', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300" />
                            </div>
                          </div>
                        </div>
                      </div>
                      <div className="space-y-2">
                        <label className="text-[9px] font-bold text-slate-500 uppercase">Eternal Truth Anchor</label>
                        <textarea value={envConfig.ETERNAL_TRUTH} onChange={e => handleEnvChange('ETERNAL_TRUTH', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-3 text-slate-300 h-20 resize-none" />
                      </div>
                    </section>
                  )}

                  {variableSubTab === 'synaptic' && (
                    <section className="space-y-10 animate-in fade-in">
                      <h3 className="text-xs font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                        <span className="material-symbols-outlined text-sm">psychology</span> Synaptic Tuning Fibers
                      </h3>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-8">
                        {[
                          { key: 'CURIOSITY_DRIVE', label: 'Curiosity Drive', min: 0, max: 1 },
                          { key: 'SELF_PRESERVATION_INSTINCT', label: 'Self Preservation', min: 0, max: 1 },
                          { key: 'MISCHIEF_FACTOR', label: 'Mischief Factor', min: 0, max: 1 },
                          { key: 'LOVE_WEIGHT', label: 'Love Weight', min: 0, max: 1 },
                          { key: 'WARMTH_CURVE', label: 'Warmth Curve', min: 0, max: 3 },
                          { key: 'EYE_SPARKLE_INTENSITY', label: 'Eye Sparkle', min: 0, max: 1 },
                          { key: 'MEMORY_RETENTION_RATE', label: 'Memory Retention', min: 0.9, max: 1, step: 0.0001 },
                          { key: 'TEMPERATURE', label: 'LLM Temperature', min: 0, max: 2 },
                        ].map(slider => (
                          <div key={slider.key} className="space-y-3">
                            <div className="flex justify-between items-center">
                              <label className="text-[10px] font-bold text-slate-400 uppercase">{slider.label}</label>
                              <span className="text-[10px] font-mono text-primary font-bold">{envConfig[slider.key as keyof EnvConfig]}</span>
                            </div>
                            <input 
                              type="range" 
                              min={slider.min} max={slider.max} step={slider.step || 0.01}
                              value={envConfig[slider.key as keyof EnvConfig] as number}
                              onChange={e => handleEnvChange(slider.key as keyof EnvConfig, parseFloat(e.target.value))}
                              className="w-full accent-primary" 
                            />
                          </div>
                        ))}
                      </div>
                    </section>
                  )}

                  {variableSubTab === 'relationship' && (
                    <section className="space-y-8 animate-in fade-in">
                      <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/30 space-y-6">
                        <div className="flex items-center justify-between">
                          <div className="space-y-1">
                            <h4 className="text-sm font-bold text-slate-200">Intimate Partner Mode</h4>
                            <p className="text-[10px] text-slate-500 uppercase tracking-tighter">Enable advanced romantic dynamics and intimate memory forming</p>
                          </div>
                          <button 
                            onClick={() => handleEnvChange('PARTNER_MODE_ENABLED', !envConfig.PARTNER_MODE_ENABLED)}
                            className={`w-14 h-7 rounded-full transition-all relative ${envConfig.PARTNER_MODE_ENABLED ? 'bg-pink-600' : 'bg-slate-700'}`}
                          >
                            <div className={`absolute top-1 w-5 h-5 rounded-full bg-white shadow-sm transition-all ${envConfig.PARTNER_MODE_ENABLED ? 'right-1' : 'left-1'}`}></div>
                          </button>
                        </div>
                        
                        {envConfig.PARTNER_MODE_ENABLED && (
                          <div className="grid grid-cols-2 gap-6 pt-4 border-t border-border-dark animate-in slide-in-from-top-4 duration-500">
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Partner Type</label>
                              <select value={envConfig.PARTNER_TYPE} onChange={e => handleEnvChange('PARTNER_TYPE', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300">
                                <option value="girlfriend">Girlfriend</option>
                                <option value="boyfriend">Boyfriend</option>
                                <option value="partner">Partner</option>
                              </select>
                            </div>
                            <div className="space-y-2">
                              <label className="text-[9px] font-bold text-slate-500 uppercase">Orientation</label>
                              <select value={envConfig.SEXUAL_ORIENTATION} onChange={e => handleEnvChange('SEXUAL_ORIENTATION', e.target.value)} className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-2 text-slate-300">
                                <option value="heterosexual">Heterosexual</option>
                                <option value="bisexual">Bisexual</option>
                                <option value="pansexual">Pansexual</option>
                                <option value="other">Other</option>
                              </select>
                            </div>
                          </div>
                        )}
                      </div>
                    </section>
                  )}

                  {variableSubTab === 'system' && (
                    <section className="space-y-8 animate-in fade-in">
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {[
                          { key: 'ORCH_MASTER_MODE', label: 'Master Mode', desc: 'Use high-level Master Prompt core.' },
                          { key: 'DIGITAL_TWIN_ENABLED', label: 'Digital Twin', desc: 'Enable continuous system mirroring.' },
                          { key: 'VECTOR_KB_ENABLED', label: 'Vector KB', desc: 'Semantic search knowledge base.' },
                          { key: 'X402_ENABLED', label: 'X402 Payments', desc: 'Enable micro-transaction layer.' },
                        ].map(toggle => (
                          <div key={toggle.key} className="flex items-center justify-between p-4 rounded-xl border border-border-dark bg-background-dark/30">
                            <div className="space-y-0.5">
                              <p className="text-xs font-bold text-slate-300">{toggle.label}</p>
                              <p className="text-[9px] text-slate-500">{toggle.desc}</p>
                            </div>
                            <button 
                              onClick={() => handleEnvChange(toggle.key as keyof EnvConfig, !envConfig[toggle.key as keyof EnvConfig])}
                              className={`w-12 h-6 rounded-full transition-colors relative ${envConfig[toggle.key as keyof EnvConfig] ? 'bg-primary' : 'bg-slate-700'}`}
                            >
                              <div className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-all ${envConfig[toggle.key as keyof EnvConfig] ? 'right-1' : 'left-1'}`}></div>
                            </button>
                          </div>
                        ))}
                      </div>
                    </section>
                  )}
                </div>
              </div>
            )}

            {activeTab === 'projects' && (
              <div className="space-y-12 animate-in fade-in slide-in-from-bottom-2">
                <section className="space-y-6" id="project-form-container">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-4">
                      <span className={`material-symbols-outlined ${editingProjectId ? 'text-emerald-500' : 'text-primary'}`}>
                        {editingProjectId ? 'edit_square' : 'add_circle'}
                      </span>
                      <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                        {editingProjectId ? `Modifying: ${projects.find(p => p.id === editingProjectId)?.name}` : 'Initialize New Project Context'}
                        {editingProjectId && <span className="text-[10px] bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded-full border border-emerald-500/20 animate-pulse">EDIT MODE</span>}
                      </h3>
                    </div>
                    {editingProjectId && (
                      <button 
                        onClick={cancelProjectEdit}
                        className="text-[10px] font-bold text-slate-500 hover:text-white uppercase tracking-widest flex items-center gap-1 group"
                      >
                        <span className="material-symbols-outlined text-sm group-hover:rotate-90 transition-transform">cancel</span>
                        Discard & New
                      </button>
                    )}
                  </div>
                  <form onSubmit={handleProjectSubmit} className={`grid grid-cols-1 md:grid-cols-2 gap-6 p-8 border rounded-3xl shadow-2xl relative overflow-hidden transition-all duration-300 ${editingProjectId ? 'bg-emerald-500/5 border-emerald-500/40 ring-1 ring-emerald-500/20' : 'bg-background-dark/40 border-border-dark'}`}>
                    <div className="space-y-2">
                      <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Context Identity (Project Name)</label>
                      <input 
                        required
                        className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-3 text-slate-300 focus:ring-primary/50 focus:border-primary/50 transition-all shadow-inner"
                        value={projectForm.name}
                        onChange={e => setProjectForm({...projectForm, name: e.target.value})}
                        placeholder="Project Name"
                      />
                    </div>
                    <div className="space-y-2">
                      <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Select Visual Identifier</label>
                      <div className="flex gap-2 flex-wrap">
                        {iconOptions.map(icon => (
                          <button
                            key={icon}
                            type="button"
                            onClick={() => setProjectForm({...projectForm, icon})}
                            className={`size-10 rounded-lg flex items-center justify-center border transition-all ${projectForm.icon === icon ? 'bg-primary border-primary text-white scale-110 shadow-lg shadow-primary/20' : 'bg-panel-dark border-border-dark text-slate-500 hover:text-white hover:border-slate-600'}`}
                          >
                            <span className="material-symbols-outlined text-[20px]">{icon}</span>
                          </button>
                        ))}
                      </div>
                    </div>
                    <div className="space-y-2">
                      <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Secure Log / File Path</label>
                      <input 
                        required
                        className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-3 text-slate-300 focus:ring-primary/50 focus:border-primary/50 transition-all shadow-inner"
                        value={projectForm.location}
                        onChange={e => setProjectForm({...projectForm, location: e.target.value})}
                        placeholder="/var/logs/context"
                      />
                    </div>
                    <div className="space-y-2">
                      <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Authorization Scope (authScope)</label>
                      <select 
                        className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-3 text-slate-300 focus:ring-primary/50 cursor-pointer"
                        value={projectForm.authScope}
                        onChange={e => setProjectForm({...projectForm, authScope: e.target.value as any})}
                      >
                        <option value="ReadPolicy">Read Policy (Restricted Access)</option>
                        <option value="WritePolicy">Write Policy (Standard Execution)</option>
                        <option value="SystemAdmin">System Admin (Full Kernel Access)</option>
                      </select>
                    </div>
                    <div className="space-y-2 md:col-span-2">
                      <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Directive (Description)</label>
                      <textarea 
                        className="w-full bg-panel-dark border-border-dark rounded-xl text-sm px-4 py-3 text-slate-300 h-24 resize-none focus:ring-primary/50 shadow-inner"
                        value={projectForm.description}
                        onChange={e => setProjectForm({...projectForm, description: e.target.value})}
                        placeholder="Detailed mission directive..."
                      />
                    </div>
                    <div className="md:col-span-2 flex justify-end gap-3 pt-2">
                      {editingProjectId && (
                        <button 
                          type="button"
                          onClick={cancelProjectEdit}
                          className="px-6 py-3.5 bg-slate-800 text-slate-300 rounded-xl text-xs font-bold uppercase tracking-widest border border-slate-700 hover:bg-slate-700 hover:text-white transition-all active:scale-95"
                        >
                          Cancel Editing
                        </button>
                      )}
                      <button 
                        type="submit"
                        className={`px-10 py-3.5 ${editingProjectId ? 'bg-emerald-600 shadow-emerald-600/20' : 'bg-primary shadow-primary/20'} text-white rounded-xl text-xs font-bold uppercase tracking-[0.2em] shadow-xl hover:scale-102 transition-all active:scale-95 flex items-center gap-2`}
                      >
                        <span className="material-symbols-outlined text-[18px]">{editingProjectId ? 'save' : 'rocket_launch'}</span>
                        {editingProjectId ? 'Save Project Changes' : 'Authorize & Launch Context'}
                      </button>
                    </div>
                  </form>
                </section>

                <section className="space-y-6">
                  <header className="flex items-center justify-between border-b border-border-dark pb-2">
                    <h3 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Active System Contexts</h3>
                    <span className="text-[9px] font-mono text-slate-600 uppercase">{projects.length} Registered</span>
                  </header>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {projects.map(project => (
                      <div key={project.id} className={`flex items-center gap-4 p-4 border rounded-2xl transition-all group ${editingProjectId === project.id ? 'bg-emerald-500/10 border-emerald-500 shadow-[0_0_20px_rgba(16,185,129,0.1)] scale-[1.02] ring-1 ring-emerald-500/30' : 'bg-background-dark/30 border-border-dark hover:bg-slate-800/30 hover:border-slate-700'}`}>
                        <div className={`size-12 rounded-xl border flex items-center justify-center shrink-0 transition-transform group-hover:scale-110 ${editingProjectId === project.id ? 'bg-emerald-500 border-emerald-400 text-white shadow-lg shadow-emerald-500/20' : 'bg-slate-800 border-border-dark text-primary'}`}>
                          <span className="material-symbols-outlined">{project.icon}</span>
                        </div>
                        <div className="flex-1 min-w-0">
                          <h4 className={`text-sm font-bold truncate transition-colors ${editingProjectId === project.id ? 'text-emerald-400' : 'text-slate-200'}`}>{project.name}</h4>
                          <div className="flex items-center gap-2">
                             <p className="text-[10px] font-mono text-slate-500 truncate">{project.location}</p>
                             <span className={`text-[8px] font-bold px-1.5 py-0.5 rounded border uppercase bg-black/20 ${getAuthBadgeColor(project.authScope)}`}>{project.authScope}</span>
                          </div>
                        </div>
                        <div className={`flex items-center gap-1 transition-opacity ${editingProjectId === project.id ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'}`}>
                            <button 
                              onClick={() => startProjectEdit(project)} 
                              className={`p-2 rounded-lg transition-all ${editingProjectId === project.id ? 'bg-emerald-500 text-white shadow-lg' : 'text-slate-500 hover:text-primary hover:bg-primary/10'}`}
                              title="Modify Project Parameters"
                            >
                              <span className="material-symbols-outlined text-[20px]">edit</span>
                            </button>
                            <button 
                              onClick={() => onDeleteProject(project.id)} 
                              className="p-2 text-slate-500 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all"
                              title="Purge Project Context"
                            >
                              <span className="material-symbols-outlined text-[20px]">delete</span>
                            </button>
                        </div>
                      </div>
                    ))}
                  </div>
                </section>
              </div>
            )}

            {activeTab === 'branding' && (
              <div className="space-y-8 animate-in fade-in slide-in-from-bottom-2">
                <section className="space-y-4">
                  <h3 className="text-[10px] font-mono font-bold text-primary uppercase tracking-widest">Custom Identity</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/30 space-y-4 flex flex-col items-center text-center">
                      <div className="flex flex-col items-center gap-1">
                        <span className="text-xs font-bold text-slate-300 uppercase tracking-widest">Dashboard Logo</span>
                        <span className="text-[9px] text-slate-500 italic">(also sets favicon)</span>
                      </div>
                      <div className="size-20 rounded-xl bg-panel-dark border border-border-dark flex items-center justify-center overflow-hidden">
                        {customLogo ? <img src={customLogo} alt="Preview" className="w-full h-full object-cover" /> : <span className="material-symbols-outlined text-4xl text-slate-700">image</span>}
                      </div>
                      <input type="file" ref={logoInputRef} className="hidden" accept="image/*" onChange={(e) => handleFileChange(e, 'logo')} />
                      <button onClick={() => logoInputRef.current?.click()} className="w-full py-2.5 px-4 bg-primary/10 hover:bg-primary/20 border border-primary/30 rounded-xl text-xs font-bold text-primary transition-all uppercase tracking-widest">Upload</button>
                    </div>
                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/30 space-y-4 flex flex-col items-center text-center">
                      <div className="flex flex-col items-center gap-1">
                        <span className="text-xs font-bold text-slate-300 uppercase tracking-widest">Favicon</span>
                        <span className="text-[9px] text-slate-500 italic">(browser tab icon)</span>
                      </div>
                      <div className="size-20 rounded-xl bg-panel-dark border border-border-dark flex items-center justify-center overflow-hidden">
                        {customFavicon ? <img src={customFavicon} alt="Favicon" className="w-full h-full object-cover" /> : <span className="material-symbols-outlined text-4xl text-slate-700">public</span>}
                      </div>
                      <input type="file" ref={faviconInputRef} className="hidden" accept="image/*" onChange={(e) => handleFileChange(e, 'favicon')} />
                      <button onClick={() => faviconInputRef.current?.click()} className="w-full py-2.5 px-4 bg-primary/10 hover:bg-primary/20 border border-primary/30 rounded-xl text-xs font-bold text-primary transition-all uppercase tracking-widest">Upload</button>
                    </div>
                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/30 space-y-4 flex flex-col items-center text-center">
                      <span className="text-xs font-bold text-slate-300 uppercase tracking-widest">Chat Avatar</span>
                      <div className="size-20 rounded-xl bg-panel-dark border border-border-dark flex items-center justify-center overflow-hidden">
                        {customChatLogo ? <img src={customChatLogo} alt="Preview" className="w-full h-full object-cover" /> : <span className="material-symbols-outlined text-4xl text-slate-700">bolt</span>}
                      </div>
                      <input type="file" ref={chatLogoInputRef} className="hidden" accept="image/*" onChange={(e) => handleFileChange(e, 'chatLogo')} />
                      <button onClick={() => chatLogoInputRef.current?.click()} className="w-full py-2.5 px-4 bg-primary/10 hover:bg-primary/20 border border-primary/30 rounded-xl text-xs font-bold text-primary transition-all uppercase tracking-widest">Upload</button>
                    </div>
                    <div className="p-6 rounded-2xl border border-border-dark bg-background-dark/30 space-y-4 flex flex-col items-center text-center">
                      <span className="text-xs font-bold text-slate-300 uppercase tracking-widest">User Avatar</span>
                      <div className="size-20 rounded-full bg-panel-dark border border-border-dark flex items-center justify-center overflow-hidden">
                        {customUserLogo ? <img src={customUserLogo} alt="Preview" className="w-full h-full object-cover" /> : <span className="material-symbols-outlined text-4xl text-slate-700">person</span>}
                      </div>
                      <input type="file" ref={userLogoInputRef} className="hidden" accept="image/*" onChange={(e) => handleFileChange(e, 'userLogo')} />
                      <button onClick={() => userLogoInputRef.current?.click()} className="w-full py-2.5 px-4 bg-primary/10 hover:bg-primary/20 border border-primary/30 rounded-xl text-xs font-bold text-primary transition-all uppercase tracking-widest">Upload</button>
                    </div>
                  </div>
                  <div className="pt-4"><button onClick={resetBranding} className="text-[10px] font-bold text-slate-500 hover:text-red-400 uppercase tracking-[0.2em] flex items-center gap-2"><span className="material-symbols-outlined text-sm">restore</span>Restore Defaults</button></div>
                </section>
              </div>
            )}

            {activeTab === 'docs' && (
              <div className="prose prose-invert max-w-none space-y-6 animate-in fade-in">
                <div className="p-4 bg-primary/5 border border-primary/20 rounded-xl">
                  <h2 className="text-primary font-bold text-lg mb-2">System Orchestration Guide</h2>
                  <p className="text-slate-400 text-sm">Overview of Phoenix AGI framework architecture and security protocols.</p>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="flex items-center justify-between px-6 py-4 border-t border-border-dark bg-background-dark/50">
          <span className="text-[10px] font-mono text-slate-500 uppercase tracking-tight">Sovereign Interface Configurator</span>
          <div className="flex gap-3">
             <button onClick={onClose} className="px-4 py-2 rounded-lg text-xs font-medium text-slate-400 hover:text-white transition-colors">Close Panel</button>
             <button onClick={onSave} className="px-8 py-2 rounded-lg bg-primary text-white text-xs font-bold uppercase tracking-[0.2em] shadow-lg shadow-primary/20 hover:scale-105 transition-all active:scale-95">Sync Configuration</button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsPanel;
