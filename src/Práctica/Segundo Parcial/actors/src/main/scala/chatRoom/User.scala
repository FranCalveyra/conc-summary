package org.edu.austral
package chatRoom

import akka.actor.{Actor, ActorRef}


class User(name: String) extends Actor {
  private val messages: List[String] = List.empty

  override def receive: Receive = onMessage(messages)

  private def onMessage(messages: List[String]): Receive = {
    case UserMessage.Message(from: String, text: String) =>
      println(s"Current user ($name) received a message!")
      val message = s"User $from says: $text"
      context.become(onMessage(message :: messages))
      println(message)
    case UserMessage.JoinRoom(roomRef: ActorRef) =>
      roomRef ! RoomOperation.Join(name, self)
    case UserMessage.LeaveRoom(roomRef: ActorRef) =>
      roomRef ! RoomOperation.Leave(name)
  }
}

object UserMessage {
  case class Message(from: String, text: String)

  case class JoinRoom(roomRef: ActorRef)

  case class LeaveRoom(roomRef: ActorRef)
}
