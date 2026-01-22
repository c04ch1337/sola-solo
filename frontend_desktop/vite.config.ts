import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, '.', '');
    const phoenixApiUrl = env.VITE_PHOENIX_API_URL || 'http://localhost:8888';
    
    return {
      server: {
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
        'process.env.VITE_PHOENIX_API_URL': JSON.stringify(phoenixApiUrl),
      },
      resolve: {
        alias: {
          '@': path.resolve(__dirname, '.'),
        }
      }
    };
});
