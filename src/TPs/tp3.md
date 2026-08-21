# TP 3 — Thread Pool

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

# Preguntas TP3
1. Bajo carga concurrente intensa (aumentando -n y -c en ab), ¿qué efectos se observan en el comportamiento del servidor? ¿Cómo se comparan con los resultados obtenidos en el TP2?
2. ¿Cómo se ve afectado el comportamiento ante carga concurrente intensa para diferentes tamaños de thread pool ?
# Conclusiones TP3
1. Es significativamente más performante que el servidor desarrollado en el TP2, dado que no existe el problema de los cambios de contexto por tener `Workers`, o hilos que se mantienen vivos.
2. En caso de que el Thread Pool tenga pocos threads a su disposición, su rendimiento se verá reducido dado que podrá procesar menos requests simultáneas.

---

## Código fuente

- [`tp3/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp3)
- [`thread_pool/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/thread_pool)

## Enunciado

[TP3_Programacion_Concurrente.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP3_Programacion_Concurrente.pdf)
