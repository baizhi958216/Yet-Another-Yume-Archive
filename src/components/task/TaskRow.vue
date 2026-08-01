<script setup lang="ts">
import type { TaskSnapshot } from '../../types'
import { computed, ref } from 'vue'
import { formatBytes } from '../../composables/useFormat'
import { useTasksStore } from '../../stores/tasks'
import { useUiStore } from '../../stores/ui'
import ConfirmDialog from '../ui/ConfirmDialog.vue'
import SmartImage from '../ui/SmartImage.vue'

const props = defineProps<{ task: TaskSnapshot }>()
const tasks = useTasksStore()
const ui = useUiStore()

const percent = computed(() => {
  if (props.task.status === 'completed')
    return 100
  return props.task.total
    ? Math.min(100, props.task.completed / props.task.total * 100)
    : 0
})

const labels: Record<TaskSnapshot['status'], string> = {
  queued: '排队中',
  running: '运行中',
  paused: '已暂停',
  completed: '已完成',
  failed: '失败',
  canceled: '已取消',
}

const progressText = computed(() => {
  if (!props.task.completed && !props.task.total)
    return ''
  if (props.task.status === 'completed')
    return formatBytes(props.task.total || props.task.completed)
  if (props.task.total)
    return `${formatBytes(props.task.completed)} / ${formatBytes(props.task.total)}`
  return formatBytes(props.task.completed)
})

const confirmingDelete = ref(false)

async function act(action: 'pause' | 'resume' | 'retry' | 'cancel' | 'delete') {
  try {
    await tasks.act(action, props.task.id)
  }
  catch (error) {
    ui.showError(error)
  }
}

function confirmDelete() {
  confirmingDelete.value = false
  act('delete')
}
</script>

<template>
  <article class="grid grid-cols-[88px_minmax(0,1fr)] items-center gap-4 py-4 sm:grid-cols-[112px_minmax(0,1fr)_auto]">
    <SmartImage v-if="task.draft.imageUrl" class="aspect-video w-[88px] rounded-control bg-soft object-cover sm:w-[112px]" :src="task.draft.imageUrl" alt="" :provider-id="task.provider" />
    <div v-else class="aspect-video w-[88px] grid place-items-center rounded-control bg-soft text-muted sm:w-[112px]">
      <span class="i-lucide-box text-xl" />
    </div>
    <div class="min-w-0">
      <div class="flex items-center justify-between gap-3">
        <strong class="min-w-0 truncate text-sm font-600">{{ task.draft.title }}</strong>
        <span class="shrink-0 text-2xs font-600" :class="task.status === 'failed' ? 'text-danger' : task.status === 'completed' ? 'text-success' : task.status === 'running' ? 'text-accent' : 'text-muted'">{{ labels[task.status] }}</span>
      </div>
      <div class="my-3 h-1.5 overflow-hidden rounded-full bg-soft">
        <span class="block h-full rounded-full bg-accent transition-[width] duration-200" :style="{ width: `${percent}%` }" />
      </div>
      <div class="flex flex-wrap gap-x-4 gap-y-1 text-2xs text-muted">
        <span v-if="task.status === 'completed'">下载完成</span>
        <span v-else-if="task.message">{{ task.message }}</span>
        <span v-if="progressText">{{ progressText }}</span>
        <span v-if="task.rate && task.status === 'running'">{{ formatBytes(task.rate) }}/s</span>
        <span v-if="task.error" class="text-danger">{{ task.error }}</span>
        <span v-else-if="task.warnings.length" class="text-warning">{{ task.warnings.join('；') }}</span>
      </div>
    </div>
    <div class="col-start-2 flex justify-end gap-1 sm:col-auto">
      <button v-if="['running', 'queued'].includes(task.status)" class="icon-btn" title="暂停" @click="act('pause')">
        <span class="i-lucide-pause" />
      </button>
      <button v-if="task.status === 'paused'" class="icon-btn" title="继续" @click="act('resume')">
        <span class="i-lucide-play" />
      </button>
      <button v-if="task.status === 'failed'" class="icon-btn" title="重试" @click="act('retry')">
        <span class="i-lucide-rotate-ccw" />
      </button>
      <button v-if="!['completed', 'canceled'].includes(task.status)" class="icon-btn" title="取消" @click="act('cancel')">
        <span class="i-lucide-circle-stop" />
      </button>
      <button v-if="['completed', 'canceled'].includes(task.status)" class="icon-btn hover:!bg-transparent hover:!text-danger" title="删除记录" @click="confirmingDelete = true">
        <span class="i-lucide-trash-2" />
      </button>
    </div>
    <ConfirmDialog
      :open="confirmingDelete"
      title="删除任务记录"
      :message="`将删除「${task.draft.title}」的记录，已下载的文件不受影响。`"
      confirm-text="删除"
      danger
      @confirm="confirmDelete"
      @cancel="confirmingDelete = false"
    />
  </article>
</template>
