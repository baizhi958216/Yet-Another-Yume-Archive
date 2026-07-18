// The "new download" workflow: raw input → provider view → selected tasks
// with their per-task and shared option values.

import type { FormField, ProviderView } from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { inspectSource } from '../services/api/input'
import * as tasksApi from '../services/api/tasks'
import { isDesktop } from '../services/transport'
import { useTasksStore } from './tasks'

export const useResolveStore = defineStore('resolve', () => {
  const source = ref('')
  const outputDir = ref('')
  const inspection = ref<ProviderView | null>(null)
  const selected = ref<Record<string, boolean>>({})
  const options = ref<Record<string, Record<string, unknown>>>({})
  const sharedOptions = ref<Record<string, unknown>>({})
  const resolving = ref(false)
  const creating = ref(false)

  async function resolve() {
    if (!source.value.trim())
      return
    resolving.value = true
    try {
      const view = await inspectSource(source.value.trim())
      inspection.value = view
      selected.value = Object.fromEntries(view.tasks.map(task => [task.key, task.selected]))
      options.value = Object.fromEntries(view.tasks.map(task => [task.key, defaults(task.fields)]))
      sharedOptions.value = defaults(view.fields)
    }
    catch (error) {
      inspection.value = null
      throw error
    }
    finally {
      resolving.value = false
    }
  }

  /** Returns created task ids; throws when nothing is selected. */
  async function create() {
    const view = inspection.value
    if (!view)
      return []
    const drafts = view.tasks
      .filter(task => selected.value[task.key])
      .map(draft => ({
        draft,
        options: { ...sharedOptions.value, ...options.value[draft.key] },
      }))
    if (!drafts.length)
      throw new Error('请至少选择一个任务')
    creating.value = true
    try {
      const created = await tasksApi.createTasks({
        provider: view.provider,
        source: source.value,
        outputDir: outputDir.value,
        tasks: drafts,
        batchId: drafts.length > 1 ? crypto.randomUUID() : undefined,
        group: drafts.length > 1 ? view.title : undefined,
      })
      const tasks = useTasksStore()
      created.forEach(tasks.upsert)
      if (!isDesktop())
        created.forEach(task => tasksApi.downloadTaskFile(task.id))
      return created
    }
    finally {
      creating.value = false
    }
  }

  return { source, outputDir, inspection, selected, options, sharedOptions, resolving, creating, resolve, create }
})

function defaults(fields: FormField[]) {
  return Object.fromEntries(fields.map(field => [field.key, field.default]))
}
