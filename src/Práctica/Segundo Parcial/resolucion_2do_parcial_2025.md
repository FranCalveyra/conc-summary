# Segundo Parcial 2025
## Primer ejercicio - Non-Blocking Concurrent Stack
Dada la siguiente implementación de un Stack, haz los cambios necesarios para que sea una estructura concurrente no bloqueante:
```kotlin
class Stack<E> {
    class Node<E>(val item: E, var next: Node<E>? = null)

    private var top: Node<E>? = null

    fun push(item: E) {
        val newHead = Node(item)
        newHead.next = top
        top = newHead
    }
    
    fun pop(): E? { 
      val oldHead = top
      if (oldHead == null) return null
      top.set(oldHead.next)
      return oldHead.item
    }
}
```
> Nota: puede resolverse tanto en Kotlin como en Rust

## Segundo ejercicio - Corutinas
Dada la siguiente corutina, predecir el output:
```kotlin
suspend fun compute(name: String, delayTime: Long): Int {
    delay(delayTime)
    println("Done with $name")
    return name.length
}

fun main(): Unit = runBlocking {
    println("Start")

    launch {
        val launchValue = compute("LaunchTask", 300L)
        println("Launch result: $launchValue")
    }

    val deferred = async {
        val result = compute("AsyncTask", 200L)
        println("Async result: $result")
        result
    }

    println("Middle")

    val final = deferred.await()
    println("Final result: $final")
}
```

## Tercer ejercicio - Actores de una subasta
Implementar un _**sistema de subastas concurrente**_ usando actores en `Scala`, con la librería `Akka`. El sistema involucra 3 tipos de actores:
- **Auction / Subasta**: recibe pujas y registra la más alta.
- **Bidder / Pujador** : envía pujas cada cierto tiempo
- **Organizer / Organizador**: empieza la subasta y recibe el resultado

La subasta empieza cuando el **Auction** recibe un mensaje del tipo `StartAuction`. Los Bidders envían las pujas (con un ID y una cantidad) por un tiempo específico.

Cuando la subasta termina, se le anuncia el ganador al Organizer.

1. Define el protocolo (los mensajes que se intercambian)
2. Implementar el comportamiento de cada actor

### Código de ejemplo (main del programa)
```scala
object ActorSystem extends App {
  val system = ActorSystem("AuctionSystem")

  val organizer = system.actorOf(Props[Organizer], "organizer")
  val auctionDuration = 15.seconds
  
  val auction = system.actorOf(Props(new Auction(auctionDuration)), "auction")

  auction ! StartAuction(organizer)

  // Bidder A places bids up to 50 for 8 seconds
  system.actorOf(Props(new Bidder(auction, "A", 50, 8.seconds)), "bidderA")

  // Bidder B places bids up to 100 for 10 seconds
  system.actorOf(Props(new Bidder(auction, "B", 100, 10.seconds)), "bidderB")

  // Bidder C places bids up to 75 for 6 seconds
  system.actorOf(Props(new Bidder(auction, "C", 75, 6.seconds)), "bidderC")
}
```

### Operaciones útiles
```scala
// Send a "HELLO" message to itself after some duration
context.system.scheduler.scheduleOnce(duration, self, "HELLO")

// From the first moment the actor is created, after 50 milliseconds it sends itself a Tick message
context.system.scheduler.scheduleWithFixedDelay(Duration.Zero, 50.millis, self, Tick)

// Returns a random number from 0 to max
private val random = new Random()
random.nextInt(max)

```

# Resolución
## Primer ejercicio - Non-Blocking Concurrent Stack

### Resolución en Kotlin
```kotlin
class NonBlockingStack<E> {
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
        while (oldHead != null && !top.compareAndSet(oldHead, oldHead.next))
            oldHead = top.get()
        return oldHead?.item
    }
}
```

### Resolución en Rust
```rust
struct NonBlockingStack<T> {
    head: AtomicPtr<Node<T>>,
    size: AtomicUsize,
}

impl<T> NonBlockingStack<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node::dummy()));
        NonBlockingStack {
            head: AtomicPtr::new(dummy),
            size: AtomicUsize::new(0),
        }
    }

    fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node::new(value)));
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe { (*new_node).next.store(head, Ordering::Relaxed) };
            if self
                .head
                .compare_exchange(head, new_node, Ordering::Release, Ordering::Acquire)
                .is_ok()
            {
                self.size.fetch_add(1, Ordering::Release);
                break;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        loop {
            let cur_head = self.head.load(Ordering::Acquire);
            if cur_head.is_null() {
                return None;
            }
            let next_node = unsafe { (*cur_head).next.load(Ordering::Acquire) };
            if self
                .head
                .compare_exchange(cur_head, next_node, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                self.size.fetch_sub(1, Ordering::Release);
                let old_head_node = unsafe { Box::from_raw(cur_head) };
                return old_head_node.item;
            }
        }
    }
}

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn dummy() -> Self {
        Node {
            item: None,
            next: AtomicPtr::new(null_mut()),
        }
    }
    fn new(item: T) -> Self {
        Node {
            item: Some(item),
            next: AtomicPtr::new(null_mut()),
        }
    }
}
```


## Segundo ejercicio - Corutinas
El output va a ser el siguiente:
```
Start
Middle
Done with AsyncTask
Async result: 9
Final result: 9
Done with LaunchTask
Launch result: 10
```
### ¿Por qué?
El programa inicia printeando "Start". Luego, al llegar al `launch` (el cual es un scope de corutina que no devuelve ningún valor), entra a la función `compute`, ve que hay un `delay` y le cede el control a la corutina principal (la función `main`).

Acto seguido, vemos la inicialización de la corutina `async`, que también tiene un `delay` dado que llama a `compute` dentro suyo, por lo que también cede el control a la corutina principal. 

Lo siguiente que se va a ejecutar es el `print("Middle")`.

Luego, se queda esperando por el valor que devuelve el `deferred`, por lo que se ejecutará el print dentro de su llamado a `compute` (`Done with AsyncTask`), y luego se printeará el `Async Result: 9`.

Lo siguiente que pasará es que se printeará el valor de `final result`, dado que depende del valor de la tarea asíncrona.

Por último, se ejecutará el print del `LaunchTask` por volver a cederle el control al `launch`, seguido del `Launch Result: 10`.

## Tercer ejercicio - Actores de una subasta

```scala
object AuctionProtocol {
  case class Winner(id: String, amount: Int) // Se lo manda al Organizer

  case object AnnounceWinner

  case class StartAuction(organizer: ActorRef)
}


class Organizer extends Actor {
  override def receive: Receive = {
    case Winner(id: String, amount: Int) =>
      println(s"The winner is $id with the incredible amount of $amount !")
  }
}

class Auction(duration: FiniteDuration, organizer: ActorRef) extends Actor {
  // Cuando se me termina el tiempo, anuncio el ganador
  context.system.scheduler.scheduleOnce(duration, self, AuctionProtocol.AnnounceWinner)
  private var currentMaxBid: (String, Int) = ("", 0)

  override def receive: Receive = {

    case AuctionProtocol.StartAuction(organizer: ActorRef) =>
      println(s"The auction has started! It's being organized by ${organizer.toString}")

    case BidderProtocol.Bid(id: String, amount: Int) =>
      if (currentMaxBid._2 < amount) { // Si es una puja mayor, la actualizo
        currentMaxBid = (id, amount)
      }

    case AuctionProtocol.AnnounceWinner =>
      organizer ! Winner(currentMaxBid._1, currentMaxBid._2) // Le mando el ganador al organizador
      context.stop(self) // Freno este actor
  }
}

class Bidder(auction: ActorRef, id: String, maxBid: Int, duration: FiniteDuration) extends Actor {
  // Programo para pujar cada cierto tiempo
  context.system.scheduler.scheduleWithFixedDelay(Duration.Zero, 50.millis, self, BidderProtocol.SendBid)
  private val random = Random()
  private var currentBid: Int = 0

  // Tengo que dejar de pujar después de un cierto tiempo
  context.system.scheduler.scheduleOnce(duration, self, BidderProtocol.Stop)

  override def receive: Receive = {

    case BidderProtocol.Stop =>
      context.stop(self) // Freno este actor cuando se me termina el tiempo (es decir, cuando dejo de pujar)

    case BidderProtocol.SendBid =>
      auction ! BidderProtocol.Bid(id, getCurrentMaxBid)
  }

  private def getCurrentMaxBid: Int = {
    val newBid = random.nextInt(maxBid)
    if (newBid > currentBid) {
      currentBid = newBid
    }
    currentBid
  }
}

object BidderProtocol {
  case object SendBid

  case class Bid(id: String, amount: Int)

  case object Stop
}

```
