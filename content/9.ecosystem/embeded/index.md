# Sistemas Embebidos con Rust
Rust es una excelente opción para el desarrollo de sistemas embebidos debido a su rendimiento y seguridad. Aquí hay algunas de las principales bibliotecas y herramientas utilizadas en el desarrollo de sistemas embebidos.

## 1. Embedded-hal (Hardware Abstraction Layer)
Proporciona una capa de abstracción de hardware para diversos periféricos como **GPIO**, **SPI**, **I2C**, **UART**, etc.

**Características**

Define una serie de traits que las bibliotecas específicas de hardware pueden implementar, lo que permite escribir código genérico que puede funcionar en diferentes plataformas.

[Repositorio](https://github.com/rust-embedded/embedded-hal)

## 2. Cortex-m (para microcontroladores ARM Cortex-M)
Es una biblioteca específica para trabajar con microcontroladores **ARM Cortex-M**.

**Características**

* *cortex-m*: Proporciona controladores para los periféricos internos y funciones de bajo nivel.

[Repositorio](https://github.com/rust-embedded/cortex-m)



## 3.  Stm32-rs (para microcontroladores STM32)
Es una colección de crates para trabajar con diferentes familias de microcontroladores **STM32**.

**Características**
Proporciona acceso a los registros de los periféricos y soporte para diversas funciones específicas de los microcontroladores STM32.

[Repositorio](https://github.com/stm32-rs)

## 4. Nrf52-hal (para microcontroladores nRF52)
es una biblioteca para trabajar con microcontroladores **nRF52** y **nRF52840**.

**Características**
Proporciona abstracciones de alto nivel para periféricos como **GPIO**, **SPI**, **I2C**, **UART**, **RTC**, **PWM**, **ADC**, etc.

[Repositorio](https://github.com/nrf-rs/nrf-hal)


## 5. Avr-hal (para microcontroladores AVR)
Proporciona soporte para microcontroladores **AVR**, como los que se encuentran en las placas Arduino.

**Características** 
Ofrece implementaciones **HAL** para varios periféricos y soporta muchas de las **placas Arduino populares**.

[Repositorio](https://github.com/Rahix/avr-hal)

## 6. Embedded-graphics
Es una biblioteca para dibujar gráficos en pantallas embebidas.

**Características** 
Soporte para diferentes tipos de pantallas (**monocromáticas**, en **escala de grises**, **RGB**) y varias primitivas de dibujo como **líneas**, **rectángulos**, **círculos**, y **texto**.

[Repositorio](https://github.com/embedded-graphics/embedded-graphics)

## 7. Heapless
Es una biblioteca para trabajar con estructuras de datos que no utilizan el heap, adecuada para sistemas embebidos con recursos limitados.

**Características**
Proporciona implementaciones de varios contenedores como **vectores**, **colas**, **pilas**, y más, sin asignación dinámica de memoria.

[Repositorio](https://github.com/rust-embedded/heapless)

## 8. Rtic (Real-Time Interrupt-driven Concurrency)
Es un **framework** para construir aplicaciones concurrentes en sistemas embebidos.

**Características** 
Proporciona un modelo de programación basado en interrupciones, facilitando la escritura de código concurrente seguro y eficiente.

[Repositorio](https://github.com/rtic-rs/rtic)


## 9. [Espressif] espup
Es una herramienta para instalar y mantener las cadenas de herramientas necesarias para desarrollar aplicaciones para **SoC** de Espressif.

[Repositorio](https://github.com/esp-rs/espup)

## 10. [atsamd] atsamd
Es una biblioteca para trabajar con microcontroladores **SAMD** de Microchip.

[Repositorio](https://github.com/atsamd-rs/atsamd)

## 11. [SAM E70/S70/V70/V71] atsamx7x-rust
Es una biblioteca para trabajar con la familia de microcontroladores **SAMx7x** de Microchip.

[Repositorio](https://github.com/atsams-rs/atsamx7x-rust)

## 12. [Raspberry Pi RP2040] rp2040
Es una biblioteca para trabajar con el microcontrolador **RP2040** de Raspberry Pi.

[Repositorio](https://github.com/rp-rs/rp-hal)
