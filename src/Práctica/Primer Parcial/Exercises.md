# Ejercicios para practicar para el 1er parcial
[Link al código fuente de esta sección](https://github.com/FranCalveyra/conc-summary/tree/main/content/Pr%C3%A1ctica/Primer%20Parcial/practice/src)

En los siguientes ejercicios, se explorarán diversos aspectos de la programación concurrente utilizando Rust.

1.  La consigna detallará **(qué implementar)**.
2.  Los resultados o comportamientos esperados se describirán a alto nivel **(tests genéricos, sin código específico, o ejemplos de uso)**.

---

## 1. Fundamentos de Concurrencia en Rust

*Instrucciones comunes:*

*   Implementar las soluciones utilizando las primitivas de concurrencia de la librería estándar de Rust (`std::thread`, `std::sync`, etc.).
*   Prestar atención a la seguridad de hilos (`Send`, `Sync`) y al manejo de datos compartidos.
*   Considerar el uso de `Mutex`, `RwLock`, `Arc`, y `Atomics` según sea apropiado.

---

### Ejercicio 1.1: Contador Concurrente y Condiciones de Carrera

**Consigna**

Implementar un struct `Counter` que pueda ser compartido y modificado de forma segura por múltiples hilos.

*   El `Counter` debe tener un método `new(initial_value: i32) -> Self`.
*   Debe proveer un método `increment(&self)` que aumente su valor interno en 1.
*   Debe proveer un método `get_value(&self) -> i32` que retorne el valor actual.

Se deben considerar dos enfoques o discusiones:
1.  Analizar por qué una implementación ingenua que intente modificar directamente un `i32` compartido entre hilos sin sincronización no es segura o no compilaría fácilmente en Rust sin `unsafe`. (Referencia: `race_conditions.rs` muestra cómo Rust previene esto).
2.  Implementar una versión segura del `Counter` utilizando `std::sync::Mutex` para proteger el acceso al valor.
3.  (Opcional) Implementar una versión alternativa utilizando `std::sync::atomic::AtomicI32`.

**Resultados esperados**

*   Al lanzar N hilos, donde cada hilo llama al método `increment()` M veces sobre una instancia compartida del `Counter` (protegido con `Mutex` o `AtomicI32`).
*   Después de que todos los hilos terminen, el valor final retornado por `get_value()` debe ser `initial_value + (N * M)`.
*   La implementación debe ser segura frente a condiciones de carrera, garantizando que cada incremento se aplique correctamente.

---

### Ejercicio 1.2: Cuenta Bancaria Concurrente

**Consigna**

Implementar una estructura que simule las operaciones de una cuenta bancaria, permitiendo depósitos, extracciones y consultas de saldo de forma concurrente y segura.

Definir un trait `BankAccount`:
```rust
pub trait BankAccount {
    fn new(initial_balance: f64) -> Self;
    fn deposit(&self, amount: f64);
    fn withdraw(&self, amount: f64) -> Result<(), String>; // Retorna Ok o Err con mensaje
    fn get_balance(&self) -> f64;
}
```

Proveer dos implementaciones de este trait:
1.  `MutexBankAccount`: Utilizar `std::sync::Mutex<f64>` para gestionar el saldo.
2.  `RwLockBankAccount`: Utilizar `std::sync::RwLock<f64>` para gestionar el saldo, permitiendo múltiples lecturas concurrentes.

**Resultados esperados**

*   Múltiples hilos deben poder interactuar concurrentemente con una instancia de `MutexBankAccount` o `RwLockBankAccount`.
*   Las operaciones de `deposit` y `withdraw` deben modificar el saldo de manera atómica.
*   `withdraw` debe retornar un `Err` si los fondos son insuficientes, sin modificar el saldo. Si tiene éxito, retorna `Ok(())`.
*   `get_balance` debe retornar el saldo actual.
*   Ejemplo de escenario:
    *   Cuenta inicia con saldo 0.
    *   Hilo A deposita 100.
    *   Hilo B deposita 50.
    *   Hilo C intenta extraer 200 (debe fallar).
    *   Hilo D extrae 30.
    *   Saldo final esperado: 120.
    *   Todas las operaciones deben reflejarse correctamente sin importar la intercalación de los hilos.

---

### Ejercicio 1.3: Suma Paralela de Elementos de un Vector

**Consigna**

Implementar una función `sum_parallel(nums: &[i32], m: usize) -> i32` que calcule la suma de los elementos de un slice de enteros (`&[i32]`) de forma paralela.

*   El slice `nums` debe ser dividido en `m` sub-slices (chunks) de tamaño lo más similar posible.
*   La suma de cada sub-slice debe ser calculada por un hilo (`std::thread`) separado.
*   La función debe esperar a que todos los hilos terminen, recolectar sus sumas parciales y retornar la suma total.
*   Considerar el manejo de casos borde, como un vector vacío o `m=0` (debe entrar en pánico si `m=0` o `m` es mayor que el número de elementos de una forma que no tenga sentido, aunque dividir en más chunks que elementos es posible).

**Resultados esperados**

*   `sum_parallel(&[1, 2, 3, 4, 5], 2)` podría dividir el trabajo en `[1, 2, 3]` y `[4, 5]`, sumarlos en paralelo y retornar `15`.
*   `sum_parallel(&[], m)` debe retornar `0` para cualquier `m > 0`.
*   El resultado de `sum_parallel` debe ser idéntico al de una suma secuencial (e.g., `nums.iter().sum()`).
*   La función debe entrar en pánico si `m` es 0 (como se muestra en `parallel_vector_sum.rs`).
*   Evaluar el rendimiento (conceptualmente) en comparación con una suma secuencial para vectores grandes.

---

### Ejercicio 1.4: Problema del Productor-Consumidor con Buffer Acotado

**Consigna**

Implementar una estructura `BoundedBuffer<T>` que sirva como un canal de comunicación de capacidad limitada entre hilos productores y consumidores.

*   La `BoundedBuffer<T>` debe encapsular los datos compartidos (e.g., un `VecDeque<T>` para el buffer, su capacidad máxima y tamaño actual) protegidos por un `std::sync::Mutex`.
*   Debe utilizar dos `std::sync::Condvar`:
    *   `not_empty`: Para que los hilos consumidores esperen si el buffer está vacío.
    *   `not_full`: Para que los hilos productores esperen si el buffer está lleno.
*   Se deben implementar (o se pueden proveer como en `bounded_buffer.rs`) structs `Producer` y `Consumer` que interactúen con el `BoundedBuffer`:
    *   `Producer::produce(&self, item: T)`: Añade un `item` al buffer. Si el buffer está lleno, el productor debe bloquearse (esperar en `not_full`). Al producir, notifica a través de `not_empty`.
    *   `Consumer::consume(&self) -> T`: Extrae un `item` del buffer. Si el buffer está vacío, el consumidor debe bloquearse (esperar en `not_empty`). Al consumir, notifica a través de `not_full`.

**Resultados esperados**

*   Múltiples hilos productores y consumidores pueden operar concurrentemente sobre la misma instancia de `BoundedBuffer` sin corrupción de datos ni deadlocks.
*   Los productores se bloquean eficazmente cuando el buffer alcanza su capacidad máxima y se reanudan cuando se libera espacio.
*   Los consumidores se bloquean eficazmente cuando el buffer está vacío y se reanudan cuando se añaden nuevos ítems.
*   Los ítems producidos son consumidos correctamente (sin pérdidas ni duplicados).
*   El uso de `std::thread::sleep` dentro de `produce`/`consume` (como en el ejemplo) puede ayudar a visualizar la alternancia y el bloqueo/desbloqueo de los hilos.

---

### Ejercicio 1.5: Implementación de un Buffer Circular Concurrente

**Consigna**

Desarrollar una estructura `ConcurrentCircularBuffer<T>` que implemente un buffer circular con semántica de productor-consumidor, seguro para acceso concurrente.

*   La estructura interna debe manejar un buffer de tamaño fijo (e.g., `Vec<Option<T>>`) con punteros `head` y `tail` para la lógica circular, y contadores de `size` y `capacity`.
*   Los datos compartidos (buffer, punteros, contadores) deben estar protegidos por un `std::sync::Mutex`.
*   Se deben emplear dos `std::sync::Condvar` (`not_empty` y `not_full`) para la sincronización:
    *   Método `add(&self, item: T)`: Si el buffer está lleno, el hilo productor debe esperar en la `Condvar` `not_full`. Tras añadir el ítem, debe notificar a un posible consumidor mediante `not_empty.notify_one()` (o `notify_all()`).
    *   Método `remove(&self) -> T`: Si el buffer está vacío, el hilo consumidor debe esperar en la `Condvar` `not_empty`. Tras extraer el ítem, debe notificar a un posible productor mediante `not_full.notify_one()` (o `notify_all()`).
*   (Opcional) Considerar la implementación base de un `CircularBuffer<T>` no concurrente primero.
*   Revisar la lógica de notificación: asegurarse de que se notifica a la `Condvar` correcta (e.g., en `circular_buffer.rs` el `remove` original podría tener `not_empty.notify_one()` donde debería ser `not_full.notify_one()`).

**Resultados esperados**

*   El `ConcurrentCircularBuffer` permite la interacción segura entre múltiples productores y consumidores.
*   Se previene el desbordamiento (overflow) y el subdesbordamiento (underflow) del buffer mediante el bloqueo y desbloqueo correcto de los hilos.
*   Las `Condvar` se utilizan eficazmente para minimizar la espera activa (busy-waiting).
*   El comportamiento es robusto frente a condiciones de carrera.

---

### Ejercicio 1.6: Cola Concurrente - De Busy-Waiting a Condvar

**Consigna**

Explorar y mejorar la implementación de una cola (queue) concurrente simple, pasando de un enfoque de espera activa (busy-waiting) a uno más eficiente usando variables de condición (`Condvar`).

1.  **Análisis del Busy-Waiting**:
    *   Examinar el código de `queue_behaviour()` (presente en `queue.rs`), que utiliza un `Mutex<VecDeque<T>>`.
    *   El hilo consumidor intenta extraer elementos en un bucle, adquiriendo el lock repetidamente incluso si la cola está vacía (busy-waiting).
    *   Identificar las desventajas de este enfoque (principalmente, consumo innecesario de CPU).

2.  **Implementación con `Condvar`**:
    *   Modificar o re-implementar la lógica en una función similar a `queue_behaviour_with_condvar()`.
    *   Además del `Mutex` para la `VecDeque`, introducir una `std::sync::Condvar` (e.g., `not_empty`).
    *   El hilo consumidor, si encuentra la cola vacía, debe esperar en la `Condvar` (`condvar.wait(lock_guard)`). Es crucial que esta espera se realice dentro de un bucle que vuelva a comprobar la condición de la cola tras despertar, para manejar despertares espurios (spurious wakeups).
    *   El hilo productor, después de añadir un elemento a la cola, debe notificar a un consumidor en espera (`condvar.notify_one()`).

**Resultados esperados**

*   Al ejecutar la versión con busy-waiting, se observa un alto uso de CPU por parte del hilo consumidor, especialmente cuando la cola está vacía frecuentemente.
*   Al ejecutar la versión con `Condvar`, el hilo consumidor debe mostrar un uso de CPU significativamente menor cuando está esperando, ya que el hilo se bloquea en lugar de sondear activamente.
*   Ambas versiones deben transferir correctamente los ítems del productor al consumidor, pero la versión con `Condvar` debe hacerlo de manera mucho más eficiente en términos de recursos del sistema.

---

### Ejercicio 1.7: Comunicación Multi-Productor a Consumidor Único con Canales MPSC

**Consigna**

Implementar un patrón de concurrencia donde múltiples hilos productores envían datos a un único hilo consumidor utilizando los canales multi-productor, single-consumer (`mpsc`) de la librería estándar de Rust (`std::sync::mpsc`).

*   Crear un canal `mpsc::channel()`.
*   Lanzar N hilos productores. Cada productor debe:
    *   Obtener una copia clonada del extremo `Sender<T>` del canal.
    *   Enviar uno o más mensajes (e.g., un ID de hilo, un dato calculado, etc.) a través de su `Sender`.
    *   Asegurarse de que el `Sender` clonado se libere (`drop`) cuando el productor haya terminado de enviar mensajes para permitir que el canal se cierre eventualmente.
*   El hilo consumidor debe:
    *   Utilizar el extremo `Receiver<T>` del canal.
    *   Recibir mensajes en un bucle. El bucle debe terminar cuando el canal se cierre (es decir, cuando todos los `Sender`s hayan sido liberados).
    *   Procesar o imprimir cada mensaje recibido.

**Resultados esperados**

*   El hilo consumidor recibe todos los mensajes enviados por todos los hilos productores.
*   El programa termina correctamente después de que todos los mensajes han sido procesados y el canal se ha cerrado (el `Receiver` ya no puede obtener más mensajes).
*   Se puede observar cómo los mensajes de diferentes productores pueden intercalarse al ser recibidos por el consumidor, dependiendo del scheduling de los hilos.
*   Este ejercicio demuestra un caso de uso fundamental de los canales `mpsc` para la agregación de datos o tareas desde múltiples fuentes.

---

### Ejercicio 1.8: Pipeline de Procesamiento de N Etapas con Canales

**Consigna**

Construir un pipeline de procesamiento de datos donde un dato inicial atraviesa una secuencia de N etapas de transformación, cada una ejecutándose en un hilo separado y comunicándose con la siguiente mediante canales `mpsc`.

*   Definir una serie de funciones de transformación (e.g., `fn(T) -> T`).
*   Implementar una estructura o lógica (como la `Pipeline` y `PipelineNode` en `channels.rs`) que:
    *   Tome una lista de estas funciones de transformación.
    *   Para cada función, cree un hilo (una etapa del pipeline).
    *   Cree un canal `mpsc` entre cada par de etapas consecutivas (la salida de la etapa `i` es la entrada de la etapa `i+1`).
    *   El primer hilo del pipeline recibe un valor inicial (posiblemente a través de un `Sender` inicial).
    *   Cada hilo intermedio recibe un valor de su predecesor, aplica su función de transformación, y envía el resultado a su sucesor.
    *   El último hilo del pipeline envía su resultado a un `Receiver` final desde donde se puede obtener el resultado global.
*   La función `run` del pipeline debe tomar un valor inicial, enviarlo a la primera etapa, y retornar el valor recibido de la última etapa.

**Resultados esperados**

*   Un valor de entrada es procesado secuencialmente por todas las etapas del pipeline, con cada transformación ocurriendo en un hilo dedicado.
*   El resultado final obtenido es el producto de aplicar todas las funciones de transformación en el orden especificado.
*   Ejemplo: Si el valor inicial es `X` y las etapas son `f1, f2, f3`, el resultado final debe ser `f3(f2(f1(X)))`.
*   El sistema debe manejar la creación, el enlace y el cierre de los canales correctamente. (Considerar cómo se manejan los errores o pánicos en una etapa; el ejemplo en `channels.rs` usa `expect` y `unwrap`).

---

### Ejercicio 1.9: El Problema de los Filósofos Cenadores

**Consigna**

Implementar una simulación del clásico problema de concurrencia de los "Filósofos Cenadores", con el objetivo de evitar deadlocks y permitir que los filósofos coman.

*   Habrá N filósofos sentados en una mesa redonda. Entre cada par de filósofos adyacentes hay un tenedor (N tenedores en total).
*   Cada filósofo alterna entre dos estados: pensar y comer.
*   Para comer, un filósofo necesita adquirir los dos tenedores que tiene a su lado (el izquierdo y el derecho).
*   La implementación debe incluir:
    *   Un struct `Philosopher` que represente a un filósofo, con su ID (o posición) y la lógica para pensar y comer.
    *   Un struct `Table` (o similar) para representar el estado compartido de los tenedores. Este estado (e.g., un `Vec<bool>` indicando si cada tenedor está disponible) debe ser protegido por un `std::sync::Mutex`.
    *   Una `std::sync::Condvar` para que los filósofos puedan esperar de manera eficiente si los tenedores que necesitan no están disponibles.
*   **Lógica para comer (método `eat` del `Philosopher`):**
    1.  Adquirir el lock del `Mutex` de la mesa.
    2.  Mientras ambos tenedores necesarios (izquierdo y derecho) no estén disponibles: esperar en la `Condvar` (`condvar.wait(lock_guard)`). La guarda del mutex se libera mientras se espera y se vuelve a adquirir al despertar.
    3.  Cuando ambos tenedores estén disponibles (tras despertar y re-evaluar la condición), marcarlos como "en uso".
    4.  Liberar el lock del `Mutex` de la mesa (importante: esto permite a otros filósofos intentar tomar tenedores mientras este come).
    5.  Simular el tiempo de comida (e.g., `std::thread::sleep`).
    6.  Volver a adquirir el lock del `Mutex` de la mesa.
    7.  Marcar ambos tenedores como "disponibles".
    8.  Notificar a todos los demás filósofos que podrían estar esperando (`condvar.notify_all()`).
    9.  Liberar el lock del `Mutex`.

**Resultados esperados**

*   La simulación se ejecuta con N filósofos (e.g., 5) y N tenedores.
*   Los filósofos pueden alternar entre pensar y comer sin que el sistema entre en deadlock (donde ningún filósofo puede progresar).
*   Se demuestra que los filósofos esperan si no pueden obtener ambos tenedores y son notificados cuando los tenedores se liberan.
*   (Opcional) Considerar y discutir brevemente otras estrategias para la prevención de deadlocks en este problema (e.g., ordenamiento global de adquisición de tenedores, limitar el número de comensales).

---

### Ejercicio 1.10: Implementación de Merge Sort Paralelo

**Consigna**

Desarrollar una versión paralela del algoritmo de ordenamiento Merge Sort para un slice de enteros (`&[i32]`).

*   La implementación debe incluir las siguientes partes:
    1.  Una función `merge(first_slice: &[i32], second_slice: &[i32]) -> Vec<i32>`: Esta función toma dos slices ya ordenados y los combina en un nuevo `Vec<i32>` que también está ordenado.
    2.  (Opcional pero útil como referencia) Una función `sequential_merge_sort(slice: &[i32]) -> Vec<i32>`: La implementación recursiva estándar de Merge Sort.
    3.  Una función `parallel_merge_sort(slice: &[i32]) -> Vec<i32>`: Esta es la versión paralela.
        *   Debe manejar el caso base: si el slice es suficientemente pequeño (e.g., longitud 0 o 1), se retorna directamente o se ordena secuencialmente.
        *   Para el paso recursivo: dividir el slice en dos mitades.
        *   Ordenar al menos una de las mitades en un nuevo hilo. Por ejemplo, la primera mitad puede ordenarse en el hilo actual (recursivamente, podría ser secuencial o paralelo dependiendo de la profundidad y un umbral), y la segunda mitad se ordena en un hilo spawnneado (`std::thread::scope` es útil aquí).
        *   Esperar a que el hilo (o hilos) terminen y obtener las dos mitades ordenadas.
        *   Combinar las dos mitades ordenadas usando la función `merge`.

**Resultados esperados**

*   `parallel_merge_sort` debe producir un vector correctamente ordenado, idéntico al resultado de un Merge Sort secuencial.
*   Debe funcionar para diversos casos de prueba: arrays vacíos, de un elemento, ya ordenados, en orden inverso, con elementos duplicados, etc.
*   Para arrays de tamaño considerable, la versión `parallel_merge_sort` debería, idealmente, ejecutarse más rápido que una versión puramente secuencial. Esto se puede verificar mediante benchmarking (comparando tiempos de ejecución).
*   (Para discusión) Considerar el uso de un umbral: si el tamaño del sub-array a ordenar es menor que cierto umbral, cambiar a una versión secuencial para evitar el overhead de crear hilos para tareas muy pequeñas. También, discutir cómo se podría aumentar el grado de paralelismo (e.g., lanzando ambas mitades a hilos separados si el umbral lo permite).

---

### Ejercicio 1.11: Suma de Matrices Secuencial y Paralela

**Consigna**

Implementar una estructura `Matrix` para representar matrices de números de punto flotante (`f64`) y desarrollar métodos para su suma, tanto de forma secuencial como paralela.

*   Definir un struct `Matrix` que encapsule los datos de la matriz (e.g., `Vec<Vec<f64>>`).
*   Implementar las siguientes funcionalidades para la suma de dos matrices (`self` y `other: &Matrix`):
    1.  **Suma Secuencial (`add_sequential`):**
        *   Debe sumar las dos matrices elemento por elemento y retornar una nueva `Matrix` con el resultado.
        *   La operación asume que ambas matrices tienen las mismas dimensiones. (Considerar añadir una verificación explícita de dimensiones y manejar el error si no coinciden, e.g., retornando un `Result<Matrix, String>` o causando un `panic` controlado).
    2.  **Suma Paralela (`add_parallel`):**
        *   Debe realizar la suma de las dos matrices utilizando hilos para paralelizar el cálculo.
        *   Una estrategia común es asignar a cada hilo el cálculo de una fila completa (o un subconjunto de filas) de la matriz resultante. `std::thread::scope` puede ser útil para gestionar los hilos.
        *   Al igual que la suma secuencial, esta operación asume dimensiones compatibles.
*   (Opcional) Crear una enumeración `OperationMethod { SEQUENTIAL, PARALLEL }` y un método principal `add_matrix(&self, other: &Matrix, method: OperationMethod) -> Matrix` que delegue a la implementación correspondiente.

**Resultados esperados**

*   Para dos matrices de entrada dadas, tanto `add_sequential` como `add_parallel` deben producir la misma `Matrix` resultante.
*   El código debe manejar correctamente matrices de diversas dimensiones (e.g., cuadradas, rectangulares, 1x1) y con diferentes tipos de valores (positivos, negativos, cero).
*   Para matrices de tamaño suficientemente grande, la versión `add_parallel` debería mostrar una mejora en el tiempo de ejecución en comparación con `add_sequential`. Esto se puede verificar mediante benchmarking.
*   Si se implementa la verificación de dimensiones, las operaciones deben fallar de forma predecible (e.g., retornar `Err` o `panic`) cuando se intentan sumar matrices de dimensiones incompatibles.
*   (Para discusión) Explorar y comparar diferentes granularidades para la paralelización: ¿por fila, por columna, por bloques de elementos? ¿Cuál podría ser más eficiente y por qué?
