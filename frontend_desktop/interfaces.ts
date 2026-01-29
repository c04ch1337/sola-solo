// Type definitions for Sola Desktop application Tauri APIs

export interface SolaAppWindow {
  // Consent toggle capability
  setAccessPornCapability: (enabled: boolean) => void;
  
  // GPU monitoring
  getGpuUsage: () => Promise<{
    gpu: number;    // GPU usage percentage (0-100)
    vram: number;   // VRAM usage percentage (0-100)
    temperature: number; // GPU temperature in Celsius
  }>;
  
  // Photo vault capabilities
  listPhotoLibrary: () => Promise<PhotoLibraryItem[]>;
  deletePhoto: (path: string) => Promise<boolean>;
  exportPhoto: (path: string) => Promise<string>; // Returns path where exported
  
  // Other app capabilities
  getVersion: () => Promise<string>;
  checkForUpdates: () => Promise<{
    available: boolean;
    version?: string;
  }>;
}

export interface PhotoLibraryItem {
  name: string;
  path: string;
  lastModified: string; // ISO date string
  size: number;         // Size in bytes
  tags?: string[];      // Optional tags like "explicit"
}

// Declare global window interface with SolaApp
declare global {
  interface Window {
    solaApp?: SolaAppWindow;
  }
}