import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ViewName = 'download' | 'tasks' | 'providers' | 'settings'

export const useUiStore = defineStore('ui', () => {
  const view = ref<ViewName>('download')
  const error = ref('')

  function showError(reason: unknown) {
    error.value = reason instanceof Error ? reason.message : String(reason)
  }

  function clearError() {
    error.value = ''
  }

  return { view, error, showError, clearError }
})
