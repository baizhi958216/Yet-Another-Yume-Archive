<script setup lang="ts">
defineProps<{ modelValue: boolean, disabled?: boolean }>()
defineEmits<{ (event: 'update:modelValue', value: boolean): void }>()
</script>

<template>
  <label class="yaya-toggle"><input type="checkbox" :checked="modelValue" :disabled="disabled" @change="$emit('update:modelValue', ($event.target as HTMLInputElement).checked)"><span /></label>
</template>

<style scoped>
.yaya-toggle{display:inline-block;width:40px;height:23px}.yaya-toggle input{position:absolute;opacity:0}.yaya-toggle span{display:block;width:40px;height:23px;border-radius:99px;background:var(--yaya-line);transition:.15s}.yaya-toggle span:after{content:'';display:block;width:17px;height:17px;margin:3px;border-radius:50%;background:#fff;transition:.15s}.yaya-toggle input:checked+span{background:var(--yaya-accent)}.yaya-toggle input:checked+span:after{transform:translateX(17px)}
</style>
