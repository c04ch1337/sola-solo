/// <reference types="vite/client" />

// Typed access to Vite env in TS files.
interface ImportMetaEnv {
  readonly VITE_PHOENIX_WS_URL?: string;
  readonly VITE_PHOENIX_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

