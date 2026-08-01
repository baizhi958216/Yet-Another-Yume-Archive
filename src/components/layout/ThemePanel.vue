<script setup lang="ts">
import type { ThemeMode } from '../../composables/useTheme'
import { DEFAULT_ACCENT, useTheme } from '../../composables/useTheme'

const { mode, accent } = useTheme()
const colors = [
  { value: DEFAULT_ACCENT, label: 'YAYA' },
  { value: '#10a37f', label: '青绿' },
  { value: '#0f8bd7', label: '蓝色' },
  { value: '#7c6ee6', label: '紫色' },
  { value: '#e07a32', label: '橙色' },
  { value: '#6b8e23', label: '橄榄绿' },
]
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
        :key="color.value"
        class="h-7 w-7 grid place-items-center rounded-full transition-transform hover:scale-105"
        :style="{ background: color.value }"
        :title="color.label"
        :aria-label="`使用${color.label}主题色`"
        @click="accent = color.value"
      >
        <span v-if="accent === color.value" class="i-lucide-check text-sm text-white" />
        <span v-else-if="color.value === DEFAULT_ACCENT" class="text-2xs text-white font-700">Y</span>
      </button>
      <label class="relative h-7 w-7 shrink-0 cursor-pointer overflow-hidden rounded-full border border-line bg-surface" title="自定义主题色">
        <span class="i-lucide-plus pointer-events-none absolute inset-0 m-auto text-xs text-muted" />
        <input v-model="accent" type="color" class="absolute inset-0 h-10 w-10 cursor-pointer opacity-0">
      </label>
    </div>
  </div>
</template>
