import type { AuthQrPoll, AuthQrSession, AuthStatus, ProviderInfo } from '../../types'
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

export function authQrStart(providerId: string) {
  return transport<AuthQrSession>(
    'provider_auth_qr_start',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/auth/qr/start`,
    { method: 'POST' },
  )
}

export function authQrPoll(providerId: string, key: string) {
  return transport<AuthQrPoll>(
    'provider_auth_qr_poll',
    { providerId, key },
    `/providers/${encodeURIComponent(providerId)}/auth/qr/poll`,
    { method: 'POST', body: JSON.stringify({ key }) },
  )
}

export function authStatus(providerId: string) {
  return transport<AuthStatus>(
    'provider_auth_status',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/auth/status`,
  )
}

export function authLogout(providerId: string) {
  return transport<void>(
    'provider_auth_logout',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/auth/logout`,
    { method: 'POST' },
  )
}
