<script setup lang="ts">
// Images from provider content are fetched through the provider's own
// network context (referrers, cookies) and rendered as data URLs.
import { ref, watch } from 'vue'
import { providerImage } from '../services/api/input'

const props = withDefaults(defineProps<{
  src: string
  alt?: string
  providerId: string
}>(), { alt: '' })

const displayedSrc = ref('')
let loadSequence = 0

watch(() => props.src, async (src) => {
  const sequence = ++loadSequence
  displayedSrc.value = /^https?:\/\//i.test(src) ? '' : src
  if (!src)
    return
  try {
    const resolved = props.providerId ? await providerImage(props.providerId, src) : src
    if (sequence === loadSequence)
      displayedSrc.value = resolved
  }
  catch {
    if (sequence === loadSequence)
      displayedSrc.value = src
  }
}, { immediate: true })
</script>

<template>
  <img :src="displayedSrc" :alt="alt">
</template>
