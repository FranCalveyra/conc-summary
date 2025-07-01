package org.austral

import kotlinx.coroutines.async
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.util.concurrent.atomic.AtomicReference

// Ejercicio 1
class Stack<E> {
    class Node<E>(val item: E, var next: Node<E>? = null)

    private var top: Node<E>? = null

    fun push(item: E) {
        val newHead = Node(item)
        newHead.next = top
        top = newHead
    }

    fun pop(): E? {
        val oldHead = top
        if (oldHead == null) return null
        top = oldHead.next
        return oldHead.item
    }
}

class NonBlockingStack<E> {
    class Node<E>(val item: E, var next: Node<E>? = null)

    private var top = AtomicReference<Node<E>?>()

    fun push(item: E) {
        val newHead = Node(item)
        var oldHead: Node<E>?
        do {
            oldHead = top.get()
            newHead.next = oldHead
        } while (!top.compareAndSet(oldHead, newHead))
    }

    fun pop(): E? {
        var oldHead: Node<E>? = top.get()
        while (oldHead != null && !top.compareAndSet(oldHead, oldHead.next))
            oldHead = top.get()
        return oldHead?.item
    }
}


// Ejercicio 2
suspend fun compute(name: String, delayTime: Long): Int {
    delay(delayTime)
    println("Done with $name")
    return name.length
}

fun main(): Unit = runBlocking {
    println("Start")

    launch {
        val launchValue = compute("LaunchTask", 300L)
        println("Launch result: $launchValue")
    }

    val deferred = async {
        val result = compute("AsyncTask", 200L)
        println("Async result: $result")
        result
    }

    println("Middle")

    val final = deferred.await()
    println("Final result: $final")
}