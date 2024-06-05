---
title: 'Tu Primer Programa'
description: 'Escribiendo Tu Primer Programa en Rust: Una Guía Paso a Paso'
draft: true
data:
  type: 'custom'
  topicLevel: 'start'
  position:
    x: 620
    y: 320
  width: 320
  externalLinks:
    - name: '¡Hola, mundo!'
      english: false
      link: 'https://book.rustlang-es.org/ch01-02-hello-world'
    - name: '¡Hola, Cargo!'
      english: false
      link: 'https://book.rustlang-es.org/ch01-03-hello-cargo'
---
## Escribiendo Tu Primer Programa en Rust: Una Guía Paso a Paso

Rust es un lenguaje de programación moderno que se destaca por su seguridad y rendimiento. Si estás listo para dar tus primeros pasos en Rust, esta guía te ayudará a configurar tu entorno y a escribir tu primer programa sencillo. Vamos a crear un proyecto llamado "hola_mundo" utilizando Cargo, la herramienta de gestión de proyectos de Rust.

### ¿Qué es Cargo?

Cargo es la navaja suiza de Rust. Permite gestionar proyectos, probar código, manejar dependencias, y mucho más. Es una herramienta esencial para cualquier desarrollador de Rust.

### Paso 1: Crear un Proyecto con Cargo

Para crear tu primer proyecto con Cargo, abre una terminal y ejecuta el siguiente comando:

```sh
cargo new hola_mundo
```

Este comando creará un nuevo directorio y un proyecto llamado "hola_mundo". Cargo generará los archivos necesarios y un directorio para ti. Vamos a explorar lo que ha creado.

### Paso 2: Explorar la Estructura del Proyecto

Navega al directorio del proyecto y lista los archivos:

```sh
cd hola_mundo
ls
```

Deberías ver los siguientes elementos:

- `Cargo.toml`: Archivo de configuración de Cargo.
- `src`: Directorio de código fuente.
  - `main.rs`: Archivo principal de Rust que contiene el punto de entrada del programa.

Cargo también inicializa un nuevo repositorio Git junto con un archivo `.gitignore`.

### Paso 3: Entender el Código Generado

Abre el archivo `src/main.rs` en tu editor de texto favorito. Verás el siguiente código:

```rs
fn main() {
    println!("Hello, world!");
}
```

Vamos a desglosar este código línea por línea.

#### La Función `main`

```rs
fn main() {

}
```

Estas líneas definen una función llamada `main`. La función `main` es especial: siempre es el primer código que se ejecuta en cada programa ejecutable de Rust. Aquí, la primera línea declara una función llamada `main` que no tiene parámetros y no devuelve nada. Si hubiera parámetros, irían dentro de los paréntesis `()`.

El cuerpo de la función está envuelto en `{}`. Rust requiere llaves alrededor de todos los cuerpos de función. Es buena costumbre colocar la llave de apertura en la misma línea que la declaración de la función, agregando un espacio entre ambos.

#### El Cuerpo de la Función `main`

```rs
println!("Hello, world!");
```

Esta línea hace todo el trabajo en este pequeño programa: imprime texto en la pantalla.

`println!` llama a una macro de Rust. Si hubiéramos llamado a una función en su lugar, habríamos ingresado `println` (sin el `!`). Hablaremos de macros en Rust más adelante. Por ahora, solo necesitas saber que usar un `!` significa que estamos llamando a una macro en lugar de una función normal y que las macros no siempre siguen las mismas reglas que las funciones.

Terminamos la línea con un punto y coma (`;`), lo que indica que esta expresión ha terminado y la siguiente está lista para comenzar. La mayoría de las líneas de código de Rust terminan con un punto y coma.

### Paso 4: Ejecutar el Programa

Ahora que entendemos el código, es hora de ejecutarlo. En la terminal, ejecuta:

```sh
cargo run
```

Deberías ver una salida similar a esta:

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hola_mundo`
Hello, world!
```

Si todo funciona correctamente, ¡felicitaciones! Has escrito y ejecutado tu primer programa en Rust.
