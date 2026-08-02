<script setup lang="ts">
// Generic confirm dialog; works on both desktop and web hosts.
import { onBeforeUnmount, onMounted } from 'vue'

const props = withDefaults(defineProps<{
  open: boolean
  title: string
  message?: string
  confirmText?: string
  danger?: boolean
}>(), { confirmText: '确定', danger: false })

const emit = defineEmits<{ confirm: [], cancel: [] }>()

function onKeydown(event: KeyboardEvent) {
  if (props.open && event.key === 'Escape')
    emit('cancel')
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="open" class="overlay z-50" @click.self="emit('cancel')">
        <section class="modal-panel max-w-[340px] p-6" role="dialog" aria-modal="true" :aria-label="title">
          <h2 class="m-0 text-base font-600">
            {{ title }}
          </h2>
          <p v-if="message" class="mb-0 mt-2 text-sm text-muted [overflow-wrap:anywhere]">
            {{ message }}
          </p>
          <div class="mt-6 flex justify-end gap-2">
            <button class="secondary-btn" @click="emit('cancel')">
              取消
            </button>
            <button class="primary-btn" :class="danger ? '!bg-danger hover:!bg-danger-hover' : ''" @click="emit('confirm')">
              {{ confirmText }}
            </button>
          </div>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>
