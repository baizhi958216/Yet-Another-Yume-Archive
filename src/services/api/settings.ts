import type { AppSettings } from '../../types'
import { transport } from '../transport'

export function getSettings() {
  return transport<AppSettings>('get_settings', undefined, '/settings')
}

export function updateSettings(settings: AppSettings) {
  return transport<AppSettings>(
    'update_settings',
    { settings },
    '/settings',
    { method: 'PUT', body: JSON.stringify(settings) },
  )
}
