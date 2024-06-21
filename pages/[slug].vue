<template>
  <div class="sticky right-0 top-0 z-40 w-screen min-h-screen">
    <Sidebar v-if="showSidebar" :node="nodeData" />
    <FullView v-else :node="nodeData" />
  </div>
  <Header />
  <HeroSection />
  <Roadmap />
</template>

<script setup>
import { useRoute } from 'vue-router'
import { ref, onMounted } from 'vue'
import Sidebar from '@/components/Sidebar.vue'
import FullView from '@/components/FullView.vue'
import Roadmap from '@/components/Roadmap.vue'
import HeroSection from '@/layouts/hero.vue'

const route = useRoute()
const nodeId = route.params.slug
const nodeData = ref(null)
const showSidebar = ref(true)

onMounted(async () => {
  // Fetch the node data based on the nodeId
  const response = await fetchNodeData(nodeId)
  nodeData.value = response

  showSidebar.value = response && (route.query.fromClick || false)
})

async function fetchNodeData(id) {
  return {
    id,
    title: 'Nodo ' + id,
    content: 'Contenido del nodo ' + id
  }
}
</script>
