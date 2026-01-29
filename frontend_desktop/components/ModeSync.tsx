import { useAtom } from 'jotai';
import { useEffect } from 'react';
import { modeAtom } from '../stores/modeStore';

export default function ModeSync() {
  const [mode] = useAtom(modeAtom);

  useEffect(() => {
    const syncMode = async () => {
      try {
        // Mock implementation - replace with actual Tauri command
        console.log(`Mode would be synchronized with backend: ${mode}`);
        
        // Add actual backend synchronization here later
        // await invoke('set_orchestrator_mode', { mode });
      } catch (error) {
        console.error('Failed to sync mode with backend:', error);
      }
    };

    syncMode();
  }, [mode]);

  return null;
}