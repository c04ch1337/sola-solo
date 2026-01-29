import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, '.', '');
    const phoenixApiUrl = env.VITE_PHOENIX_API_URL;
    if (!phoenixApiUrl) {
      throw new Error(
        'VITE_PHOENIX_API_URL is required. Set it to your backend base URL (e.g. http://localhost:8888).'
      );
    }
    
    // Derive WebSocket URL from API URL if not explicitly set
    let phoenixWsUrl = env.VITE_PHOENIX_WS_URL;
    if (!phoenixWsUrl) {
      const url = new URL(phoenixApiUrl);
      const wsProtocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
      phoenixWsUrl = `${wsProtocol}//${url.host}/ws`;
    }
    
    return {
      server: {
        // Desktop UI dev server port (align with project standard)
        port: 3000,
        strictPort: true,
        host: '0.0.0.0',
        proxy: {
          '/api': {
            target: phoenixApiUrl,
            changeOrigin: true,
            secure: false,
          },
          '/health': {
            target: phoenixApiUrl,
            changeOrigin: true,
            secure: false,
          },
        },
      },
      plugins: [react()],
      define: {
        'import.meta.env.VITE_PHOENIX_API_URL': JSON.stringify(phoenixApiUrl),
        'import.meta.env.VITE_PHOENIX_WS_URL': JSON.stringify(phoenixWsUrl),
      },
      resolve: {
        alias: {
          '@': path.resolve(__dirname, '.'),
        }
      }
    };
});
