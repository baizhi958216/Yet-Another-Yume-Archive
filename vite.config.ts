import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

export default defineConfig({
  plugins: [vue(), UnoCSS()],
  // keep rust compiler errors visible during `tauri dev`
  clearScreen: false,
  server: {
    // tauri expects a fixed port
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: {
      ignored: ['**/src-tauri/**', '**/crates/**', '**/providers/**', '**/src-web/**', '**/target/**'],
    },
    // web mode: `vite dev` fronts the yaya-web axum host
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:9527',
        changeOrigin: true,
      },
    },
  },
})
