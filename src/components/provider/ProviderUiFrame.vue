<script setup lang="ts">
import type { ProviderUiBundle } from '../../types'
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { providerImage } from '../../services/api/input'
import { useProvidersStore } from '../../stores/providers'

defineOptions({ name: 'ProviderUiFrame' })

const props = defineProps<{
  providerId: string
  bundle: ProviderUiBundle
  surface: string
  context: Record<string, unknown>
  title: string
}>()

const emit = defineEmits<{
  (_event: 'state', _value: unknown): void
  (_event: 'close', _value: unknown): void
}>()

const providers = useProvidersStore()
const frame = ref<HTMLIFrameElement>()
const frameError = ref('')
const modalPanel = ref<HTMLElement>()
const modal = ref<{
  requestId: string
  surface: string
  title: string
  context: Record<string, unknown>
  width: number
} | null>(null)
const channel = `yaya-provider-ui-${crypto.randomUUID()}`
const surface = computed(() => props.bundle.surfaces.find(value => value.id === props.surface))
const height = ref(clampHeight(surface.value?.initialHeight ?? 320))
const frameHeight = computed(() => `${height.value}px`)

function encodeBase64(value: string) {
  const bytes = new TextEncoder().encode(value)
  let binary = ''
  for (const byte of bytes)
    binary += String.fromCharCode(byte)
  return btoa(binary)
}

const source = computed(() => {
  const module = encodeBase64(props.bundle.module)
  const style = encodeBase64(props.bundle.style)
  const safeChannel = JSON.stringify(channel)
  return `<!doctype html><html><head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' blob:; style-src 'unsafe-inline'; img-src data:; connect-src 'none'; font-src 'none'; media-src 'none'; object-src 'none'; frame-src 'none'; form-action 'none'; base-uri 'none'">
</head><body><div id="app"></div><script type="module">
const channel=${safeChannel};
const moduleBase64='${module}';
const styleBase64='${style}';
const decode=value=>new TextDecoder().decode(Uint8Array.from(atob(value),char=>char.charCodeAt(0)));
const baseStyle=document.createElement('style');baseStyle.textContent='html,body,#app{width:100%;max-width:100%;margin:0;overflow-x:hidden;background:transparent!important}';document.head.append(baseStyle);
const style=document.createElement('style');style.textContent=decode(styleBase64);document.head.append(style);
const pending=new Map();let sequence=0;let mounted;
const send=message=>parent.postMessage(JSON.parse(JSON.stringify({channel,version:1,...message})),'*');
const request=(type,payload={})=>new Promise((resolve,reject)=>{const requestId=String(++sequence);pending.set(requestId,{resolve,reject});send({type,requestId,...payload});});
const bridge={
  invoke:(action,payload=null)=>request('invoke',{action,payload}),
  asset:url=>request('asset',{url}),
  openModal:options=>request('modal',{options}),
  closeModal:result=>send({type:'close-modal',result}),
  updateState:state=>send({type:'state',state}),
};
function applyTheme(tokens){for(const [key,value] of Object.entries(tokens||{}))document.documentElement.style.setProperty(key,String(value));document.documentElement.style.colorScheme=tokens?.['--yaya-color-scheme']||'light';}
addEventListener('message',async event=>{
  const message=event.data;
  if(!message||message.channel!==channel||message.version!==1)return;
  if(message.type==='response'){
    const entry=pending.get(message.requestId);if(!entry)return;pending.delete(message.requestId);
    message.ok?entry.resolve(message.result):entry.reject(new Error(message.error));return;
  }
  if(message.type==='theme'){applyTheme(message.tokens);return;}
  if(message.type==='context'){mounted?.updateContext?.(message.context);return;}
  if(message.type==='mount'&&!mounted){
    try{
      applyTheme(message.theme);
      const url=URL.createObjectURL(new Blob([decode(moduleBase64)],{type:'text/javascript'}));
      const providerModule=await import(url);URL.revokeObjectURL(url);
      const ui=providerModule.default;
      if(!ui||typeof ui.mount!=='function')throw new Error('Provider UI bundle 必须默认导出 mount()');
      mounted=await ui.mount({surface:message.surface,root:document.querySelector('#app'),context:message.context,bridge});
      send({type:'mounted'});
    }catch(error){send({type:'error',error:error instanceof Error?error.message:String(error)});}
  }
});
new ResizeObserver(()=>send({type:'resize',height:Math.max(document.body.scrollHeight,document.documentElement.scrollHeight)})).observe(document.documentElement);
send({type:'ready'});
<\/script></body></html>`
})

function clampHeight(value: number) {
  return Math.min(900, Math.max(72, Math.round(value || 320)))
}

function post(message: Record<string, unknown>) {
  const payload = JSON.parse(JSON.stringify({ channel, version: 1, ...message }))
  frame.value?.contentWindow?.postMessage(payload, '*')
}

function themeTokens() {
  const style = getComputedStyle(document.documentElement)
  const mapping: Record<string, string> = {
    '--yaya-canvas': '--canvas',
    '--yaya-surface': '--surface',
    '--yaya-elevated': '--elevated',
    '--yaya-soft': '--soft',
    '--yaya-line': '--line',
    '--yaya-text': '--ink',
    '--yaya-muted': '--muted',
    '--yaya-accent': '--accent',
    '--yaya-accent-hover': '--accent-hover',
    '--yaya-accent-soft': '--accent-soft',
    '--yaya-danger': '--danger',
    '--yaya-warning': '--warning',
    '--yaya-success': '--success',
  }
  return {
    ...Object.fromEntries(Object.entries(mapping).map(([target, source]) => [target, style.getPropertyValue(source).trim()])),
    '--yaya-color-scheme': document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light',
  }
}

function finishModal(result: unknown = { dismissed: true }) {
  if (!modal.value)
    return
  post({ type: 'response', requestId: modal.value.requestId, ok: true, result })
  modal.value = null
}

function onModalKeydown(event: KeyboardEvent) {
  event.stopPropagation()
  if (event.key === 'Escape') {
    event.preventDefault()
    finishModal()
  }
}

async function onMessage(event: MessageEvent) {
  if (event.source !== frame.value?.contentWindow)
    return
  const message = event.data
  if (!message || typeof message !== 'object' || message.channel !== channel || message.version !== 1)
    return
  if (message.type === 'ready') {
    post({ type: 'mount', surface: props.surface, context: props.context, theme: themeTokens() })
    return
  }
  if (message.type === 'resize' && typeof message.height === 'number') {
    height.value = clampHeight(message.height + 2)
    return
  }
  if (message.type === 'state') {
    emit('state', message.state)
    return
  }
  if (message.type === 'close-modal') {
    emit('close', message.result)
    return
  }
  if (message.type === 'error') {
    frameError.value = typeof message.error === 'string' ? message.error : 'Provider UI 加载失败'
    return
  }
  const requestId = typeof message.requestId === 'string' ? message.requestId : ''
  if (!requestId)
    return
  try {
    let result: unknown
    if (message.type === 'invoke') {
      if (typeof message.action !== 'string' || !message.action)
        throw new Error('Provider UI action 不能为空')
      result = await providers.invokeUi(props.providerId, message.action, message.payload ?? null)
    }
    else if (message.type === 'asset') {
      if (typeof message.url !== 'string')
        throw new Error('资源 URL 无效')
      result = await providerImage(props.providerId, message.url)
    }
    else if (message.type === 'modal') {
      const options = message.options
      if (!options || typeof options !== 'object')
        throw new Error('Provider 子弹窗参数无效')
      if (typeof options.surface !== 'string' || !props.bundle.surfaces.some(value => value.id === options.surface))
        throw new Error(`Provider UI 未声明 surface：${String(options.surface)}`)
      if (typeof options.title !== 'string' || !options.title.trim())
        throw new Error('Provider 子弹窗标题不能为空')
      if (modal.value)
        finishModal()
      modal.value = {
        requestId,
        surface: options.surface,
        title: options.title,
        context: options.context && typeof options.context === 'object' ? options.context : {},
        width: Math.min(720, Math.max(320, Number(options.width) || 440)),
      }
      await nextTick()
      modalPanel.value?.focus()
      return
    }
    else {
      throw new Error(`不支持的 Provider UI 请求：${String(message.type)}`)
    }
    post({ type: 'response', requestId, ok: true, result })
  }
  catch (error) {
    post({ type: 'response', requestId, ok: false, error: error instanceof Error ? error.message : String(error) })
  }
}

const themeObserver = new MutationObserver(() => post({ type: 'theme', tokens: themeTokens() }))
onMounted(() => {
  window.addEventListener('message', onMessage)
  themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme', 'style', 'class'] })
})
onBeforeUnmount(() => {
  window.removeEventListener('message', onMessage)
  themeObserver.disconnect()
})
watch(() => props.context, context => post({ type: 'context', context }), { deep: true })
watch(surface, (value) => {
  height.value = clampHeight(value?.initialHeight ?? 320)
})
</script>

<template>
  <div>
    <div v-if="frameError" class="rounded-control border border-danger-line bg-danger-soft p-3 text-xs text-danger">
      {{ frameError }}
    </div>
    <iframe
      v-else
      ref="frame"
      class="w-full border-0 bg-transparent"
      :style="{ height: frameHeight }"
      :srcdoc="source"
      sandbox="allow-scripts"
      referrerpolicy="no-referrer"
      :title="title"
    />
    <Teleport to="body">
      <Transition name="modal">
        <div
          v-if="modal"
          class="fixed inset-0 grid place-items-center bg-overlay p-4 backdrop-blur-sm"
          style="z-index: 80"
          @click.self="finishModal()"
          @keydown="onModalKeydown"
        >
          <section
            ref="modalPanel"
            class="modal-panel relative max-h-[85vh] w-full overflow-hidden p-6 outline-none md:p-7"
            :style="{ maxWidth: `${modal.width}px` }"
            role="dialog"
            aria-modal="true"
            :aria-label="modal.title"
            tabindex="-1"
          >
            <button class="icon-btn absolute right-4 top-4 z-10" :title="`关闭${modal.title}`" @click="finishModal()">
              <span class="i-lucide-x text-lg" />
            </button>
            <h2 class="mb-5 mt-0 pr-10 text-xl font-700">
              {{ modal.title }}
            </h2>
            <div class="max-h-[70vh] overflow-y-auto">
              <ProviderUiFrame
                :provider-id="providerId"
                :bundle="bundle"
                :surface="modal.surface"
                :context="modal.context"
                :title="modal.title"
                @close="finishModal"
              />
            </div>
          </section>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>
