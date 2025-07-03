# Parcial 2C 2024

## Enunciado
**1.** Para cada uno de los siguientes fragmentos de código indique si es o no es un busy wait. Justifique en cada caso.
1. 
```rust
fn busy_wait_1() {
    loop {
        match TcpStream::connect("127.0.0.1:8080") {
            Ok(mut stream) => {
                stream.write_all(message.as_bytes()).expect("Error")
            }
            Err(_) => {
                let random_result: f64 = rand::thread_rng().gen();
                thread::sleep(
                    Duration::from_millis((5000f64 * random_result) as u64)
                );
            }
        }
    }
}
```
2. 
```rust
fn busy_wait_2() {
    let random_result: f64 = rand::thread_rng().gen();
    thread::sleep(Duration::from_millis(random_result as u64))
    let mut items = self.pending_acks.lock().unwrap();
    let now = Instant::now();
    let mut i = 0;
    while i < items.len() {
        if items[i].expiration <= now {
            if items[i].item_type == "ACK" {
                let _ = items.remove(i);
                drop(items);
                self.send_result_interfaces();
                break;
            }
        } else { i += 1 }
    }
}
```
3. 
```rust
fn busy_wait_3(){
    let copper = Arc::clone(&resource);
    thread::spawn(move || loop {
       let mined_amount = rand::thread_rng().gen_range(1..10);
        *copper.write().expect("failed to mine") += mined_amount;
        let delay = rand::thread_rng().gen_range(3000..7000);
        thread::sleep(Duration::from_millis(delay));
    });
}
```



**2.** Modelar una Red de Petri para el problema del Lector-Escritor sin preferencia. Luego, modele una solución que contemple preferencia de escritura.


**3.** Se quiere abrir un restaurante en el barrio de San Telmo. Se espera que los clientes lleguen y sean atendidos por alguno de los mozos de turno, cada uno de los cuales tomará los pedidos de cada mesa, los notificará a la cocina y luego seguirá tomando otros pedidos. 

Como la cocina es chica los cocineros pueden entrar a buscar los ingredientes al depósito de a uno a la vez, y buscar entre los distintos alimentos les puede llevar un tiempo variable.

Cuando los cocineros hayan terminado de preparar un pedido deben notificar a los mozos para que lo lleven a la mesa. Además, los mozos deben estar disponibles para cobrarle a los clientes. 

Diseñe el sistema utilizando el modelo de actores, y para cada entidad defina cuáles son los estados internos y los mensajes que intercambian.

**4.** Verdadero o Falso. Justifique:
- **a.** Procesos, hilos y tareas asincrónicas poseen espacios de memoria independientes.
- **b.** El scheduler del sistema operativo puede detener una tarea asincrónica puntual y habilitar la ejecución de otra para el mismo proceso.
- **c.** Tanto los threads, como las tareas asincrónicas disponen de un stack propio.
- **d.** En un ambiente de ejecución con una única CPU, un conjunto de hilos de procesamiento intensivo tomarán un tiempo de ejecución significativamente menor a un conjunto de tareas asincrónicas que ejecuten el mismo procesamiento.

**5.** Describa y justifique con qué modelo de concurrencia modelaría la implementación para cada uno de los siguientes casos de uso:
- **a.** Convertir un conjunto extenso de archivos de .DOC a .PDF
- **b.** El backend para una aplicación de preguntas & respuestas competitiva al estilo Menti o Kahoot.
- **c.** Una memoria caché utilizada para reducir la cantidad de requests en un servidor web a una base de datos.
- **d.** Una API HTTP que ejecuta un modelo de procesamiento de lenguaje natural para clasificar el sentimiento de un mensaje.

## Resolución
### Primer ejercicio
1. **No es un busy wait**. Si le llega un stream por TCP, le genera una respuesta. Si no, duerme el thread y no consume recursos innecesarios.
2. **Es un busy wait**. Además de que es una implementación ineficiente (porque duerme el thread de manera innecesaria una cantidad de tiempo aleatoria), se queda esperando e iterando indefinidamente por el estado de los ACKs pendientes. Nunca droppea el lock para que otro thread pushee un ACK al arreglo.
3. **No es un busy wait**, no pregunta por el estado del cobre, sino que independientemente del resultado (es decir, si pudo adquirir el lock para ejecutar o no), duerme el thread para no seguir consumiendo recursos.

### Segundo ejercicio
Red de Petri, skippeado

### Tercer ejercicio


### Cuarto ejercicio

a) **Falso.** Cada proceso tiene su propio espacio de memoria; los hilos usan y comparten el espacio de memoria de su proceso padre, y las tareas asincrónicas son "hilos más ligeros", por lo que también usan el mismo espacio de memoria.

b) **Verdadero.** Esto tiene que ver con el scheduling preventivo, que constaba de que un proceso puede ser interrumpido o terminado en un momento dado por parte del scheduler. Las tareas asíncronas tienen el mismo comportamiento

c) **Verdadero.** (No sé cómo justificarlo)

d) **Verdadero.** Las tareas se caen a los pedazos para procesamientos pesados.

### Quinto ejercicio

- **a.** Convertir un conjunto extenso de archivos de .DOC a .PDF
  - Usaría vectorización, tanto para la cantidad de archivos como por si los archivos son demasiado grandes. Como en el TP de Grep.
- **b.** El backend para una aplicación de preguntas & respuestas competitiva al estilo Menti o Kahoot.
  - Usaría asincronismo, dado que las preguntas y respuestas son rápidas y ligeras. No necesito estar escuchando <img src="escuchando.png" alt="escuchando" width="32" height="32"/> constantemente
- **c.** Una memoria caché utilizada para reducir la cantidad de requests en un servidor web a una base de datos.
  - Usaría un RwLock, o un Monitor, dado que estás accediendo a estado mutable compartido constantemente.
- **d.** Una API HTTP que ejecuta un modelo de procesamiento de lenguaje natural para clasificar el sentimiento de un mensaje.
  - Es una API, necesitás usar asincronismo sí o sí. No hay ninguna API HTTP que no ejecute requests asincrónicas.
