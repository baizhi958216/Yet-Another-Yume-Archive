<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import ThemePanel from '../components/layout/ThemePanel.vue'
import FolderPicker from '../components/ui/FolderPicker.vue'
import NumberStepper from '../components/ui/NumberStepper.vue'
import { isDesktop } from '../services/transport'
import { useSettingsStore } from '../stores/settings'
import { useUiStore } from '../stores/ui'

const store = useSettingsStore()
const ui = useUiStore()
const { settings } = storeToRefs(store)
const saved = ref(false)
const desktop = isDesktop()

async function save() {
  try {
    await store.save()
    saved.value = true
    window.setTimeout(() => saved.value = false, 1800)
  }
  catch (error) {
    ui.showError(error)
  }
}
</script>

<template>
  <section>
    <div class="divide-y divide-line">
      <div v-if="desktop" class="flex flex-col gap-3 py-5 sm:flex-row sm:items-center sm:justify-between sm:gap-8">
        <div class="min-w-0">
          <h2 class="m-0 text-sm font-600">
            默认保存目录
          </h2>
          <p class="mb-0 mt-1 text-xs text-muted">
            新任务未选择目录时使用
          </p>
        </div>
        <div class="w-full sm:w-[400px] sm:shrink-0">
          <FolderPicker v-model="settings.defaultOutputDir" />
        </div>
      </div>

      <div class="flex items-center justify-between gap-8 py-5">
        <div class="min-w-0">
          <h2 class="m-0 text-sm font-600">
            任务并发
          </h2>
          <p class="mb-0 mt-1 text-xs text-muted">
            同时运行的下载任务数量
          </p>
        </div>
        <NumberStepper v-model="settings.maxActiveTasks" :min="1" :max="10" />
      </div>

      <!-- on mobile the header (and its palette popover) is hidden, so the
           theme controls live here instead -->
      <div class="py-5 md:hidden">
        <h2 class="m-0 text-sm font-600">
          外观
        </h2>
        <p class="mb-3 mt-1 text-xs text-muted">
          显示模式与主题色，自动保存
        </p>
        <ThemePanel />
      </div>
    </div>

    <div class="flex justify-end pt-6">
      <button class="primary-btn min-w-[128px]" @click="save">
        <span :class="saved ? 'i-lucide-check' : 'i-lucide-save'" /> {{ saved ? '已保存' : '保存设置' }}
      </button>
    </div>
  </section>
</template>
