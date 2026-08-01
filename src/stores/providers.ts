import type { ProviderInfo, ProviderUiBundle } from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as providersApi from '../services/api/providers'

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<ProviderInfo[]>([])
  const uiBundles = ref<Record<string, ProviderUiBundle | null | undefined>>({})
  const loadingBundles = new Map<string, Promise<ProviderUiBundle | null>>()

  async function refresh() {
    providers.value = await providersApi.listProviders()
  }

  async function setEnabled(id: string, enabled: boolean) {
    const updated = await providersApi.setProviderEnabled(id, enabled)
    const index = providers.value.findIndex(provider => provider.id === id)
    if (index >= 0)
      providers.value[index] = updated
    if (!enabled)
      delete uiBundles.value[id]
  }

  function supportsSurface(id: string, surface: string) {
    return providers.value
      .find(provider => provider.id === id)
      ?.ui
      ?.surfaces
      .some(value => value.id === surface) ?? false
  }

  async function loadUiBundle(id: string) {
    const cached = uiBundles.value[id]
    if (cached !== undefined)
      return cached
    const pending = loadingBundles.get(id)
    if (pending)
      return pending
    const request = providersApi.uiBundle(id)
      .then((bundle) => {
        uiBundles.value[id] = bundle
        return bundle
      })
      .catch(() => {
        uiBundles.value[id] = null
        return null
      })
      .finally(() => loadingBundles.delete(id))
    loadingBundles.set(id, request)
    return request
  }

  function invokeUi(id: string, action: string, payload: unknown = null) {
    return providersApi.uiInvoke(id, { action, payload })
  }

  return {
    providers,
    uiBundles,
    refresh,
    setEnabled,
    supportsSurface,
    loadUiBundle,
    invokeUi,
  }
})
