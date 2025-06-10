package org.edu.austral

import counter.{Counter, Printer}

import akka.actor.{ActorSystem, Props}
import akka.util.Timeout

import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.*
import scala.language.postfixOps

object Main extends App {
  implicit val system: ActorSystem = ActorSystem("CounterSystem")

  val counter = system.actorOf(Props[Counter](), "counter")
  private val printer = system.actorOf(Props[Printer](), "printer")


  implicit val timeout: Timeout = Timeout(5 seconds)

  counter ! "incr"
  counter ! "incr"
  counter ! "incr"

  //  private val future: Future[Any] = counter ? "get" // El operador `?` te devuelve un Future
  //  // Como es un Future[Any], se puede iterar para procesar los resultados de manera asíncrona
  //  future.foreach(result => printer ! result)

  counter.tell("get", printer)

  // Optionally, shut down the actor system after a delay
  system.scheduler.scheduleOnce(5 seconds) {
    system.terminate()
  }
}