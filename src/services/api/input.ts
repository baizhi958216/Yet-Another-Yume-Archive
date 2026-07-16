import type { BinaryAsset, ProviderView } from '../../types'
import { transport } from '../transport'

export function inspectSource(source: string) {
  return transport<ProviderView>(
    'inspect_source',
    { source },
    '/inspect',
    { method: 'POST', body: JSON.stringify({ source }) },
  )
}

const assetCache = new Map<string, Promise<string>>()

/** Fetch a provider-context image and return it as a data URL (cached). */
export function providerImage(providerId: string, url: string): Promise<string> {
  if (!/^https?:\/\//i.test(url))
    return Promise.resolve(url)
  const key = `${providerId}\0${url}`
  const existing = assetCache.get(key)
  if (existing)
    return existing
  const request = transport<BinaryAsset>(
    'fetch_provider_asset',
    { providerId, url },
    `/providers/${encodeURIComponent(providerId)}/asset?url=${encodeURIComponent(url)}`,
  )
    .then(asset => `data:${asset.contentType};base64,${asset.bytes}`)
    .catch((error) => {
      assetCache.delete(key)
      throw error
    })
  assetCache.set(key, request)
  return request
}
