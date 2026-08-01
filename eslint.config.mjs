import antfu from '@antfu/eslint-config'

export default antfu({
  vue: true,
  typescript: true,
  ignores: [
    '**/dist',
    '**/target',
    '**/node_modules',
    '**/*.lock',
    'crates/**',
    'src-tauri/**',
    'src-web/**',
    'docs/**',
    '**/docs/**',
  ],
})
