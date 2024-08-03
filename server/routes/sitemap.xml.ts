import { serverQueryContent } from '#content/server'
import { SitemapStream, streamToPromise } from 'sitemap'

export default defineEventHandler(async (event) => {
  // Fetch all documents
  const articles = await serverQueryContent(event).find()
  const sitemap = new SitemapStream({ hostname: 'https://roadmap.rustlang-es.org/' })

  console.log(articles)

  articles.forEach(article => sitemap.write({ url: article._path, img: [ {
    url: `https://roadmap.rustlang-es.org/previews/${article._path.substring(1).replaceAll('/', "-")}.png`,
    caption: article.description,
    title: article.title,
  }, ] }))
  sitemap.end()

  return (await streamToPromise(sitemap))
})
