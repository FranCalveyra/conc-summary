# TP 1 — Servidor de Leibniz (single-threaded)

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

# Cálculo:
```rust
fn sequential_leibniz(begin: i32, end: i32) -> f64 {
    (begin..=end).map(|i| term(i)).sum::<f64>() * 4f64
}

fn term(i: i32) -> f64 {
    (-1f64).powi(i) / (2 * i + 1) as f64
}
```

# Preguntas abiertas
1. ¿Qué sucede con dos requests simultáneas que tardan en procesarse?
2. ¿Por qué se observa este comportamiento?
3. ¿Cómo solucionar usando solo librerías estándar de Rust?
# Conclusiones
1. Una se encola después de la otra.
2. Es un servidor single-threaded.
3. Como veremos en el TP N°2, levantando un `thread` por cada request, usando `thread::spawn`

---

## Código fuente

- [`tp1/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp1)
- [`leibniz-calculator/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/leibniz-calculator)

## Enunciado

[TP1__Programacion_Concurrente-1.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP1__Programacion_Concurrente-1.pdf)
