<script setup>
  import { Handle, Position } from '@vue-flow/core'
  const props = defineProps({
    data: Object,
    label: {
      type: String,
    },
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

  <div class="py-1 px-2">
    {{ props.label }}
  </div>
</template>
