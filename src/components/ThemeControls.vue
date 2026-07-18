<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'
import ThemePanel from './ThemePanel.vue'

const open = ref(false)
const root = ref<HTMLElement>()

function closeOutside(event: PointerEvent) {
  if (open.value && !root.value?.contains(event.target as Node))
    open.value = false
}

window.addEventListener('pointerdown', closeOutside)
onBeforeUnmount(() => window.removeEventListener('pointerdown', closeOutside))
</script>

<template>
  <div ref="root" class="relative">
    <button class="icon-btn" title="外观设置" aria-label="外观设置" :aria-expanded="open" @click="open = !open">
      <span class="i-lucide-palette text-[18px]" />
    </button>
    <Transition name="popover">
      <div v-if="open" class="popover-panel absolute right-0 top-12 z-30 w-[276px] p-4">
        <div class="mb-4 flex items-center justify-between">
          <strong class="text-sm">外观</strong>
          <span class="text-2xs text-muted">自动保存</span>
        </div>
        <ThemePanel />
      </div>
    </Transition>
  </div>
</template>
