import { defineConfig, presetIcons, presetWind3 } from 'unocss'

export default defineConfig({
  presets: [
    presetWind3(),
    presetIcons({
      cdn: 'https://esm.sh/',
      scale: 1.15,
      extraProperties: {
        'display': 'inline-block',
        'flex-shrink': '0',
      },
    }),
  ],
  theme: {
    colors: {
      'canvas': 'var(--canvas)',
      'surface': 'var(--surface)',
      'elevated': 'var(--elevated)',
      'soft': 'var(--soft)',
      'soft-hover': 'var(--soft-hover)',
      'line': 'var(--line)',
      'ink': 'var(--ink)',
      'muted': 'var(--muted)',
      'accent': 'var(--accent)',
      'accent-hover': 'var(--accent-hover)',
      'accent-soft': 'var(--accent-soft)',
      'accent-ring': 'var(--accent-ring)',
      'overlay': 'var(--overlay)',
      'danger': 'var(--danger)',
      'danger-soft': 'var(--danger-soft)',
      'danger-line': 'var(--danger-line)',
      'danger-hover': 'var(--danger-hover)',
      'warning': 'var(--warning)',
      'success': 'var(--success)',
    },
    borderRadius: {
      control: '8px',
      panel: '14px',
    },
    boxShadow: {
      raised: 'var(--shadow-raised)',
      popover: 'var(--shadow-popover)',
      modal: 'var(--shadow-modal)',
    },
    fontSize: {
      '2xs': ['11px', '16px'],
    },
  },
  shortcuts: {
    'ring-focus': 'outline-none focus-visible:ring-3 focus-visible:ring-accent-ring',
    'icon-btn': 'h-9 w-9 shrink-0 inline-grid place-items-center rounded-control text-muted transition-colors duration-150 hover:bg-accent-soft hover:text-accent disabled:pointer-events-none disabled:opacity-40',
    'primary-btn': 'h-10 inline-flex items-center justify-center gap-2 rounded-control bg-accent px-4.5 text-sm text-white font-600 transition-all duration-150 hover:bg-accent-hover active:scale-95 disabled:pointer-events-none disabled:opacity-45',
    'secondary-btn': 'h-10 inline-flex items-center justify-center gap-2 rounded-control bg-soft px-4 text-sm text-muted font-600 transition-all duration-150 hover:bg-accent-soft hover:text-accent active:scale-95 disabled:pointer-events-none disabled:opacity-45',
    'field': 'h-10 w-full rounded-control border-0 bg-soft px-3.5 text-sm text-ink ring-focus transition-all duration-200 hover:bg-soft-hover focus-visible:bg-surface disabled:cursor-not-allowed disabled:opacity-50',
    'section-label': 'text-xs text-muted font-600',
    'modal-panel': 'w-full rounded-panel border border-line bg-elevated shadow-modal',
    'popover-panel': 'rounded-panel border border-line bg-elevated shadow-popover',
    'overlay': 'fixed inset-0 grid place-items-center bg-overlay p-5 backdrop-blur-sm',
    'tag': 'inline-flex items-center gap-1.5 rounded-md bg-soft px-2.5 py-1 text-2xs text-muted',
  },
})
