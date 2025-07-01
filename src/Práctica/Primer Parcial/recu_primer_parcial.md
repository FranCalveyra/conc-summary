# Recuperatorio Primer Parcial 2025
## 1) Deadlock y livelock (qué son y ejemplos)
## 2) V o F “Two processes can execute their critical sections simultaneously if they use binary semaphore”
## 3) Barber Shop
```rust
// Each client takes a seat or leaves
struct BarberShop {
    capacity: usize,
    //...
}
impl BarberShop{
    //...
}

fn main() {
    let shop = Arc::new(BarberShop::new(3));
    let mut handles = Vec::new();
    for i in 0..10 {
        let shop = Arc::clone(&shop);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100));
            if shop.enter(i as usize){
                shop.wait_for_turn_and_cut(i as usize);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

```
## 4) Manejar mas barbers
## 5) Manejar clientes VIP

# Resolución

> Nota: no estuve en este recu, entonces no sé qué consideraciones especiales tuvieron los docentes. Tomen la resolución con pinzas. Si algo no les hace sentido, díganme.

## 1) Deadlock y livelock (qué son y ejemplos)
Un **deadlock** es una condición de bloqueo en el contexto de la programación concurrente, la cual consta de 2 o más hilos/procesos esperando por la liberación de recursos de manera mutua, bloqueando completamente la ejecución del programa.
### Ejemplo
Tengo 2 personas (`p1` y `p2`) queriendo escribir; en una base de datos:
- `p1` comienza a escribir el recurso `A` , lockeándolo.
- `p2` comienza a escribir el recurso `B`, lockeándolo.
- Previo a finalizar su operación de escritura, `p1` quiere leer `B`. Como está lockeado por `p2`, no puede avanzar
- A su vez, `p2` quiere leer `A`. Como está lockeado por `p1`, no puede avanzar.

De esta manera, la ejecución del programa se bloquea por completo.

En cuanto al **livelock**, también es una condición de bloqueo, pero consta de que 2 procesos/hilos se queden esperando por que otro proceso avance / cambie de estado. No se frena la ejecución como tal, pero los procesos quedan cambiando constantemente de estado, sin poder avanzar.

### Ejemplo
Tengo un matrimonio (`p1` y `p2`) queriendo comer, pero sólo hay 1 cuchara.
Ambos quieren que el otro coma primero por cuestiones de educación, entonces se produce la siguiente secuencia:
- `p1` ve la cuchara, la quiere agarrar, pero ve a `p2` que no tiene, y le dice que coma primero.
- `p2` ve la cuchara, la quiere agarrar, pero ve a `p1` que no tiene, y le dice que coma primero.

Esto se repite infinitamente, sin ningún avance.

## 2) V o F “Two processes can execute their critical sections simultaneously if they use binary semaphore”

Esto es **falso**. Justamente la sección crítica no se va a poder ejecutar en paralelo si el recurso es lockeado por el otro proceso. Precisamente porque el _semáforo binario_ es análogo a un `Mutex`.

## 3) Barber Shop
```rust
struct BarberShop {
    capacity: usize,
    clients: Mutex<VecDeque<usize>>,
    can_cut: Condvar
}
impl BarberShop{
    fn new(capacity: usize)-> Self{
        let clients = Mutex::new(VecDeque::with_capacity(capacity));
        let can_cut = Condvar::new();
        BarberShop{ capacity, clients, can_cut }
    }

    fn enter(&self, client: usize)-> bool{
        let mut clients = self.clients.lock().unwrap();
        if clients.len() == self.capacity{
            return false;
        }
        clients.push_back(client);
        self.can_cut.notify_all();
        true
    }

    fn wait_for_turn_and_cut(&mut self, client: usize){
        let mut clients = self.clients.lock().unwrap();
        while *clients.front().unwrap() != client{
            clients = self.can_cut.wait(clients).unwrap();
        }
        clients.pop_front();
        drop(clients);
        self.can_cut.notify_all()
    }
}
```
## 4) Manejar más barbers
Para manejar más barberos, se deberían modelar los barberos como recursos de un semáforo.

El `BarberShop` debería tener un `Semaphore(N)`, con tal de poder asignar un barbero por cliente.
Se debería modificar el `wait_for_turn_and_cut`, para que trate de adquirir el recurso sobre estos. Además, debería de chequear que haya un barbero disponible en el `while`, verificando que pueda adquirir un recurso del semáforo.

El uso de la `Condvar` se mantiene, y cada cliente adquiere un recurso del semáforo, señalizando a todos cuando termina de cortarse. A su vez, al finalizar, libera el recurso del semáforo que adquirió

## 5) Manejar clientes VIP

Para manejar clientes VIP, además de mantener los cambios anteriores (del punto 4), hay que establecer un esquema de prioridad. Por ejemplo, modelando los clientes como structs (`struct Client { id: usize, priority: usize }`).

Se debería modificar el `wait_for_turn_and_cut` para que los ordene según su prioridad (independientemente de si llegaron antes o después, se debe ordenar la `VecDeque`).