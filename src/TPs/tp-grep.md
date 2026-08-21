# TP Grep — Búsqueda concurrente en archivos

# TP Grep
## Herramienta tipo grep
### Problema planteado:
- Desarrollar un clon de 'grep' con modos secuencial, concurrente y por chunks

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

# Preguntas TP Grep
1. ¿Cómo se comparan los tiempos de ejecución entre la implementación secuencial y la concurrente?
2. Al reducir el tamaño de los segmentos (chunks), ¿qué patrón se observa en los tiempos de ejecución? ¿A qué se debe esto?
# Conclusiones TP Grep
1. En principio, la ejecución concurrente es más rápida, dado que se paraleliza el procesamiento de archivos.
2. Si el tamaño del chunk se vuelve muy pequeño (respecto al tamaño del archivo), la implementación `C-Chunk` se vuelve casi igual o menos performante que la concurrente. Esto se debe al problema de los cambios de contexto. El overhead que supone crear un thread para procesar una parte muy pequeña de un archivo y luego unirlo a la ejecución principal es lo suficientemente alto como para empeorar el rendimiento al punto de que no valga la pena operar por **chunks**.

---

## Código fuente

- [`tp_grep/grep-app/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp_grep/grep-app)
- [`tp_grep/grep-lib/`](https://github.com/FranCalveyra/concurrent-programming/tree/main/tp_grep/grep-lib)

## Enunciado

[TP_Grep_1.pdf](https://github.com/FranCalveyra/concurrent-programming/tree/main/pdfs/TP_Grep_1.pdf)
