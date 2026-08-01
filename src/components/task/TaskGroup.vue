<script setup lang="ts">
import type { TaskSnapshot } from '../../types'
import { computed, ref } from 'vue'
import { formatBytes } from '../../composables/useFormat'
import SmartImage from '../ui/SmartImage.vue'
import TaskRow from './TaskRow.vue'

const props = defineProps<{ tasks: TaskSnapshot[] }>()

const expanded = ref(false)

const title = computed(() => {
  const first = props.tasks[0]
  return first.group
    || first.outputDir.split(/[\\/]/).filter(Boolean).pop()
    || `${props.tasks.length} 个任务`
})

const cover = computed(() => props.tasks.find(task => task.draft.imageUrl)?.draft.imageUrl)

const doneCount = computed(() => props.tasks.filter(task => task.status === 'completed').length)
const failedCount = computed(() => props.tasks.filter(task => task.status === 'failed').length)
const runningRate = computed(() => props.tasks.reduce((sum, task) => sum + (task.status === 'running' ? task.rate : 0), 0))

const percent = computed(() => {
  const sum = props.tasks.reduce((value, task) => {
    if (task.status === 'completed')
      return value + 100
    return value + (task.total ? Math.min(100, task.completed / task.total * 100) : 0)
  }, 0)
  return sum / props.tasks.length
})

const summary = computed(() => {
  if (failedCount.value)
    return { text: `${failedCount.value} 个失败`, class: 'text-danger' }
  if (doneCount.value === props.tasks.length)
    return { text: '已完成', class: 'text-success' }
  if (props.tasks.some(task => task.status === 'running'))
    return { text: '运行中', class: 'text-accent' }
  if (props.tasks.every(task => task.status === 'paused'))
    return { text: '已暂停', class: 'text-muted' }
  return { text: '排队中', class: 'text-muted' }
})
</script>

<template>
  <article>
    <div class="grid grid-cols-[88px_minmax(0,1fr)] cursor-pointer items-center gap-4 py-4 sm:grid-cols-[112px_minmax(0,1fr)_auto]" role="button" :aria-expanded="expanded" @click="expanded = !expanded">
      <div class="relative">
        <SmartImage v-if="cover" class="aspect-video w-[88px] rounded-control bg-soft object-cover sm:w-[112px]" :src="cover" alt="" :provider-id="tasks[0].provider" />
        <div v-else class="aspect-video w-[88px] grid place-items-center rounded-control bg-soft text-muted sm:w-[112px]">
          <span class="i-lucide-folder-down text-xl" />
        </div>
        <span class="absolute bottom-1 right-1 rounded-md bg-overlay px-1.5 py-0.5 text-2xs text-white">{{ tasks.length }}</span>
      </div>
      <div class="min-w-0">
        <div class="flex items-center justify-between gap-3">
          <strong class="min-w-0 flex items-center gap-2 text-sm font-600">
            <span class="truncate">{{ title }}</span>
            <span class="i-lucide-chevron-down shrink-0 text-muted transition-transform" :class="expanded ? 'rotate-180' : ''" />
          </strong>
          <span class="shrink-0 text-2xs font-600" :class="summary.class">{{ summary.text }}</span>
        </div>
        <div class="my-3 h-1.5 overflow-hidden rounded-full bg-soft">
          <span class="block h-full rounded-full bg-accent transition-[width] duration-200" :style="{ width: `${percent}%` }" />
        </div>
        <div class="flex flex-wrap gap-x-4 gap-y-1 text-2xs text-muted">
          <span>{{ doneCount }} / {{ tasks.length }} 已完成</span>
          <span v-if="runningRate">{{ formatBytes(runningRate) }}/s</span>
        </div>
      </div>
    </div>
    <div v-if="expanded" class="border-t border-line pl-6 divide-y divide-line sm:pl-8">
      <TaskRow v-for="task in tasks" :key="task.id" :task="task" />
    </div>
  </article>
</template>
