<template>
  <Header />
  <HeroSection />
  <Roadmap />
  <Teleport to="body">
  <div v-if="content" :class="['fixed top-0 right-0 h-screen', sidebarClass]">
    <div
        class="flex overflow-y-auto overflow-x-none h-full w-full flex-col items-center bg-white p-4 focus:outline-0 sm:p-6"
    >
      <div class="w-full flex flex-row justify-between text-black px-4 pb-2">
        <span class="hover:cursor-pointer" @click="closeSidebar">X</span>
      </div>
      <ContentRenderer :key="content._id" :value="content">
        <ContentRendererMarkdown class="prose flex flex-col w-full m:max-w-[800px] sm:max-w-[600px]" tag="article" :value="content" />
        <div class="flex flex-col items-start w-full m:max-w-[800px] sm:max-w-[600px]">
          <h4 id="related-content" class="text-black mb-3">
            <a href="#related-content">Contenido Extra Relacionado</a>
          </h4>
          <a
            class="gap-x-2 mb-1"
            v-for="(link, i) in content.data.externalLinks"
            :key="i"
            :href="link.link"
            >
              <Card moreTransparency >
                <strong v-if="link.english">[Contenido en Ingles]</strong>
                {{link.name}}
              </Card>
          </a>
        </div>
      </ContentRenderer>
    </div>
  </div>
  </Teleport>
</template>

<script setup>
import { useRoute } from 'vue-router'
import { ref, onMounted } from 'vue'
import Roadmap from '@/components/Roadmap.vue'
import HeroSection from '@/layouts/hero.vue'
// import DropDown from '@/components/dropdown/container.vue'

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

const closeSidebar = () => {
  content.value = null;
  document.body.classList.remove('overflow-hidden')
}
const sidebarClass = computed(() => showSidebar.value ? 'w-2/4' : 'w-screen')
</script>
