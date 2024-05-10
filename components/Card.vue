<script setup lang="ts">
  const props = defineProps({
    noInteractive: {
      type: Boolean
    },
  })

  const classObject = computed(() => [
    'hover:bg-orange-500',
    'dark:hover:bg-zinc-900/40',
    'hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)'
  ].map(c => ({ [c]: !props.noInteractive })))
</script>

<template >
  <div
    class="group flex flex-col gap-y-6 border border-black p-6 bg-orange-50 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between"
    :class="classObject">
    <slot></slot>
  </div>
</template>
