# Código fuente

Todo el código vive en [`tps/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps),
fuera de `src/`, para que mdBook no arrastre los recursos de prueba al sitio
publicado. Cada TP es un crate independiente con su propio `Cargo.toml`.

## Binarios por TP

| Crate | TP | Qué hace |
|-------|----|----------|
| [`tp1`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp1) | [TP 1](./tp1.md) | Servidor HTTP secuencial que calcula la serie de Leibniz |
| [`tp2`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp2) | [TP 2](./tp2.md) | Igual que TP1 pero con `thread::spawn` por conexión |
| [`tp3`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp3) | [TP 3](./tp3.md) | Servidor sobre un pool de hilos acotado |
| [`tp4`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp4) | [TP 4](./tp4.md) | Analizador de logs con control de acceso concurrente |
| [`tp5/Queue`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp5/Queue) | [TP 5](./tp5.md) | Cola bloqueante y no bloqueante, lado a lado |
| [`tp7`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp7) | [TP 7](./tp7.md) | Simulación comparando I/O bloqueante, async y threads |

## Bibliotecas reutilizables

Extraídas para que los TPs posteriores no duplicaran el trabajo de los anteriores:

| Crate | Usado por | Qué expone |
|-------|-----------|------------|
| [`leibniz-calculator`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/leibniz-calculator) | TP 1, TP 7 | Cálculo secuencial y paralelo de la serie |
| [`thread_pool`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/thread_pool) | TP 3 | `ThreadPool` con `Worker`s y canal de trabajos |
| [`connection-handler`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/connection-handler) | TP 4 | Parseo de requests, respuestas y manejo de errores |

## TP Grep

Separado en biblioteca y aplicación para poder testear las estrategias de
búsqueda sin levantar el binario:

| Crate | Qué expone |
|-------|------------|
| [`tp_grep/grep-lib`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp_grep/grep-lib) | Estrategias secuencial, concurrente y *c-chunk*, más los archivos de prueba |
| [`tp_grep/grep-app`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/tp_grep/grep-app) | CLI que consume la biblioteca |

## Enunciados

Los PDFs originales de cada consigna están en
[`tps/pdfs/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps/pdfs).

> **Nota**: `TP_6_Atomics.pdf` está en la carpeta pero no tiene código ni
> capítulo asociado — la numeración salta del TP 5 al TP Grep.
