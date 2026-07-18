<script setup lang="ts">
import type { SelectOption } from '../types'
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'

const props = defineProps<{
  options: SelectOption[]
  modelValue: unknown
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: unknown]
}>()

const root = ref<HTMLElement>()
const menu = ref<HTMLElement>()
const optionElements = ref<HTMLButtonElement[]>([])
const open = ref(false)
const activeIndex = ref(0)
const menuStyle = ref<Record<string, string>>({})

const selectedIndex = computed(() =>
  props.options.findIndex(option => key(option.value) === key(props.modelValue)),
)
const selected = computed(() => props.options[selectedIndex.value])

function key(value: unknown) {
  return JSON.stringify(value)
}

function toggle() {
  if (props.disabled)
    return
  open.value ? close() : show()
}

function show() {
  open.value = true
  activeIndex.value = Math.max(0, selectedIndex.value)
  nextTick(() => {
    updatePosition()
    optionElements.value[activeIndex.value]?.scrollIntoView({ block: 'nearest' })
  })
}

function close() {
  open.value = false
}

function choose(index: number) {
  emit('update:modelValue', props.options[index]?.value)
  close()
  root.value?.querySelector<HTMLButtonElement>('[aria-haspopup="listbox"]')?.focus()
}

function move(offset: number) {
  if (!open.value) {
    show()
    return
  }
  activeIndex.value = (activeIndex.value + offset + props.options.length) % props.options.length
  nextTick(() => optionElements.value[activeIndex.value]?.scrollIntoView({ block: 'nearest' }))
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
    event.preventDefault()
    move(event.key === 'ArrowDown' ? 1 : -1)
  }
  else if (event.key === 'Enter' || event.key === ' ') {
    event.preventDefault()
    open.value ? choose(activeIndex.value) : show()
  }
  else if (event.key === 'Escape') {
    close()
  }
}

function onPointerDown(event: PointerEvent) {
  const target = event.target as Node
  if (!root.value?.contains(target) && !menu.value?.contains(target))
    close()
}

function updatePosition() {
  const rect = root.value?.getBoundingClientRect()
  if (!rect)
    return
  const gap = 6
  const viewportGap = 12
  const estimatedHeight = 220
  const width = Math.min(rect.width, window.innerWidth - viewportGap * 2)
  const left = Math.min(Math.max(rect.left, viewportGap), window.innerWidth - width - viewportGap)
  const openUpward = window.innerHeight - rect.bottom < estimatedHeight && rect.top > window.innerHeight - rect.bottom
  menuStyle.value = {
    left: `${left}px`,
    width: `${width}px`,
    ...(openUpward
      ? { bottom: `${window.innerHeight - rect.top + gap}px` }
      : { top: `${rect.bottom + gap}px` }),
  }
}

function onViewportScroll(event: Event) {
  const target = event.target
  if (target instanceof Node && menu.value?.contains(target))
    return
  if (open.value)
    updatePosition()
}

function onViewportResize() {
  if (open.value)
    updatePosition()
}

onMounted(() => {
  document.addEventListener('pointerdown', onPointerDown)
  window.addEventListener('resize', onViewportResize)
  window.addEventListener('scroll', onViewportScroll, true)
})
onBeforeUnmount(() => {
  document.removeEventListener('pointerdown', onPointerDown)
  window.removeEventListener('resize', onViewportResize)
  window.removeEventListener('scroll', onViewportScroll, true)
})
</script>

<template>
  <div ref="root" class="relative min-w-0">
    <button
      type="button"
      class="ring-focus h-9 w-full flex items-center justify-between gap-2 rounded-control bg-soft px-3 text-left text-xs text-ink transition-all duration-200 hover:bg-accent-soft disabled:cursor-not-allowed disabled:opacity-50"
      :disabled="disabled"
      aria-haspopup="listbox"
      :aria-expanded="open"
      @click="toggle"
      @keydown="onKeydown"
    >
      <span class="min-w-0 truncate">{{ selected?.label || '请选择' }}</span>
      <span class="i-lucide-chevron-down shrink-0 text-sm text-muted transition-transform" :class="{ 'rotate-180': open }" />
    </button>

    <Teleport to="body">
      <Transition name="dropdown">
        <div v-if="open" ref="menu" class="popover-panel fixed z-100 overflow-hidden p-1.5" :style="menuStyle" role="listbox">
          <div class="max-h-[208px] overflow-y-auto overscroll-contain">
            <button
              v-for="(option, index) in options"
              :key="key(option.value)"
              :ref="element => optionElements[index] = element as HTMLButtonElement"
              type="button"
              class="min-h-9 w-full flex items-center gap-2 rounded-control px-2.5 py-2 text-left text-xs transition-colors"
              :class="index === selectedIndex ? 'bg-accent-soft text-accent font-600' : index === activeIndex ? 'bg-soft text-ink' : 'text-muted hover:bg-soft hover:text-ink'"
              role="option"
              :aria-selected="index === selectedIndex"
              @mouseenter="activeIndex = index"
              @click="choose(index)"
            >
              <span class="h-4 w-4 shrink-0 grid place-items-center">
                <span v-if="index === selectedIndex" class="i-lucide-check text-sm" />
              </span>
              <span class="min-w-0 truncate" :title="option.label">{{ option.label }}</span>
            </button>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 120ms ease, transform 120ms ease;
  transform-origin: top;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px) scale(.98);
}
</style>
