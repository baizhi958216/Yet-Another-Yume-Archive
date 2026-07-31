import type {
  ProviderAuthActionRequest,
  ProviderAuthPage,
  ProviderInfo,
  ProviderSettingsActionRequest,
  ProviderSettingsActionResult,
  ProviderSettingsState,
  ProviderSettingsView,
} from '../../types'
import { transport } from '../transport'

export function listProviders() {
  return transport<ProviderInfo[]>('list_providers', undefined, '/providers')
}

export function setProviderEnabled(id: string, enabled: boolean) {
  return transport<ProviderInfo>(
    'set_provider_enabled',
    { id, enabled },
    `/providers/${encodeURIComponent(id)}/enabled`,
    { method: 'POST', body: JSON.stringify({ enabled }) },
  )
}

export function authDescribe(providerId: string) {
  return transport<ProviderAuthPage>(
    'provider_auth_describe',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/auth/describe`,
  )
}

export function authInvoke(providerId: string, request: ProviderAuthActionRequest) {
  return transport<unknown>(
    'provider_auth_invoke',
    { providerId, request },
    `/providers/${encodeURIComponent(providerId)}/auth/actions`,
    { method: 'POST', body: JSON.stringify(request) },
  )
}

export function settingsDescribe(providerId: string) {
  return transport<ProviderSettingsView>(
    'provider_settings_describe',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/settings/describe`,
  )
}

export function settingsGet(providerId: string) {
  return transport<ProviderSettingsState>(
    'provider_settings_get',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/settings`,
  )
}

export function settingsUpdate(providerId: string, state: ProviderSettingsState) {
  return transport<ProviderSettingsState>(
    'provider_settings_update',
    { providerId, state },
    `/providers/${encodeURIComponent(providerId)}/settings`,
    { method: 'PUT', body: JSON.stringify(state) },
  )
}

export function settingsInvoke(providerId: string, request: ProviderSettingsActionRequest) {
  return transport<ProviderSettingsActionResult>(
    'provider_settings_invoke',
    { providerId, request },
    `/providers/${encodeURIComponent(providerId)}/settings/actions`,
    { method: 'POST', body: JSON.stringify(request) },
  )
}
