<script setup>
  import { useRoute } from 'vue-router'

  const route = useRoute()
  const nodeId = route.params.slug

  useHead({
    bodyAttrs: {
      class: 'bg-orange-200 dark:bg-[#131313]/90 w-screen min-h-screen bg-center bg-fixed dark:bg-kaku dark:bg-cover dark:bg-blend-darken overflow-x-hidden dark:text-[#e2cea9]' + ((nodeId && !route.query.fromClick) ? ' overflow-hidden' : '')
    }
  })
</script>

<template>
  <NuxtPage />
</template>
