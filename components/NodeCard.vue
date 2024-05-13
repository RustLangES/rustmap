<script setup>
  import Card from './Card.vue'
  import { useVueFlow } from '@vue-flow/core'
  import { Handle, Position } from '@vue-flow/core'
  const props = defineProps({
    data: Object,
    withoutIcon: {
      type: Boolean
    },
    label: {
      type: String,
    },
  })

  const { 
    onNodeClick, 
    onNodeDoubleClick, 
  } = useVueFlow()
  // TODO: check if this node is complete to change style

  // TODO: open side content
  onNodeClick(({ node }) => {
    if (node.type === "topics") return
    console.log(node)
  })
  // TODO: animate all path if have event.node.data.topicLevel.eq()
  onNodeDoubleClick(({ node }) => {
    if (node.type === "topics") return
    console.log(node)
  })

  const checkSourceHandle = (ty) => {
    return Object.values(props.data.sourcePosition || {}).includes(ty)
  }
  const checkTargetHandle = (ty) => {
    return Object.values(props.data.targetPosition || {}).includes(ty)
  }
</script>

<template>
  <Handle v-if="checkSourceHandle('right')" type="source" :position="Position.Right"  />
  <Handle v-if="checkSourceHandle('top')" type="source" :position="Position.Top"  />
  <Handle v-if="checkSourceHandle('left')" type="source" :position="Position.Left" />
  <Handle v-if="checkSourceHandle('bottom')" type="source" :position="Position.Bottom"/>

  <Handle v-if="checkTargetHandle('right')" type="target" :position="Position.Right" />
  <Handle v-if="checkTargetHandle('top')" type="target" :position="Position.Top" />
  <Handle v-if="checkTargetHandle('left')" type="target" :position="Position.Left" />
  <Handle v-if="checkTargetHandle('bottom')" type="target" :position="Position.Bottom" />

  {{ props.data.position }}
  <Card class="flex flex-row gap-x-2" :class="'topic-' + props.data.topicLevel" :moreTransparency="props.data.moreTransparency" :noInteractive="props.data.noInteractive">
    <svg v-if="!props.data.withoutIcon" stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"></path></svg>
    <div class="w-full text-center">
      {{ props.label }}
    </div>
  </Card>
</template>

<style>
  .topic-start svg { width: 10px; fill: #359A19 }
  .topic-medium svg { width: 10px; fill: #702692 }
  .topic-hard svg { width: 10px; fill: #E78123 }
  .topic-other svg { width: 10px; fill: #999999 }
</style>
