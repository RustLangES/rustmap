<script setup>
  import Card from './Card.vue'

  const props = defineProps({
    label: {
      type: String,
    },
  })
</script>

<template>
  <Card no-interactive>
    {{ props.label }}
  </Card>
</template>
