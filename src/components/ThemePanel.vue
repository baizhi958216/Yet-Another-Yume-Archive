<script setup lang="ts">
import type { ThemeMode } from '../composables/useTheme'
import { useTheme } from '../composables/useTheme'

const { mode, accent } = useTheme()
const colors = ['#10a37f', '#0f8bd7', '#7c6ee6', '#e4557b', '#e07a32', '#6b8e23']
const modes: Array<{ value: ThemeMode, label: string, icon: string }> = [
  { value: 'light', label: '日间', icon: 'i-lucide-sun' },
  { value: 'system', label: '系统', icon: 'i-lucide-monitor' },
  { value: 'dark', label: '夜间', icon: 'i-lucide-moon' },
]
</script>

<template>
  <div>
    <p class="mb-2 mt-0 text-2xs text-muted font-600">
      显示模式
    </p>
    <div class="grid grid-cols-3 gap-1 rounded-control bg-soft p-1">
      <button
        v-for="item in modes"
        :key="item.value"
        class="h-9 inline-flex items-center justify-center gap-1.5 rounded-md text-xs transition-all duration-200 active:scale-95"
        :class="mode === item.value ? 'bg-surface text-ink font-600 shadow-sm' : 'text-muted hover:text-ink'"
        :title="item.label"
        @click="mode = item.value"
      >
        <span :class="item.icon" /> {{ item.label }}
      </button>
    </div>
    <p class="mb-2 mt-4 text-2xs text-muted font-600">
      主题色
    </p>
    <div class="flex items-center gap-2">
      <button
        v-for="color in colors"
        :key="color"
        class="h-7 w-7 grid place-items-center rounded-full transition-transform hover:scale-105"
        :style="{ background: color }"
        :aria-label="`使用主题色 ${color}`"
        @click="accent = color"
      >
        <span v-if="accent === color" class="i-lucide-check text-sm text-white" />
      </button>
      <label class="relative h-7 w-7 shrink-0 cursor-pointer overflow-hidden rounded-full border border-line bg-surface" title="自定义主题色">
        <span class="i-lucide-plus pointer-events-none absolute inset-0 m-auto text-xs text-muted" />
        <input v-model="accent" type="color" class="absolute inset-0 h-10 w-10 cursor-pointer opacity-0">
      </label>
    </div>
  </div>
</template>
