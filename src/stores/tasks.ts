import type { TaskSnapshot } from '../types'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import * as tasksApi from '../services/api/tasks'

export const useTasksStore = defineStore('tasks', () => {
  const tasks = ref<TaskSnapshot[]>([])

  const active = computed(() => tasks.value.filter(task => task.status !== 'completed'))
  const completed = computed(() => tasks.value.filter(task => task.status === 'completed'))

  async function refresh() {
    tasks.value = await tasksApi.listTasks()
  }

  /** Events and freshly created tasks funnel through here. */
  function upsert(task: TaskSnapshot) {
    const index = tasks.value.findIndex(value => value.id === task.id)
    if (index < 0)
      tasks.value.unshift(task)
    else
      tasks.value[index] = task
  }

  async function act(action: 'pause' | 'resume' | 'retry' | 'cancel' | 'delete', id: string) {
    await ({
      pause: tasksApi.pauseTask,
      resume: tasksApi.resumeTask,
      retry: tasksApi.retryTask,
      cancel: tasksApi.cancelTask,
      delete: tasksApi.deleteTask,
    })[action](id)
    if (action === 'delete')
      tasks.value = tasks.value.filter(task => task.id !== id)
  }

  return { tasks, active, completed, refresh, upsert, act }
})
