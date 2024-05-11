<script setup>
  import { ref } from 'vue'
  import Card from './Card.vue'

  const links = [
    {
        "id": "eVhn5ab1v4wQDqqqwWoBJ",
        "label": "Pagina Principal",
        "href": "web",
        "url": "https://rustlanges.github.io"
    },
    {
        "id": "A6oMZMVb0sueEBkyVGTJ6",
        "label": "Blog",
        "url": "https://rustlanges.github.io/blog"
    },
    {
        "id": "YxcctOkSbbBtaG8oDQEvP",
        "label": "GitHub",
        "url": "https://github.com/RustLangES"
    },
    {
        "id": "rzjRuAPd9oEN2Rihieal8",
        "label": "Linkedin",
        "url": "https://www.linkedin.com/company/rustlanges"
    },
    {
        "id": "6jH2huWW4SyF-ywHBf7hg",
        "label": "Discord",
        "url": "https://discord.gg/4ng5HgmaMg"
    }
  ]
  const topicClass = ref("")

  const changeLevel = (e) => {
    console.log(e)
  };
</script>

<template>
  <Card no-interactive>
    <div class="flex flex-col items-start gap-y-1">
      <a
        v-for="link in links"
        :key="link.id"
        @click="changeLevel"
        class="text-[0.5rem] w-full px-2 py-1 dark:text-orange-400 flex flex-row gap-x-2 hover:cursor-pointer"
        :class="'topic-' + link.label.toLowerCase()"
        :href="link.url"
        target="_blank"
        >
        {{ link.label }}
      </a>
    </div>
  </Card>
</template>

<style>
  .topic-start svg { width: 10px; fill: #359A19 }
  .topic-medium svg { width: 10px; fill: #702692 }
  .topic-hard svg { width: 10px; fill: #E78123 }
  .topic-other svg { width: 10px; fill: #999999 }
</style>
