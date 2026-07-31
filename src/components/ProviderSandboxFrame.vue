<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, onMounted, ref, watch } from 'vue'

interface SandboxPage {
  html: string
  height: number
}

type SandboxMessage = Record<string, unknown>
type SandboxHandler = (message: SandboxMessage) => unknown | Promise<unknown>

const props = withDefaults(defineProps<{
  page: SandboxPage
  channel: string
  title: string
  handlers: Record<string, SandboxHandler>
  initialMessage?: SandboxMessage
  allowExternalLinks?: boolean
}>(), {
  initialMessage: undefined,
  allowExternalLinks: false,
})

const frame = ref<HTMLIFrameElement>()
const activeRequests = new Set<string>()
const height = ref(clampHeight(props.page.height))
const frameHeight = computed(() => `${height.value}px`)
const sandbox = computed(() => props.allowExternalLinks
  ? 'allow-scripts allow-popups allow-popups-to-escape-sandbox'
  : 'allow-scripts')
const source = computed(() => {
  const navigation = props.allowExternalLinks ? '' : ' navigate-to \'none\';'
  return [
    `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src data:; connect-src 'none'; font-src 'none'; media-src 'none'; object-src 'none'; frame-src 'none'; form-action 'none';${navigation} base-uri 'none'">`,
    '<meta name="viewport" content="width=device-width, initial-scale=1">',
    props.page.html,
  ].join('')
})

function clampHeight(value: number) {
  return Math.min(800, Math.max(96, Math.round(value || 240)))
}

function post(message: SandboxMessage) {
  const payload = JSON.parse(JSON.stringify({
    channel: props.channel,
    version: 1,
    ...message,
  }))
  frame.value?.contentWindow?.postMessage(payload, '*')
}

function initializeFrame() {
  if (!props.initialMessage)
    return
  window.setTimeout(() => post(props.initialMessage!), 0)
}

async function onMessage(event: MessageEvent) {
  if (event.source !== frame.value?.contentWindow)
    return
  const message = event.data
  if (
    !message
    || typeof message !== 'object'
    || message.channel !== props.channel
    || message.version !== 1
  ) {
    return
  }
  if (message.type === 'resize') {
    if (typeof message.height === 'number' && Number.isFinite(message.height))
      height.value = clampHeight(message.height)
    return
  }
  const requestId = typeof message.requestId === 'string' ? message.requestId : ''
  if (!requestId || activeRequests.has(requestId))
    return
  const handler = typeof message.type === 'string' ? props.handlers[message.type] : undefined
  if (!handler)
    return
  activeRequests.add(requestId)
  try {
    const result = await handler(message as SandboxMessage)
    post({ type: 'response', requestId, ok: true, result })
  }
  catch (error) {
    post({
      type: 'response',
      requestId,
      ok: false,
      error: error instanceof Error ? error.message : String(error),
    })
  }
  finally {
    activeRequests.delete(requestId)
  }
}

onBeforeMount(() => window.addEventListener('message', onMessage))
onMounted(initializeFrame)
onBeforeUnmount(() => window.removeEventListener('message', onMessage))
watch(() => props.page.height, value => {
  height.value = clampHeight(value)
})
</script>

<template>
  <iframe
    ref="frame"
    class="w-full border-0 bg-transparent"
    :style="{ height: frameHeight }"
    :srcdoc="source"
    :sandbox="sandbox"
    referrerpolicy="no-referrer"
    :title="title"
    @load="initializeFrame"
  />
</template>
