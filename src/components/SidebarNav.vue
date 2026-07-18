<script setup lang="ts">
import type { ViewName } from '../stores/ui'
import { useUiStore } from '../stores/ui'

const ui = useUiStore()
const items: Array<{ view: ViewName, label: string, icon: string }> = [
  { view: 'download', label: '下载', icon: 'i-lucide-arrow-down-to-line' },
  { view: 'tasks', label: '任务', icon: 'i-lucide-list-checks' },
  { view: 'providers', label: '插件', icon: 'i-lucide-blocks' },
  { view: 'settings', label: '设置', icon: 'i-lucide-settings-2' },
]
</script>

<template>
  <aside class="fixed bottom-0 left-0 right-0 z-20 flex border-t border-line bg-elevated px-2 pb-[env(safe-area-inset-bottom,0px)] md:sticky md:top-0 md:h-screen md:flex-col md:border-r md:border-t-0 md:px-3 md:py-6">
    <div class="hidden items-center gap-3 px-3 pb-8 md:flex">
      <div class="h-9 w-9 grid place-items-center rounded-control bg-accent text-base text-white font-700">
        梦
      </div>
      <div>
        <strong class="block text-base">YAYA</strong>
        <span class="text-2xs text-muted">YUME ARCHIVE</span>
      </div>
    </div>
    <nav class="grid h-[68px] min-w-0 flex-1 grid-cols-4 gap-1 md:h-auto md:flex-none md:grid-cols-1" aria-label="主导航">
      <button
        v-for="item in items"
        :key="item.view"
        class="min-w-0 inline-flex flex-col items-center justify-center gap-1 rounded-control px-2 text-2xs transition-all duration-200 active:scale-95 md:h-11 md:flex-row md:justify-start md:gap-3 md:px-3.5 md:text-sm"
        :class="ui.view === item.view ? 'bg-accent-soft text-accent font-600' : 'text-muted hover:bg-soft hover:text-ink'"
        @click="ui.view = item.view"
      >
        <span :class="item.icon" class="text-lg" />
        <span>{{ item.label }}</span>
      </button>
    </nav>
  </aside>
</template>
