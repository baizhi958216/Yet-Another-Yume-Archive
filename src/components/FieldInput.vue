<script setup lang="ts">
// Blind renderer for one provider-declared form field.
import type { FormField } from '../types'
import CustomSelect from './CustomSelect.vue'

const props = defineProps<{
  field: FormField
  modelValue: unknown
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: unknown]
}>()

function onInput(event: Event) {
  const element = event.target as HTMLInputElement
  if (props.field.type === 'toggle')
    emit('update:modelValue', element.checked)
  else if (props.field.type === 'number')
    emit('update:modelValue', element.valueAsNumber)
  else
    emit('update:modelValue', element.value)
}
</script>

<template>
  <input
    v-if="field.type === 'toggle'"
    type="checkbox"
    class="h-4 w-4 accent-[var(--accent)]"
    :checked="Boolean(modelValue)"
    :disabled="disabled"
    @change="onInput"
  >
  <CustomSelect
    v-else-if="field.type === 'select'"
    :options="field.options"
    :model-value="modelValue"
    :disabled="disabled"
    @update:model-value="emit('update:modelValue', $event)"
  />
  <input
    v-else
    :type="field.type === 'secret' ? 'password' : field.type"
    class="field h-9 text-xs"
    :disabled="disabled"
    :value="modelValue as string | number"
    :placeholder="field.type === 'text' || field.type === 'secret' ? field.placeholder : undefined"
    :min="field.type === 'number' ? field.min : undefined"
    :max="field.type === 'number' ? field.max : undefined"
    :step="field.type === 'number' ? field.step : undefined"
    @input="onInput"
  >
</template>
