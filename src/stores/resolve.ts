// The "new download" workflow: raw input → provider view → selected tasks
// with their per-task and shared option values.

import type { FormField, ProviderView } from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { inspectSource } from '../services/api/input'
import * as tasksApi from '../services/api/tasks'
import { isDesktop } from '../services/transport'
import { useProvidersStore } from './providers'
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
      const providers = useProvidersStore()
      await providers.loadUiBundle(view.provider)
    }
    catch (error) {
      inspection.value = null
      throw error
    }
    finally {
      resolving.value = false
    }
  }

  function reset() {
    source.value = ''
    outputDir.value = ''
    inspection.value = null
    selected.value = {}
    options.value = {}
    sharedOptions.value = {}
  }

  function applyUiState(value: unknown) {
    const view = inspection.value
    if (!view || !value || typeof value !== 'object' || Array.isArray(value))
      return
    const state = value as Record<string, unknown>
    const taskKeys = new Set(view.tasks.map(task => task.key))
    if (state.selected && typeof state.selected === 'object' && !Array.isArray(state.selected)) {
      for (const [key, entry] of Object.entries(state.selected as Record<string, unknown>)) {
        if (taskKeys.has(key) && typeof entry === 'boolean')
          selected.value[key] = entry
      }
    }
    if (state.sharedOptions && typeof state.sharedOptions === 'object' && !Array.isArray(state.sharedOptions)) {
      const allowed = new Set(view.fields.map(field => field.key))
      for (const [key, entry] of Object.entries(state.sharedOptions as Record<string, unknown>)) {
        if (allowed.has(key))
          sharedOptions.value[key] = entry
      }
    }
    if (state.options && typeof state.options === 'object' && !Array.isArray(state.options)) {
      for (const [taskKey, entries] of Object.entries(state.options as Record<string, unknown>)) {
        if (!taskKeys.has(taskKey) || !entries || typeof entries !== 'object' || Array.isArray(entries))
          continue
        const task = view.tasks.find(value => value.key === taskKey)
        const allowed = new Set(task?.fields.map(field => field.key) ?? [])
        for (const [key, entry] of Object.entries(entries as Record<string, unknown>)) {
          if (allowed.has(key))
            options.value[taskKey][key] = entry
        }
      }
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

  return { source, outputDir, inspection, selected, options, sharedOptions, resolving, creating, resolve, reset, applyUiState, create }
})

function defaults(fields: FormField[]) {
  return Object.fromEntries(fields.map(field => [field.key, field.default]))
}
