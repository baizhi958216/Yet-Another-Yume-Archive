<script setup lang="ts">
withDefaults(defineProps<{ variant?: 'primary' | 'secondary' | 'danger', disabled?: boolean }>(), { variant: 'primary', disabled: false })
</script>

<template>
  <button :class="`yaya-button yaya-button--${variant}`" :disabled="disabled">
    <slot />
  </button>
</template>

<style scoped>
.yaya-button{height:36px;border:0;border-radius:8px;padding:0 14px;font:inherit;font-size:12px;font-weight:700;cursor:pointer}.yaya-button--primary{color:#fff;background:var(--yaya-accent)}.yaya-button--secondary{color:var(--yaya-muted);background:var(--yaya-soft)}.yaya-button--danger{color:#fff;background:var(--yaya-danger)}.yaya-button:disabled{opacity:.5;cursor:not-allowed}
</style>
