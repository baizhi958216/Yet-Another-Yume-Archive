<script setup lang="ts">
import type {
  ProviderAuthPage,
  ProviderInfo,
  ProviderSettingAction,
  ProviderSettingsState,
  ProviderSettingsView,
} from '../types'
import { computed, ref, watch } from 'vue'
import { useProvidersStore } from '../stores/providers'
import { useUiStore } from '../stores/ui'
import FieldInput from './FieldInput.vue'
import ProviderSandboxFrame from './ProviderSandboxFrame.vue'

const props = defineProps<{
  provider: ProviderInfo
  auth: ProviderAuthPage | null | undefined
  settingsView: ProviderSettingsView | null | undefined
  settingsState: ProviderSettingsState | null | undefined
}>()

const providers = useProvidersStore()
const ui = useUiStore()
const values = ref<Record<string, unknown>>({})
const saving = ref(false)
const runningAction = ref('')
const resultMessage = ref('')
const hasFields = computed(() =>
  props.settingsView?.sections.some(section => section.fields.length) ?? false,
)
const allowedSettingFields = computed(() => new Set(
  (props.settingsView?.sections ?? []).flatMap(section => section.fields.map(field => field.key)),
))
const allowedSettingActions = computed(() => new Set(
  (props.settingsView?.sections ?? []).flatMap(section => section.actions.map(action => action.key)),
))
const settingsInitialMessage = computed(() => ({
  type: 'init',
  result: props.settingsState ?? { values: {} },
}))

function safeSettingValues(value: unknown) {
  if (!value || typeof value !== 'object' || Array.isArray(value))
    throw new Error('values 必须是对象')
  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>)
      .filter(([key]) => allowedSettingFields.value.has(key)),
  )
}

const authHandlers = {
  async invoke(message: Record<string, unknown>) {
    if (typeof message.action !== 'string' || !message.action)
      throw new Error('认证操作 key 不能为空')
    return providers.invokeAuth(props.provider.id, message.action, message.payload ?? null)
  },
}

const settingsHandlers = {
  ready() {
    return props.settingsState ?? { values: {} }
  },
  get() {
    return props.settingsState ?? { values: {} }
  },
  update(message: Record<string, unknown>) {
    return providers.updateSettings(props.provider.id, safeSettingValues(message.values))
  },
  invoke(message: Record<string, unknown>) {
    if (typeof message.action !== 'string' || !allowedSettingActions.value.has(message.action))
      throw new Error('未声明的设置操作')
    return providers.invokeSettings(
      props.provider.id,
      message.action,
      safeSettingValues(message.values ?? props.settingsState?.values ?? {}),
    )
  },
}

watch(
  () => [props.settingsView, props.settingsState] as const,
  ([view, state]) => {
    const defaults = Object.fromEntries(
      (view?.sections ?? []).flatMap(section =>
        section.fields.map(field => [field.key, field.default]),
      ),
    )
    values.value = { ...defaults, ...(state?.values ?? {}) }
  },
  { immediate: true },
)

async function save() {
  saving.value = true
  resultMessage.value = ''
  try {
    const state = await providers.updateSettings(props.provider.id, values.value)
    values.value = { ...state.values }
    resultMessage.value = '设置已保存'
  }
  catch (error) {
    ui.showError(error)
  }
  finally {
    saving.value = false
  }
}

function actionClass(action: ProviderSettingAction) {
  if (action.style === 'primary')
    return 'primary-btn'
  if (action.style === 'danger')
    return 'secondary-btn !text-danger'
  return 'secondary-btn'
}

async function invokeAction(action: ProviderSettingAction) {
  runningAction.value = action.key
  resultMessage.value = ''
  try {
    const result = await providers.invokeSettings(props.provider.id, action.key, values.value)
    resultMessage.value = result.message
  }
  catch (error) {
    ui.showError(error)
  }
  finally {
    runningAction.value = ''
  }
}
</script>

<template>
  <div>
    <div class="flex items-start gap-4 border-b border-line pb-5">
      <div class="h-11 w-11 shrink-0 grid place-items-center rounded-control bg-soft text-accent">
        <span class="i-lucide-blocks text-xl" />
      </div>
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
          <h3 class="m-0 text-base font-700">
            {{ provider.name }}
          </h3>
          <span class="text-2xs text-muted">v{{ provider.version || '0.0.0' }}</span>
        </div>
        <p class="mb-0 mt-1 text-xs leading-5 text-muted">
          {{ provider.description }}
        </p>
      </div>
    </div>

    <section v-if="provider.capabilities.authentication" class="py-5" aria-label="Provider 认证">
      <ProviderSandboxFrame
        v-if="auth"
        :page="auth"
        channel="yaya-provider-auth"
        title="Provider 认证"
        :handlers="authHandlers"
        allow-external-links
      />
      <p v-else class="m-0 text-xs text-muted">
        认证页面暂不可用
      </p>
    </section>

    <section v-if="provider.capabilities.settings" class="border-t border-line py-5" aria-label="Provider 设置">
      <ProviderSandboxFrame
        v-if="settingsView?.customPage"
        :page="settingsView.customPage"
        channel="yaya-provider-settings"
        title="Provider 自定义设置"
        :handlers="settingsHandlers"
        :initial-message="settingsInitialMessage"
      />
      <template v-else-if="settingsView">
        <section v-for="section in settingsView.sections" :key="section.key" class="border-b border-line py-5 first:pt-0 last:border-b-0 last:pb-0">
          <p class="section-label mb-1">
            {{ section.title }}
          </p>
          <p v-if="section.description" class="mb-4 mt-1 text-2xs leading-5 text-muted">
            {{ section.description }}
          </p>

          <div v-if="section.fields.length" class="space-y-4" :class="{ 'mt-4': !section.description }">
            <label v-for="field in section.fields" :key="field.key" class="block">
              <span class="mb-1.5 block text-xs font-600">{{ field.label }}</span>
              <FieldInput
                :field="field"
                :model-value="values[field.key]"
                @update:model-value="values[field.key] = $event"
              />
              <span v-if="field.description" class="mt-1.5 block text-2xs leading-5 text-muted">{{ field.description }}</span>
            </label>
          </div>

          <div v-if="section.statuses.length" class="space-y-3" :class="{ 'mt-4': section.fields.length || section.description }">
            <div v-for="item in section.statuses" :key="item.key" class="flex items-start justify-between gap-4 rounded-control bg-soft px-4 py-3">
              <div class="min-w-0">
                <p class="m-0 text-sm font-700">
                  {{ item.label }}
                </p>
                <p v-if="item.description" class="mb-0 mt-1 break-all text-2xs leading-5 text-muted">
                  {{ item.description }}
                </p>
              </div>
              <span class="mt-0.5 shrink-0 text-xs font-600" :class="item.available ? 'text-success' : 'text-warning'">
                {{ item.value }}
              </span>
            </div>
          </div>

          <div v-if="section.actions.length" class="mt-4 flex flex-wrap gap-2">
            <button
              v-for="action in section.actions"
              :key="action.key"
              :class="actionClass(action)"
              :disabled="!!runningAction"
              :title="action.description"
              @click="invokeAction(action)"
            >
              {{ runningAction === action.key ? '执行中…' : action.label }}
            </button>
          </div>
        </section>

        <div v-if="hasFields || resultMessage" class="mt-5 flex flex-wrap items-center justify-between gap-3">
          <p class="m-0 text-2xs text-success">
            {{ resultMessage }}
          </p>
          <button v-if="hasFields" class="primary-btn ml-auto" :disabled="saving" @click="save">
            {{ saving ? '保存中…' : '保存设置' }}
          </button>
        </div>
      </template>
      <p v-else class="m-0 text-xs text-muted">
        设置暂不可用
      </p>
    </section>

  </div>
</template>
