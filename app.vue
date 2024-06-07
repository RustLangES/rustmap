<script setup>
  import Roadmap from './components/Roadmap.vue'
  import HeroSection from './layouts/hero.vue'
  const title = 'Hoja de Ruta Definitiva para Aprender Rust: Desde Principiante hasta Experto'
  const description = `
¿Estás listo para dominar uno de los lenguajes de programación más potentes y eficientes? Nuestra hoja de ruta te guiará paso a paso en tu viaje de aprendizaje de Rust, desde los conceptos básicos hasta las técnicas avanzadas. Diseñada para principiantes y desarrolladores experimentados, esta guía exhaustiva te ayudará a construir una sólida base en Rust y a aprovechar al máximo su rendimiento, seguridad y concurrencia.
Aprenderás a través de ejemplos prácticos, ejercicios desafiantes y proyectos reales, lo que te permitirá aplicar tus nuevas habilidades de inmediato. Además, explorarás las mejores prácticas y las últimas tendencias en el ecosistema de Rust, asegurándote de estar siempre un paso adelante.
Ya sea que desees desarrollar aplicaciones de sistemas, videojuegos, criptomonedas o cualquier otro proyecto, nuestra hoja de ruta te preparará para dominar Rust y destacarte en el mundo del desarrollo de software. ¡Comienza tu viaje hoy y conviértete en un experto en Rust!
    `
  useHead({
    bodyAttrs: {
      class: 'bg-orange-200 dark:bg-[#131313]/90 w-screen min-h-screen bg-center bg-fixed dark:bg-kaku dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]'
    }
  })

  useSeoMeta({
    title,
    description,
    ogTitle: title,
    ogDescription: description,
    twitterTitle: title,
    twitterDescription: description,
    twitterCard: 'summary_large_image',
    icon: '/favicon.ico',
    lang: 'es',
    ogImage: '[og:image]',
    twitterImage: '[twitter:image]'
  })

  // defineOgImageComponent('Home', { title })
</script>

<template>
  <Header />
  <HeroSection />
  <Roadmap />
</template>
