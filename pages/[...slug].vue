<template>
  <Header />
  <HeroSection />
  <Roadmap />
  <div v-if="content" :class="['fixed top-0 h-screen', sidebarClass]">
    <div
        class="top-0 flex h-full w-full flex-col items-center bg-white p-4 focus:outline-0 sm:p-6"
    >
      <div class="w-full flex flex-row justify-between px-4 pb-2">
        <span>[TODO] Status</span>
        <span class="hover:cursor-pointer" @click="closeSidebar">X</span>
      </div>
      <ContentRenderer :key="content._id" :value="content">
        <ContentRendererMarkdown class="flex flex-col gap-3 text-black overscroll-x-none overflow-y-auto w-full m:max-w-[800px] sm:max-w-[600px]" :value="content" />
        <div class="flex flex-col items-start w-full m:max-w-[800px] sm:max-w-[600px]">
          <h4 id="related-content" class="text-black mb-3">
            <a href="#related-content">Contenido Extra Relacionado</a>
          </h4>
          <Card
            moreTransparency
            class="gap-x-2 mb-1"
            v-for="(link, i) in content.data.externalLinks"
            :key="i"
            :href="link.link"
          >
            <strong v-if="link.english">[Contenido en Ingles]</strong>
            {{link.name}}
          </Card>
        </div>
      </ContentRenderer>
    </div>
  </div>
</template>

<script setup>
import { useRoute } from 'vue-router'
import { ref, onMounted } from 'vue'
import Roadmap from '@/components/Roadmap.vue'
import HeroSection from '@/layouts/hero.vue'

const route = useRoute()
const nodeId = route.params.slug
const showSidebar = ref(true)
const content = ref(null)

onMounted(async () => {
  if (!nodeId) return
  const contentResult = await queryContent(nodeId.join("/")).findOne()
  content.value = contentResult
  showSidebar.value = contentResult && (route.query.fromClick || false)
})

const closeSidebar = () => content.value = null;
const sidebarClass = computed(() => showSidebar ? 'right-0' : '')
</script>
