<script setup>
  import Card from './Card.vue'
  import { useVueFlow } from '@vue-flow/core'

  const props = defineProps({
    data: Object,
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
    if (node.type == "topics") return
    console.log(node)
  })
  // TODO: animate all path if have event.node.data.topicLevel.eq()
  onNodeDoubleClick(({ node }) => {
    if (node.type == "topics") return
    console.log(node)
  })
</script>

<template>
  <Card class="flex flex-row gap-x-2" :class="'topic-' + props.data.topicLevel">
    <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"></path></svg>
    {{ props.label }}
  </Card>
</template>

<style>
  .topic-start svg { width: 10px; fill: #359A19 }
  .topic-medium svg { width: 10px; fill: #702692 }
  .topic-hard svg { width: 10px; fill: #E78123 }
  .topic-other svg { width: 10px; fill: #999999 }
</style>
