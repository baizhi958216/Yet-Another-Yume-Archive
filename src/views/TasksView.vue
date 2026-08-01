<script setup lang="ts">
import type { TaskSnapshot } from '../types'
import { computed, ref } from 'vue'
import TaskGroup from '../components/task/TaskGroup.vue'
import TaskRow from '../components/task/TaskRow.vue'
import { useTasksStore } from '../stores/tasks'

const tasks = useTasksStore()
const filter = ref<'active' | 'completed'>('active')
const visible = computed(() => filter.value === 'completed' ? tasks.completed : tasks.active)

/** Batch tasks collapse into one group row; standalone tasks stay flat. */
const rows = computed(() => {
  const groups = new Map<string, TaskSnapshot[]>()
  const out: Array<{ key: string, task?: TaskSnapshot, group?: TaskSnapshot[] }> = []
  for (const task of visible.value) {
    if (!task.batchId) {
      out.push({ key: task.id, task })
      continue
    }
    let members = groups.get(task.batchId)
    if (!members) {
      members = []
      groups.set(task.batchId, members)
      out.push({ key: task.batchId, group: members })
    }
    members.push(task)
  }
  return out
})
</script>

<template>
  <section>
    <div class="inline-flex rounded-control bg-soft p-1">
      <button class="h-9 rounded-md px-5 text-sm transition-colors duration-150" :class="filter === 'active' ? 'bg-surface text-ink font-600' : 'text-muted hover:text-ink'" @click="filter = 'active'">
        进行中
      </button>
      <button class="h-9 rounded-md px-5 text-sm transition-colors duration-150" :class="filter === 'completed' ? 'bg-surface text-ink font-600' : 'text-muted hover:text-ink'" @click="filter = 'completed'">
        已完成
      </button>
    </div>
    <Transition name="content" mode="out-in">
      <div v-if="visible.length" :key="filter" class="mt-4 divide-y divide-line">
        <template v-for="row in rows" :key="row.key">
          <TaskGroup v-if="row.group" :tasks="row.group" />
          <TaskRow v-else-if="row.task" :task="row.task" />
        </template>
      </div>
      <div v-else :key="`${filter}-empty`" class="min-h-[400px] grid place-items-center text-center">
        <div>
          <span class="i-lucide-list-todo text-4xl text-muted opacity-60" />
          <h2 class="mb-2 mt-4 text-base font-600">
            {{ filter === 'active' ? '队列为空' : '暂无完成记录' }}
          </h2>
          <p class="m-0 text-sm text-muted">
            新任务会在这里显示实时状态。
          </p>
        </div>
      </div>
    </Transition>
  </section>
</template>
