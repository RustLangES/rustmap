<script setup>
  import { ref } from 'vue'
  import { VueFlow } from '@vue-flow/core'
  import { MiniMap } from '@vue-flow/minimap'
  import { Controls } from '@vue-flow/controls'
  import NodeCard from './NodeCard.vue'
  import NodeTopics from './NodeTopics.vue'
  import TransparentCard from './NodeTransparent.vue'

  const { data } = await useAsyncData('content', () => queryContent().find())

  const roadmapNodes = ref([
    { id: 'topics', type: 'topics', position: { x: 0, y: -150 }, data: { topicLevel: 'start' } },
    ...data.value.map(({ title, _path, data: { position, width, sourcePosition, targetPosition, topicLevel, type, align } }) => (
    { id: _path.substring(1).replaceAll('/', "-").replaceAll('_', ''), type, position, width, label: title, data: { align, topicLevel, sourcePosition, targetPosition, path: _path } }
  ))
  ])

  const roadmapEdges = ref([
    ...data.value.flatMap(({_path, data: { topicLevel, sourcePosition, targetPosition } }) => {
    const mySlug = _path.substring(1).replaceAll('/', "-").replaceAll('_', '')
    return [
      ...Object.keys(sourcePosition || {}).map(k => {
        const source = sourcePosition[k]
        return { id: `${mySlug}-${k}`, type: 'smoothstep', source: mySlug, target: k, data: { level: topicLevel } }
      }),
      ...Object.keys(targetPosition || {}).map(k => {
        const target = targetPosition[k]
        return { id: `${k}-${mySlug}`, type: 'smoothstep', source: k, target: mySlug, data: { level: topicLevel } }
      }),
    ]
    })
  ])
</script>

<template>
  <VueFlow
    fit-view-on-init
    :nodes="roadmapNodes"
    :edges="roadmapEdges"
    :apply-changes="false"
    :nodes-draggable="false"
    :edges-updatable="false"
    :min-zoom="0.65"
    :zoom-on-scroll="false"
    :prevent-scrolling="false"
    class="min-h-[220vh] min-w-full"
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
      <TransparentCard :label="props.label" :data="props.data" />
    </template>
    <template #node-topics="props">
      <NodeTopics />
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
