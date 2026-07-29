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
  <aside class="app-navigation md:sticky md:top-0 md:h-screen md:flex md:flex-col md:border-r md:border-line md:bg-elevated md:px-3 md:py-6">
    <div class="hidden px-3 pb-8 md:block">
      <div class="h-16 overflow-hidden">
        <img class="relative top-[-42px] mx-auto w-[145px] max-w-none" src="/yaya.png" alt="YAYA">
      </div>
    </div>
    <nav class="grid h-full min-w-0 grid-cols-4 gap-1 md:h-auto md:flex-none md:grid-cols-1" aria-label="主导航">
      <button
        v-for="item in items"
        :key="item.view"
        type="button"
        class="nav-item min-w-0 inline-flex flex-col items-center justify-center gap-0.5 px-2 text-2xs transition-all duration-200 active:scale-95 md:h-11 md:flex-row md:justify-start md:gap-3 md:rounded-control md:px-3.5 md:text-sm"
        :class="ui.view === item.view ? 'text-accent font-600 md:bg-accent-soft' : 'text-muted hover:text-ink md:hover:bg-soft'"
        :aria-current="ui.view === item.view ? 'page' : undefined"
        @click="ui.view = item.view"
      >
        <span :class="item.icon" class="text-xl md:text-lg" />
        <span>{{ item.label }}</span>
      </button>
    </nav>
  </aside>
</template>

<style scoped>
@media (max-width: 767px) {
  .app-navigation {
    position: fixed;
    z-index: 20;
    right: max(14px, env(safe-area-inset-right, 0px));
    bottom: calc(24px + env(safe-area-inset-bottom, 0px));
    left: max(14px, env(safe-area-inset-left, 0px));
    height: 68px;
    padding: 6px;
    border: 1px solid color-mix(in srgb, var(--line) 86%, transparent);
    border-radius: 24px;
    background: color-mix(in srgb, var(--elevated) 88%, transparent);
    box-shadow:
      0 12px 36px rgba(10, 16, 14, .16),
      0 2px 8px rgba(10, 16, 14, .08);
    backdrop-filter: saturate(1.35) blur(20px);
  }

  .nav-item {
    border-radius: 18px;
  }
}
</style>
