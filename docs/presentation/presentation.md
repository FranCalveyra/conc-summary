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
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

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

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
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

# Trabajo Práctico N°4
## Servidor de análisis de logs con control de concurrencia
### Problema planteado:
- Permitir la subida de archivos de logs, calcular estadísticas de excepciones y limitar a 4 procesamientos simultáneos
---
# Solución
```rust
pub struct Server {
    pub file_stats: RwLock<HashMap<String, usize>>,
    pub file_semaphore: Semaphore,
}

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
        self.file_stats
            .try_read()
            .unwrap()
            .values()
            .map(|&v| v as i64)
            .sum()
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
        loop {
            let cur_tail = self.tail.load(acquire);
            let tail_next = unsafe { (*cur_tail).next.load(acquire) };
            if cur_tail == self.tail.load(acquire) {
                unsafe {
                    if !tail_next.is_null() {
                        let _ = self
                            .tail
                            .compare_exchange(cur_tail, tail_next, acquire, acquire);
                    } else if (*cur_tail)
                        .next
                        .compare_exchange(null_mut(), new_node, acquire, acquire)
                        .is_ok()
                    {
                        self.size.fetch_add(1, Ordering::Release);
                        let _ = self
                            .tail
                            .compare_exchange(cur_tail, new_node, acquire, acquire);
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