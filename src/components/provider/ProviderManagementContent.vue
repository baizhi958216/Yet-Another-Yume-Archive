<script setup lang="ts">
import type { ProviderInfo, ProviderUiBundle } from '../../types'
import ProviderUiFrame from './ProviderUiFrame.vue'

defineProps<{ provider: ProviderInfo, bundle: ProviderUiBundle | null | undefined }>()
</script>

<template>
  <div>
    <div class="flex items-start gap-4 border-b border-line pb-5">
      <div class="h-11 w-11 shrink-0 grid place-items-center rounded-control bg-soft text-accent">
        <span class="i-lucide-blocks text-xl" />
      </div>
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
          <h3 class="m-0 text-base font-700">
            {{ provider.name }}
          </h3><span class="text-2xs text-muted">v{{ provider.version || '0.0.0' }}</span>
        </div>
        <p class="mb-0 mt-1 text-xs leading-5 text-muted">
          {{ provider.description }}
        </p>
      </div>
    </div>
    <ProviderUiFrame v-if="bundle" class="pt-5" :provider-id="provider.id" :bundle="bundle" surface="management" :context="{ provider }" :title="`${provider.name} 管理界面`" />
    <p v-else class="m-0 py-5 text-xs text-muted">
      正在加载 Provider 组件…
    </p>
  </div>
</template>
