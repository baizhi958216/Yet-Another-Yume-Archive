import { defineConfig, presetWind3 } from 'unocss'

export function defineProviderUnoConfig() {
  return defineConfig({
    presets: [presetWind3()],
    theme: {
      colors: {
        'canvas': 'var(--yaya-canvas)',
        'surface': 'var(--yaya-surface)',
        'elevated': 'var(--yaya-elevated)',
        'soft': 'var(--yaya-soft)',
        'line': 'var(--yaya-line)',
        'ink': 'var(--yaya-text)',
        'muted': 'var(--yaya-muted)',
        'accent': 'var(--yaya-accent)',
        'accent-hover': 'var(--yaya-accent-hover)',
        'accent-soft': 'var(--yaya-accent-soft)',
        'danger': 'var(--yaya-danger)',
        'warning': 'var(--yaya-warning)',
        'success': 'var(--yaya-success)',
        'brand': 'var(--provider-brand, var(--yaya-accent))',
        'brand-soft': 'var(--provider-brand-soft, var(--yaya-accent-soft))',
      },
    },
    shortcuts: {
      'provider-root': 'w-full min-w-0 text-ink',
      'provider-card': 'min-w-0 rounded-xl border border-line bg-surface',
      'provider-field': 'h-8.5 min-w-0 rounded-lg border-0 bg-soft px-2.5 text-xs text-ink outline-none',
      'provider-primary': 'h-8.5 shrink-0 rounded-lg border-0 bg-brand px-3.5 text-xs text-white font-700 disabled:cursor-wait disabled:opacity-50',
      'provider-secondary': 'h-8.5 shrink-0 rounded-lg border-0 bg-soft px-3.5 text-xs text-muted font-700 disabled:cursor-wait disabled:opacity-50',
    },
    preflights: [
      {
        getCSS: () => `:root{color-scheme:var(--yaya-color-scheme,light);font-family:Inter,ui-sans-serif,system-ui,sans-serif}*,::before,::after{box-sizing:border-box;border-width:0;border-style:solid}html,body,#app{width:100%;min-width:0;margin:0;overflow-x:hidden;background:transparent}body{color:var(--yaya-text)}button,input,select,textarea{font:inherit}button{cursor:pointer}`,
      },
    ],
  })
}

export default defineProviderUnoConfig()
