# TP 2 — Servidor concurrente (un hilo por conexión)

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

# Preguntas abiertas
- Bajo carga concurrente intensa (aumentando -n y -c en ab), ¿qué
efectos se observan en el comportamiento del servidor? ¿Se nota alguna
diferencia en los tiempos de respuesta, la latencia o el comportamiento
general? ¿A qué se debe?
# Conclusiones
Siendo N el número de requests y C la cantidad de usuarios concurrentes, lo que notamos en cuanto a las diferencias de rendimiento es que, bajo una carga muy alta de concurrencia, el servidor se empieza a ralentizar, llegando a una performance similar a la versión `single-threaded` del TP N°1. Es decir, **empeora**.

Esto se debe al overhead que suponen los cambios de contexto entre threads, ya que se crea un thread por cada request que llega.

---

## Código fuente

- [`tp2/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp2)

## Enunciado

[TP2_Programacion_Concurrente.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP2_Programacion_Concurrente.pdf)
