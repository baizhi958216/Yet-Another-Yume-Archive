import { computed, ref, watch } from 'vue'

export type ThemeMode = 'light' | 'dark' | 'system'

const storedMode = localStorage.getItem('yaya-theme-mode') as ThemeMode | null
const mode = ref<ThemeMode>(['light', 'dark', 'system'].includes(storedMode || '') ? storedMode! : 'system')
const accent = ref(normalizeHex(localStorage.getItem('yaya-theme-accent')) || '#10a37f')
const systemDark = ref(matchMedia('(prefers-color-scheme: dark)').matches)
const resolvedMode = computed(() => mode.value === 'system' ? (systemDark.value ? 'dark' : 'light') : mode.value)

matchMedia('(prefers-color-scheme: dark)')
  .addEventListener('change', event => systemDark.value = event.matches)

watch([mode, accent, resolvedMode], () => {
  const root = document.documentElement
  const normalized = normalizeHex(accent.value) || '#10a37f'
  const rgb = hexToRgb(normalized)
  accent.value = normalized
  root.dataset.theme = resolvedMode.value
  root.style.colorScheme = resolvedMode.value
  root.style.setProperty('--accent', normalized)
  root.style.setProperty('--accent-hover', mix(normalized, resolvedMode.value === 'dark' ? '#ffffff' : '#000000', 0.14))
  root.style.setProperty('--accent-soft', `rgba(${rgb}, ${resolvedMode.value === 'dark' ? 0.16 : 0.1})`)
  root.style.setProperty('--accent-ring', `rgba(${rgb}, 0.18)`)
  localStorage.setItem('yaya-theme-mode', mode.value)
  localStorage.setItem('yaya-theme-accent', normalized)
}, { immediate: true })

function normalizeHex(value: string | null) {
  if (!value)
    return null
  const color = value.trim()
  if (/^#[0-9a-f]{6}$/i.test(color))
    return color.toLowerCase()
  if (/^#[0-9a-f]{3}$/i.test(color))
    return `#${color.slice(1).split('').map(char => char + char).join('')}`.toLowerCase()
  return null
}

function hexToRgb(hex: string) {
  return [1, 3, 5].map(index => Number.parseInt(hex.slice(index, index + 2), 16)).join(', ')
}

function mix(source: string, target: string, weight: number) {
  const channel = (hex: string, index: number) => Number.parseInt(hex.slice(index, index + 2), 16)
  return `#${[1, 3, 5]
    .map(index => Math.round(channel(source, index) * (1 - weight) + channel(target, index) * weight)
      .toString(16)
      .padStart(2, '0'))
    .join('')}`
}

export function useTheme() {
  return { mode, accent, resolvedMode }
}
