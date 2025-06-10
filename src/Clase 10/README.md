# Actors

Lo mejor para actores es Erlang, pero vemos todo en Scala con una
librería que se llama Akka por cuestiones de simplicidad

## Sincronización tradicional

Varios threads se pisan entre sí. Esto se resuelve:

- Demarcando regiones de código con semánticas para "no molestar"
- Asegurando que el acceso a estado compartido sea protegido

```scala
class BankAccount {
  private var balance = 0

  def deposit(amount: Int): Unit = synchronized {
    if (amount > 0) balance = balance + amount
  }

  def withdraw(amount: Int): Int = synchronized {
    if (0 < amount & amount <= balance) {
      balance = balance - amount
      balance
    } else throw new Error("insufficient funds")
  }
}
```

## Qué es un Actor?

Lo que pretende el modelo de actores es pensar las interacciones de un sistema como personas interactuando entre sí.
Lo modela como personas que envían mensajes entre sí.

Un actor es:

- Un objeto con identidad
- Que tiene comportamiento
- Y solo interactúa usando pasaje de mensajes asincrónico

Es OOP + Mensajes, en definitiva. Forzado dentro de un sistema donde todo tiene que seguir este estándar.

## Actor trait

```scala
// Se define un type alias llamado 'Receive'
/* Representa una funcon que maneja los mensajes que se le envían a un Actor
- Es una PartialFunction, lo que significa que puede no manejar cualquier posible input
- Toma un mensaje de cualquier tipo, y no devuelve nada ('Unit')
 */
type Receive = PartialFunction[Any, Unit]
```

Receive es esencialmente una función que recibe cualquier parámetro y no devuelve nada.

#### ¿Por qué `PartialFunction`?

Si las funciones matemáticas están bien definidas, deberían tener dominio para todos los reales.
Si no, tienen "agujeros" en el dominio.

Trasladado a Scala y al contexto de Actors, que sea una función parcial quiere decir que hay valores que no soporta.
Esto le sirve al lenguaje para hacer algún tipo de chequeo.

```scala
// Se define un trait llamado Actor
trait Actor {
  // Este método abstracto DEBE ser implementado por el Actor en cuestión
  // Define la lógica con la que se reciben los mensajes (qué se hace cuando se recibe un mensaje)
  def receive: Receive
}
```
- Los `traits` en Scala son equivalentes a las interfaces de Java, o a los `traits` (justamente) de Rust
- `receive` es un lambda.

### Un Actor Simple

```scala
// La clase 'Counter' extiende el trait 'Actor' e implementa el método 'receive'
class Counter extends Actor {
  // Variable mutable (contador actual)
  var count = 0

  // El método 'receive' define cómo el actor maneja los mensajes que le llegan
  def receive = {
    // Si el mensaje es el string "incr", incrementa el contador
    case "incr" => count += 1
    // El resto de mensajes se ignoran, porque la PartialFunction no define un caso para estos.
    // Si se quiere definir un caso default, se usa el underscore (_)  
  }
}
```

#### Exponiendo el estado

```scala
class Counter extends Actor {
  var count = 0

  // Añadir el mensaje para get
  def receive = {
    case "incr" => count += 1
    case ("get", customer: ActorRef) => customer ! count
  }
}
```

- `!` es el operador para mandar mensajes en `Akka`
    - Akka es la librería de Scala para actores
- `customer` es un `ActorRef`
    - get le manda el count a un actor que puede recibir un entero en su método `receive`

#### Ejemplo para apoyar lo anterior

```scala
// Es un actor simplón que recibe el contador y lo imprime
class Printer extends Actor {
  def receive = {
    // Que el nombre del parámetro sea el mismo en un actor y otro es casualidad
    // Es buena práctica pero no es necesario
    case count: Int => println("Printer received count: " + $count)
  }
}
```

**Uso**:

```scala
// Inicializar el sistema de actores (posteriormente se ve cómo)
// Supongamos que counter y printer ya está inicializados anteriormente
counter ! "incr"
counter ! "incr"
counter ! "incr"

// Se le pide al contador que envíe su valor actual al printer
counter ! ("get", printer)
```

## ¿Cómo se mandan los mensajes?

```scala
trait Actor {
  // 'self' es una referencia implícita a su propia instancia de actor
  // Le permite al actor referirse a su propia dirección sin pasarla de manera explícita
  implicit val self: ActorRef

  // 'sender' nos da acceso a quien envía el mensaje que actualmente está siendo procesado
  // Esto es útil para responder mensajes - se puede hacer `sender ! reply`
  def sender: ActorRef
  // ...
}
```

### Qué es un ActorRef?

```scala
abstract class ActorRef {
  // El "bang" o ! es la manera principal de enviarle un mensaje a otro actor
  // - 'msg: Any': se puede mandar cualquier tipo de mensaje
  // - 'implicit sender': el sender se pasa de manera implícita, de tal manera que el receptor sabe quién lo mandó
  def !(msg: Any)(implicit sender: ActorRef = Actor.noSender): Unit

  // `tell` es un alias para el !
  // Hace que el llamado sea más explícito al pasar tanto el mensaje como el remitente
  def tell(msg: Any, sender: ActorRef) = this.!(msg)(sender)
}
```

- En definitiva, un `ActorRef` es una referencia utilizable hacia un `Actor`
    - Se suelen pensar como la "dirección de mail" del actor en cuestión
- Justamente como el sender está implícito, si no le paso nada me lo mando a mí mismo
- `implicit` es syntax sugar de Scala

### Usando el Sender

```scala
class Counter extends Actor {
  var count = 0

  def receive = {
    case "incr" => count += 1
    case "get" => sender ! count
  }
}
```

Un ejemplo para verlo de afuera sería:
> Nota: este ejemplo me lo crafteé yo

```scala
class Multiplier extends Actor {
  def receive = {
    case x: Int => if (x < 10) self ! (x * 2)
    case ("ask", customer: ActorRef) => customer ! "get"
  }
}

// Suponer multiplier ya inicializado
counter ! "incr"
counter ! "incr"
multiplier ! ("ask", counter) // --> esto va a multiplicar por 2 recursivamente de manera infinita el valor de counter
// El flujo va a ser multiplier ask => counter get => le mando count al multiplier => Multiplier se llama a sí mismo recursivamente hasta que sea mayor a 10
```

[//]: # (Más slides)

### Interactuando con el Printer

```scala
class Printer extends Actor {
  def receive = {
    // Acá muestra que cuando le llega un mensaje cualquiera lo imprime
    // y después le manda al sender un mensaje con un re texto
    case count: Int =>
      // Imprimir el count que le llegó
      println(s"[${self.path.name}] received count: $count")
      // Le mando un ACK a quien me lo envió
      // `sender` me da acceso a la referencia de quien sea que me mandó el mensaje en primer lugar
      sender ! s"Acknowledged count $count from ${self.path.name}"
  }
}
```

```scala
class CounterClient(printer: ActorRef) extends Actor {
  def receive = {
    // Este actor recibe el ack del Printer
    case ack: String => println(s"[${self.path.name}] got reply: $ack")
  }

  // Este método se ejecuta on init del objeto
  override def preStart(): Unit = {
    // envía un número al printer usando '!' (asynchronous fire-and-forget)
    // 'self' se va a usar implícitamente como sender
    // Esta instancia de CounterClient va a ser el sender la primera vez
    printer ! 42
    // Le mando otro número de manera explícita usando 'tell' y 'self'
    printer.tell(99, self)
  }
}
```

- En un programa estándar con estos 2 objetos instanciados:
    - se le manda al printer un 42
    - luego se le manda un 99
    - en ambos casos con el CounterClient como Sender

## Actor Context

En el modelo de los actores, el contexto es el ambiente donde el actor está corriendo.
Dentro de lo que puede hacer, le puedo pedir al contexto:

- Crear otros actores
- Cambiar su comportamiento de manera dinámica
- Acceder a referencias de sí mismo y de los remitentes
- "Frenarse" a sí mismo o a otros actores

El actor describe el comportamiento, la ejecución la realiza su `ActorContext`

### En código

```scala
trait ActorContext {

  // Me permite actualizarle el receive al actor actual
  def become(behavior: Receive, discardOld: Boolean = true): Unit

  // Vuelve para atrás al último comportamiento guardado en caso de que
  // discardOld era `false` en el llamado del become
  def unbecome(): Unit
}
```

- Otros métodos útiles del contexto pueden ser:
    - `actorOf(...)` para instanciar actores hijos
    - `stop(...)` para frenar un actor
    - `self`, `sender`, `parent`, `children`

### Ejemplo

```scala
class ToggleActor extends Actor {
  def on: Receive = {
    case "switch" =>
      println("Turning off...")
      context.become(off)
  }

  def off: Receive = {
    case "switch" =>
      println("Turning on...")
      context.become(on)
  }

  // Este es el comportamiento inicial, arranca prendido
  def receive = on
}
```

### Functional Counter

Se puede definir a la clase Counter de manera funcional (sin variables mutables):

```scala
class Counter extends Actor {
  def counter(n: Int): Receive = {
    case "incr" => context.become(counter(n + 1))
    case "get" => sender ! n
  }

  def receive = counter(0)
}
```

- Se crea un lambda con un parámetro preseteado
    - En el fondo guarda el valor del parámetro en la definición del lambda
- Cuando se instancia el Counter, n = 0

## Crear y detener actores

Definiendo el trait de ActorContext...

```scala
trait ActorContext {
  // Se spawnea un actor hijo del actor actual
  // - 'p' : es un objeto `Props`, define el tipo de actor y los parámetros de su constructor 
  // - 'name': es un nombre único para este nuevo actor dentro del contexto actual
  def actorOf(p: Props, name: String): ActorRef

  // Se frena o termina el actor
  def stop(a: ActorRef): Unit
}
```

## Aplicación completa de actores

```scala
class CounterMain extends Actor {
  // Create an instance of the Counter actor as a child of this actor
  val counter: ActorRef = context.actorOf(Props[Counter], "counter")

  // Send some increment messages to the counter
  counter ! "incr"
  counter ! "incr"
  counter ! "incr"

  // Ask the counter to send its current value back (reply goes to this actor)
  counter ! "get"

  // This actor handles the reply from the counter
  def receive: Receive = {
    case count: Int =>
      println(s"Count was $count") // Print the count
      context.stop(self) // Stop this actor (ends the app)
  }
}
```

### El main sobre el que corre:

```scala
object CounterMainApp extends App {

  // Create the actor system
  val system = ActorSystem("CounterSystem")

  // Create the main actor that orchestrates everything
  system.actorOf(Props[CounterMain], "main")

  // The system will shut down after the CounterMain actor stops (not shown here)
  // For a clean shutdown, you could use CoordinatedShutdown or watch termination manually
}
```

## ¿Qué es el modelo del que venimos hablando?

Siempre que un actor reciba un mensaje puede hacer cualquier combinación de las siguientes acciones:

- **Crear mensajes**: comunicarse con otros actores de manera asíncrona
- **Crear actores** (hijos de sí mismo): crear actores hijos para delegar trabajo o estructurar el sistema de manera
  jerárquica
- **Cambiar su comportamiento para próximos mensajes de manera dinámica**

Los actores encapsulan tanto estado como comportamiento, permitiendo concurrencia sin locks y segura al reaccionar a los
mensajes.

## Encapsulación de los Actores

No tienen getters ni setters, se debe manejar su estado a través de mensajes

Están aislados: no se puede acceder al estado ni a su comportamiento de manera directa, sólamente interactuando desde el
lado de otro actor (via pasaje de mensajes usando direcciones conocidas, sus `ActorRef`)

- Cada actor conoce su **referencia** (`self`)
- Crear un actor devuelve su **propia referencia**.
- Las referencias (o direcciones) se pueden compartir y pasar entre mensajes (ej: usando `sender`)

Este modelo fuerza aislamiento y previene problemas de memoria compartida como condiciones de carrera

### Orden de evaluación de los Actores

* Cada actor dentro de sí mismo es single-threaded, con lo cual los mensajes van llegando secuencialmente
    * Llamar a `context.become` cambia su comportamiento frente al próximo mensaje
    * Cada mensaje es **atómico**, ya que no existe el interleaving entre actores

* Los actores procesan un mensaje a la vez
    * No hay overlap entre manejadores de mensajes
    * Los cambios de comportamiento aplican al próximo mensaje
    * La atomicidad asegura actualizaciones seguras del estado local

> Es muy parecido al `synchronized` de Java, solo que sin bloqueo; en su lugar se encolan los mensajes.

## Trade-Offs

> Esto lo anoté en base a lo que me dijeron los profes

- Te atás al asincronismo, no tenés respuestas inmediatas
- No existe memoria compartida (**esto es importante**)
    - Cada actor tiene sus propias variables y espacios de memoria alocados
    - Solo se comparte memoria a través de mensajes
- Añade una capa de complejidad importante
    - Es más difícil de debuggear