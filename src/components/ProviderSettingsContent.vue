<script setup lang="ts">
import type { AuthStatus, ProviderInfo } from '../types'
import SmartImage from './SmartImage.vue'

defineProps<{
  provider: ProviderInfo
  auth: AuthStatus | null | undefined
}>()

const emit = defineEmits<{
  login: []
  logout: []
}>()
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

    <section class="py-5" aria-labelledby="provider-account-heading">
      <p id="provider-account-heading" class="section-label mb-3">
        账户
      </p>
      <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div class="flex items-center gap-3">
          <SmartImage v-if="auth?.loggedIn && auth.user?.avatarUrl" class="h-9 w-9 shrink-0 rounded-full bg-soft object-cover" :src="auth.user.avatarUrl" :provider-id="provider.id" alt="" />
          <div v-else class="h-9 w-9 shrink-0 grid place-items-center rounded-full bg-soft text-muted">
            <span class="i-lucide-user-round text-base" />
          </div>
          <div>
            <p class="m-0 text-sm font-700">
              {{ auth?.loggedIn ? auth.user?.name || provider.name : '未连接' }}
            </p>
            <p class="mb-0 mt-1 text-2xs text-muted">
              {{ auth?.loggedIn ? (auth.user?.badge || '身份已连接') : `连接 ${provider.name} 提供的身份` }}
            </p>
          </div>
        </div>
        <button v-if="auth?.loggedIn" class="secondary-btn self-start sm:self-auto" @click="emit('logout')">
          <span class="i-lucide-log-out" />退出登录
        </button>
        <button v-else class="primary-btn self-start sm:self-auto" @click="emit('login')">
          <span class="i-lucide-scan-line" />扫码登录
        </button>
      </div>
    </section>

    <p class="m-0 border-t border-line pt-4 text-2xs leading-5 text-muted">
      登录凭据由此插件保存在独立数据目录中。停用插件不会删除账户信息。
    </p>
  </div>
</template>
