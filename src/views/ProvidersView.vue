<script setup lang="ts">
import type { ProviderInfo } from '../types'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import ProviderList from '../components/provider/ProviderList.vue'
import ProviderManagementContent from '../components/provider/ProviderManagementContent.vue'
import { useProvidersStore } from '../stores/providers'
import { useUiStore } from '../stores/ui'

const providers = useProvidersStore()
const ui = useUiStore()
const managementProvider = ref<ProviderInfo | null>(null)
const changing = ref<string[]>([])
const mobileTransition = ref('mobile-forward')

async function toggle(provider: ProviderInfo, event: Event) {
  const enabled = (event.target as HTMLInputElement).checked
  changing.value.push(provider.id)
  try {
    await providers.setEnabled(provider.id, enabled)
    if (!enabled && managementProvider.value?.id === provider.id)
      managementProvider.value = null
  }
  catch (error) {
    ;(event.target as HTMLInputElement).checked = provider.enabled
    ui.showError(error)
  }
  finally {
    changing.value = changing.value.filter(id => id !== provider.id)
  }
}

async function openManagement(provider: ProviderInfo) {
  mobileTransition.value = 'mobile-forward'
  managementProvider.value = provider
  await providers.loadUiBundle(provider.id)
}

function closeManagement() {
  mobileTransition.value = 'mobile-back'
  managementProvider.value = null
}

function onKeydown(event: KeyboardEvent) {
  if (managementProvider.value && event.key === 'Escape')
    closeManagement()
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <div class="relative">
    <Transition :name="mobileTransition">
      <section v-if="managementProvider" key="management" class="md:hidden">
        <button class="mb-6 inline-flex items-center gap-2 border-0 bg-transparent p-0 text-sm text-muted cursor-pointer" @click="closeManagement">
          <span class="i-lucide-arrow-left text-lg" />返回插件
        </button>
        <h2 class="mb-6 mt-0 text-xl font-700">
          {{ managementProvider.name }} 管理
        </h2>
        <ProviderManagementContent :provider="managementProvider" :bundle="providers.uiBundles[managementProvider.id]" />
      </section>
      <ProviderList v-else key="list" class="md:hidden" :providers="providers.providers" :changing="changing" @manage="openManagement" @toggle="toggle" />
    </Transition>

    <ProviderList class="hidden md:block" :providers="providers.providers" :changing="changing" @manage="openManagement" @toggle="toggle" />

    <Teleport to="body">
      <Transition name="modal">
        <div v-if="managementProvider" class="fixed inset-0 z-50 hidden place-items-center bg-overlay p-4 backdrop-blur-sm md:grid" @click.self="closeManagement">
          <section class="modal-panel relative flex h-auto max-h-[85vh] w-full max-w-[560px] flex-col overflow-hidden p-6 md:p-7" role="dialog" aria-modal="true" :aria-label="`${managementProvider.name} 管理`">
            <button class="icon-btn absolute right-4 top-4 z-10" title="关闭管理界面" @click="closeManagement">
              <span class="i-lucide-x text-lg" />
            </button>
            <h2 class="mb-4 mt-0 shrink-0 pr-10 text-xl font-700">
              {{ managementProvider.name }} 管理
            </h2>
            <div class="min-h-0 shrink overflow-y-auto pr-1">
              <ProviderManagementContent :provider="managementProvider" :bundle="providers.uiBundles[managementProvider.id]" />
            </div>
          </section>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>
