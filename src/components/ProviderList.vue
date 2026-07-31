<script setup lang="ts">
import type { ProviderInfo } from '../types'

defineProps<{
  providers: ProviderInfo[]
  changing: string[]
}>()

const emit = defineEmits<{
  settings: [provider: ProviderInfo]
  toggle: [provider: ProviderInfo, event: Event]
}>()
</script>

<template>
  <section>
    <div v-if="!providers.length" class="min-h-[360px] grid place-items-center border-y border-line py-16 text-center">
      <div>
        <span class="i-lucide-package-open text-3xl text-muted" />
        <h2 class="mb-1 mt-4 text-base font-700">
          没有发现插件
        </h2>
        <p class="m-0 text-xs text-muted">
          Provider 目录中暂无可用项目
        </p>
      </div>
    </div>

    <article v-for="provider in providers" :key="provider.id" class="border-b border-line py-5 first:border-t">
      <div class="grid grid-cols-[42px_minmax(0,1fr)_auto] items-center gap-4">
        <div class="h-10 w-10 grid place-items-center rounded-control bg-soft text-accent">
          <span class="i-lucide-blocks text-xl" />
        </div>
        <div class="min-w-0">
          <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
            <h2 class="m-0 text-sm font-700">
              {{ provider.name }}
            </h2>
            <span class="text-2xs text-muted">v{{ provider.version || '0.0.0' }}</span>
            <span class="text-2xs font-600" :class="provider.enabled ? 'text-success' : 'text-muted'">{{ provider.enabled ? '已启用' : '已停用' }}</span>
          </div>
          <p class="mb-0 mt-1 max-w-[720px] text-xs leading-5 text-muted">
            {{ provider.description }}
          </p>
        </div>
        <div class="flex items-center gap-2">
          <button v-if="provider.capabilities.authentication || provider.capabilities.settings" class="icon-btn" :disabled="!provider.enabled" :title="`${provider.name} 设置`" @click="emit('settings', provider)">
            <span class="i-lucide-settings-2" />
          </button>
          <label class="relative h-6 w-11 shrink-0 cursor-pointer" :title="provider.enabled ? '停用插件' : '启用插件'">
            <input class="peer sr-only" type="checkbox" :checked="provider.enabled" :disabled="changing.includes(provider.id)" :aria-label="`${provider.name} 启用状态`" @change="emit('toggle', provider, $event)">
            <span class="absolute inset-0 rounded-full bg-line transition-colors peer-checked:bg-accent peer-disabled:opacity-50" />
            <span class="absolute left-1 top-1 h-4 w-4 rounded-full bg-white shadow-sm transition-transform peer-checked:translate-x-5" />
          </label>
        </div>
      </div>
    </article>
  </section>
</template>
