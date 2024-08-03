<template>
  <Header />
  <HeroSection />
  <Roadmap />
  <Teleport to="body">
    <div v-if="content" :class="['fixed top-0 h-screen', sidebarClass]">
      <div
        id="sidebar"
        class="scroll-smooth prose dark:prose-invert max-w-full flex overflow-y-auto overflow-x-hidden h-full w-full flex-col items-center p-4 focus:outline-0 sm:p-6 bg-orange-50 dark:bg-[#131313]/90 border-l-2 dark:backdrop-blur-xl border-black dark:border-0"
        @scroll="onScroll"
      >
        <div
          :class="[
            'sticky border-b -top-6 bg-orange-50 dark:bg-[#131313]/90 w-full flex flex-row items-center justify-between px-4',
            !isScrolled
              ? '!border-transparent pb-2'
              : 'border-gray-300 dark:border-white mb-2 py-2  dark:backdrop-blur-2xl',
          ]"
        >
          <client-only>
            <ReadStatus :name="nodeId" />
          </client-only>
          <span
            class="hover:cursor-pointer dark:text-white"
            @click="closeSidebar"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              fill="currentColor"
              viewBox="0 0 256 256"
            >
              <path
                d="M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z"
              ></path>
            </svg>
          </span>
        </div>
        <ContentRenderer :key="content._id" :value="content">
          <ContentRendererMarkdown
            class="flex flex-col w-full m:max-w-[800px] sm:max-w-[600px]"
            tag="article"
            :value="content"
          />
          <div
            class="flex flex-col items-start w-full m:max-w-[800px] sm:max-w-[600px]"
          >
            <h4 id="related-content" class="mb-3">
              <a href="#related-content">Contenido Extra Relacionado</a>
            </h4>
            <a
              class="gap-x-2 mb-1 no-underline"
              v-for="(link, i) in content.data.externalLinks.sort(
                (a, b) => a.english - b.english
              )"
              :key="i"
              :href="link.link"
              target="_blank"
            >
              <Card
                moreTransparency
                class="border-gray-300 hover:text-gray-100 transition-colors dark:border-white dark:hover:bg-black dark:bg-transparent"
              >
                <strong v-if="link.english">[Contenido en Ingles]</strong>
                {{ link.name }}
              </Card>
            </a>
          </div>
        </ContentRenderer>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { ref, onMounted } from "vue";
import Roadmap from "@/components/Roadmap.vue";
import HeroSection from "@/layouts/hero.vue";
import ReadStatus from "@/components/sidebar/ReadStatus.vue";

const { push } = useRouter();
const route = useRoute();
const nodeId = route.params.slug;
const showSidebar = ref(true);
const content = ref(null);
const isScrolled = ref(false);

useSeoMeta({
  twitterCard: 'summary_large_image',
  icon: '/favicon.ico',
  lang: 'es',
  ogImage: `/previews/${(nodeId && nodeId.join('-')) || 'ogpreview'}.png`,
  twitterImage: `/previews/${(nodeId && nodeId.join('-')) || 'ogpreview'}.png`,
})

onMounted(async () => {
  if (!nodeId) return;
  const contentResult = await queryContent((nodeId || []).join("/")).findOne();
  content.value = contentResult;
  showSidebar.value = contentResult && (route.query.fromClick || false);
});

const closeSidebar = () => {
  content.value = null;
  document.body.classList.remove("overflow-hidden");
  push({ path: "/" });
};
const onScroll = (e) => {
  const { scrollTop, offsetHeight, scrollHeight } = e.target;
  if (scrollTop !== 0) {
    isScrolled.value = true;
  } else {
    isScrolled.value = false;
  }
};
const sidebarClass = computed(() =>
  showSidebar.value ? "right-0 w-screen lg:w-2/4" : "w-screen"
);
</script>

<style>
article [id] {
  scroll-margin-top: 3.5rem;
}
</style>
