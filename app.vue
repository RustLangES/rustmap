<script setup>
  const title = 'Hoja de Ruta Definitiva para Aprender Rust: Desde Principiante hasta Experto'
  const description = `
¿Estás listo para dominar uno de los lenguajes de programación más potentes y eficientes? Nuestra hoja de ruta te guiará paso a paso en tu viaje de aprendizaje de Rust, desde los conceptos básicos hasta las técnicas avanzadas. Diseñada para principiantes y desarrolladores experimentados, esta guía exhaustiva te ayudará a construir una sólida base en Rust y a aprovechar al máximo su rendimiento, seguridad y concurrencia.
Aprenderás a través de ejemplos prácticos, ejercicios desafiantes y proyectos reales, lo que te permitirá aplicar tus nuevas habilidades de inmediato. Además, explorarás las mejores prácticas y las últimas tendencias en el ecosistema de Rust, asegurándote de estar siempre un paso adelante.
Ya sea que desees desarrollar aplicaciones de sistemas, videojuegos, criptomonedas o cualquier otro proyecto, nuestra hoja de ruta te preparará para dominar Rust y destacarte en el mundo del desarrollo de software. ¡Comienza tu viaje hoy y conviértete en un experto en Rust!
    `
  useHead({
    bodyAttrs: {
      class: 'bg-orange-200 dark:bg-[#131313]/90 w-screen h-screen bg-center bg-fixed dark:bg-kaku dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]'
    }
  })

  useSeoMeta({
    title,
    description,
    ogTitle: title,
    ogDescription: description,
    twitterTitle: title,
    twitterDescription: description,
    twitterCard: 'summary_large_image',
    icon: '/favicon.ico',
    lang: 'es',
    ogImage: '[og:image]',
    twitterImage: '[twitter:image]'
  })

  defineOgImageComponent('Home', { title })


  import { ref } from 'vue'
  import { VueFlow } from '@vue-flow/core'
  import { MiniMap } from '@vue-flow/minimap'
  import { Controls } from '@vue-flow/controls'
  import NodeCard from './components/NodeCard.vue'

  const elements = ref([
    { id: '1', type: 'custom', label: 'Node 1', position: { x: 250, y: 5 } },
    { id: '2', type: 'custom', label: 'Node 2', position: { x: 100, y: 100 }, },
    { id: '3', type: 'custom', label: 'Node 3', position: { x: 400, y: 100 } },
    { id: '4', type: 'custom', label: 'Node 4', position: { x: 400, y: 200 } },
    { id: 'e1-3', source: '1', target: '3', animated: true },
    { id: 'e1-2', source: '1', target: '2', animated: true },
  ])
</script>

<template>
  <Header />
  <VueFlow
    fit-view-on-init
    v-model="elements"
    :apply-changes="false"
    :nodes-draggable="false"
    class="min-h-screen min-w-full"
    >
    <MiniMap position="top-right" pannable zoomable />
    <Controls position="top-left" :show-interactive="false" />

    <template #node-custom="props">
      <NodeCard :label="props.label" />
    </template>
  </VueFlow>
</template>

<style>
  @import '@vue-flow/core/dist/style.css';

  @import '@vue-flow/core/dist/theme-default.css';
  /* Plugins */
  @import '@vue-flow/minimap/dist/style.css';
  @import '@vue-flow/controls/dist/style.css';

  html,
  body,
  #app {
    margin: 0;
    height: 100%;
  }

.vue-flow__minimap { transform: scale(75%); }
.vue-flow__edge-path { background: #fff; }
</style>
