
# Ejercicios para practicar para el 2do parcial

[Link al código fuente de esta sección](https://github.com/FranCalveyra/conc-summary/tree/main/src/Pr%C3%A1ctica/Segundo%20Parcial)

En los siguientes ejercicios, se tendrán:

1. La consigna **(qué implementar)**.
2. Resultados o comportamientos esperados a alto nivel **(tests genéricos, sin código específico).**

---

## 1. Programación Asíncrona (Kotlin + Corutinas)

*Instrucciones comunes:*

* Todos los ejercicios deben resolverse usando corutinas de Kotlin (`suspend fun`, `launch`, `async`, `flow`, etc.).
* No es necesario usar librerías adicionales: basta con `kotlinx.coroutines`.
* Al proponer resultados esperados, se asume que habrá alguna manera de comprobar (por ejemplo, listas de logs,
  contadores, resultados de funciones, o estados finales).

---

### Ejercicio 1.1: Descarga paralela y combinación de datos

**Consigna**
Implementar una función `suspend fun fetchDataFromSources(): List<String>` que simultáneamente lance tres corutinas que
"simulen" descargar datos de tres fuentes externas diferentes (por ejemplo, retrasos de 1 seg, 2 seg y 3 seg). Cada
descarga retorna una `List<String>` propia. Cuando todas las descargas terminen, hay que combinar sus listas en una sola
lista ordenada alfabéticamente y retornarla.

**Resultados esperados**

* Si las tres descargas simulan devolver `["A","B"]`, `["C"]` y `["D","E"]` (en distintos tiempos), la función debería
  retornar `["A","B","C","D","E"]`.
* El tiempo total de ejecución de `fetchDataFromSources()` debe rondar el máximo de los tres delays (≈ 3 seg), no la
  suma (≈ 6 seg).
* Si alguna descarga lanza una excepción, `fetchDataFromSources()` debe cancelarlas a todas y propagar un único error (
  `CancellationException`).

---

### Ejercicio 1.2: Timeout y fallback

**Consigna**
Crear una función `suspend fun getUserProfile(userId: Int): String` que intente obtener el perfil de un usuario desde un
"servicio remoto simulado" (delay de 5 seg), pero si supera un timeout de 2 seg, se cancele dicha llamada y retorne un
perfil por defecto (por ejemplo, `"Usuario por defecto"`). Usar `withTimeout` o `withTimeoutOrNull` según convenga.

**Resultados esperados**

* Si la llamada al servicio remoto dura 5 seg, `getUserProfile` debe terminar alrededor de 2 seg y devolver
  `"Usuario por defecto"`.
* Si en algún caso se reduce el delay simulado a 1 seg, la función debe devolver el perfil real (por ejemplo
  `"Perfil de 42"`) en ≈ 1 seg.
* No debe quedar ninguna corutina "colgada" después de que expire el timeout.

---

### Ejercicio 1.3: Flujo de eventos y back-pressure

**Consigna**
Definir un `Flow<Int>` que emita un número creciente cada 100 ms (i.e. `0,1,2,…`). Consumirlo en otra corutina que
procese cada número con un delay de 300 ms (simulando trabajo pesado). Gestionar correctamente la suscripción para que,
si el consumidor no da abasto, el emisor se suspenda (no pierda valores y no agote memoria). Usar `buffer()`,
`conflate()` o `collectLatest()` según el comportamiento deseado. Documentar brevemente qué estrategia de back-pressure
se eligió.

**Resultados esperados**

* Si se usa `buffer(capacity = 1)` y el consumidor demora 300 ms, el emisor enchufa hasta 1 valor en el buffer y luego
  se suspende. Nunca se pierden valores, pero la tasa del consumidor marca el ritmo.
* Si se usa `conflate()`, el consumidor obtiene solo el último valor generado mientras estaba ocupado (p. ej., del 0 al
  3 en buffer, llega el 3).
* Mostrar en consola los valores que el consumidor realmente procesa y la hora (timestamp) para verificar que no se
  acumulan más de lo esperado.

---

### Ejercicio 1.4: Cancelling child jobs al fallar uno

**Consigna**
Implementar una función `suspend fun processOrders(orderIds: List<Int>): List<String>` que, para cada `orderId`, lance
una corutina hija (con `async`) que simule "procesar" el pedido (delay aleatorio entre 100 ms y 500 ms) y devuelva
`"Order #<id> processed"`. Si alguno de esos `async` falla (lanza excepción), se debe cancelar toda la ventana de
procesamiento y propagar un error que incluya el `orderId` que falló. Usar un scope apropiado para que la cancelación se
propague a todos los hijos.

**Resultados esperados**

* Si la lista es `[1,2,3]` y solo el procesamiento de `orderId = 2` lanza `RuntimeException("Error en #2")`, la función
  debe lanzar `CancellationException` o `RuntimeException("Error en #2")` y ninguno de los otros pedidos completos debe
  continuar después de la falla.
* En caso exitoso (ej. `[1, 2]` todos terminan sin excepción), retorna una lista de strings:
  `["Order #1 processed", "Order #2 processed"]`, con longitud igual al número de elementos.

---

### Ejercicio 1.5: Integración con canal y productor-consumidor

**Consigna**
Crear un `Channel<Int>` con capacidad 10 que actúe como buffer circular. En una corutina "productor" (`launch`), emite
números del 1 al 100 con un delay de 50 ms entre cada emisión. En otra corutina "consumidor" (`launch`), recibe de ese
canal y procesa cada número con un delay de 200 ms (simulando trabajo). Finalmente, cuando el productor haya cerrado el
canal, el consumidor debe terminar y reportar en consola el total de números procesados.

**Resultados esperados**

* El productor emite 100 enteros y luego cierra el canal.
* El consumidor procesa exactamente 100 enteros (impresión en consola de cada uno o simplemente un contador).
* Debido a que el consumidor tarda 200 ms y el productor solo 50 ms, el canal mantendrá hasta 10 elementos "pendientes"
  y luego el productor se suspenderá cuando esté full.
* Tiempo total aproximado:

    * El productor terminará antes de 5 seg (100 × 50 ms = 5 000 ms).
    * El consumidor tardará \~ 100 × 200 ms = 20 000 ms.
* Finalmente imprimir: `"Consumo finalizado: 100 elementos procesados"`.

---

## 2. Algoritmos No Bloqueantes (Rust)

*Instrucciones comunes:*

* Implementar exclusivamente con la librería estándar de Rust (`std::sync::atomic`, `std::thread`, etc.).
* No usar mutex ni bloqueos de sistema; usar operaciones atómicas (`AtomicPtr`, `AtomicUsize`, `CompareAndSwap`, etc.).
* Indicar claramente qué estructuras atómicas se usan y en qué orden.
* Para cada ejercicio basta con describir la API pública, el comportamiento deseado y cómo verificarlo con pruebas
  genéricas (p.ej. lanzar muchas hebras y comprobar invariantes).

---

### Ejercicio 2.1: Pila no bloqueante (Lock-free Stack)

**Consigna**
Implementar una pila (`struct NonBlockingStack<T>`) con los siguientes métodos:

```rust
impl<T> NonBlockingStack<T> {
    pub fn new() -> Self { /* … */ }
    pub fn push(&self, value: T){}
    pub fn pop(&self) -> Option<T>{/*...*/}
}
```

* Cada nodo se guarda en un `Box<Node<T>>` y se enlaza mediante un `AtomicPtr<Node<T>>` apuntando a la cabeza.
* El método `push` debe crear un nuevo nodo, leer la cabeza actual y usar `compare_exchange` para insertarlo.
* El método `pop` debe leer la cabeza, tomar el valor si no es nulo, y hacer `compare_exchange` para avanzar la cabeza.
  Luego retorna el `T` contenido.
* Asegurarse de hacer `Box::from_raw` sólo cuando el `pop` tenga éxito, para evitar fugas de memoria.

**Resultados esperados**

* Generar 8 hilos (threads) que hagan cada uno 1 000 inserciones (`push(i)`) seguidas de 1 000 extracciones (`pop()`),
  todo en paralelo.
* Después de unificar (join) los hilos, la pila debe estar vacía (`pop()` retorna `None`).
* Contar cuántos valores distintos se obtuvieron al hacer `pop`. Debe coincidir exactamente con el total de
  inserciones (8 × 1 000 = 8 000).
* No debe haber pánico por doble liberación ni memory leak: ejecutar con `cargo miri` (o `cargo valgrind`) para validar.

---

### Ejercicio 2.2: Cola no bloqueante unidireccional (MPSC Queue)

**Consigna**
Crear una cola de un solo productor y múltiples consumidores (`struct MpscQueue<T>`) con API:

```rust
impl<T> MpscQueue<T> {
    pub fn new() -> Self { /* … */ }
    pub fn enqueue(&self, value: T);    // sólo un hilo (producer) llama a esto
    pub fn dequeue(&self) -> Option<T>; // varios hilos (consumers) llaman a esto
}
```

* Internamente, usar un puntero atómico a un nodo "dummy" como cabeza y otro como cola (Michael & Scott queue
  simplificada).
* `enqueue` es sólo llamado desde un hilo "productor" y hace `compare_exchange` para insertar al final.
* `dequeue` podrá ser invocado concurrentemente desde N hilos, avanzando la cabeza.
* Cuidar la operación atómica sobre el tamaño (opcional) con un `AtomicUsize` que incremente en `enqueue` y decremente
  en `dequeue`.

**Resultados esperados**

* En un test, lanzar 1 hilo productor que encola 50 000 enteros (0..50 000).
* Lanzar 4 hilos consumidores que, en loop, hagan `dequeue()` y acumulen los valores en un vector local hasta que el
  queue devuelva `None` y el productor haya terminado.
* Al final, reunir (merge) los resultados y comprobar que aparecen exactamente todos los números del 0 al 49 999, sin
  duplicados ni faltantes.
* Verificar que el tamaño reportado internamente (si se implementa `len()`) se acerca a cero cuando el procesado
  finaliza.

---

### Ejercicio 2.3: Contador atómico con escalonamiento exponencial

**Consigna**
Implementar un contador (`struct BackoffCounter`) que use un `AtomicUsize` internamente para incrementos concurrentes
desde N hilos. Para reducir la contención, tras cada "fallo" en el `compare_exchange`, el hilo hará un *backoff*
exponencial: al principio dormir 1 µs, luego 2 µs, 4 µs, hasta un tope (p. ej., 128 µs). La interfaz pública:

```rust
impl BackoffCounter {
    pub fn new(initial: usize) -> Self { /* … */ }
    pub fn increment(&self);
    pub fn get(&self) -> usize;
}
```

* En `increment`, el hilo lee el valor actual, calcula `val + 1` y usa `compare_exchange_weak`. Si falla, duerme el
  tiempo de backoff actual (doblándolo) y reintenta, hasta tenedortope.
* `get()` carga el valor actual con `Ordering::Acquire`.

**Resultados esperados**

* Lanzar 16 hilos, cada uno hace 10 000 llamadas a `increment()`.
* Al unir todos, el valor final de `get()` debe ser 16 × 10 000 = 160 000.
* Medir (aprox.) el tiempo de ejecución con vs. sin backoff (es suficiente un print de "tiempo sin backoff: X ms",
  "tiempo con backoff: Y ms") para comprobar la mejora de contención cuando hay más hilos.
* Asegurarse de que nunca se quede "en bucle infinito", es decir, tras llegar al tope de backoff se vuelve a intentar
  indefinidamente, pero con un máximo de 128 µs entre reintentos.

---

### Ejercicio 2.4: Tabla hash concurrente lock-free (Lock-free HashMap)

**Consigna**
Diseñar una tabla hash concurrente muy simplificada (`struct LockFreeHashMap<K, V>`) que soporte `insert(key, value)`,
`get(&key) -> Option<V>`, y `remove(&key) -> Option<V>`. Debe basarse en:

1. Un vector fijo de "celdas" tamaño M (e.g. M=64), donde cada celda es un puntero atómico a una lista enlazada
   lock-free de pares `(K, V)`.
2. Cada lista se implementa como un stack "lock-free" con `AtomicPtr<Node>`, similar al Ejercicio 2.1.
3. Para `insert`, se calcula `hash = hash(key) % M`, luego se hace push de `(K,V)` en la lista correspondiente. Si ya
   existe el key, se puede decidir: a) rechazar, o b) insertar duplicado.
4. Para `get`, se lee la lista en modo "snapshot": se recorre sin bloquear la lista.
5. Para `remove`, se marca el nodo "lógicamente eliminado" (opcional) o bien se hace "lazy deletion" dejando que otro
   thread lo retire al inspeccionar la lista.

La parte complicada es coordinar las listas lock-free sin mutex:

* El énfasis está en la API y en demostrar que concurrentemente varias hebras pueden hacer `insert/get/remove` sin
  panics ni corrupciones de memoria.

**Resultados esperados**

* Escribir un test con 8 hilos durante 2 segundos. Cada hilo hace, al azar:

    * `insert(random_key, random_value)`
    * `get(random_key)`
    * `remove(random_key)`
* Al terminar, imprimir cuántas claves únicas quedaron en la tabla.
* Verificar, tras el test, que todos los nodos removidos fueron liberados (usar `cargo miri` para chequear leaks).
* Comprobar también que nunca se obtiene un valor "corrupto" para ninguna clave (p.ej. comparar con un mapa secuencial
  simulado como oracle).

---

## 3. Actores (Scala + Akka)

*Instrucciones comunes:*

* Usar Scala 2.13+ y la librería Akka (versión 2.6 o superior).
* Definir actores extendiendo `akka.actor.Actor` o usando `AbstractBehavior` (Akka Typed), según prefieras.
* Cada ejercicio debe incluir la descripción de los mensajes ("case class" o "case object") y la lógica interna de los
  actores (stateful o stateless).
* Los tests esperados deben formularse de manera genérica: enviar mensajes y verificar estados finales o respuestas.

---

### Ejercicio 3.1: Chat Room con actores

**Consigna**
Implementar un sistema de chat muy simple:

1. **UserActor**: representa a cada usuario conectado. Recibe mensajes tipo `Message(from: String, text: String)` y los
   imprime o guarda en su estado local. También puede recibir `JoinRoom(roomRef: ActorRef)` y
   `LeaveRoom(roomRef: ActorRef)`.
2. **RoomActor**: maneja la sala.

    * Mensajes que acepta:

        * `Join(userName: String, userRef: ActorRef)`
        * `Leave(userName: String)`
        * `Broadcast(sender: String, text: String)`
    * Internamente mantiene un `Set[(String, ActorRef)]` de usuarios en línea.
    * Al recibir `Join`, añade el par `(userName, userRef)`.
    * Al recibir `Leave`, elimina al usuario.
    * Al recibir `Broadcast(sender, text)`, envía a todos los usuarios (menos al propio `sender`) un mensaje
      `Message(sender, text)`.

**Resultados esperados (tests de alto nivel)**

* Crear un `RoomActor` y 3 `UserActor` (Alice, Bob, Carol).
* Hacer que Alice y Bob se unan (`Join`) primero, luego Carol.
* Enviar desde Alice a la sala `Broadcast("Alice", "Hola!")`.

    * Bob y Carol deberían recibir `Message("Alice", "Hola!")`.
    * Alice no debe recibir su propio mensaje.
* Hacer que Bob salga (`Leave`) y luego Alice vuelva a `Broadcast("Alice", "¿Dónde está Bob?")`.

    * Solo Carol debe recibir ese mensaje.
* Al final, cada `UserActor` debe tener registrada una lista de mensajes recibidos que concuerde con lo anterior.

---

### Ejercicio 3.2: Supervisión y reinicio de actores hijos

**Consigna**
Construir dos actores:

1. **WorkerActor**: cada vez que reciba un mensaje `DoWork(n: Int)`, si `n < 0` lanza un
   `RuntimeException("n negativo")`; en otro caso, "procesa" imprimendo `Trabajando con n` y devuelve `Done(n*2)` al
   remitente.
2. **SupervisorActor**: al iniciarse, crea un hijo `WorkerActor` y envía mensajes `DoWork` al hijo. Debe usar un
   `OneForOneStrategy` que, si el hijo falla por `RuntimeException`, lo reinicie automáticamente.

    * Mensajes que procesa:

        * `StartWork(values: List[Int], replyTo: ActorRef)` — itera sobre la lista y envía cada `DoWork(v)` al hijo.
        * `Done(result: Int)` — cuando recibe esta respuesta del hijo, acumula resultados en su estado.
        * `GetResults(replyTo: ActorRef)` — envía al `replyTo` la lista de resultados acumulados.

**Resultados esperados (tests de alto nivel)**

* Enviar a `SupervisorActor` un `StartWork(List(1, -1, 2), testProbeRef)`.

    1. `DoWork(1)` → hijo responde `Done(2)`.
    2. `DoWork(-1)` → hijo lanza `RuntimeException`, se reinicia, **no** envía `Done`.
    3. `DoWork(2)` → hijo (estado limpio) responde `Done(4)`.
* Al final, hacer `GetResults(testProbeRef)`. El supervisor debe devolver la lista `[2, 4]` (es decir, se ignora el `-1`
  que causó la excepción).
* Verificar que el hijo fue reiniciado exactamente una vez (usar un contador en el Supervisor o testProbe que observe el
  ciclo de vida del hijo).

---

### Ejercicio 3.3: Router de balanceo de carga (RoundRobinPool)

**Consigna**
Implementar un grupo de 5 actores trabajadores (“workers”) que resuelven operaciones simples (por ejemplo, cuadrados de
enteros). Usar un `Router` de tipo `RoundRobinPool(5)` para distribuir las solicitudes.

* **WorkerActor**:

    * Mensaje recibido: `ComputeSquare(n: Int, replyTo: ActorRef)` → responde con `Result(n, n*n)` al `replyTo`.
* **ClientActor**:

    * Al recibir `Start(numbers: List[Int], routerRef: ActorRef, replyTo: ActorRef)`, envía a `routerRef` un
      `ComputeSquare` para cada número de la lista.
    * Recibe múltiples `Result(n, square)` y los acumula en un `Map[Int,Int]` en su estado. Cuando reciba tantos
      resultados como números envió, envía `AllResults(Map)` a `replyTo`.

**Resultados esperados (tests de alto nivel)**

* Crear un `Router` con `RoundRobinPool(5)(Props[WorkerActor])`.
* Crear un `ClientActor` y enviarle `Start(List(2,3,4,5), routerRef, testProbeRef)`.
* El `ClientActor` envía 4 mensajes `ComputeSquare` en secuencia: 2→worker1, 3→worker2, 4→worker3, 5→worker4 (round
  robin).
* Usar un `TestProbe` como `replyTo`. Al final, debe recibir un solo mensaje `AllResults(Map(2→4,3→9,4→16,5→25))`.
* Opción adicional: verificar que cada `WorkerActor` haya procesado al menos un mensaje (con un contador interno o
  `TestProbe` en cada worker).

---

### Ejercicio 3.4: Actores con estado y temporizadores (Periodic Ticker)

**Consigna**
Crear un actor llamado **TickerActor** que cada cierto intervalo envíe un mensaje a sí mismo y, en cada tick, imprima la
hora actual o incremente un contador interno.

* Mensajes:

    * `StartTick(interval: FiniteDuration)` — arranca un ticker interno que cada `interval` envía al propio actor
      `Tick`.
    * `Tick` (interno) — al recibirlo, incrementa un contador `count` y emite a todos los "suscriptores" (otros actores
      en una lista) un mensaje `Ticked(count)`.
    * `Subscribe(subscriberRef: ActorRef)` — agrega `subscriberRef` a la lista de suscriptores.
    * `GetCount(replyTo: ActorRef)` — responde con el valor actual de `count`.

**Resultados esperados (tests de alto nivel)**

* Crear `TickerActor`, luego dos actores "listener" que se suscriben (`Subscribe`) antes de arrancar.
* Enviar `StartTick(200.millis)` a `TickerActor`.
* Esperar 1 seg en el test. Los listeners deben recibir aproximadamente 5 mensajes `Ticked(1)`, `Ticked(2)`, … hasta
  `Ticked(5)`.
* Al enviar `GetCount(testProbeRef)` al `TickerActor` después de 1 seg, debe responder con (`Count(5)`).
* Luego enviar `Subscribe` a un tercer listener y observar que, a partir de ese momento, el tercero también reciba
  `Ticked(6)`, `Ticked(7)`, ….

---

### Ejercicio 3.5: Persistencia simplificada (Stateful Actor con snapshots)

**Consigna**
Simular un actor que mantiene un contador persistente en memoria (sin usar Akka Persistence); grabar periódicamente su
estado a disco como "snapshot" (por ejemplo, serializando a un archivo local). A grandes rasgos:

* **PersistentCounterActor**:

    * Mensajes:

        * `Increment` → incrementa un `count: Int` en memoria.
        * `GetValue(replyTo: ActorRef)` → envía `Value(count)` al `replyTo`.
        * `Snapshot` → cuando lo recibe, fuerza a serializar el valor de `count` a un archivo (p.ej.
          `/tmp/counter.snapshot`).
        * `Recover` → lee el archivo `/tmp/counter.snapshot` (si existe) y recarga `count` con ese valor.
* Al arrancar, el actor recibe `Recover` automáticamente (en `preStart`).
* Cada 10 mensajes `Increment`, el actor se envía a sí mismo un `Snapshot` para persistir.

**Resultados esperados (tests de alto nivel)**

* Inicializar sin fichero de snapshot. Enviar 5×`Increment`. Luego `GetValue(testProbe)`: debe responder `Value(5)`. No
  hay snapshot todavía.
* Enviar 5× más `Increment` (total 10) → automáticamente el actor hace `Snapshot` (genera `/tmp/counter.snapshot` con
  "10").
* Parar (detener) el actor, reiniciarlo y verificar que en `Recover` lea "10" → `count` arranca en 10.
* Enviar 3×`Increment` y al hacer `GetValue`, debe dar `Value(13)`.
* Borrar manualmente el archivo, reiniciar actor → en `Recover` no encuentra snapshot, arranca en `0`.

---

