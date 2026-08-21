# TP 4 — Análisis de logs con control de concurrencia

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

> Si bien no había una sección de preguntas en este TP, nos gustaría hacer un par de observaciones.
# Observaciones TP4
- El uso del `RwLock` sobre el `HashMap` de estadísticas es **imperativo**; debe haber obligatoriamente una forma de controlar las escrituras y lecturas sobre dichos datos para evitar condiciones de carrera.
- Consideramos que el semáforo (de `tokio`) es una manera simple de controlar la cantidad de usuarios simultáneos, dado que sólo necesitamos una primitivas:
  - `Semaphore.try_acquire() -> Result<SemaphorePermit<'_>, TryAcquireError>`:
    - Esta primitiva devuelve un `Result` que determina si se pudo adquirir el semáforo (si se pudo adquirir uno de los recursos limitados que "protege")
      - En caso positivo, se obtiene el "permiso" del semáforo
      - En caso negativo, se obtiene un **error** sin más

---

## Código fuente

- [`tp4/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp4)
- [`connection-handler/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/connection-handler)

## Enunciado

[TP4_Programacion_Concurrente.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP4_Programacion_Concurrente.pdf)
