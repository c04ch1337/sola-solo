import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import { VitePWA } from 'vite-plugin-pwa';

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
  const port = Number(env.VITE_MOBILE_PORT || 3000);
  const phoenixApiUrl = env.VITE_PHOENIX_API_URL;
  if (!phoenixApiUrl) {
    throw new Error(
      'VITE_PHOENIX_API_URL is required. Set it to your backend base URL (e.g. http://localhost:8888).'
    );
  }

  return {
    plugins: [
      react(),
      VitePWA({
        registerType: 'autoUpdate',
        includeAssets: ['vite.svg'],
        manifest: {
          name: 'L9 Mobile',
          short_name: 'L9M',
          theme_color: '#5D6D7E',
          background_color: '#0b1110',
          display: 'standalone',
          start_url: '/',
          icons: [
            {
              src: '/vite.svg',
              sizes: 'any',
              type: 'image/svg+xml',
              purpose: 'any',
            },
          ],
        },
      }),
    ],
    server: {
      host: true,
      port,
    },
    preview: {
      host: true,
      port,
    },
    define: {
      'import.meta.env.VITE_PHOENIX_API_URL': JSON.stringify(phoenixApiUrl),
    },
  };
});
