<template>
  <Header />
  <HeroSection />
  <Roadmap />
  <Teleport to="body">
  <div v-if="content" :class="['fixed top-0 right-0 h-screen', sidebarClass]">
    <div
        class="prose dark:prose-invert flex overflow-y-auto overflow-x-hidden h-full w-full flex-col items-center p-4 focus:outline-0 sm:p-6 bg-orange-50 dark:bg-gray-800"
    >
      <div class="w-full flex flex-row justify-between px-4 pb-2">
        <Dropdown ref="statusDropDown" :border="false">
          <!-- trigger element -->
          <template #trigger>
            <button type="button" :class="['px-4 py-2 border-black border', status.toLowerCase()]">{{status}}</button>
          </template>

          <!-- contents display in dropdown -->
          <ul class="flex flex-col bg-orange-100 dark:bg-gray-600">
            <li
              v-for="(s, i) in allStatus"
              :key="'status-' + i"
              @click="changeStatus(s)"
              :class="['px-4 py-2 hover:cursor-pointer border-b border-black hover:bg-orange-50 dark:hover:bg-gray-500', s.toLowerCase()]"
              v-text="s"
            >
            </li>
          </ul>
        </Dropdown>
        <span class="hover:cursor-pointer" @click="closeSidebar">X</span>
      </div>
      <ContentRenderer :key="content._id" :value="content">
        <ContentRendererMarkdown class="flex flex-col w-full m:max-w-[800px] sm:max-w-[600px]" tag="article" :value="content" />
        <div class="flex flex-col items-start w-full m:max-w-[800px] sm:max-w-[600px]">
          <h4 id="related-content" class="mb-3">
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

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref, onMounted } from 'vue'
import Roadmap from '@/components/Roadmap.vue'
import HeroSection from '@/layouts/hero.vue'
import Dropdown from 'v-dropdown'

const { $locally } = useNuxtApp()

const allStatus = [ 'Pendiente', 'Leyendo', 'Completado', 'Omitir' ]
const route = useRoute()
const nodeId = route.params.slug
const showSidebar = ref(true)
const content = ref(null)
const statusDropDown = ref(null)
const status = ref($locally.get(nodeId) ?? 'Pendiente')


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

function changeStatus(val) {
  if (val == status.value) return
  if (!statusDropDown.value) return
  status.value = val
  $locally.set(nodeId, val)
  statusDropDown.value.close()
}
</script>

<style>
.pendiente::before,
.leyendo::before,
.completado::before,
.omitir::before {
  content: '•';
  margin-right: 5px;
}

.pendiente::before { color: var(--tw-text-gray-950); }
.leyendo::before { color: var(--tw-text-orange-500); }
.completado::before { color: var(--tw-text-green-600); }
.omitir::before { color: var(--tw-text-slate-500); }
</style>
