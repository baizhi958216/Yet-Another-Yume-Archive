<script setup lang="ts">
import type { ProviderInfo } from '../types'
import { ref } from 'vue'
import LoginModal from '../components/LoginModal.vue'
import ProviderList from '../components/ProviderList.vue'
import ProviderSettingsContent from '../components/ProviderSettingsContent.vue'
import { useProvidersStore } from '../stores/providers'
import { useUiStore } from '../stores/ui'

const providers = useProvidersStore()
const ui = useUiStore()
const settingsProvider = ref<ProviderInfo | null>(null)
const authenticating = ref<ProviderInfo | null>(null)
const changing = ref<string[]>([])
const mobileTransition = ref('mobile-forward')

async function toggle(provider: ProviderInfo, event: Event) {
  const enabled = (event.target as HTMLInputElement).checked
  changing.value.push(provider.id)
  try {
    await providers.setEnabled(provider.id, enabled)
    if (!enabled && settingsProvider.value?.id === provider.id)
      settingsProvider.value = null
  }
  catch (error) {
    ;(event.target as HTMLInputElement).checked = provider.enabled
    ui.showError(error)
  }
  finally {
    changing.value = changing.value.filter(id => id !== provider.id)
  }
}

async function openSettings(provider: ProviderInfo) {
  mobileTransition.value = 'mobile-forward'
  settingsProvider.value = provider
  if (provider.capabilities.authentication)
    await providers.refreshAuth(provider.id)
}

function closeSettings() {
  mobileTransition.value = 'mobile-back'
  settingsProvider.value = null
}

async function authenticated(provider: ProviderInfo) {
  await providers.refreshAuth(provider.id)
  authenticating.value = null
}
</script>

<template>
  <div class="relative">
    <Transition :name="mobileTransition">
      <section v-if="settingsProvider" key="settings" class="md:hidden">
        <button class="mb-6 inline-flex items-center gap-2 border-0 bg-transparent p-0 text-sm text-muted cursor-pointer" @click="closeSettings">
          <span class="i-lucide-arrow-left text-lg" />返回插件
        </button>
        <h2 class="mb-6 mt-0 text-xl font-700">
          {{ settingsProvider.name }} 设置
        </h2>
        <ProviderSettingsContent
          :provider="settingsProvider"
          :auth="providers.auth[settingsProvider.id]"
          @login="authenticating = settingsProvider"
          @logout="providers.logout(settingsProvider.id)"
        />
      </section>
      <ProviderList
        v-else
        key="list"
        class="md:hidden"
        :providers="providers.providers"
        :changing="changing"
        @settings="openSettings"
        @toggle="toggle"
      />
    </Transition>

    <ProviderList
      class="hidden md:block"
      :providers="providers.providers"
      :changing="changing"
      @settings="openSettings"
      @toggle="toggle"
    />

    <Transition name="modal">
      <div v-if="settingsProvider" class="fixed inset-0 z-30 hidden place-items-center bg-overlay p-6 backdrop-blur-sm md:grid" @click.self="closeSettings">
        <section class="modal-panel relative max-w-[560px] p-7" role="dialog" aria-modal="true" :aria-label="`${settingsProvider.name} 设置`">
          <button class="icon-btn absolute right-4 top-4" title="关闭设置" @click="closeSettings">
            <span class="i-lucide-x text-lg" />
          </button>
          <h2 class="mb-6 mt-0 pr-10 text-xl">
            {{ settingsProvider.name }} 设置
          </h2>
          <ProviderSettingsContent
            :provider="settingsProvider"
            :auth="providers.auth[settingsProvider.id]"
            @login="authenticating = settingsProvider"
            @logout="providers.logout(settingsProvider.id)"
          />
        </section>
      </div>
    </Transition>

    <Transition name="modal">
      <LoginModal
        v-if="authenticating"
        :provider-id="authenticating.id"
        :provider-name="authenticating.name"
        @authenticated="authenticated(authenticating)"
        @close="authenticating = null"
      />
    </Transition>
  </div>
</template>
