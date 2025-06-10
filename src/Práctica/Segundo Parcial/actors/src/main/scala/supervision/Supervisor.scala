package org.edu.austral
package supervision

import supervision.WorkerMessage.StartWork

import akka.actor.{Actor, ActorRef, Props}

class Supervisor extends Actor {
  private val accumulatedResults: List[Int] = List.empty

  override def receive: Receive = onMessage(accumulatedResults)

  private def onMessage(accumulatedResults: List[Int]): Receive = {
    case StartWork(list: List[Int], testProbeRef: ActorRef) =>
      createWorkers(list)
      println(s"Children: ${context.children}")
      println(s"Children amount: ${context.children.size}")
      context.children.zip(list).foreach { case (worker, v) =>
        worker ! WorkerMessage.DoWork(v)
      }

    case WorkerMessage.Done(n: Int) =>
      println(s"Received value $n")
      context.become(onMessage(n :: accumulatedResults))
    case WorkerMessage.GetResults(testProbeRef: ActorRef) =>
      testProbeRef ! SupervisorProtocol.Results(accumulatedResults.reverse)
  }

  private def createWorkers(values: List[Int]): Unit = {
    values.map(v => context.actorOf(Props[Worker]()))
  }

}

object SupervisorProtocol {
  case class Results(list: List[Int])
}
