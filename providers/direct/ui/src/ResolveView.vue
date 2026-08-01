<script setup lang="ts">
import { formatBytes, useProviderUi } from '@yaya/provider-ui'
import { computed } from 'vue'

interface Task { key: string, title: string, description: string, size?: number }
const props = defineProps<{ context: Record<string, any> }>()
const bridge = useProviderUi()
const task = computed<Task | undefined>(() => props.context.inspection?.tasks?.[0])
const selected = computed({
  get: () => !!(task.value && props.context.selected?.[task.value.key]),
  set: (value: boolean) => task.value && bridge.updateState({ selected: { [task.value.key]: value } }),
})
</script>

<template>
  <section v-if="task" class="provider-root grid grid-cols-[22px_42px_minmax(0,1fr)] items-center gap-3 border-b border-line px-0.5 py-4.5">
    <label class="relative h-4.5 w-4.5">
      <input v-model="selected" class="absolute opacity-0" type="checkbox">
      <span class="h-4.5 w-4.5 grid place-items-center rounded-1.25 border-1.5 text-xs text-white font-700" :class="selected ? 'border-accent bg-accent' : 'border-line bg-soft'">{{ selected ? '✓' : '' }}</span>
    </label>
    <div class="h-10.5 w-10.5 grid place-items-center rounded-2.25 bg-accent-soft text-xl text-accent">
      ↓
    </div>
    <div class="min-w-0">
      <strong class="block truncate text-sm">{{ task.title }}</strong>
      <small class="mt-1.25 block truncate text-[11px] text-muted">{{ [task.description, formatBytes(task.size)].filter(Boolean).join(' · ') }}</small>
    </div>
  </section>
</template>
