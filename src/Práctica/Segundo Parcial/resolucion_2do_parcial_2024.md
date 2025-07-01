# Segundo Parcial 2024
![Segundo Parcial](./assets/segundo_parcial.png)

# Resolución
## 1 - NonBlocking Algorithms
Completar el siguiente código para que funcione el `pop` de una implementación no bloqueante de un `Stack` concurrente

```kotlin
fun pop(): E {
    var oldHead: Node<E>? = top.get()
    while (oldHead != null && /* Completar */)
        oldHead = top.get()
    return oldHead?.item
}
```

### Respuesta:
```kotlin
fun pop(): E {
    var oldHead: Node<E>? = top.get()
    while (oldHead != null && !top.compareAndSet(oldHead, oldHead?.next))
        oldHead = top.get()
    return oldHead?.item
}
```

## 2 - Garantías de Message Passing en el sistema de actores
### Respuesta:
>Nota: para estos ejemplos yo voy a ser el remitente y Juan va a ser el receptor en todos los casos

Las 3 garantías que existen son:
- `At most once`: garantiza que el envío de un mensaje se reciba como mucho una vez (0 o 1 veces).
  - Yo le mando un mensaje a Juan, y puede llegarle una vez como puede no llegarle
  - En lo que a espacio adicional refiere, no usa más que el espacio que el mensaje requiere
  - No usa identificadores
- `At least once`: garantiza que el envío de un mensaje se reciba al menos una vez (1 a N veces)
  - Yo le mando un mensaje a Juan, y seguro que una vez le llega, pero puede llegarle más de una vez. Es decir, le mando "Hola", y puede registrar un sólo "Hola" como puede registrar 30.
  - En lo que a espacio adicional refiere usa, al menos, el espacio que puede ocupar el mensaje enviado. Puede usar más que eso, dependiendo de las veces que reciba el mensaje.
  - No usa identificadores
- `Exactly once`: garantiza que el primer envío de un mensaje se reciba exactamente 1 vez
  - Yo le mando un mensaje a Juan y le llega una única vez.
  - Se usa sólo el espacio que ocupa este mensaje
  - Para garantizar esto, se requieren IDs únicos para trackear los mensajes que se intercambian

## 3 - Ejemplo de Actores
```scala
class Fib extends Actor{
    def fib(prev: Int, last: Int): Receive = {
        case "incr" => context.become(last, prev + last)
        case "get" => sender ! (prev + last)
        case "boom" => throw new IllegalStateException()
    }
    def receive = fib(0, 1)
}
```
### a) Qué recibiríamos si mandamos los siguientes mensajes?
```scala
fib ! "get"; fib ! "incr"; fib ! "incr"; fib ! "get"; fib ! "incr"; fib ! "get";
```
La secuencia sería la siguiente:
1. Se recibe `1`
2. No se recibe nada, se incrementa y el actor pasa a ser `fib(1, 1)`
3. No se recibe nada, se incrementa y el actor pasa a ser `fib(1, 2)`
4. Se recibe `3`
5. No se recibe nada, se incrementa y el actor pasa a ser `fib(2, 3)`
6. Se recibe `5`

### b) Y si mandamos estos mensajes?
```scala
fib ! "incr"; fib ! "incr"; fib ! "incr"; fib ! "boom"; fib ! "get" 
```
La secuencia sería la siguiente (suponiendo que arranca de 0):
1. No se recibe nada, se incrementa y el actor pasa a ser `fib(1, 1)`
2. No se recibe nada, se incrementa y el actor pasa a ser `fib(1, 2)`
3. No se recibe nada, se incrementa y el actor pasa a ser `fib(2, 3)`
4. Revienta el actor, y dependiendo de la estrategia de supervisión que tenga su actor padre (o el `ActorSystem`) puede:
   - Morir y no recibir más mensajes, por ejemplo
   - Reiniciar su estado (que es lo que hace `Akka` por default)
   - Planear otra estrategia custom
5. Se recibe `1`, porque se reinició el estado del actor (está en estado `fib(0, 1)`)

## 4 - Corutinas
Qué hace? Cómo funciona?
```kotlin
val mystery: Sequence<Int> = sequence {
    var result = 1
    var n = 1
    while (true){
        yield(result)
        result *= ++n
    }
}
```
- Funciona como una secuencia que va generando el factorial del número que se le pide con `take(n: Int)`
- Justamente, lo primero que hace es "ceder" (devolver) un `1` con el `yield`
- En caso de pedirle más de un elemento (es decir, `n>=2`), sigue el siguiente ciclo:
  - Cede el resultado actual (que para `n=1` es 1), y actualiza result multiplicándolo por (`n+1`), actualizando n con su siguiente valor (`n+1`)
  - Entonces, si siempre multiplica por el siguiente de n, eso implica una secuencia factorial:

$$

n! \;=\; \prod_{k=1}^{n} k
\quad\text{con}\quad
0! = 1.

$$

$$
\mathrm{fact}(n) =
\begin{cases}
1, & n = 0,\\
n \times \mathrm{fact}(n-1), & n > 0.
\end{cases}

$$

## 5 - Qué es un `Future` y qué problemas resuelve?
Un Future es objeto que representa un "registro" de que se llamó a una función asíncrona. Representa el valor eventual que va a devolver esa función asíncrona.

Encapsula el resultado una función que se ejecuta de manera asíncrona, que se puede ejecutar en paralelo y, en algún momento, devolver un valor.

Viene a resolver problemas competentes al contexto de la programación concurrente, tales como:
- **Coordinación de concurrencia y bloqueo de hilos**: al ser una tarea paralelizable (porque computa un valor de manera asíncrona), otorga una forma adicional de implementar concurrencia. Además, gracias a esto, evita tener que bloquear hilos por esperar a computar un valor (usando `thread.join()` o `get()`).
  - Sumado a esto, los Futures (como objeto) tienen la funcionalidad de computar varios a la vez y esperar a que todos los resultados se terminen de computar, usando `Future.thenAccept(...)`
  ```kotlin
  future1.thenAccept{ (future2) {
    result1, result2 -> hacerAlgoConLosResultados(result1, result2)
  } }
  ```
- **Composición de tareas asíncronas**: previo a la existencia del `Future`, era necesario componer funciones entre sí, llegando a niveles de indentación y llamados ilegibles, siendo este un problema conocido como `callback hell`. 
  - Justamente el `Future` permite escribir de manera más prolija esos llamados asíncronos a funciones.


### Ejemplo en pseudocódigo (en Kotlin)
Supongamos que nos levantamos a la mañana y nuestra rutina implica:
- Tomar un baño
- Hacer el desayuno (implica un café y 2 tostadas)
- Trabajar

> Nota: suponer que tomar un baño y hacer el desayuno toma más o menos el mismo tiempo.

Y obligatoriamente tenemos que comenzar a trabajar luego de hacer ambas anteriores.
Si yo quisiera primero tomar un baño y luego desayunar, tengo 2 alternativas:
- Me baño y después me preparo el desayuno
- Dejo el café calentándose en el microondas y las tostadas haciéndose en la tostadora

Intuitivamente el segundo approach parece más eficiente, ¿no?
Eso es porque implementa `asincronismo`, yo consumo el café una vez me bañé, y no tuve que esperar a que se haga el desayuno luego de haberme bañado porque lo dejé haciendo.
```kotlin
fun morningRoutine(){
  // Pongo a hacer el desayuno
  // Esto también se puede escribir de la forma:
  // `CompletableFuture.supplyAsync { makeBreakfast() }`
  CompletableFuture.supplyAsync(() -> makeBreakfast())
  .thenAccept(breakfast -> { // Cuando mi desayuno esté listo
  
  // Tomo mi desayuno
  haveBreakfast(breakfast)
  
  // Una vez tomé mi desayuno, me pongo a trabajar
  work()
  })
  // Me tomo un baño en paralelo. Cuando mi desayuno esté listo, me lo tomo y arranco a trabajar
  takeBath()
}
```


