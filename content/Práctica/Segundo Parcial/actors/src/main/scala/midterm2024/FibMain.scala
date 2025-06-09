package org.edu.austral
package midterm2024

import akka.actor.{ActorSystem, Props}

object FibMain extends App {
  implicit val system: ActorSystem = ActorSystem("ChatRoomSystem")
  private val fib = system.actorOf(Props[Fib](), "Fib") // Estado inicial = Fib(0,1)
  fib ! "incr" // Fib(1,2)
  fib ! "get" // 3
  fib ! "incr" // Fib(2, 3)
  fib ! "get" // 5
  fib ! "incr" // Fib(3, 5)
  fib ! "get" // 8
  fib ! "boom" // Kabum, se resetea el actor => Fib(0,1)
  fib ! "get" // 1
}
