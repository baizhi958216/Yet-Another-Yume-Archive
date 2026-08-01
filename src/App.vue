<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { computed, onMounted } from 'vue'
import SidebarNav from './components/layout/SidebarNav.vue'
import ThemeControls from './components/layout/ThemeControls.vue'
import { onTaskEvent } from './services/events'
import { isApp, isDesktop } from './services/transport'
import { useProvidersStore } from './stores/providers'
import { useSettingsStore } from './stores/settings'
import { useTasksStore } from './stores/tasks'
import { useUiStore } from './stores/ui'
import DownloadView from './views/DownloadView.vue'
import ProvidersView from './views/ProvidersView.vue'
import SettingsView from './views/SettingsView.vue'
import TasksView from './views/TasksView.vue'
import './composables/useTheme'

const ui = useUiStore()
const tasks = useTasksStore()
const providers = useProvidersStore()
const settings = useSettingsStore()

const currentView = computed(() => ({
  download: DownloadView,
  tasks: TasksView,
  providers: ProvidersView,
  settings: SettingsView,
})[ui.view])

const viewTitle = computed(() => ({
  download: '新建下载',
  tasks: '任务中心',
  providers: '插件',
  settings: '偏好设置',
})[ui.view])

onMounted(async () => {
  // the window starts hidden (tauri.conf.json `visible: false`) so the user
  // never sees the webview's blank page; reveal it on first render
  if (isApp()) {
    const appWindow = getCurrentWindow()
    void appWindow.show().then(() => appWindow.setFocus())
  }
  try {
    await Promise.all([tasks.refresh(), providers.refresh(), settings.refresh()])
    await onTaskEvent(({ task }) => tasks.upsert(task))
  }
  catch (error) {
    ui.showError(error)
  }
})
</script>

<template>
  <div class="min-h-screen bg-canvas text-ink md:grid md:grid-cols-[220px_minmax(0,1fr)]">
    <SidebarNav />
    <main class="min-w-0 px-5 pb-[calc(120px+env(safe-area-inset-bottom,0px))] pt-[calc(env(safe-area-inset-top,0px)+32px)] md:px-9 md:pb-12 md:pt-8 xl:px-12">
      <header class="mx-auto mb-7 hidden max-w-[1120px] items-center justify-between gap-4 md:flex">
        <Transition name="title" mode="out-in">
          <h1 :key="ui.view" class="m-0 text-2xl font-700">
            {{ viewTitle }}
          </h1>
        </Transition>
        <div class="flex items-center gap-3">
          <div class="hidden items-center gap-2 text-xs text-muted sm:flex">
            <span class="h-2 w-2 rounded-full bg-success" />
            {{ isDesktop() ? '桌面端' : '网页端' }}
          </div>
          <ThemeControls />
        </div>
      </header>

      <Transition name="notice">
        <div v-if="ui.error" class="mx-auto mb-5 flex max-w-[1120px] items-center gap-3 border border-danger-line rounded-control bg-danger-soft px-4 py-3 text-sm text-danger" role="alert">
          <span class="i-lucide-circle-alert text-lg" />
          <span class="min-w-0 flex-1">{{ ui.error }}</span>
          <button class="h-7 w-7 grid place-items-center rounded-md text-danger transition-colors hover:bg-danger-soft" title="关闭" @click="ui.clearError()">
            <span class="i-lucide-x" />
          </button>
        </div>
      </Transition>

      <div class="mx-auto max-w-[1120px]">
        <Transition name="page" mode="out-in">
          <component :is="currentView" :key="ui.view" />
        </Transition>
      </div>
    </main>
  </div>
</template>
