# Trabajos Prácticos

Los TPs de la cursada, resueltos en Rust. Cada capítulo contiene el problema
planteado, la solución comentada, las preguntas abiertas de la consigna y las
conclusiones, más enlaces al código fuente y al enunciado original.

El hilo conductor va de menor a mayor control sobre la concurrencia: un servidor
secuencial (TP1), un thread por conexión (TP2), un pool acotado (TP3), control
explícito de acceso a recursos compartidos (TP4), sincronización sin locks (TP5),
paralelismo sobre datos (TP Grep) y, finalmente, async frente a threads (TP7).

| TP | Tema | Crates |
|----|------|--------|
| [TP 1](./tp1.md) | Servidor de Leibniz, single-threaded | `tp1`, `leibniz-calculator` |
| [TP 2](./tp2.md) | Un hilo por conexión | `tp2` |
| [TP 3](./tp3.md) | Thread Pool | `tp3`, `thread_pool` |
| [TP 4](./tp4.md) | Análisis de logs con control de concurrencia | `tp4`, `connection-handler` |
| [TP 5](./tp5.md) | Cola no bloqueante | `tp5/Queue` |
| [TP Grep](./tp-grep.md) | Búsqueda concurrente en archivos | `tp_grep/grep-app`, `tp_grep/grep-lib` |
| [TP 7](./tp7.md) | I/O bloqueante vs async vs threads | `tp7` |

El código vive en [`tps/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tps),
fuera de `src/`, para no arrastrar los recursos de prueba al sitio publicado.

## Presentación

Estos capítulos se derivan de la presentación de cierre de la materia, que se
conserva íntegra en formato [Marp](https://marp.app):
[`tps/docs/presentation/presentation.md`](https://github.com/FranCalveyra/concurrent-programming/blob/main/tps/docs/presentation/presentation.md).
