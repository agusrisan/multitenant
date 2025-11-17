import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './js'),
    },
  },

  build: {
    outDir: 'dist',
    manifest: true,
    rollupOptions: {
      input: {
        app: path.resolve(__dirname, 'js/app.tsx'),
      },
    },
  },

  server: {
    port: 5173,
    strictPort: true,
    origin: 'http://localhost:5173',
    proxy: {
      // Proxy API requests to Rust backend
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/web': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
})
