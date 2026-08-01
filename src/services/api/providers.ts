import type {
  ProviderInfo,
  ProviderUiActionRequest,
  ProviderUiBundle,
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

export function uiBundle(providerId: string) {
  return transport<ProviderUiBundle>(
    'provider_ui_bundle',
    { providerId },
    `/providers/${encodeURIComponent(providerId)}/ui`,
  )
}

export function uiInvoke(providerId: string, request: ProviderUiActionRequest) {
  return transport<unknown>(
    'provider_ui_invoke',
    { providerId, request },
    `/providers/${encodeURIComponent(providerId)}/ui/actions`,
    { method: 'POST', body: JSON.stringify(request) },
  )
}
