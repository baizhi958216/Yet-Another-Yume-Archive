// Task event stream: Tauri push in the app, SSE in the browser, polling as a
// last-resort fallback. The backend snapshot is the single source of truth.

import type { TaskEvent, TaskSnapshot } from '../types'
import { isTauri } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { webCall } from './transport'

export type Unsubscribe = () => void

export async function onTaskEvent(callback: (event: TaskEvent) => void): Promise<Unsubscribe> {
  if (isTauri())
    return listen<TaskEvent>('task://event', event => callback(event.payload))
  return sseTaskEvents(callback)
}

function sseTaskEvents(callback: (event: TaskEvent) => void): Unsubscribe {
  let fallback: Unsubscribe | undefined
  const source = new EventSource('/api/events')
  source.onmessage = (message) => {
    try {
      callback(JSON.parse(message.data) as TaskEvent)
    }
    catch {
      // malformed frame; the next snapshot event will restore consistency
    }
  }
  source.onerror = () => {
    // EventSource reconnects on its own; add a slow poll as belt-and-braces
    if (!fallback)
      fallback = pollTaskEvents(callback)
  }
  return () => {
    source.close()
    fallback?.()
  }
}

function pollTaskEvents(callback: (event: TaskEvent) => void): Unsubscribe {
  let stopped = false
  let sequence = 0
  const poll = async () => {
    try {
      const tasks = await webCall<TaskSnapshot[]>('/tasks')
      if (!stopped)
        tasks.forEach(task => callback({ sequence: ++sequence, task }))
    }
    catch {
      // transient backend restart; keep polling
    }
  }
  const timer = window.setInterval(poll, 2000)
  void poll()
  return () => {
    stopped = true
    window.clearInterval(timer)
  }
}
