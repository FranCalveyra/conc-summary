package org.edu.austral
package chatRoom

import chatRoom.RoomOperation.Broadcast
import chatRoom.UserMessage.{JoinRoom, LeaveRoom}

import akka.actor.{ActorSystem, Props}

import scala.concurrent.duration.*

/// Lógicamente, este programa está bien, pero se hace un broadcast que Bob también recibe
/// porque no se espera a que Bob se vaya de la sala, no es una operación que se termine de ejecutar
object ChatRoomMain extends App {
  implicit val system: ActorSystem = ActorSystem("ChatRoomSystem")

  private val alice = system.actorOf(Props(classOf[User], "Alice"), "Alice")
  private val bob = system.actorOf(Props(classOf[User], "Bob"), "Bob")
  private val carol = system.actorOf(Props(classOf[User], "Carol"), "Carol")

  val chatRoom = system.actorOf(Props[ChatRoom](), "ChatRoom")

  alice ! JoinRoom(chatRoom)
  bob ! JoinRoom(chatRoom)
  carol ! JoinRoom(chatRoom)

  chatRoom ! Broadcast("Alice", "Hola!")

  bob ! LeaveRoom(chatRoom)

  chatRoom ! Broadcast("Alice", "¿Dónde está Bob?")

  import system.dispatcher

  system.scheduler.scheduleOnce(5.seconds)(system.terminate())
}
