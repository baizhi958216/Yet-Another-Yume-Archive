<script setup lang="ts">
// Cute rounded stepper: [-] value [+], replaces the native number input.
const props = withDefaults(defineProps<{
  modelValue: number
  min?: number
  max?: number
  suffix?: string
}>(), { min: 1, max: 99, suffix: '' })

const emit = defineEmits<{
  'update:modelValue': [value: number]
}>()

function step(delta: number) {
  const next = Math.min(props.max, Math.max(props.min, (props.modelValue || props.min) + delta))
  emit('update:modelValue', next)
}
</script>

<template>
  <div class="inline-flex items-center gap-1 rounded-control bg-soft p-1">
    <button
      type="button"
      class="h-7 w-7 grid place-items-center rounded-md bg-surface text-muted shadow-raised transition-all duration-150 disabled:cursor-not-allowed hover:text-accent disabled:opacity-35 active:scale-95 disabled:hover:text-muted"
      :disabled="modelValue <= min"
      aria-label="减少"
      @click="step(-1)"
    >
      <span class="i-lucide-minus text-sm" />
    </button>
    <span class="min-w-9 text-center text-sm text-ink font-700 tabular-nums">
      {{ modelValue }}<span v-if="suffix" class="ml-0.5 text-2xs text-muted font-500">{{ suffix }}</span>
    </span>
    <button
      type="button"
      class="h-7 w-7 grid place-items-center rounded-md bg-surface text-muted shadow-raised transition-all duration-150 disabled:cursor-not-allowed hover:text-accent disabled:opacity-35 active:scale-95 disabled:hover:text-muted"
      :disabled="modelValue >= max"
      aria-label="增加"
      @click="step(1)"
    >
      <span class="i-lucide-plus text-sm" />
    </button>
  </div>
</template>
