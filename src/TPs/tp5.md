# TP 5 — Cola no bloqueante

# Trabajo Práctico N°5
## Cola No Bloqueante
### Problema planteado:
- Implementar una cola no bloqueante para múltiples productores y consumidores
### Solución:
```rust
impl<T> Queue<T> for NonBlockingQueue<T> {
    fn enqueue(&self, item: T) {...}

    fn dequeue(&self) -> Option<T> {...}
}
```

# Enqueue
```rust
fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node::new(item)));
        let acquire = Ordering::Acquire;
        let release = Ordering::Release;
        loop {
            let cur_tail = self.tail.load(acquire);
            let tail_next = unsafe { (*cur_tail).next.load(acquire) };
            if cur_tail == self.tail.load(acquire) {
                unsafe {
                    if !tail_next.is_null() {
                        self.tail.compare_exchange(cur_tail, tail_next, acquire, release);
                    } else if (*cur_tail)
                        .next
                        .compare_exchange(null_mut(), new_node, acquire, release)
                        .is_ok()
                    {
                        self.size.fetch_add(1, Ordering::Release);
                        let _ = self
                            .tail
                            .compare_exchange(cur_tail, new_node, acquire, release);
                        return;
                    }
                }
            }
        }
    }
```
---
# Dequeue
```rust
fn dequeue(&self) -> Option<T> {
        let acquire = Ordering::Acquire;
        let release = Ordering::Release;

        loop {
            let current_head_ptr = self.head.load(acquire);

            if current_head_ptr.is_null() {
                return None;
            }
            let next_node_ptr = unsafe { (*current_head_ptr).next.load(acquire) };

            if self
                .head
                .compare_exchange(current_head_ptr, next_node_ptr, release, acquire)
                .is_ok()
            {
                self.size.fetch_sub(1, Ordering::Release);
                let old_head_node = unsafe { Box::from_raw(current_head_ptr) };
                return old_head_node.item;
            }
        }
    }
```

---

# Preguntas TP5
1. ¿Qué diferencias observás en el rendimiento entre la versión bloqueante y la no bloqueante?
2. ¿Qué dificultades técnicas encontraste al implementar la versión no bloqueante?
3. ¿Bajo qué escenarios conviene usar cada una?
4. ¿Qué pasaría si se mezclan productores bloqueantes con consumidores no bloqueantes (o viceversa)?
---
# Conclusiones TP5
1. Bajo concurrencia baja, la versión bloqueante es más performante, mientras que bajo alta concurrencia gana la versión no bloqueante.
2. Principalmente:
   - Lidiar con el `ownership` de Rust
   - Saber cómo y dónde aplicar las operaciones `unsafe`
   - Entender el funcionamiento del `Ordering`
3. Precisamente, conviene usar cada una en los escenarios donde gana cada una (los previamente mencionados):
   - Si se va a tener una concurrencia relativamente baja, o pocas operaciones de acceso a memoria compartida, se usa la versión bloqueante
   - En caso contrario, se usa la versión bloqueante
4. Puede incurrir en un deadlock, livelock o memoria no liberada (por nodos sin referenciar).
---

---

## Código fuente

- [`tp5/Queue/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp5/Queue)

## Enunciado

[TP_5_Non_Blocking_Queue.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP_5_Non_Blocking_Queue.pdf)
