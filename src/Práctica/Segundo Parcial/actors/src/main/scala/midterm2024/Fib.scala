package org.edu.austral
package midterm2024

import akka.actor.Actor

class Fib extends Actor {

  override def receive: Receive = fib(0, 1)

  private def fib(n1: Int, n2: Int): Receive = {
    case "incr" => context.become(fib(n2, n1 + n2))
    case "get" =>
      sender() ! (n1 + n2)
      println(n1 + n2)
    case "boom" => throw new UnsupportedOperationException()
  }
}
