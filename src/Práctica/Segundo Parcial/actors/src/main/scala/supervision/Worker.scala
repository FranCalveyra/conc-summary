package org.edu.austral
package supervision

import akka.actor.{Actor, ActorRef}

class Worker extends Actor {

  override def receive: Receive = {
    case WorkerMessage.DoWork(n: Int) =>
      if (n < 0) throw new RuntimeException()
      else sender() ! WorkerMessage.Done(n * 2)
  }
}

object WorkerMessage {
  case class DoWork(n: Int)

  case class Done(n: Int)

  case class GetResults(replyTo: ActorRef)

  case class StartWork(list: List[Int], testProbeRef: ActorRef)
}