---
layout: page
title: Clase 3
nav_order: 3
---
# Paralelismo
Esta clase todo lo que hace es mostrar implementaciones concurrentes de resolver algoritmos iterativos, que terminan siendo dividir los cálculos en algún punto, spawnear un thread en esa división y seguir calculando.

Después de eso se espera que el thread termine y se junten los resultados.
## Ejemplo
Si queremos calcular $$2*k*a*t*e^{-a * t}$$, podemos:
- Primero calcular $$ 2*k*a*t $$
- Después calcular $ e^{-a * t} $
- Finalmente, multiplicar los resultados de los dos pasos anteriores.

### Implementación secuencial
```rust
fn evaluate_sequential(k: f64, a: f64, t: f64) -> f64 {

    let a = 2.0 * k * a * t;
    let b = f64::exp(-a * t * t);
    
    return a * b;
}
```

### Implementación concurrente
```rust
use std::thread;

fn evaluate_parallel(k: f64, a: f64, t: f64) -> f64 {

    let thread1 = thread::spawn(
        || 2.0 * k * a * t
    );
    let thread2 = thread::spawn(
        || f64::exp(-a * t * t)
    );

    let a = thread1.join().unwrap();
    let b = thread2.join().unwrap();

    return a * b;
}
```

Se puede hacer una pequena optimización, en vez de crear dos threads, podemos crear uno solo y hacer el cálculo de $e^{-a * t}$ dentro del thread. Esto es porque el thread no se va a quedar esperando a que termine el otro thread, sino que va a seguir ejecutando el código.

```rust
use std::thread;

fn evaluate_parallel_opt(k: f64, a: f64, t: f64) -> f64 {
    let first_half = 2.0 * k * a * t;

    let thread2 = thread::spawn(|| f64::exp(-a * t * t));

    let second_half = thread2.join().unwrap();

    return first_half * second_half;
}
```

## Observación importante
A veces, no siempre es mejor hacer implementaciones paralelas, dado que puede suponer \
un overhead a partir de los cambios de contexto.

Si el cálculo es muy rápido, el overhead de crear un thread puede ser mayor \
que el tiempo que se tarda en calcular el resultado de manera iterativa.

### Ejemplo - MergeSort
```rust
pub fn merge(first: &[i32], second: &[i32]) -> Vec<i32>{
    let mut result = Vec::new();
    let mut i = 0; let mut j = 0;
    
    // Merge until one of the inputs is exhausted
    while i < first.len() && j < second.len() {
        if first[i] <= second[j] {
            result.push(first[i]);
            i += 1
        } else {
            result.push(second[j]);
            j += 1
        }
    }
    // Copy the remaining items
    result.extend_from_slice(&first[i..]);
    result.extend_from_slice(&second[j..]);
    result
}

pub fn sort(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1 {
        array.to_vec()
    }
    else {
        let (first, second) = thread::scope(|s| {
            let x = sort(&array[..len / 2]); // First half
            let y = s.spawn(|| sort(&array[(len / 2) + 1..])); // Second half
            (x, y.join().unwrap()) // <-- Scope also returns a value
        });
        merge(&first, &second)
    }
}
```

Por lo general se optimiza de la siguiente manera (conceptualmente):
- Se implementan ambos métodos de cálculo (secuencial y concurrente)
- Si se supera un cierto threshold o límite, se llama al método concurrente
  - Este threshold va a variar dependiendo del caso y del algoritmo a implementar
  - Inclusive puede variar según la máquina donde se esté corriendo
- Si no, se opera con el método secuencial.

