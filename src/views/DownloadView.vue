<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import ResolvePanel from '../components/task/ResolvePanel.vue'
import { isDesktop } from '../services/transport'
import { useProvidersStore } from '../stores/providers'
import { useResolveStore } from '../stores/resolve'
import { useUiStore } from '../stores/ui'

const resolve = useResolveStore()
const providers = useProvidersStore()
const ui = useUiStore()
const { source, outputDir, inspection, resolving, creating } = storeToRefs(resolve)
const desktop = isDesktop()

async function doResolve() {
  ui.clearError()
  try {
    await resolve.resolve()
  }
  catch (error) {
    ui.showError(error)
  }
}

async function doCreate() {
  ui.clearError()
  try {
    await resolve.create()
    ui.view = 'tasks'
  }
  catch (error) {
    ui.showError(error)
  }
}

async function chooseFolder() {
  if (!desktop)
    return
  const chosen = await open({ directory: true, multiple: false })
  if (typeof chosen === 'string')
    outputDir.value = chosen
}

function resetSearch() {
  ui.clearError()
  resolve.reset()
}
</script>

<template>
  <section>
    <div
      class="grid items-center gap-3 rounded-panel bg-elevated p-2 shadow-raised transition-all duration-200 focus-within:ring-3 focus-within:ring-accent-ring"
      :class="inspection ? 'grid-cols-[40px_minmax(0,1fr)_auto] pl-2' : 'grid-cols-[20px_minmax(0,1fr)_auto] pl-5'"
    >
      <button
        v-if="inspection"
        type="button"
        class="h-10 w-10 grid place-items-center rounded-control text-muted transition-colors hover:bg-soft hover:text-ink active:scale-95"
        title="清空搜索结果"
        aria-label="清空搜索结果"
        @click="resetSearch"
      >
        <span class="i-lucide-arrow-left text-xl" />
      </button>
      <span v-else class="i-lucide-link-2 text-lg text-muted" />
      <input v-model="source" class="h-10 min-w-0 border-0 bg-transparent text-sm text-ink outline-none placeholder:text-muted" placeholder="输入要下载的地址或标识" @keyup.enter="doResolve">
      <button class="primary-btn min-w-[88px]" :disabled="resolving || !source" @click="doResolve">
        <span v-if="resolving" class="i-lucide-loader-circle animate-spin" />
        <span v-else class="i-lucide-sparkles" />
        {{ resolving ? '检查中' : '继续' }}
      </button>
    </div>

    <Transition name="content" mode="out-in">
      <div v-if="!inspection" class="min-h-[420px] grid place-items-center px-6 py-20 text-center">
        <div>
          <div class="mx-auto h-16 w-16 grid place-items-center rounded-panel bg-accent-soft text-accent">
            <span class="i-lucide-cloud-download text-3xl" />
          </div>
          <h2 class="mb-2 mt-5 text-base font-600">
            还没有添加下载
          </h2>
          <p class="m-0 text-xs text-muted">
            娅娅会识别输入并提供对应的任务选项
          </p>
          <div class="mt-3 flex flex-wrap items-center justify-center gap-2">
            <span v-for="provider in providers.providers.filter(value => value.enabled)" :key="provider.id" class="tag">
              <span class="i-lucide-blocks" />{{ provider.name }}
            </span>
          </div>
        </div>
      </div>
      <div v-else>
        <ResolvePanel />
        <div class="mt-1 flex flex-col gap-3 border-t border-line pt-5 sm:flex-row sm:items-center sm:justify-between">
          <button v-if="desktop" class="secondary-btn min-w-0" @click="chooseFolder">
            <span class="i-lucide-folder-open text-lg" />
            <span class="max-w-[460px] truncate">{{ outputDir || '默认保存目录' }}</span>
          </button>
          <button class="primary-btn min-w-[160px]" :disabled="creating" @click="doCreate">
            <span :class="creating ? 'i-lucide-loader-circle animate-spin' : 'i-lucide-download'" />
            {{ creating ? '正在加入队列' : (desktop ? '加入下载队列' : '下载') }}
          </button>
        </div>
      </div>
    </Transition>
  </section>
</template>
