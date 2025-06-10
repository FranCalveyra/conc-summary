# Actores - Parte II

En qué beneficia hacer Garbage Collections atómicos, chiquitos y periódicos contra una pasada grande?

- Dependiendo de si las variables se alocan en el Heap o en el Stack (en el caso de la barrida enorme):
    - Si se alocan en el Stack, todo pelota, no pasa natalia
    - Si se alocan en el Heap, hay que revisar (como si fuese un grafo, porque justamente se aloca un puntero a ese
      elemento), el cual puede terminar teniendo más referencias, te comés el garrón de revisar toda la memoria
        - Justamente como tenés que revisar toda la memoria en estos casos, se ralentiza el programa
        - Traducido a un programa interactivo o In Real Time, ves la ruedita cargando, o se te caga la performance, en
          definitiva

En este sentido, como el Garbage Collection en el modelo de Actores se hace por actor, se cae en el caso más optimizado,
o mejor dicho, \
dejás de tener todas las complicaciones que tiene el primer caso.

## Bank Account - Revisitado con actores

```scala
object BankAccount {
  case class Deposit(amount: BigInt)

  case class Withdraw(amount: BigInt)

  case object Done

  case object Failed
}
```

- Acá no estamos definiendo un actor, sino un objeto/clase
- Las `case classes` son clases que se pueden declarar de manera corta, como si fuese un enum
    - Son análogos a los `Records` de `Java`

```scala
class BankAccount extends Actor {
  var balance: BigInt = BigInt(0)

  def receive: Receive = {
    case Deposit(amount) =>
      balance += amount
      sender ! Done
    case Withdraw(amount) if amount <= balance =>
      balance -= amount
      sender ! Done
    case _ => sender ! Failed
  }
}
```

- El pattern matching se hace por el tipo de objeto, en este caso
- El if en el caso del `Withdraw` hace que falle (o lo deriva al caso default, mejor dicho) si `amount` es **mayor**
  a `balance`

### Colaboración de actores

- La idea es imaginarse cada actor como una persona
- O cada acción o actividad como actores

Dependiendo del caso, usamos un approach u otro

Puedo modelar un actor encargado de hacer las transferencias bancarias, que interactúe entre cuentas bancarias.

Es decir, uso un actor intermedio

```scala
object WireTransfer {
  case class Transfer(from: BankAccount, to: BankAccount, amount: BigInt)

  case object Done

  case object Failed
}

class WireTransfer extends Actor {
  def receive: Receive = {
    case Transfer(from, to, amount) =>
      from ! BankAccount.Withdraw(amount)
      context.become(awaitWithdraw(to, amount, sender))
  }

  def awaitWithdraw(to: ActorRef, amount: BigInt, client: ActorRef): Receive = {
    case BankAccount.Done =>
      to ! Deposit(amount)
      context.become(awaitDeposit(client))
    case BankAccount.Failed =>
      client ! Failed
      context.stop(self)
  }

  def awaitDeposit(client: ActorRef): Receive = {
    case BankAccount.Done =>
      client ! Done
      context.stop(self)
    case BankAccount.Failed => // Este caso lo escribí yo, que dijo Emilio que faltaba en el slide
      client ! Failed
      context.stop(self)
  }
}
```

- No puedo confirmar si la transferencia salió bien o no hasta que efectivamente me llegó el Withdraw de la primera
  cuenta
- Justamente, me quedo esperando el Withdraw, y una vez me llegó lo mando a la cuenta de destino
- Es decir, lo **deposito** al destinatario.
    - Si el depósito sale bien, lo propago para arriba y freno el actor, o lo elimino, justamente para que sea atómico
      todo.
- Si falló hago 2 cosas:
    - Propago el fallo
    - Freno al actor (esto en realidad se puede resolver de otra manera)
- El depósito en principio no debería fallar, porque no depende de si tenés saldo o no
    - Pero se modela (por las dudas?)

## Garantías de entrega de mensajes

- Si no se piden explícitamente, las garantías de entrega son más bajas
- _"Todo se puede ir al diablo en cualquier momento"_ a.k.a. _"Let it crash"_
- La idea es que pienses que como que todo puede fallar, te asegures de que dejás el sistema en un estado consistente
- La entrega del mensaje requiere disponibilidad eventual del canal y del receptor

### Garantías

Dependiendo del caso se implementa uno u otro protocolo, y un manejo de estados diferentes.

- **at-most-once**: enviar el mensaje lo entrega 0 o 1 veces
    - Puede no llegar
- **at-least-once**: enviar el mensaje entrega 1 - N veces el mensaje
    - Llega una o más veces
- **exactly-once**: procesar sólo la primera recepción entrega el mensaje 1 vez
    - Este approach es muchísimo más burocrático
    - Es más caro en recursos y en implementación
    - Tenés que:
        - Recibir el mensaje efectivamente
        - Mandar un ACK para notificarle al otro que recibiste el mensaje
        - Que el otro te mande un ACK para notificarte que recibió el ACK

### Mensajería confiable

Los mensajes soportan confiabilidad:

- Todos los mensajes se pueden persistir
- Pueden incluir correlation IDs únicos
- Se puede reintentar hasta que la entrega sea exitosa

**La confiabilidad solo puede ser asegurada por acknowledgement a nivel lógica de negocio**

### En el caso de la transferencia...
Para volverla confiable habría que:
- Registrar actividades del `WireTransfer` a almacenamiento persistente
- Hacer que cada transferencia tenga un ID único
- Añadirle un ID al Withdraw y al Deposit
- Almacenar IDs de acciones completadas en la `BankAccount`

## Orden de mensajes

Si un actor manda varios mensajes al mismo destinatario, no van a llegar desordenados (esto es específico de Akka)

## Diseñando un modelo de actores

- Imagínate darle una tarea a un grupo de personas y dividirla en partes
- Considerá que el grupo puede ser muy grande
- Empezá a pensar como las personas asignadas a las diferentes tareas van a comunicarse entre sí
- Considerá que cada "persona" puede ser fácilmente reemplazable
- Dibujá un diagrama de cómo se va a dividir la tarea, incluyendo líneas de comunicación

Los problemas de escalabilidad de este tipo de diseños se dan si se quiere hacer un `Actor` "superpoderoso", \
o pensando que hay un actor irreemplazable

En un sistema de actores bien diseñado, el grafo no debería ser muy complejo. Si tengo muchas vueltas para atrás estoy
haciendo algo mal

En el caso de las transferencias, se nos complejiza por las vueltas para atrás, porque es un problema transaccional con
el que hay que tener cuidado

## Let It Crash

Si uno quiere hacer un diseño razonable con actores, tiene que pensar por este lado
> Abrazar el fallo antes que prevenirlo (?)

- Se esperan errores en sistemas distribuidos
- La Programación Defensiva lleva a complejidad y rigidez
- El modelo de actores aísla fallas: los actores crashean y restartean sin afectar otros

> En Erlang/Elixir: "fail fast, recover quickly"

### Por qué funca Let It Crash?

- Cada actor está aislado: un crasheo afecta a un único actor
- Si falla un actor, su supervisor puede reiniciarlo o manejarlo
    - Cuando se creaba un actor, el de arriba era "responsable" por los de abajo
    - Se crea una jerarquía
    - Por ejemplo, en caso del fallo de un hijo, el padre lo puede restartear a manopla
- No se necesita un manejo de errores complejo dentro de cada actor

## Árboles de supervisión
Como los actores pueden supervisar a sus hijos:
- Los supervisores detectan fallos y aplican estrategias de reinicio
- Los fallos no propagan el caos, sino que se contienen
- La estructura forma una **jerarquía de supervisión** (en forma de árbol)
```scala
val child = context.actorOf(Props[Worker], "worker")
```
> Los árboles reflejan modularidad y controlan el alcance de la recuperación

## Estrategias de supervisión

Las estrategias más comunes incluyen:

- `Restart`: recrear el actor de 0
- `Resume`: ignorar el fallo y continuar
- `Stop`: terminar al actor, eliminarlo
- `Escalate`: propagar el error hacia arriba

```scala
override val supervisorStrategy =
  OneForOneStrategy() {
    case _: ArithmeticException => Resume
    case _: NullPointerException => Restart
    case _: Exception => Stop
  }
```

## Diseñando en torno a la resiliencia

Tips para diseñar:

- Componer el sistema de actores chiquitos, que puedan crashear tranquilamente
- Asignar supervision claramente: quién es responsable de quién?
- Evitar try-catches complejos: lo mejor es apoyarse en el esquema de supervisión
- La estructura sigue límites en base a los posibles fallos
> La resiliencia es una decisión de arquitectura, no un pensamiento posterior