package org.edu.austral
package chatRoom

import akka.actor.{Actor, ActorRef}


type UserTuple = (String, ActorRef)

// Chad Scala implementation:
class ChatRoom extends Actor {
  private val onlineUsers: Set[UserTuple] = Set()

  override def receive: Receive = onMessage(onlineUsers)

  private def onMessage(onlineUsers: Set[(String, ActorRef)]): Receive = {
    case RoomOperation.Join(userName: String, actorRef: ActorRef) =>
      context.become(onMessage(onlineUsers.incl(userName, actorRef))) // Esto es ilegible igual
    case RoomOperation.Leave(userName: String) =>
      val currentUser: Option[UserTuple] = onlineUsers.find((name, _) => name == userName)
      currentUser match
        case Some(userTuple: UserTuple) => context.become(onMessage(onlineUsers.excl(userTuple)))
        case None => // Do sth?
    
    case RoomOperation.Broadcast(sender: String, text: String) =>
      onlineUsers.filter((username, _) => username != sender).foreach((_, ref) => ref ! UserMessage.Message(sender, text))
  }
}

object RoomOperation {
  case class Join(userName: String, actorRef: ActorRef)

  case class Leave(userName: String)

  case class Broadcast(sender: String, text: String)
}

/*
Mi humilde implementación:
class ChatRoom extends Actor {
  private var onlineUsers: Set[UserTuple] = Set()

  override def receive: Receive = {
    case RoomOperation.Join(userName: String, actorRef: ActorRef) =>
      onlineUsers = onlineUsers.incl(userName, actorRef)
    case RoomOperation.Leave(userName: String) =>
      val currentUser: Option[UserTuple] = onlineUsers.find((name, _) => name == userName)
      if (currentUser.isEmpty) {
        // Do sth?
      }
      onlineUsers = onlineUsers.excl(currentUser.get)

    case RoomOperation.Broadcast(sender: String, text: String) =>
      onlineUsers.filter((username, _) => username != sender).foreach((_, ref) => ref ! text)
  }
}
*/
