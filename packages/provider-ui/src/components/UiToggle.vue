<script setup lang="ts">
defineProps<{ modelValue: boolean, disabled?: boolean }>()
defineEmits<{ (event: 'update:modelValue', value: boolean): void }>()
</script>

<template>
  <label class="yaya-toggle"><input type="checkbox" :checked="modelValue" :disabled="disabled" @change="$emit('update:modelValue', ($event.target as HTMLInputElement).checked)"><span /></label>
</template>

<style scoped>
.yaya-toggle { position: relative; display: inline-block; width: 40px; height: 23px; flex: 0 0 auto; cursor: pointer; }
.yaya-toggle input { position: absolute; opacity: 0; width: 0; height: 0; }
.yaya-toggle span { position: relative; display: block; width: 40px; height: 23px; border-radius: 99px; background: var(--yaya-line); transition: background-color 0.15s ease; }
.yaya-toggle span::after { content: ''; position: absolute; top: 3px; left: 3px; width: 17px; height: 17px; border-radius: 50%; background: #ffffff; transition: left 0.15s ease; }
.yaya-toggle input:checked + span { background: var(--yaya-accent); }
.yaya-toggle input:checked + span::after { left: 20px; }
</style>
