---
marp: true
theme: gaia
paginate: true
style: |
  /* mdBook Coal dark theme */
  section {
    background-color: #282c34;
    color: #abb2bf;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 1.8rem;
  }
  section::before {
    background: none;
  }
  h1 {
    color: #61afef;
    font-size: 2rem;
  }
  h2 {
    color: #61afef;
    font-size: 1.75rem;
  }
  h3 {
    color: #61afef;
    font-size: 1.5rem;
  }
  h4 {
    color: #61afef;
    font-size: 1.25rem;
  }
  h5 {
    color: #61afef;
    font-size: 1rem;
  }
  pre {
    background-color: #21252b;
    border: 1px solid #3e4451;
    border-radius: 3px;
    padding: 1em;
    max-height: 70vh;
    overflow: auto;
  }
  code {
    background-color: #21252b;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    color: #abb2bf;
  }
---

# Programación Concurrente

### Docentes:
- Emilio López Gabeiras
- Rodrigo Pazos

### Integrantes:
- Marcos Sasaki
- Francisco Calveyra
___

# Trabajo Práctico N°1
## Server de Serie de Leibniz - Single Threaded
### Problema planteado:
- Calcular el término `i` de la serie de Leibniz
- Hacerlo en un servidor single-threaded de Rust usando sólo la librería estándar (`std`)

### Solución:
```rust
pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
```
___
# Cálculo:
```rust
fn sequential_leibniz(begin: i32, end: i32) -> f64 {
    (begin..=end).map(|i| term(i)).sum::<f64>() * 4f64
}

fn term(i: i32) -> f64 {
    (-1f64).powi(i) / (2 * i + 1) as f64
}
```
___

# Preguntas abiertas
1. ¿Qué sucede con dos requests simultáneas que tardan en procesarse?
2. ¿Por qué se observa este comportamiento?
3. ¿Cómo solucionar usando solo librerías estándar de Rust?
# Conclusiones
1. Una se encola después de la otra.
2. Es un servidor single-threaded.
3. Como veremos en el TP N°2, levantando un `thread` por cada request, usando `thread::spawn`
___

# Trabajo Práctico N°2
## Servidor Concurrente - Un hilo por conexión
### Problema planteado:
- Mejorar el servidor de TP1 para manejar múltiples solicitudes concurrentes usando threads
### Solución:
```rust
fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Now spawns a thread per request
        thread::spawn(|| handle_connection(stream));
    }
}
```
___

# Preguntas abiertas
- Bajo carga concurrente intensa (aumentando -n y -c en ab), ¿qué
efectos se observan en el comportamiento del servidor? ¿Se nota alguna
diferencia en los tiempos de respuesta, la latencia o el comportamiento
general? ¿A qué se debe?
# Conclusiones
Siendo N el número de requests y C la cantidad de usuarios concurrentes, lo que notamos en cuanto a las diferencias de rendimiento es que, bajo una carga muy alta de concurrencia, el servidor se empieza a ralentizar, llegando a una performance similar a la versión `single-threaded` del TP N°1. Es decir, **empeora**.

Esto se debe al overhead que suponen los cambios de contexto entre threads, ya que se crea un thread por cada request que llega.
___

# Trabajo Práctico N°3
## Thread Pool
### Problema planteado:
- Reducir overhead de creación de hilos en TP2 mediante un pool de hilos reutilizables
### Solución:
```rust
let pool = ThreadPool::new(thread_amount);
for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| handle_connection(stream));
}
```
___
# Thread Pool
```rust
pub struct ThreadPool { workers: Vec<Worker>, sender: Sender<Job> }
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    } 
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static, {
        self.sender.send(Box::new(f)).unwrap();
    }
}
```
___
## Workers
```rust
type JobFunctionType = dyn FnOnce() + Send + 'static;
type Job = Box<JobFunctionType>;
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}
```
___

# Preguntas TP3
1. Bajo carga concurrente intensa (aumentando -n y -c en ab), ¿qué efectos se observan en el comportamiento del servidor? ¿Cómo se comparan con los resultados obtenidos en el TP2?
2. ¿Cómo se ve afectado el comportamiento ante carga concurrente intensa para diferentes tamaños de thread pool ?
# Conclusiones TP3
1. Es significativamente más performante que el servidor desarrollado en el TP2, dado que no existe el problema de los cambios de contexto por tener `Workers`, o hilos que se mantienen vivos.
2. En caso de que el Thread Pool tenga pocos threads a su disposición, su rendimiento se verá reducido dado que podrá procesar menos requests simultáneas.
___

# Trabajo Práctico N°4
## Servidor de análisis de logs con control de concurrencia
### Problema planteado:
- Permitir la subida de archivos de logs, calcular estadísticas de excepciones y limitar a 4 procesamientos simultáneos
---
# Solución
```rust
pub struct Server { pub file_stats: RwLock<HashMap<String, usize>>, pub file_semaphore: Semaphore }
impl Server {
    pub fn start(self: Arc<Self>, thread_amount: usize) {
        let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
        let pool = ThreadPool::new(thread_amount);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let server_ref = Arc::clone(&self);
            pool.execute(move || handle_connection(stream, server_ref));
        }
    }
    pub fn get_exceptions(self: Arc<Self>) -> i64 {
        self.file_stats.try_read().unwrap().values().map(|&v| v as i64).sum()
    }
}
```
___
# Get Stats
```rust
fn get_stats(server: Arc<Server>) -> Response {
    let exceptions: i64 = server.clone().get_exceptions();
    let stats = server.file_stats.try_read().unwrap();

    let body = format!(
        "Total exceptions: {exceptions}\nFiles processed: {}\nPer file:{}\n",
        stats.len(),
        format_map(&stats)
    );
    Response::new(200, body)
}
```
___
# Process Exceptions
```rust
fn process_file(request: Request, server: Arc<Server>) -> Response {
    if request.content_type != ContentType::MultipartFormData {
        return Response::from_status(400);
    }

    if request.body.is_empty() {
        return Response::new(400, "File not found or empty".to_string());
    }

    let acquire_result = server.file_semaphore.try_acquire();

    if acquire_result.is_err() {
        // Server is full
        return Response::from_status(429);
    }

    let mut files = server.file_stats.write().unwrap();

    files.insert(
        get_file_name(&request.body.join("")),
        analyze_logs("exception".to_string(), &request.body),
    );

    // Everything went right
    Response::from_status(200)
}
```
___
> Si bien no había una sección de preguntas en este TP, nos gustaría hacer un par de observaciones.
# Observaciones TP4
- El uso del `RwLock` sobre el `HashMap` de estadísticas es **imperativo**; debe haber obligatoriamente una forma de controlar las escrituras y lecturas sobre dichos datos para evitar condiciones de carrera.
- Consideramos que el semáforo (de `tokio`) es una manera simple de controlar la cantidad de usuarios simultáneos, dado que sólo necesitamos una primitivas:
  - `Semaphore.try_acquire() -> Result<SemaphorePermit<'_>, TryAcquireError>`:
    - Esta primitiva devuelve un `Result` que determina si se pudo adquirir el semáforo (si se pudo adquirir uno de los recursos limitados que "protege")
      - En caso positivo, se obtiene el "permiso" del semáforo
      - En caso negativo, se obtiene un **error** sin más
___

# Trabajo Práctico N°5
## Cola No Bloqueante
### Problema planteado:
- Implementar una cola no bloqueante para múltiples productores y consumidores
### Solución:
```rust
impl<T> Queue<T> for NonBlockingQueue<T> {
    fn enqueue(&self, item: T) {...}

    fn dequeue(&self) -> Option<T> {...}
}
```
___

# Enqueue
```rust
fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node::new(item)));
        let acquire = Ordering::Acquire;
        let release = Ordering::Release;
        loop {
            let cur_tail = self.tail.load(acquire);
            let tail_next = unsafe { (*cur_tail).next.load(acquire) };
            if cur_tail == self.tail.load(acquire) {
                unsafe {
                    if !tail_next.is_null() {
                        self.tail.compare_exchange(cur_tail, tail_next, acquire, release);
                    } else if (*cur_tail)
                        .next
                        .compare_exchange(null_mut(), new_node, acquire, release)
                        .is_ok()
                    {
                        self.size.fetch_add(1, Ordering::Release);
                        let _ = self
                            .tail
                            .compare_exchange(cur_tail, new_node, acquire, release);
                        return;
                    }
                }
            }
        }
    }
```
---
# Dequeue
```rust
fn dequeue(&self) -> Option<T> {
        let acquire = Ordering::Acquire;
        let release = Ordering::Release;

        loop {
            let current_head_ptr = self.head.load(acquire);

            if current_head_ptr.is_null() {
                return None;
            }
            let next_node_ptr = unsafe { (*current_head_ptr).next.load(acquire) };

            if self
                .head
                .compare_exchange(current_head_ptr, next_node_ptr, release, acquire)
                .is_ok()
            {
                self.size.fetch_sub(1, Ordering::Release);
                let old_head_node = unsafe { Box::from_raw(current_head_ptr) };
                return old_head_node.item;
            }
        }
    }
```

---

# Preguntas TP5
1. ¿Qué diferencias observás en el rendimiento entre la versión bloqueante y la no bloqueante?
2. ¿Qué dificultades técnicas encontraste al implementar la versión no bloqueante?
3. ¿Bajo qué escenarios conviene usar cada una?
4. ¿Qué pasaría si se mezclan productores bloqueantes con consumidores no bloqueantes (o viceversa)?
---
# Conclusiones TP5
1. Bajo concurrencia baja, la versión bloqueante es más performante, mientras que bajo alta concurrencia gana la versión no bloqueante.
2. Principalmente:
   - Lidiar con el `ownership` de Rust
   - Saber cómo y dónde aplicar las operaciones `unsafe`
   - Entender el funcionamiento del `Ordering`
3. Precisamente, conviene usar cada una en los escenarios donde gana cada una (los previamente mencionados):
   - Si se va a tener una concurrencia relativamente baja, o pocas operaciones de acceso a memoria compartida, se usa la versión bloqueante
   - En caso contrario, se usa la versión bloqueante
4. Puede incurrir en un deadlock, livelock o memoria no liberada (por nodos sin referenciar).
---

# TP Grep
## Herramienta tipo grep
### Problema planteado:
- Desarrollar un clon de 'grep' con modos secuencial, concurrente y por chunks
___
# Solución
```rust
const CHUNK_SIZE: i32 = 20000; // Line amount

pub trait Grep {
    fn find_sequence_in_file(&self, pattern: &String, file_paths: Vec<String>) -> Vec<String>;
}

impl Grep for SearchType {
    fn find_sequence_in_file(&self, pattern: &String, file_paths: Vec<String>) -> Vec<String> {
        match self {
            SearchType::Sequential => sequential_grep(&pattern, file_paths),
            SearchType::Concurrent => concurrent_grep(&pattern, file_paths),
            SearchType::ChunkConcurrent => chunk_concurrent_grep(&pattern, file_paths),
        }
    }
}
```
___
# Sequential Grep
```rust
fn sequential_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    file_paths
        .into_iter()
        .map(|path| get_file_reader(path.to_string()).unwrap())
        .map(|reader| find_sequence_in_file(&get_vector(reader), &search_term))
        .flatten()
        .collect()
}

fn get_vector(reader: BufReader<File>) -> Vec<String> {
    reader.lines().filter_map(|line| line.ok()).collect()
}
```
---
# Concurrent Grep
```rust
fn concurrent_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    let mut handles: Vec<JoinHandle<Vec<String>>> = Vec::new();
    file_paths
        .into_iter()
        .map(|path| {
            let value = search_term.clone();
            thread::spawn(move || {
                find_sequence_in_file(
                    &get_vector(get_file_reader(path.to_string()).unwrap()),
                    &value,
                )
            })
        })
        .for_each(|handle| handles.push(handle));
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect()
}
```
---
# Find sequence in file - Seq & Conc
```rust
pub fn find_sequence_in_file(buf_reader: &Vec<String>, pattern: &String) -> Vec<String> {
    buf_reader
        .clone()
        .into_iter()
        .map(|line: String| {
            line.to_lowercase()
        })
        .filter(|cured_line: &String| cured_line.contains(pattern))
        .collect::<Vec<String>>()
}
```
___
# C-Chunk
```rust
fn chunk_concurrent_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    let mut handles: Vec<JoinHandle<Vec<String>>> = Vec::new();
    file_paths
        .into_iter()
        .map(|path| {
            let value = search_term.clone();
            thread::spawn(move || {
                find_sequence_in_file_per_chunk(
                    &get_vector(get_file_reader(path.to_string()).unwrap()),
                    &value,
                    CHUNK_SIZE,
                )
            })
        })
        .for_each(|handle| handles.push(handle));
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect()
}
```
___
# Find sequence in file - C-Chunk
```rust
pub fn find_sequence_in_file_per_chunk(
    lines: &Vec<String>,
    pattern: &String,
    chunk_size: i32,
) -> Vec<String> {
    let chunks: i32 = (lines.len() as i32).div(chunk_size);

    let mut threads = vec![];

    for chunk in 0..chunks {
        let pattern_copy = pattern.clone();
        let lines_copy = lines.clone();
        let handle = thread::spawn(move || {
            lines_copy[(chunk_size * chunk) as usize..(chunk_size * (chunk + 1)) as usize]
                .iter()
                .filter(|line| line.contains(&pattern_copy))
                .cloned()
                .collect::<Vec<String>>()
        });
        threads.push(handle);
    }

    let mut result = vec![];
    for thread in threads {
        result.extend(thread.join().unwrap());
    }

    result
}
```
___
# Preguntas TP Grep
1. ¿Cómo se comparan los tiempos de ejecución entre la implementación secuencial y la concurrente?
2. Al reducir el tamaño de los segmentos (chunks), ¿qué patrón se observa en los tiempos de ejecución? ¿A qué se debe esto?
# Conclusiones TP Grep
1. En principio, la ejecución concurrente es más rápida, dado que se paraleliza el procesamiento de archivos.
2. Si el tamaño del chunk se vuelve muy pequeño (respecto al tamaño del archivo), la implementación `C-Chunk` se vuelve casi igual o menos performante que la concurrente. Esto se debe al problema de los cambios de contexto. El overhead que supone crear un thread para procesar una parte muy pequeña de un archivo y luego unirlo a la ejecución principal es lo suficientemente alto como para empeorar el rendimiento al punto de que no valga la pena operar por **chunks**.
___
# Trabajo Práctico N°7
## Simulación I/O vs Async y Threads
### Problema planteado:
- Comparar los tiempos de ejecución para I/O simulado entre threads y async con diferentes valores de tareas.
- Comparar los tiempos de ejecución para cálculo de Pi intensivo en CPU entre threads y async.
- Experimentar con diferentes cantidades de tareas y divisiones para evaluar la escalabilidad de cada enfoque.
___
### Solución

#### I/O simulado

```rust
// Async version
impl Simulation for AsyncSimulation  {
    async fn simulate_io_tasks(&self, tasks: usize) {
        let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_| {
            let handle = tokio::spawn(async {
                tokio::time::sleep(Duration::from_millis(500)).await;
                println!("sleeping 500 milliseconds");
            });
            handles.push(handle);
        });

        for handle in handles {
            handle.await.unwrap_or_else(|_| { println!("Errorubi"); });
        }
    }
}

// Thread version
impl Simulation for ThreadSimulation {
    async fn simulate_io_tasks(&self, tasks: usize) {
        let mut handles: Vec<JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_| {
            let handle = thread::spawn(|| {
                thread::sleep(Duration::from_millis(500));
                println!("sleeping 500 milliseconds");
            });
            handles.push(handle);
        });
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```
___
#### Cálculo de Pi

```rust
pub fn calculate_leibniz_thread(term: usize, parts: usize) -> f64 {
    let parts_ranges = split_range(term, parts);
    let handles: Vec<JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_concurrent(start, end)
    ).collect();

    handles.into_iter().map(|handle| handle.join().unwrap()).sum::<f64>()
}

pub async fn calculate_leibniz_async(term: usize, parts: usize) -> f64 {
    let parts_ranges = split_range(term, parts);
    let handles: Vec<tokio::task::JoinHandle<f64>> = parts_ranges.into_iter().map(
        |(start, end)| split_async(start, end)
    ).collect();
    let mut result: f64 = 0.0;
    for handle in handles {
        result += handle.await.unwrap();
    }
    result
}
```
___
### Splitting methods
#### Async
```rust
fn split_async(start: usize, end: usize) -> tokio::task::JoinHandle<f64> {
    tokio::spawn(async move{leibniz_pi_partial(start, end)})
}
```
- Este método crea una task de tokio
#### Thread
```rust
fn split_concurrent(start: usize, end: usize)->JoinHandle<f64>{
    thread::spawn(move || leibniz_pi_partial(start, end))
}
```
- Mientras que este crea un thread a nivel S.O.
___
# Preguntas TP7
1. Compare los tiempos de ejecución  para I/O simulado entre `threads` y  `async` con diferentes valores de  tasks. ¿Cuál es más eficiente en  manejar muchas tareas con esperas?
2. Compare los tiempos de ejecución  para cálculo de Pi intensivo en CPU  entre `threads` y `async`. ¿Qué modelo se desempeña mejor?
3. Experimente con diferentes cantidades de tareas y términos para Pi para evaluar cómo escala cada  enfoque.
___
# Conclusiones TP7
1. En estos contextos de I/O simulado con varias esperas, para cantidades de tareas relativamente pequeñas tienen desempeños similares, pero a medida que incrementa, el modelo `async` se desenvuelve mejor, ya que las `tasks` de `tokio` no tienen cambio de contexto.
2. Para cálculos que requieren CPU intensivo se desenvuelve mejor el modelo de `threads`, ya que se paralelizan verdaderamente los cálculos, mientras que el modelo `async` es **single-threaded**.
3. Al aumentar tareas y términos  para Pi, el enfoque de `threads`  escala hasta saturar los cores  disponibles (con menor ganancia tras ese punto por overhead), mientras que `async` no mejora en uso de CPU y su rendimiento se mantiene estable pero inferior en carga de cálculo.