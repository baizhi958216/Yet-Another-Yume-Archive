import type { CreateTasksRequest, TaskSnapshot } from '../../types'
import { isApp, transport } from '../transport'

export function createTasks(request: CreateTasksRequest) {
  return transport<TaskSnapshot[]>(
    'create_tasks',
    { request },
    '/tasks',
    { method: 'POST', body: JSON.stringify(request) },
  )
}

export function listTasks() {
  return transport<TaskSnapshot[]>('list_tasks', undefined, '/tasks')
}

function action(command: string, id: string, suffix: string, method = 'POST') {
  return transport<void>(
    command,
    { id },
    `/tasks/${encodeURIComponent(id)}${suffix}`,
    { method },
  )
}

export const pauseTask = (id: string) => action('pause_task', id, '/pause')
export const resumeTask = (id: string) => action('resume_task', id, '/resume')
export const retryTask = (id: string) => action('retry_task', id, '/retry')
export const cancelTask = (id: string) => action('cancel_task', id, '/cancel')
export const deleteTask = (id: string) => action('delete_task', id, '', 'DELETE')

/** Browser-only: stream the finished file out of the web host. */
export function downloadTaskFile(id: string) {
  if (isApp())
    return
  const link = document.createElement('a')
  link.href = `/api/tasks/${encodeURIComponent(id)}/file`
  link.download = ''
  link.hidden = true
  document.body.appendChild(link)
  link.click()
  link.remove()
}
