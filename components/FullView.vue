<template>
  <div class="right-0 top-0 z-40 flex min-h-screen w-full flex-col overflow-y-auto bg-white p-4 focus:outline-0 sm:p-6">
    <h2>{{ node.title }}</h2>
    <p>{{ node.content }}</p>
  </div>
</template>

<script setup>
defineProps({
  node: Object
})
</script>
