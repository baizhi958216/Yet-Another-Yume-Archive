import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ViewName = 'download' | 'tasks' | 'providers' | 'settings'

export const useUiStore = defineStore('ui', () => {
  const view = ref<ViewName>('download')
  const error = ref('')

  function showError(reason: unknown) {
    if (!reason) {
      error.value = ''
      return
    }
    if (typeof reason === 'string') {
      error.value = reason
      return
    }
    if (reason instanceof Error) {
      error.value = reason.message
      return
    }
    if (typeof reason === 'object' && reason !== null) {
      const obj = reason as Record<string, unknown>
      if (typeof obj.message === 'string' && obj.message) {
        error.value = obj.message
        return
      }
      if (typeof obj.error === 'string' && obj.error) {
        error.value = obj.error
        return
      }
    }
    error.value = String(reason)
  }

  function clearError() {
    error.value = ''
  }

  return { view, error, showError, clearError }
})
