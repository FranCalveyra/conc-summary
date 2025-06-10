# Thread Programming

Hay que saber cómo levantar un thread en los lenguajes que vayamos a usar, \
para entender cómo los tratan por abajo.

## Java

```java
private static void hello() {
    var t1 = new Thread(new Runnable() {
        @Override
        public void run() {
            System.out.println("Hello from thread 1");
        }
    });
    var t2 = new Thread(new Runnable() {
        @Override
        public void run() {
            System.out.println("Hello from thread 2");
        }
    });
    t1.start();
    t2.start();
}

// Con Lambdas
private static void helloLambda() {
    var t1 = new Thread(() -> System.out.println("Hello from thread 1"));
    var t2 = new Thread(() -> System.out.println("Hello from thread 1"));
    t1.start();
    t2.start();
}
```

> Cabe destacar que lo anterior simplemente levanta los threads, que se ejecutarán como el procesador los quiera
> ejecutar.

### Administrar manualmente la ejecución

```java
private static void hello() throws InterruptedException {
    var t1 = new Thread(() -> System.out.println("Hello from thread 1"));
    var t2 = new Thread(() -> System.out.println("Hello from thread 2"));
    t1.start();
    t2.start();
    t1.join();
    t2.join();
}
```

El método `join` espera a que el thread termine su ejecución.\
Si no se llama a `join`, el thread principal puede terminar antes que los threads secundarios.

> Denle importancia al `throws InterruptedException`. La firma de esta función es así porque, al querer unir los threads
> al principal (o esperar a que terminen), puede haber una interrupción.
>
> Al llamar a este método, se habilita a que cualquier thread interrumpa el actual (donde se está ejecutando esta
> función) por la razón que sea.

## Rust

```rust
fn hello() {
    thread::spawn(|| println!("Hello from thread 1"));
    thread::spawn(|| println!("Hello from thread 2"));
}
```

### Esperar a que termine la ejecución

```rust
fn hello() {
    let t1 = thread::spawn(|| println!("Hello from thread 1"));
    let t2 = thread::spawn(|| println!("Hello from thread 2"));

    // Esperar a que los threads se completen
    t1.join().expect("t1 failed");
    t2.join().expect("t2 failed");
}
```

## Thread Scope

El Scope es una forma de agrupar threads, de forma tal que se ejecuten en un mismo contexto. \
Además, el Scope permite que los threads se compartan variables entre ellos, sin necesidad de usar `Arc` o `Mutex` (para
casos de solo lectura).

Una vez termina el Scope, los threads se unen automáticamente, por lo que no es necesario llamar a `join` manualmente.

```rust
fn hello() {
    thread::scope(|s| {
        s.spawn(|| println!("Hello from thread 1"));
        s.spawn(|| println!("Hello from thread 2"));
    });
}
```

### Lifetime

Sabiendo que Rust sabe que los threads no escapan del scope: \

- Las reglas de Lifetime son más simples
- Podemos usar variables del scope externo

```rust
fn hello() {
    let n = 10;
    let mut m = 10;
    thread::scope(|s| {
        s.spawn(|| {
            m += 1;
            println!("Hello from thread 1, n = {n}")
        });
        s.spawn(|| println!("Hello from thread 2, n = {n}"));
    });
    println!("{m}");
}
// Output: 11
```

