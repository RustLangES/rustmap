<script setup lang="ts">
  const props = defineProps({
    noInteractive: {
      type: Boolean
    },
    moreTransparency: {
      type: Boolean
    },
  })

  const classObject = computed(() => [
    'hover:bg-orange-500',
    'dark:hover:bg-zinc-900/40',
    'hover:cursor-pointer'
  ].map(c => ({ [c]: !props.noInteractive })))

  const moreTransparency = computed(() => [
    'dark:bg-black/20',
    'dark:hover:bg-black/20',
  ].map(c => ({ [c]: props.moreTransparency })))
</script>

<template >
  <div
    class="group border border-black py-1 px-2 h-full bg-orange-50 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] transition w-full justify-start" :class="[classObject, moreTransparency]">
    <slot></slot>
  </div>
</template>
