<script setup>
  import { ref } from 'vue'
  import Card from './Card.vue'

  const topics = [
    { slug: 'start', label: "Conceptos Principales / Empieza con Esto" },
    { slug: 'medium', label: "Conceptos Intermedios / Continua con Esto" },
    { slug: 'hard', label: "Conceptos Avanzados / Tomalos al final" },
    { slug: 'other', label: "Temas Opcionales" },
  ]
  const topicClass = ref("")

  const changeLevel = (e) => {
    console.log(e)
  };
</script>

<template>
  <Card no-interactive>
    <div class="flex flex-col items-start gap-y-1">
      <button
        v-for="topic in topics"
        :key="topic.slug"
        @click="changeLevel"
        class="text-[0.5rem] w-full py-1 flex flex-row gap-x-2 hover:cursor-pointer"
        :class="'topic-' + topic.slug"
        >
        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"></path></svg>
        {{ topic.label }}
      </button>
    </div>
  </Card>
</template>

<style>
  .topic-start svg { width: 10px; fill: #359A19 }
  .topic-medium svg { width: 10px; fill: #702692 }
  .topic-hard svg { width: 10px; fill: #E78123 }
  .topic-other svg { width: 10px; fill: #999999 }
</style>
