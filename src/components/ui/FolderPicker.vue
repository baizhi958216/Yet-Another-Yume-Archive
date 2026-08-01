<script setup lang="ts">
// Static path bar; only the embedded "更改" button is interactive.
import { open } from '@tauri-apps/plugin-dialog'

const props = withDefaults(defineProps<{
  modelValue: string
  placeholder?: string
}>(), { placeholder: '选择目录' })

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

async function choose() {
  const chosen = await open({
    directory: true,
    multiple: false,
    defaultPath: props.modelValue || undefined,
  })
  if (typeof chosen === 'string')
    emit('update:modelValue', chosen)
}
</script>

<template>
  <div class="h-11 w-full flex items-center gap-2.5 rounded-control bg-soft p-1.5 pl-2" :title="modelValue || placeholder">
    <span class="h-8 w-8 shrink-0 grid place-items-center rounded-md bg-surface text-accent">
      <span class="i-lucide-folder-heart text-base" />
    </span>
    <span class="min-w-0 flex-1 truncate text-xs" :class="modelValue ? 'text-ink' : 'text-muted'">
      {{ modelValue || placeholder }}
    </span>
    <button
      type="button"
      class="h-8 shrink-0 inline-flex items-center rounded-md bg-surface px-3 text-xs text-accent font-600 transition-colors duration-150 hover:bg-accent hover:text-white"
      @click="choose"
    >
      更改
    </button>
  </div>
</template>
