<script setup>
  import { ref } from 'vue'
  import { VueFlow } from '@vue-flow/core'
  import { MiniMap } from '@vue-flow/minimap'
  import { Controls } from '@vue-flow/controls'
  import NodeCard from './NodeCard.vue'
  import NodeTopics from './NodeTopics.vue'
  import NodeLinks from './NodeLinks.vue'
  import TransparentCard from './NodeTransparent.vue'


const information = [
    { "id": "SOOyPLIqs2dE5QDJ0Ec_5", "width": 200, "type": "links", "position": { "x": 0, "y": -300 } },
    { id: "_MzQ9h_cwZ05xuQbed5rx", type: "custom", position: { x: 0, y: -350 }, label: "Hecho con ❤️ por RustLangES", data: { label: "Hecho con ❤️ por RustLangES", withoutIcon: true } },
    { id: '00', type: 'topics', position: { x: 0, y: -150 }, data: { topicLevel: 'start' } },
]

const introductionSection = [
    { id: 'eintro-intro1', source: 'intro', target: 'intro1', data: { level: 'start' }},
    { id: 'eintro-intro3', source: 'intro', target: 'intro3', data: { level: 'start' }},
    { id: 'eintro-intro4', source: 'intro', target: 'intro4', data: { level: 'start' }},

    { id: "intro1", type: "custom", position: { x: 500, y: 0 }, width: 145, label: "¿Que es Rust?", data: { topicLevel: 'start', sourcePosition: ['left'] } },
    { id: "intro2", type: "custom", position: { x: 650, y: 0 }, width: 145, label: "¿Por qué Rust?", data: { topicLevel: 'start' } },
    { id: "intro3", type: "custom", position: { x: 500, y: 40 }, width: 300, label: "Stable, Beta, Nightly. ¿Que es esto?", data: { topicLevel: 'start', sourcePosition: ['left'] } },
    
    { id: "intro4", type: "custom", position: { x: 500, y: 100 }, width: 300, height: 200, label: "Configura tu entorno", data: { sourcePosition: ['left'], withoutIcon: true, noInteractive: true, moreTransparency: true } },

    { id: "intro4-a", type: "custom", position: { x: 500, y: 150 }, width:300,label: "Instalar Rustup", data: { topicLevel: 'start' } },
    { id: "intro4-b", type: "custom", position: { x: 500, y: 200 }, width:300,label: "Configura tu Editor de código", data: { topicLevel: 'start' } },
    { id: "intro4-c", type: "custom", position: { x: 500, y: 250 }, width:300, label: "Ejecuta tu primer programa", data: { topicLevel: 'start' } },
]

  const elements = ref([
    ...information,
    { id: '0', type: 'transparent', label: 'Rust', data: { topicLevel: 'none' }, position: { x: 262, y: 0 } },
    { id: 'intro', type: 'custom', label: 'Introducción al Lenguaje', position: { x: 180, y: 50 }, data: { topicLevel: 'start', targetPosition: ['right'] } },
    
    
    ...introductionSection,
    { id: 'basico', type: 'custom', label: 'Aprende lo Básico', position: { x: 0, y: 250 }, data: { topicLevel: 'start', targetPosition: ['right'] } },
    { id: '1', type: 'custom', label: 'Introducción al Lenguaje', position: { x: 180, y: 50 }, data: { topicLevel: 'start' } },
    { id: '2', type: 'custom', label: 'Node 2', position: { x: 100, y: 100 }, data: { topicLevel: 'medium' } },
    { id: '3', type: 'custom', label: 'Node 3', position: { x: 400, y: 100 }, data: { topicLevel: 'hard' } },
    { id: '4', type: 'custom', label: 'Node 4', position: { x: 400, y: 200 }, data: { topicLevel: 'other' } },
    { id: 'e0-1', source: '0', target: '1', data: { level: 'start' } },
    { id: 'e1-3', source: '1', target: '3', data: { level: 'hard' } },
    { id: 'e1-2', source: '1', target: '2', data: { level: 'medium' } },
    { id: 'e3-4', source: '3', target: '4', data: { level: 'other' } },
  ])
</script>

<template>
  <VueFlow
    fit-view-on-init
    v-model="elements"
    :apply-changes="false"
    :nodes-draggable="false"
    :edges-updatable="false"
    class="min-h-screen min-w-full"
    >
    <MiniMap
      pannable zoomable
      nodeBorderRadius="0"
      position="top-right"
      />
    <Controls
      position="top-left"
      :show-interactive="false"
      class="group flex flex-col border border-black bg-orange-50 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between"
      />

    <template #node-custom="props">
      <NodeCard :label="props.label" :data="props.data" />
    </template>
    <template #node-transparent="props">
      <TransparentCard :label="props.label" />
    </template>
    <template #node-topics="props">
      <NodeTopics />
    </template>
    <template #node-links="props">
      <NodeLinks />
    </template>
  </VueFlow>
</template>

<style>
  @import '@vue-flow/core/dist/style.css';

  .vue-flow__edge { pointer-events: none; }

  /*
   * Controls
   */
  .vue-flow__controls,
  .vue-flow__controls-button,
  .vue-flow__controls-button svg {
    background-color: #fff7ed;
    box-shadow: none;
    border: none;
  }

  .vue-flow__controls:hover {
    box-shadow: 4px 4px 0 rgb(0 0 0 / 40%);
  }

  .vue-flow__controls-button:hover,
  .vue-flow__controls-button:hover svg {
    background-color: #f97316;
  }
  .vue-flow__controls-button:disabled { pointer-events: none; }
  .vue-flow__controls-button:disabled svg { fill-opacity: 0.4; }
  .vue-flow__controls-button {
    box-sizing: content-box;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 16px;
    height: 16px;
    cursor: pointer;
    user-select: none;
    padding: 5px;
  }
  .vue-flow__controls-button svg {
    width: 100%;
    max-width: 12px;
    max-height: 12px;
  }

  /*
   * MiniMap
   */
  .vue-flow__minimap {
    background-color: rgb(254 215 170);
    margin-right: 1rem;
    transform: scale(70%);
  }
  .vue-flow__minimap-mask {
    fill: rgb(255 247 237 / 60%);
  }

  .vue-flow__minimap-node {
    fill: #f97316;
  }

  /*
   * Dark
   */
  @media (prefers-color-scheme: dark) {
    /*
     * Controls
     */
    .vue-flow__controls,
    .vue-flow__minimap-node,
    .vue-flow__controls-button,
    .vue-flow__controls-button svg {
      background-color: rgb(24 24 27 / 40%);
      fill: rgb(226, 206, 169);
    }

    .vue-flow__controls-button:hover,
    .vue-flow__controls-button:hover svg {
      background-color: rgb(0 0 0 / 40%) !important;
      fill: #f97316;
    }

    .vue-flow__minimap { background-color: rgb(24 24 27) }
    .vue-flow__minimap-mask { fill: rgb(0 0 0 / 70%) }
  }
</style>
