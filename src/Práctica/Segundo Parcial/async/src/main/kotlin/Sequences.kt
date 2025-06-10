package org.austral

// In neither of these sequences you can call the 0 case, because a sequence
// can't return a value when taking 0 elements (it doesn't make sense)
val fibonacciSequence: Sequence<Int> = sequence {
    yield(1) // Fib(1)
    yield(1) // Fib(2)
    var a = 1;
    var b = 1;
    while (true) {
        val result = a + b
        yield(result)
        a = b; b = result
    }
}

val factorialSequence: Sequence<Int> = sequence {
    var result = 1 // Fact(1)
    var n = 1
    while (true) {
        yield(result)
        result *= ++n
    }
}

// seq from i to N
val mostCommonSequence: Sequence<Int> = sequence {
    var i = 1
    while (true)
        yield(i++)
}