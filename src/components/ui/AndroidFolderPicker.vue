<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import { useUiStore } from '../../stores/ui'

interface DownloadDirectory {
  uri?: string
  name: string
}

const ui = useUiStore()
const directory = ref<DownloadDirectory>({ name: 'Download/YAYA（系统默认）' })
const choosing = ref(false)

onMounted(async () => {
  try {
    directory.value = await invoke<DownloadDirectory>('get_android_download_directory')
  }
  catch (error) {
    ui.showError(error)
  }
})

async function choose() {
  choosing.value = true
  try {
    directory.value = await invoke<DownloadDirectory>('pick_android_download_directory')
  }
  catch (error) {
    // Closing Android's system picker is a cancellation, not an app error.
    if (!String(error).includes('未选择下载文件夹'))
      ui.showError(error)
  }
  finally {
    choosing.value = false
  }
}
</script>

<template>
  <div class="h-11 w-full flex items-center gap-2.5 rounded-control bg-soft p-1.5 pl-2" :title="directory.name">
    <span class="h-8 w-8 shrink-0 grid place-items-center rounded-md bg-surface text-accent">
      <span class="i-lucide-folder-heart text-base" />
    </span>
    <span class="min-w-0 flex-1 truncate text-xs text-ink">
      {{ directory.name }}
    </span>
    <button
      type="button"
      class="h-8 shrink-0 inline-flex items-center rounded-md bg-surface px-3 text-xs text-accent font-600 transition-colors duration-150 hover:bg-accent hover:text-white disabled:opacity-60"
      :disabled="choosing"
      @click="choose"
    >
      {{ choosing ? '选择中…' : '更改' }}
    </button>
  </div>
</template>
