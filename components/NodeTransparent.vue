<script setup>
  const props = defineProps({
    label: {
      type: String,
    },
  })
</script>

<template>
  <div class="py-1 px-2">
    {{ props.label }}
  </div>
</template>
