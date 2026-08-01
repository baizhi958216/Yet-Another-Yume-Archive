<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { computed } from 'vue'
import { useProvidersStore } from '../../stores/providers'
import { useResolveStore } from '../../stores/resolve'
import ProviderUiFrame from '../provider/ProviderUiFrame.vue'

const resolve = useResolveStore()
const providers = useProvidersStore()
const { inspection, selected, options, sharedOptions } = storeToRefs(resolve)
const bundle = computed(() => inspection.value ? providers.uiBundles[inspection.value.provider] : null)
const context = computed(() => ({
  inspection: inspection.value,
  selected: selected.value,
  options: options.value,
  sharedOptions: sharedOptions.value,
}))
</script>

<template>
  <div v-if="inspection" class="mt-7">
    <ProviderUiFrame
      v-if="bundle"
      :provider-id="inspection.provider"
      :bundle="bundle"
      surface="resolve"
      :context="context"
      :title="`${inspection.provider} 下载选项`"
      @state="resolve.applyUiState"
    />
    <div v-else class="rounded-panel border border-danger-line bg-danger-soft p-5 text-sm text-danger">
      此 Provider 没有提供 resolve 前端组件，无法呈现下载列表。
    </div>
  </div>
</template>
