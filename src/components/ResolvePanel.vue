<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { computed, ref, watch, watchPostEffect } from 'vue'
import { formatBytes } from '../composables/useFormat'
import { useResolveStore } from '../stores/resolve'
import FieldInput from './FieldInput.vue'
import SmartImage from './SmartImage.vue'

const resolve = useResolveStore()
const { inspection, selected, options, sharedOptions } = storeToRefs(resolve)

const descriptionEl = ref<HTMLElement>()
const descriptionExpanded = ref(false)
const descriptionOverflows = ref(false)

watch(() => inspection.value?.description, () => {
  descriptionExpanded.value = false
})

// runs after every DOM update that touches the ref, the text or the
// expanded state, so the measurement never races the (transition-delayed)
// mount of the panel
watchPostEffect(() => {
  const el = descriptionEl.value
  const expanded = descriptionExpanded.value
  descriptionOverflows.value = !!el && !expanded && el.scrollHeight > el.clientHeight + 1
})

const allSelected = computed(() => !!inspection.value && inspection.value.tasks.every(task => selected.value[task.key]))
const someSelected = computed(() => !!inspection.value && inspection.value.tasks.some(task => selected.value[task.key]))
const selectedCount = computed(() => inspection.value?.tasks.filter(task => selected.value[task.key]).length ?? 0)

function toggleAll() {
  const target = !allSelected.value
  for (const task of inspection.value?.tasks ?? [])
    selected.value[task.key] = target
}
</script>

<template>
  <div v-if="inspection" class="mt-7">
    <section v-if="inspection.provider !== 'direct'" class="grid grid-cols-1 gap-5 border-b border-line pb-6 sm:grid-cols-[180px_minmax(0,1fr)] sm:items-center">
      <SmartImage v-if="inspection.imageUrl" class="aspect-video w-full rounded-control bg-soft object-cover sm:w-[180px]" :src="inspection.imageUrl" :alt="inspection.title" :provider-id="inspection.provider" />
      <div v-else class="aspect-video w-full grid place-items-center rounded-control bg-soft text-muted sm:w-[180px]">
        <span class="i-lucide-package-open text-3xl" />
      </div>
      <div class="min-w-0">
        <span class="section-label inline-flex items-center gap-1.5"><span class="i-lucide-blocks" />{{ inspection.provider }}</span>
        <h2 class="mb-2 mt-1 truncate text-lg font-700">
          {{ inspection.title }}
        </h2>
        <p v-if="inspection.description" ref="descriptionEl" class="m-0 text-sm text-muted" :class="descriptionExpanded ? '' : 'line-clamp-2'">
          {{ inspection.description }}
        </p>
        <button
          v-if="descriptionOverflows || descriptionExpanded"
          class="mt-1 inline-flex items-center gap-1 text-xs text-muted transition-colors hover:text-ink"
          @click="descriptionExpanded = !descriptionExpanded"
        >
          {{ descriptionExpanded ? '收起' : '展开' }}
          <span :class="descriptionExpanded ? 'i-lucide-chevron-up' : 'i-lucide-chevron-down'" />
        </button>
        <div class="mt-3 flex items-center gap-1.5 text-xs text-muted">
          <span class="i-lucide-list-checks" /> {{ inspection.tasks.length }} 个任务
        </div>
      </div>
    </section>

    <section v-if="inspection.fields.length" class="grid gap-3.5 border-b border-line py-4">
      <div v-if="inspection.fields.some(value => value.type !== 'toggle')" class="flex flex-wrap items-end gap-4">
        <label v-for="field in inspection.fields.filter(value => value.type !== 'toggle')" :key="field.key" class="grid min-w-[150px] gap-1 text-2xs text-muted">
          <span>{{ field.label }}</span>
          <FieldInput :field="field" :model-value="sharedOptions[field.key]" @update:model-value="sharedOptions[field.key] = $event" />
        </label>
      </div>
      <div v-if="inspection.fields.some(value => value.type === 'toggle')" class="flex flex-wrap items-center gap-x-5 gap-y-2">
        <label v-for="field in inspection.fields.filter(value => value.type === 'toggle')" :key="field.key" class="inline-flex cursor-pointer items-center gap-2 text-xs text-muted">
          <FieldInput :field="field" :model-value="sharedOptions[field.key]" @update:model-value="sharedOptions[field.key] = $event" />
          <span>{{ field.label }}</span>
        </label>
      </div>
    </section>

    <section v-if="inspection.tasks.length > 1" class="flex items-center justify-between border-b border-line py-3">
      <label class="inline-flex cursor-pointer items-center gap-2.5 text-xs text-muted">
        <span class="relative h-5 w-5 grid place-items-center">
          <input type="checkbox" class="peer sr-only" :checked="allSelected" aria-label="全选" @change="toggleAll">
          <span class="h-[18px] w-[18px] grid place-items-center rounded-md bg-soft text-white shadow-[inset_0_0_0_1.5px_var(--line)] transition-all peer-checked:bg-accent peer-checked:shadow-none">
            <span v-show="allSelected" class="i-lucide-check text-sm" />
            <span v-show="!allSelected && someSelected" class="i-lucide-minus text-sm text-muted" />
          </span>
        </span>
        全选
      </label>
      <span class="text-xs text-muted">已选 {{ selectedCount }} / {{ inspection.tasks.length }}</span>
    </section>

    <section class="max-h-[480px] overflow-auto">
      <article v-for="(task, index) in inspection.tasks" :key="task.key" class="grid grid-cols-[24px_40px_minmax(0,1fr)] items-center gap-x-3 gap-y-4 border-b border-line py-4 lg:grid-cols-[24px_40px_minmax(220px,.9fr)_minmax(400px,1.1fr)]">
        <label class="relative h-5 w-5 grid cursor-pointer place-items-center" title="选择此下载任务">
          <input v-model="selected[task.key]" type="checkbox" class="peer sr-only" :aria-label="`选择下载 ${task.title}`">
          <span class="h-[18px] w-[18px] grid place-items-center rounded-md bg-soft text-white shadow-[inset_0_0_0_1.5px_var(--line)] transition-all peer-checked:bg-accent peer-checked:shadow-none">
            <span v-show="selected[task.key]" class="i-lucide-check text-sm" />
          </span>
        </label>
        <SmartImage v-if="task.imageUrl" class="h-10 w-10 rounded-md object-cover bg-soft" :src="task.imageUrl" alt="" :provider-id="inspection.provider" />
        <div v-else class="h-10 w-10 grid place-items-center rounded-md bg-soft text-2xs text-muted">
          {{ index + 1 }}
        </div>
        <div class="min-w-0">
          <strong class="block truncate text-sm font-600">{{ task.title }}</strong>
          <span v-if="task.description || task.size != null" class="mt-1.5 block truncate text-2xs text-muted">
            {{ [task.description, task.size != null ? formatBytes(task.size) : ''].filter(Boolean).join(' · ') }}
          </span>
        </div>
        <div v-if="task.fields.length" class="col-span-3 grid grid-cols-1 gap-3 rounded-control bg-soft/55 p-3 sm:grid-cols-2 lg:col-span-1 lg:col-start-4">
          <label v-for="field in task.fields.filter(value => value.type !== 'toggle')" :key="field.key" class="grid min-w-0 gap-1.5 text-2xs text-muted">
            <span>{{ field.label }}</span>
            <FieldInput :field="field" :model-value="options[task.key]?.[field.key]" :disabled="!selected[task.key]" @update:model-value="options[task.key][field.key] = $event" />
          </label>
          <div v-if="task.fields.some(value => value.type === 'toggle')" class="col-span-full flex flex-wrap items-center gap-x-5 gap-y-2 border-t border-line pt-3">
            <label v-for="field in task.fields.filter(value => value.type === 'toggle')" :key="field.key" class="inline-flex cursor-pointer items-center gap-2 text-xs text-muted">
              <FieldInput :field="field" :model-value="options[task.key]?.[field.key]" :disabled="!selected[task.key]" @update:model-value="options[task.key][field.key] = $event" />
              <span>{{ field.label }}</span>
            </label>
          </div>
        </div>
      </article>
    </section>
  </div>
</template>
