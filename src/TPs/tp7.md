# TP 7 — I/O bloqueante vs async vs threads

# Trabajo Práctico N°7
## Simulación I/O vs Async y Threads
### Problema planteado:
- Comparar los tiempos de ejecución para I/O simulado entre threads y async con diferentes valores de tareas.
- Comparar los tiempos de ejecución para cálculo de Pi intensivo en CPU entre threads y async.
- Experimentar con diferentes cantidades de tareas y divisiones para evaluar la escalabilidad de cada enfoque.

### Solución

#### I/O simulado

```rust
// Async version
impl Simulation for AsyncSimulation  {
    async fn simulate_io_tasks(&self, tasks: usize) {
        let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_| {
            let handle = tokio::spawn(async {
                tokio::time::sleep(Duration::from_millis(500)).await;
                println!("sleeping 500 milliseconds");
            });
            handles.push(handle);
        });

        for handle in handles {
            handle.await.unwrap_or_else(|_| { println!("Errorubi"); });
        }
    }
}

// Thread version
impl Simulation for ThreadSimulation {
    async fn simulate_io_tasks(&self, tasks: usize) {
        let mut handles: Vec<JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_| {
            let handle = thread::spawn(|| {
                thread::sleep(Duration::from_millis(500));
                println!("sleeping 500 milliseconds");
            });
            handles.push(handle);
        });
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

#### Cálculo de Pi

```rust
pub fn calculate_leibniz_thread(term: usize, parts: usize) -> f64 {
    let parts_ranges = split_range(term, parts);
    let handles: Vec<JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_concurrent(start, end)
    ).collect();

    handles.into_iter().map(|handle| handle.join().unwrap()).sum::<f64>()
}

pub async fn calculate_leibniz_async(term: usize, parts: usize) -> f64 {
    let parts_ranges = split_range(term, parts);
    let handles: Vec<tokio::task::JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_async(start, end)
    ).collect();
    let mut result: f64 = 0.0;
    for handle in handles {
        result += handle.await.unwrap();
    }
    result
}
```

### Splitting methods
#### Async
```rust
fn split_async(start: usize, end: usize) -> tokio::task::JoinHandle<f64> {
    tokio::spawn(async move{leibniz_pi_partial(start, end)})
}
```
- Este método crea una task de tokio
#### Thread
```rust
fn split_concurrent(start: usize, end: usize)->JoinHandle<f64>{
    thread::spawn(move || leibniz_pi_partial(start, end))
}
```
- Mientras que este crea un thread a nivel S.O.

# Preguntas TP7
1. Compare los tiempos de ejecución  para I/O simulado entre `threads` y  `async` con diferentes valores de  tasks. ¿Cuál es más eficiente en  manejar muchas tareas con esperas?
2. Compare los tiempos de ejecución  para cálculo de Pi intensivo en CPU  entre `threads` y `async`. ¿Qué modelo se desempeña mejor?
3. Experimente con diferentes cantidades de tareas y términos para Pi para evaluar cómo escala cada  enfoque.

# Conclusiones TP7
1. En estos contextos de I/O simulado con varias esperas, para cantidades de tareas relativamente pequeñas tienen desempeños similares, pero a medida que incrementa, el modelo `async` se desenvuelve mejor, ya que las `tasks` de `tokio` no tienen cambio de contexto.
2. Para cálculos que requieren CPU intensivo se desenvuelve mejor el modelo de `threads`, ya que se paralelizan verdaderamente los cálculos, mientras que el modelo `async` es **single-threaded**.
3. Al aumentar tareas y términos  para Pi, el enfoque de `threads`  escala hasta saturar los cores  disponibles (con menor ganancia tras ese punto por overhead), mientras que `async` no mejora en uso de CPU y su rendimiento se mantiene estable pero inferior en carga de cálculo.

---

## Código fuente

- [`tp7/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp7)

## Enunciado

[TP_7_Async.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP_7_Async.pdf)
