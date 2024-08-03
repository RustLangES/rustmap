const generateContentRoutes = require('./generateContentRoutes');

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@nuxt/content", "@nuxtjs/tailwindcss", "@nuxtjs/seo"],
  site: {
    url: 'https://roadmap.rustlang-es.org',
    name: 'Hoja de Ruta Definitiva para Aprender Rust',
    description: 'Nuestra hoja de ruta te guiará paso a paso en tu viaje de aprendizaje de Rust diseñada para principiantes y desarrolladores experimentados',
    defaultLocale: 'es'
  },
  ogImage: { enabled: false },
  nitro: {
    prerender: {
      routes: ['/sitemap.xml'],
    }
  },
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      link: [
        { rel: "preconnect", href:"https://fonts.googleapis.com" },
        { rel: "preconnect", href:"https://fonts.gstatic.com", crossorigin: true },
        { rel: 'stylesheet', href:"https://fonts.googleapis.com/css2?family=Work+Sans:ital,wght@0,100..900;1,100..900&display=swap" },
        { rel: 'stylesheet', href:"https://fonts.googleapis.com/css2?family=Alfa+Slab+One&family=Work+Sans:ital,wght@0,100..900;1,100..900&display=swap" },
      ]
    }
  },
  content: {
    highlight: {
      langs: ['c', 'cpp', 'rs', 'java', 'js', 'cs', 'asm', 'toml', 'console', 'sh', 'bash', "vim"],
      // Themes from https://github.com/shikijs/textmate-grammars-themes/tree/main/packages/tm-themes
      theme: {
        default: 'vitesse-dark',
        dark: 'vitesse-dark',
        sepia: 'monokai'
      }
    },
  },
  hooks: {
    async 'prerender:routes'(ctx) {
      const contentRoutes = await generateContentRoutes();

      for (const route of contentRoutes) {
        ctx.routes.add(route)
      }
    }
  }
})
