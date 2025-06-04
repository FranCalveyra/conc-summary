---
layout: home
---

# Resumen de Programación Concurrente

Esta página, creada con Jekyll, recopila las explicaciones y apuntes de los conceptos clave vistos en la materia de Programación Concurrente. A continuación se listan los temas principales. También puedes consultar ejemplos de código en la [Carpeta de Práctica](https://github.com/FranCalveyra/conc-summary/Práctica), donde encontrarás implementaciones y ejercicios resueltos.

## Temas vistos

- **Thread Programming**  
  Introducción a la creación y manejo de hilos (threads), su ciclo de vida, sincronización básica y comunicación entre hilos.

- **Parallelism**  
  Estrategias para dividir un problema en subproblemas que puedan ejecutarse en paralelo y aprovechar varios núcleos de CPU.

- **Mutual Exclusion**  
  Exclusión mutua para evitar condiciones de carrera: secciones críticas y protocolos básicos como test-and-set.

- **Concurrency Abstractions**  
  Mecanismos de más alto nivel para coordinar hilos:  
  - **Locks**  
    Cerraduras simples para proteger secciones críticas.  
  - **Reader‐Writer Locks**  
    Locks que permiten múltiples lectores simultáneos o un único escritor.  
  - **Semaphores**  
    Contadores sincronizados que controlan el acceso a recursos compartidos.  
  - **Condition Variables**  
    Variables de condición que permiten a un hilo esperar hasta que se cumpla cierta condición, liberando antes el mutex.  
  - **Monitors**  
    Abstracción que combina mutex y condition variables en un solo bloque para proteger datos compartidos.  
  - **Messages**  
    Comunicación mediante envío y recepción de mensajes entre hilos, evitando acceso directo a variables compartidas.

- **Mutex Implementation**  
  Ejemplos de implementación de mutex en bajo nivel usando operaciones atómicas (`test-and-set`, `compare-and-swap`).

- **Non Blocking Algorithms**  
  Algoritmos lock-free y wait-free: pilas y colas no bloqueantes, contadores atómicos con backoff y estructuras de datos concurrentes sin mutex.

- **Asynchronicity**  
  Programación asíncrona con corutinas (en Kotlin):  
  - `suspend fun`, `async/await`  
  - `Channel` y `Flow`  
  - Timeouts y manejo de errores

- **Actors (en Scala)**  
  Modelo de actores usando Akka: cada actor mantiene su propio estado y coordina con otros mediante mensajes, evitando la necesidad de memoria compartida.

---

## Ejemplos de código en la Carpeta de Práctica

Para ver implementaciones concretas y ejercicios resueltos en cada uno de los temas anteriores, visita la siguiente ruta dentro de este sitio:

[Carpeta de Práctica](https://github.com/FranCalveyra/conc-summary/Práctica)

Allí encontrarás carpetas organizadas por tema con el código fuente y las explicaciones correspondientes.
