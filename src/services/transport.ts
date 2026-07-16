// Single switch between the Tauri IPC transport and the web REST transport.

import { invoke, isTauri } from '@tauri-apps/api/core'

const mobileTauri = isTauri() && /Android|iPhone|iPad|iPod/i.test(navigator.userAgent)

export function isDesktop() {
  return isTauri() && !mobileTauri
}

export function isApp() {
  return isTauri()
}

async function webCall<T>(path: string, init?: RequestInit): Promise<T> {
  const headers = new Headers(init?.headers)
  if (init?.body)
    headers.set('Content-Type', 'application/json')
  const response = await fetch(`/api${path}`, { ...init, headers })
  if (!response.ok)
    throw new Error((await response.text()) || `请求失败 (${response.status})`)
  return response.status === 204 ? undefined as T : response.json() as Promise<T>
}

export function transport<T>(
  command: string,
  args: Record<string, unknown> | undefined,
  path: string,
  init?: RequestInit,
): Promise<T> {
  return isTauri() ? invoke<T>(command, args) : webCall<T>(path, init)
}

export { webCall }
