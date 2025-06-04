---
layout: page
title: Clase 7
nav_order: 7
---
# Algoritmos No Bloqueantes

Hasta ahora bloqueábamos el acceso (Mutex, Condvars, Locks) al resto de hilos para evitar condiciones de carrera.
Con este tipo de algoritmos vamos a tratar de resolver los problemas de los algoritmos bloqueantes, que son:

- **Performance**: se reduce la performance bajo alta concurrencia, debido a tener que contener el **Lock**
- **Deadlocks**
- **Uso de recursos**: teniendo threads esperando, se puede dar un uso ineficiente de los recursos del sistema.

## Ventajas de algoritmos no bloqueantes

- **Aumento de eficiencia**: operaciones más granulares
- **Escalabilidad**: operaciones concurrentes sin locks
- **Inexistencia de Deadlocks**

## Variables Atómicas

### Rol

- Operaciones atómicas: permiten operaciones atómicas
    - Las operaciones atómicas se hacen en un único paso no divisible. No puedo tener el problema del lock porque no lo
      puedo "partir al medio".
- Integridad de los datos: asegura integridad sin usar locks
- Utilidad:
    - Contadores y estadísticas
    - Implementaciones concurrentes "lockless" de estructuras de datos

### Implementación de un counter usando AtomicInteger

```java
// Sin usar Atomic, deberíamos usar el keyword synchronized.
public class AtomicCounter {
    AtomicInteger value = new AtomicInteger(0);

    void increment() {
        value.incrementAndGet(); // Análogo a un value++
    }

    int getValue() {
        return value.get(); // return value
    }
}
```

### Operaciones típicas sobre variables atómicas:

- `get()`, `set(int newValue)`, `getAndSet()`
- `compareAndSet(int expect, int update)`: compara el valor actual con el esperado y si son iguales lo cambia al nuevo
  valor.
- `getAndIncrement()`, `getAndDecrement()`, `getAndAdd(int delta)`
- `getAndUpdate(IntUnaryOperator lambda)`:
    - `IntUnaryOperator` es una interfaz funcional que recibe un int y devuelve un int.
    - `getAndUpdate` aplica la función al valor actual y lo actualiza.

### En un lenguaje de verdad como Rust

```rust
struct Counter {
    value: AtomicU64
}

impl Counter {
    // Initialize a new counter
    fn new() -> Counter { Counter { value: AtomicU64::new(0) } }

    // Increment the counter by 1
    fn increment(&self) {
        // Relaxed ordering is often sufficient for simple counters.
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    // Get the current value of the counter
    fn get(&self) -> usize { self.value.load(Ordering::Relaxed) }
}
```

#### Ordering

Es más de bajo nivel, justamente porque Rust permite hacer controles a bajo nivel del procesador.

Cada tipo de ordering tiene diferentes garantías a nivel CPU.
Refiere a cómo se ordenan las instrucciones a nivel procesador.

- Sequentially Consistent (`SeqCst`): más restrictivo, pero es el más lento. Debe funcionar para TODOS LOS CASOS.
    - Java lo usa por defecto.
    - No se puede reordenar las operaciones de lectura y escritura.
- `Release`: más restrictivo que `Relaxed`, pero menos que `Acquire`
- `Acquire`: más restrictivo que `Release`, pero menos que `SeqCst`
- `Relaxed`: menos restrictivo

##### Operaciones típicas

- `new(val: i32) -> AtomicI32`: lo crea
- `load(order: Ordering) -> i32`, `store(val: i32, order: Ordering)`: load lo lee, store le graba un nuevo valor
- `compare_exchange(expected: i32, new: i32, ...)`: si encuentra el valor, cambia por el valor del new y devuelve el
  valor viejo.
- `fetch_add(val: i32, order: Ordering) -> i32`, `fetch_sub(val: i32, order: Ordering) -> i32`: suman y restan,
  respectivamente
- `fetch_update<F>(set_order: Ordering, fetch_order: Ordering, lambda: F)`: exactamente igual al `getAndUpdate` de Java.
    - Le paso un lambda a aplicar sobre el valor almacenado.

## Estructuras de Datos No Bloqueantes

### Stack

```kotlin
class Stack<E> {
    class Node<E>(val item: E, var next: Node<E>? = null)

    private var top: Node<E>? = null

    fun push(item: E) {
        val newHead = Node(item)
        newHead.next = top // Me pueden interrumpir acá, y reemplazar el top
        top = newHead
    }

    // fun pop(): E? { ... }
}
```

### Non-Blocking Concurrent Stack

```kotlin
  class ConcurrentStack<E> {
    class Node<E>(val item: E, var next: Node<E>? = null)

    private var top = AtomicReference<Node<E>?>()

    fun push(item: E) {
        val newHead = Node(item)
        var oldHead: Node<E>?
        do {
            oldHead = top.get()
            newHead.next = oldHead
        } while (!top.compareAndSet(oldHead, newHead))
    }
    fun pop(): E? {
        var oldHead: Node<E>? = top.get()
        // Siempre que el top sea el mismo que el oldHead, lo reemplazo por el siguiente
        while (oldHead != null && !top.compareAndSet(oldHead, oldHead?.next))
            oldHead = top.get()
        return oldHead?.item
    }
}
```

### Queue

![A typical Queue implementation using a linked list](queue.png)

```kotlin

class Queue<E> {
    class Node<E>(val item: E?, var next: Node<E>? = null)

    val dummy = Node<E>(null)
    var head = dummy
    var tail = dummy

    fun enqueue(item: E) {
        val newNode = Node(item)
        tail.next = newNode
        tail = newNode
    }
    fun dequeue(): E? {
        val headNext = head.next ?: return null
        head = headNext
        return head.item
    }
}
```

### Non-Blocking Concurrent Queue

```kotlin
import java.util.concurrent.atomic.AtomicReference

class ConcurrentQueue<E> {
    class Node<E>(val item: E?, var next: AtomicReference<Node<E>>? = null)

    val dummy = Node<E>(null)
    val head = AtomicReference(dummy)
    val tail = AtomicReference(dummy)

    fun enqueue(item: E) {
        val newNode = Node(item)
        while (true) {
            val curTail = tail.get()
            val tailNext = curTail.next?.get()
            // Check if the tail has not moved, which could've happened given a context switch
            if (curTail == tail.get()) {
                if (tailNext != null) {
                    // Queue in intermediate state, advance tail (complete operation)
                    tail.compareAndSet(curTail, tailNext)
                }
                // If the next to tail is still the same, update the tail
                else if (curTail.next?.compareAndSet(null, newNode) == true) {
                    tail.compareAndSet(curTail, newNode)
                    return
                }
                // Try again
            }
        }
    }
}
```

## Problema ABA

El problema ABA es un problema que ocurre en algoritmos no bloqueantes cuando una variable es leída, luego se modifica y
finalmente se vuelve a modificar a su valor original. Esto puede llevar a que un hilo crea que la variable no ha
cambiado, cuando en realidad sí lo ha hecho.
El valor A cambia a B, y luego vuelve a su valor original A.

No es detectable por operaciones concurrentes, lo cual puede llevar a asunciones incorrectas.

### ¿Por qué es un problema?

- Operations like compare-and-swap (CAS) can be tricked into thinking no change occurred.
- Potentially causing incorrect program behavior.
- For example. If I pop an item from a stack and modify it if still is the same

### Soluciones posibles

- **Versioning**: agregar un contador o un timestamp a la variable, y cada vez que se modifica, se incrementa el
  contador.
    - ABA se vuelve A1 - B2 - A3.
- En Java se puede usar `AtomicStampedReference`, que es una referencia atómica que incluye un "timestamp" o versión.
    - `ref.compareAndSet(currentValue, newValue, currentStamp, newStamp);`
- En Rust no puede existir este problema. ¿Por qué?
    - Por el borrow checker y por la inexistencia del Garbage Collector. No puedo tener una pasa del GC en el medio de
      la operación.

## Pros y Contras de los Algoritmos No Bloqueantes

| Aspecto                | Pros                                           | Contras                                                   |
|------------------------|------------------------------------------------|-----------------------------------------------------------|
| Rendimiento            | Alto en baja contención.                       | Puede degradarse en alta contención.                      |
| Escalabilidad          | Mejorada debido a la ausencia de bloqueos.     | Limitada por la contención y el costo de reintentos.      |
| Interbloqueo           | Evitado por completo.                          | Pueden ocurrir livelocks.                                 |
| Simplicidad            | Directo para operaciones simples.              | Las operaciones complejas son difíciles de diseñar.       |
| Sobrecarga del Sistema | Menor, sin cambios de contexto.                | Aumentada por espera activa en contención.                |
| Recuperación           | Sin estados inconsistentes en fallos de hilos. | Recuperación compleja para mantener la consistencia.      |
| Equidad (fairness)     | No inherente; puede causar starvation.         | Difícil de garantizar la equidad.                         |
| Modelo de Memoria      | Puede ser eficiente con CPUs modernas.         | Requiere un entendimiento profundo para evitar problemas. |

> Alta contención es cuando múltiples threads frecuentemente intentan acceder y modificar el mismo recurso compartido al
> mismo tiempo.