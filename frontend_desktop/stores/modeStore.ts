import { atom } from 'jotai/vanilla';

export type Mode = 'Professional' | 'Personal';

export const modeAtom = atom<Mode>('Professional');

export const toggleModeAtom = atom(
  null,
  (get, set) => {
    const currentMode = get(modeAtom);
    const newMode = currentMode === 'Professional' ? 'Personal' : 'Professional';
    set(modeAtom, newMode);
    
    // Update CSS variables for color palette
    updateColorPalette(newMode);
    
    // Sync with backend
    syncModeWithBackend(newMode);
  }
);

const updateColorPalette = (mode: Mode) => {
  const root = document.documentElement;
  
  if (mode === 'Professional') {
    // Professional: Slate/Indigo theme
    root.style.setProperty('--primary-color', '#475569');
    root.style.setProperty('--secondary-color', '#4f46e5');
    root.style.setProperty('--background-color', '#f8fafc');
    root.style.setProperty('--surface-color', '#ffffff');
    root.style.setProperty('--text-color', '#1e293b');
    root.style.setProperty('--accent-color', '#6366f1');
  } else {
    // Personal: Deep Maroon/Gold theme (Scorpio)
    root.style.setProperty('--primary-color', '#7f1d1d');
    root.style.setProperty('--secondary-color', '#d97706');
    root.style.setProperty('--background-color', '#fef7ed');
    root.style.setProperty('--surface-color', '#fff7ed');
    root.style.setProperty('--text-color', '#431407');
    root.style.setProperty('--accent-color', '#dc2626');
  }
};

const syncModeWithBackend = async (mode: Mode) => {
  try {
    // Tauri command will be implemented later
    console.log(`Mode changed to: ${mode}`);
  } catch (error) {
    console.error('Failed to sync mode with backend:', error);
  }
};

// Phase 13: Predictive Cooling (Deep Calm)
export type CoolingState = 'auto_cooling' | 'off';

export const coolingStateAtom = atom<CoolingState>('off');

export const setCoolingStateAtom = atom(
  null,
  (get, set, newState: CoolingState) => {
    set(coolingStateAtom, newState);
  }
);