package org.edu.austral
package supervision

import akka.actor.{Actor, ActorSystem, Props}

import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.*

object SupervisorMain extends App {
  implicit val system: ActorSystem = ActorSystem("SupervisorTestSystem")

  val client = system.actorOf(Props(new Actor {
    override def receive: Receive = {
      case SupervisorProtocol.Results(results) =>
        println(s"Received results: $results")
        context.system.terminate()
    }
  }), "client")

  private val supervisor = system.actorOf(Props[Supervisor](), "supervisor")

  supervisor ! WorkerMessage.StartWork(List(1, -1, 2), client)

  // Await a little bit in order to get the results correctly
  system.scheduler.scheduleOnce(500.millis) {
    supervisor ! WorkerMessage.GetResults(client)
  }
}
