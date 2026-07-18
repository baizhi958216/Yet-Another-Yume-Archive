import type { AppSettings } from '../types'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as settingsApi from '../services/api/settings'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({ defaultOutputDir: '', maxActiveTasks: 3 })

  async function refresh() {
    settings.value = await settingsApi.getSettings()
  }

  async function save() {
    settings.value.maxActiveTasks = clamp(settings.value.maxActiveTasks, 1, 10)
    settings.value = await settingsApi.updateSettings({ ...settings.value })
  }

  return { settings, refresh, save }
})

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, Number(value) || min))
}
