package org.edu.austral
package counter

import akka.actor.{Actor, ActorRef}

class Counter extends Actor {

  override def receive: Receive = counter(BigInt(0))

  private def counter(n: BigInt): Receive = {
    case "incr" => context.become(counter(n + 1))
    case ("get", sender: ActorRef) => sender ! n
  }
}

class Printer extends Actor {

  override def receive: Receive = {
    case n: BigInt => println(s"Received number: $n")
  }
}
