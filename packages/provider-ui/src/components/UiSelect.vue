<script setup lang="ts">
defineProps<{ modelValue: unknown, options: Array<{ label: string, value: unknown }>, disabled?: boolean }>()
defineEmits<{ (event: 'update:modelValue', value: unknown): void }>()
</script>

<template>
  <select class="yaya-select" :value="modelValue as any" :disabled="disabled" @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)">
    <option v-for="option in options" :key="String(option.value)" :value="option.value as any">
      {{ option.label }}
    </option>
  </select>
</template>

<style scoped>
.yaya-select{height:36px;border:0;border-radius:8px;padding:0 30px 0 11px;color:var(--yaya-text);background:var(--yaya-soft);font:inherit;font-size:12px}
</style>
