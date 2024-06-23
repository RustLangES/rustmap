<template>
  <div class="fixed right-0 top-0 z-40 w-screen h-screen">
    <div
        v-if="content"
        class="right-0 top-0 flex h-full w-full overflow-y-auto flex-col items-center bg-white p-4 focus:outline-0 sm:p-6"
    >
      <ContentRenderer :key="content._id" :value="content">
        <ContentRendererMarkdown class="flex flex-col gap-3 text-black m:max-w-[800px] sm:max-w-[600px]" :value="content" />
        <div class="flex flex-col items-start w-full m:max-w-[800px] sm:max-w-[600px]">
          <h4 id="related-content" class="text-black mb-3">
            <a href="#related-content">Contenido Extra Relacionado</a>
          </h4>
          <a
            class="text-black mb-1"
            v-for="(link, i) in content.data.externalLinks"
            :key="i"
            :href="link.link"
          >
            <span v-if="link.english">[Contenido en Ingles]</span>
            {{link.name}}
          </a>
        </div>
      </ContentRenderer>
    </div>
  </div>
  <Header />
  <HeroSection />
  <Roadmap />
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
  content.value = await queryContent(nodeId).findOne()
  console.log(content)

  showSidebar.value = content && (route.query.fromClick || false)
})
</script>
