<script setup lang="ts">
  defineProps<{}>()
</script>

<template>
  <a class=" tracking-wider text-center font-work-sans border border-black dark:border-white flex items-center px-4 transition w-fit gap-x-4 max-w-[10rem] sm:max-w-none bg-orange-200 dark:bg-transparent hover:bg-black hover:text-white h-9  drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)]">
    {{ props.data.label }}
  </a>
</template>
