import type {
  ProviderAuthPage,
  ProviderInfo,
  ProviderSettingsActionResult,
  ProviderSettingsState,
  ProviderSettingsView,
} from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as providersApi from '../services/api/providers'

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<ProviderInfo[]>([])
  const auth = ref<Record<string, ProviderAuthPage | null | undefined>>({})
  const settingsViews = ref<Record<string, ProviderSettingsView | null | undefined>>({})
  const settingsStates = ref<Record<string, ProviderSettingsState | null | undefined>>({})

  async function refresh() {
    providers.value = await providersApi.listProviders()
  }

  async function setEnabled(id: string, enabled: boolean) {
    const updated = await providersApi.setProviderEnabled(id, enabled)
    const index = providers.value.findIndex(provider => provider.id === id)
    if (index >= 0)
      providers.value[index] = updated
    if (!enabled) {
      delete auth.value[id]
      delete settingsViews.value[id]
      delete settingsStates.value[id]
    }
  }

  async function refreshAuth(id: string) {
    try {
      auth.value[id] = await providersApi.authDescribe(id)
    }
    catch {
      auth.value[id] = null
    }
  }

  async function invokeAuth(
    id: string,
    action: string,
    payload: unknown,
  ) {
    return providersApi.authInvoke(id, { action, payload })
  }

  async function refreshSettings(id: string) {
    try {
      const [view, state] = await Promise.all([
        providersApi.settingsDescribe(id),
        providersApi.settingsGet(id),
      ])
      settingsViews.value[id] = view
      settingsStates.value[id] = state
    }
    catch {
      settingsViews.value[id] = null
      settingsStates.value[id] = null
    }
  }

  async function updateSettings(id: string, values: Record<string, unknown>) {
    const state = await providersApi.settingsUpdate(id, { values })
    settingsStates.value[id] = state
    await refreshSettings(id)
    return settingsStates.value[id] ?? state
  }

  async function invokeSettings(
    id: string,
    action: string,
    values: Record<string, unknown>,
  ): Promise<ProviderSettingsActionResult> {
    const result = await providersApi.settingsInvoke(id, { action, values })
    if (result.refresh)
      await refreshSettings(id)
    return result
  }

  return {
    providers,
    auth,
    settingsViews,
    settingsStates,
    refresh,
    setEnabled,
    refreshAuth,
    invokeAuth,
    refreshSettings,
    updateSettings,
    invokeSettings,
  }
})
