<script setup lang="ts">
import QRCode from 'qrcode'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { authQrPoll, authQrStart } from '../services/api/providers'

const props = defineProps<{ providerId: string, providerName: string }>()
const emit = defineEmits<{ close: [], authenticated: [] }>()
const qrImage = ref('')
const status = ref('正在生成二维码')
const error = ref('')
let timer: number | undefined

onMounted(async () => {
  try {
    const session = await authQrStart(props.providerId)
    qrImage.value = await QRCode.toDataURL(session.url, { width: 240, margin: 1 })
    status.value = `使用 ${props.providerName} 客户端扫码登录`
    timer = window.setInterval(async () => {
      try {
        const result = await authQrPoll(props.providerId, session.key)
        if (result.status === 'scanned')
          status.value = '已扫码，请在手机上确认'
        if (result.status === 'expired') {
          status.value = '二维码已过期，请关闭后重试'
          window.clearInterval(timer)
        }
        if (result.status === 'confirmed') {
          window.clearInterval(timer)
          emit('authenticated')
          emit('close')
        }
      }
      catch (reason) {
        error.value = reason instanceof Error ? reason.message : String(reason)
      }
    }, 1800)
  }
  catch (reason) {
    error.value = reason instanceof Error ? reason.message : String(reason)
  }
})

onBeforeUnmount(() => window.clearInterval(timer))
</script>

<template>
  <div class="overlay z-40" @click.self="emit('close')">
    <section class="modal-panel relative max-w-[380px] p-7 text-center" role="dialog" aria-modal="true" :aria-label="`登录 ${providerName}`">
      <button class="icon-btn absolute right-3 top-3" title="关闭" @click="emit('close')">
        <span class="i-lucide-x text-lg" />
      </button>
      <p class="section-label mb-1">
        {{ providerName }}
      </p>
      <h2 class="mb-5 mt-0 text-xl">
        扫码登录
      </h2>
      <div class="mx-auto mb-4 h-[240px] w-[240px] grid place-items-center overflow-hidden rounded-control border border-line bg-white p-2">
        <img v-if="qrImage" class="h-full w-full" :src="qrImage" :alt="`${providerName} 登录二维码`">
        <div v-else class="flex items-center gap-2 text-xs text-muted">
          <span class="i-lucide-loader-circle animate-spin" />生成中
        </div>
      </div>
      <p class="text-sm">
        {{ status }}
      </p>
      <p v-if="error" class="text-sm text-danger">
        {{ error }}
      </p>
      <small class="mt-4 block text-2xs leading-relaxed text-muted">登录凭据由此插件保存在独立数据目录，界面不会显示凭据。</small>
    </section>
  </div>
</template>
