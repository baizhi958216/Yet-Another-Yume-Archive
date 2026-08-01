<script setup lang="ts">
defineProps<{ modelValue: number, min?: number, max?: number, step?: number, disabled?: boolean }>()
defineEmits<{ (event: 'update:modelValue', value: number): void }>()
</script>

<template>
  <input class="yaya-number" type="number" :value="modelValue" :min="min" :max="max" :step="step" :disabled="disabled" @input="$emit('update:modelValue', ($event.target as HTMLInputElement).valueAsNumber)">
</template>

<style scoped>
.yaya-number{height:36px;border:0;border-radius:8px;padding:0 11px;color:var(--yaya-text);background:var(--yaya-soft);font:inherit;font-size:12px}
</style>
