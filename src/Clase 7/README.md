# Algoritmos No Bloqueantes

Hasta ahora bloqueábamos el acceso (`Mutex`, `Condvars`, `Locks`) al resto de hilos para evitar condiciones de carrera.
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

- **Operaciones atómicas**
    - Se hacen en un único paso no divisible. No puedo tener el problema del lock porque no lo
      puedo "partir al medio".
- **Integridad de los datos**: asegura integridad sin usar locks
- **Utilidad**:
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

- `get()`, `set(int newValue)`, `getAndSet(int newValue)`
- `compareAndSet(int expect, int update)`: compara el valor actual con el esperado y si son iguales lo cambia al nuevo
  valor.
- `getAndIncrement()`, `getAndDecrement()`, `getAndAdd(int delta)`
- `getAndUpdate(IntUnaryOperator lambda)`:
    - `IntUnaryOperator` es una interfaz funcional que recibe un int y devuelve un int.
      - Tiene una estructura como esta: `int func(int x)`
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

`Ordering` es un enum que especifica las garantías de visibilidad y orden de las operaciones atómicas entre hilos. En el contexto de algoritmos no bloqueantes, elegir el nivel de Ordering adecuado es clave para asegurar corrección (sin data races) y optimizar rendimiento (minimizando barreras de memoria).

Los órdenes son los siguientes:

- Sequentially Consistent (`SeqCst`): más restrictivo, pero es el más lento. Debe funcionar para TODOS LOS CASOS.
    - Todas las operaciones atómicas con `SeqCst` aparecen en un único orden global, simplificando el razonamiento, pero teniendo como trade-off un mayor costo en barreras de memoria.
      - Tiene que poner barreras de memoria, justamente para asegurar la consistencia.
    - Si se declara un acceso como `SeqCst`, ese acceso se queda anclado ahí.
    - No se pueden reordenar las operaciones de lectura y escritura.
    - `Java` lo usa por defecto.

- `AcqRel`: combinación de Acquire y Release
  - Combina Acquire y Release en una operación de lectura-modificación-escritura (por ejemplo, fetch_add), ideal para estructuras de lock-free donde una sola operación hace ambas cosas. 

- `Acquire`: más restrictivo que `Release`, pero menos que `AcqRel`
  - Asegura que ninguna lectura/escritura posterior al “load” pueda reordenarse antes de él. Se sincroniza con un `Release` correspondiente para “ver” los efectos previos al `store`.
  - Como se usa en conjunto con `Release`, lo que se busca (o al menos su caso de uso inicialmente intencionado) es "adquirir y liberar locks".
    - Lo que intentan ambos orderings es que las secciones críticas del programa no se solapen.
  - Todos los accesos posteriores a un `Acquire` se van a ejecutar después de este.
  - No hay ninguna garantía de que se las operaciones anteriores no se reordenen para ejecutarse después.
  - El caso de uso de estos 2 en conjunto es bastante simple:
    - Adquiero el "lock" al comenzar una sección crítica con `Acquire` y cuando termino lo libero usando `Release` (normalmente usando la operación atómica `store`).

- `Release`: más restrictivo que `Relaxed`, pero menos que `Acquire`
  - Garantiza que ninguna lectura/escritura previa al “store” pueda reordenarse después de él. Permite publicar cambios en memoria antes de que otro hilo los observe con un `Acquire`.
  - Todos los accesos anteriores a un `Release` se van a ejecutar antes de este.
  - No hay ninguna garantía de que se las operaciones posteriores no se reordenen para ejecutarse antes.

- `Relaxed`: menos restrictivo
  - Solo garantiza la atomicidad de la operación: no impone ningún orden relativo con otras lecturas o escrituras. Útil cuando solo importa el valor atómico, no la sincronización con otros datos.

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
![A typical Queue implementation using a linked list](queue.png)
La idea de esta implementación no bloqueante es poder completarle la operación a otro hilo, en caso de que se encuentre en un estado intermedio. Es decir, si tengo la `Queue` en el estado de la foto, donde tengo que mover el puntero, otro hilo puede **completarme la operación** si se da un _cambio de contexto_ .

En el caso del Stack es lo mismo.

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

- Las operaciones como `compare-and-swap` (CAS) pueden ser "engañadas" para que piensen que no ocurrió ningún tipo de cambio
- Esto potencialmente puede causar un comportamiento incorrecto del programa
- Por ejemplo, si poppeo un item de un stack y lo vuelvo a pushear

### Soluciones posibles

- **Versioning**: agregar un contador o un timestamp a la variable, y cada vez que se modifica, se incrementa el
  contador.
    - ABA se vuelve A1 - B2 - A3.
- En Java se puede usar `AtomicStampedReference`, que es una referencia atómica que incluye un "timestamp" o versión.
    - `ref.compareAndSet(currentValue, newValue, currentStamp, newStamp);`
- En `Rust` no puede existir este problema. ¿Por qué?
    - Por el borrow checker y por la inexistencia del _Garbage Collector_. **No puedo tener una pasada del GC** en el medio de la operación.
    - Como no hay GC, no puedo limpiar "memoria vieja" ni quedarme apuntando a memoria inexistente.

## Pros y Contras de los Algoritmos No Bloqueantes

| Aspecto                | Pros                                           | Contras                                                   |
| ---------------------- | ---------------------------------------------- | --------------------------------------------------------- |
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