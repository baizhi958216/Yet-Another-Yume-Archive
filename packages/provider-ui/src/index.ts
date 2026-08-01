import type { Component, InjectionKey, Ref } from 'vue'
import { createApp, h, inject, reactive, ref } from 'vue'

export { default as UiButton } from './components/UiButton.vue'
export { default as UiNumberInput } from './components/UiNumberInput.vue'
export { default as UiSelect } from './components/UiSelect.vue'
export { default as UiToggle } from './components/UiToggle.vue'

export interface ProviderUiBridge {
  invoke: <T = unknown>(action: string, payload?: unknown) => Promise<T>
  asset: (url: string) => Promise<string>
  openModal: <T = unknown>(options: ProviderUiModalOptions) => Promise<T>
  closeModal: (result?: unknown) => void
  updateState: (state: unknown) => void
}

export interface ProviderUiModalOptions {
  surface: string
  title: string
  context?: Record<string, unknown>
  width?: number
}

export interface ProviderUiRuntime {
  surface: string
  root: Element
  context: Record<string, unknown>
  bridge: ProviderUiBridge
}

export interface ProviderUiHandle {
  updateContext?: (context: Record<string, unknown>) => void
  unmount?: () => void
}

const bridgeKey: InjectionKey<ProviderUiBridge> = Symbol('yaya-provider-ui-bridge')
const contextKey: InjectionKey<Record<string, unknown>> = Symbol('yaya-provider-ui-context')

export function defineProviderUi(surfaces: Record<string, Component>) {
  return {
    mount(runtime: ProviderUiRuntime): ProviderUiHandle {
      const component = surfaces[runtime.surface]
      if (!component)
        throw new Error(`Provider UI 未实现 surface: ${runtime.surface}`)
      const context = reactive({ ...runtime.context })
      const app = createApp({
        name: 'YayaProviderUiRoot',
        render: () => h(component, { context }),
      })
      app.provide(bridgeKey, runtime.bridge)
      app.provide(contextKey, context)
      app.mount(runtime.root)
      return {
        updateContext(next) {
          for (const key of Object.keys(context)) {
            if (!(key in next))
              delete context[key]
          }
          Object.assign(context, next)
        },
        unmount: () => app.unmount(),
      }
    },
  }
}

export function useProviderUi() {
  const bridge = inject(bridgeKey)
  if (!bridge)
    throw new Error('useProviderUi() 只能在 YAYA Provider UI 中使用')
  return bridge
}

export function useProviderContext<T extends Record<string, unknown>>() {
  const context = inject(contextKey)
  if (!context)
    throw new Error('Provider UI context 不可用')
  return context as T
}

export function useProviderAsset(source: () => string): Ref<string> {
  const bridge = useProviderUi()
  const value = ref('')
  const url = source()
  if (url) {
    if (/^https?:\/\//i.test(url))
      bridge.asset(url).then((result) => { value.value = result }).catch(() => {})
    else
      value.value = url
  }
  return value
}

export function formatBytes(value?: number | null) {
  if (value == null || !Number.isFinite(value))
    return ''
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let amount = value
  let unit = 0
  while (amount >= 1024 && unit < units.length - 1) {
    amount /= 1024
    unit++
  }
  return `${amount.toFixed(unit ? 1 : 0)} ${units[unit]}`
}
