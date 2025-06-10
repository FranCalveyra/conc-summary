/*
3. **Pool de trabajadores (Worker Pool)**
   - **Descripción:** Un hilo “dispatcher” envía tareas por un canal a un grupo de K hilos trabajadores que las reciben, las ejecutan y, opcionalmente, responden por otro canal de resultados.
   - **Retos conceptuales:**
     - Balanceo de carga entre trabajadores.
     - Manejo de respuestas por un canal distinto sin bloquear al dispatcher.

4. **Broker Pub/Sub manual**
   - **Descripción:** Implementa un broker que recibe mensajes de distintos “publicadores” y, según el tópico, los reenvía a uno o más “suscriptores”. Usa canales internos para el enrutamiento.
   - **Retos conceptuales:**
     - Mantener un mapa tópico → `Sender` múltiple.
     - Detectar suscriptores que cancelan (cerrado de canal).

5. **Rate Limiter por tokens**
   - **Descripción:** Un hilo generador añade “tokens” a intervalos regulares a un canal, y los consumidores deben tomar un token antes de realizar su operación.
   - **Retos conceptuales:**
     - Controlar la frecuencia de producción de tokens.
     - Evitar que los consumidores se bloqueen indefinidamente si no hay tokens.

6. **Handshake (Rendezvous) de dos actores**
   - **Descripción:** Dos hilos A y B deben sincronizarse en un punto: A envía un mensaje por un canal y espera recibir otro de vuelta, y viceversa.
   - **Retos conceptuales:**
     - Evitar deadlocks en la espera mutua.
     - Decidir orden de envío/recepción.

7. **Monitor de salud (Heartbeat)**
   - **Descripción:** Varios hilos trabajadores envían periódicamente un “latido” (por ejemplo, su ID y timestamp) a un hilo monitor que los supervisa. Si no recibe el latido de alguno en un plazo, lo considera caído.
   - **Retos conceptuales:**
     - Diferenciar mensajes de distintos trabajadores.
     - Implementar timeouts sin bloquear el canal.

8. **Tareas con prioridades (colas separadas)**
   - **Descripción:** Dispones de dos canales (alta y baja prioridad). Un “dispatcher” lee primero de la cola alta y, si está vacía, de la baja. Los productores escogen en qué canal enviar según urgencia.
   - **Retos conceptuales:**
     - Implementar la selección no bloqueante de múltiples canales (p. ej., comprobando `try_recv`).
     - Prevenir starvation de la cola baja.
 */

/*
1. **Productor–Consumidor básico**
   - **Descripción:** Varios hilos productores generan datos (por ejemplo, enteros o mensajes) y los envían por un `Sender<T>` a un único hilo consumidor que los procesa.
   - **Retos conceptuales:**
     - Clonar el `Sender` para cada productor.
     - Detectar fin de producción (cerrar el canal).
 */
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

pub fn basic() {
    let (sender, receiver) = channel::<String>();
    for id in 0..10 {
        let sender_clone = sender.clone();
        let message = format!("Hello from thread {id}");
        thread::spawn(move || {
            match sender_clone.send(message) {
                Ok(_) => println!("Message sent successfully!"),
                Err(_) => println!("There was an error sending the message"),
            }
            drop(sender_clone)
        });
    }
    drop(sender);
    for x in receiver.into_iter() {
        println!("{}", x);
    }

    println!("Channel was closed!");
}

/*
2. **Pipeline de N etapas**
   - **Descripción:** Crea N hilos encadenados: cada hilo lee de su canal de entrada, transforma el dato y lo envía por su canal de salida. Al final, un hilo recoge los resultados finales.
   - **Retos conceptuales:**
     - Cómo encadenar múltiples canales `mpsc::channel`.
     - Propagar el cierre al final del pipeline.
 */

fn add_one(x: i32) -> i32 {
    x + 1
}
fn times_two(x: i32) -> i32 {
    x * 2
}
fn square(x: i32) -> i32 {
    x * x
}
fn neg(x: i32) -> i32 {
    -x
}
fn add_ten(x: i32) -> i32 {
    x + 10
}

pub fn pipeline() {
    let functions: Vec<NodeFunction<i32>> = vec![add_one, times_two, square, neg, add_ten];
    let pipeline = Pipeline::new(functions);

    let value = pipeline.run(10);
    println!("Pipeline final value: {}", value);


}

struct Pipeline<T> {
    first_sender: Sender<T>,
    last_receiver: Receiver<T>,
    nodes: Vec<PipelineNode<T>>,
}

type NodeFunction<T> = fn(T) -> T;
impl<T: Send + 'static> Pipeline<T> {
    pub fn new(funcs: Vec<NodeFunction<T>>) -> Self {
        let (first_sender, mut prev_rx) = channel::<T>();
        let (last_tx, last_rx) = channel::<T>();
        let n = funcs.len();
        let mut nodes = Vec::with_capacity(n);
        for (i, f) in funcs.into_iter().enumerate() {
            let (tx, rx) = if i + 1 == n {
                (last_tx.clone(), { channel::<T>().1 })
            } else {
                channel::<T>()
            };

            nodes.push(PipelineNode {
                id: i as i32,
                sender: tx,
                receiver: prev_rx,
                pipeline_function: f,
            });

            prev_rx = rx;
        }

        Pipeline {
            first_sender,
            nodes,
            last_receiver: last_rx,
        }
    }

    pub fn run(self, initial: T) -> T {
        for node in self.nodes {
            thread::spawn(move || {
                loop {
                    let x = node.receiver.recv().expect("canal cerrado");
                    let y = (node.pipeline_function)(x);
                    node.sender.send(y).expect("no se puede enviar");
                    println!("Nodo {} procesó un valor", node.id);
                }
            });
        }

        self.first_sender
            .send(initial)
            .expect("Error sending message");
        self.last_receiver.recv().unwrap()
    }
}

struct PipelineNode<T> {
    id: i32,
    sender: Sender<T>,
    receiver: Receiver<T>,
    pipeline_function: fn(T) -> T,
}

impl<T: Send> PipelineNode<T> {
    pub fn new(
        sender: Sender<T>,
        receiver: Receiver<T>,
        pipeline_function: fn(T) -> T,
        id: i32,
    ) -> Self {
        PipelineNode {
            id,
            sender,
            receiver,
            pipeline_function,
        }
    }
}
