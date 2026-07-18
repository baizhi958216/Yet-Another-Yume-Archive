import type { AuthStatus, ProviderInfo } from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as providersApi from '../services/api/providers'

export const useProvidersStore = defineStore('providers', () => {
  const providers = ref<ProviderInfo[]>([])
  const auth = ref<Record<string, AuthStatus | null | undefined>>({})

  async function refresh() {
    providers.value = await providersApi.listProviders()
  }

  async function setEnabled(id: string, enabled: boolean) {
    const updated = await providersApi.setProviderEnabled(id, enabled)
    const index = providers.value.findIndex(provider => provider.id === id)
    if (index >= 0)
      providers.value[index] = updated
    if (!enabled)
      delete auth.value[id]
  }

  async function refreshAuth(id: string) {
    try {
      auth.value[id] = await providersApi.authStatus(id)
    }
    catch {
      auth.value[id] = null
    }
  }

  async function logout(id: string) {
    await providersApi.authLogout(id)
    auth.value[id] = { loggedIn: false }
  }

  return { providers, auth, refresh, setEnabled, refreshAuth, logout }
})
