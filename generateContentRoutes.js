const fs = require('fs').promises;
const path = require('path');

const contentDir = path.join(process.cwd(), 'content');

async function walkDir(dir) {
  let filesList = [];
  const files = await fs.readdir(dir, { withFileTypes: true });
  for (const file of files) {
    const res = path.resolve(dir, file.name);
    if (file.isDirectory()) {
      filesList = filesList.concat(await walkDir(res));
    } else if (file.isFile() && path.extname(file.name).toLowerCase() === '.md' || path.extname(file.name) != 'yml') {
      filesList.push(res);
    }
  }
  return filesList;
}

function filePathToRoute(filePath) {
  let route = filePath
    .replace(contentDir, '')
    .replace(/\.md$/, '')
    .replace(/\\/g, '/'); // Reemplazar backslashes con forward slashes para compatibilidad

  // Manejar index.md
  if (route.endsWith('/index')) {
    route = route.replace('/index', '');
  }

  // Manejar _dir.md (directorios)
  route = route.replace(/\/_([^/]+)/g, '');

  // Eliminar números al inicio de los segmentos de la ruta
  route = route.split('/').map(segment => segment.replace(/^\d+\./, '')).join('/');

  // Asegurarse de que la ruta comience con '/'
  if (!route.startsWith('/')) {
    route = '/' + route;
  }

  return route || '/';
}

async function generateContentRoutes() {
  const filePaths = await walkDir(contentDir);
  const routes = filePaths.map(filePath => filePathToRoute(filePath));
  return routes;
}

generateContentRoutes()
  .then(routes => {
    console.log('Content Routes:');
    console.log(JSON.stringify(routes, null, 2));
  })
  .catch(error => {
    console.error('Error generating routes:', error);
  });

module.exports = generateContentRoutes;
