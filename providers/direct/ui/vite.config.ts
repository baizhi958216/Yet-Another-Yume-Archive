import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  root: __dirname,
  define: {
    'process.env.NODE_ENV': JSON.stringify('production'),
    '__VUE_OPTIONS_API__': 'true',
    '__VUE_PROD_DEVTOOLS__': 'false',
    '__VUE_PROD_HYDRATION_MISMATCH_DETAILS__': 'false',
  },
  plugins: [vue(), UnoCSS(resolve(__dirname, 'uno.config.ts'))],
  resolve: { alias: { '@yaya/provider-ui': resolve(__dirname, '../../../packages/provider-ui/src/index.ts') } },
  build: {
    outDir: resolve(__dirname, 'dist'),
    emptyOutDir: true,
    lib: { entry: resolve(__dirname, 'src/main.ts'), formats: ['es'], fileName: () => 'provider-ui.js' },
    cssCodeSplit: false,
    rollupOptions: { output: { assetFileNames: asset => asset.name?.endsWith('.css') ? 'provider-ui.css' : '[name][extname]' } },
  },
})
