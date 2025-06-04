package org.edu.austral
package bank

import bank.BankAccount.{Deposit, Done, Failed, Withdraw}

import akka.actor.Actor

// Ya el compilador de Scala te dice que hagas el approach funcional que nos mostraron en clase
// Goated
class BankAccount extends Actor {
  private var balance: BigInt = 0

  override def receive: Receive = {
    case Deposit(amount: BigInt) =>
      balance += amount
      sender() ! Done
    case Withdraw(amount: BigInt) if amount <= balance =>
      balance -= amount
      sender() ! Done
    case _ => sender() ! Failed
  }
}

/*
Recomendación del compilador:
class BankAccount extends Actor {
  private val balance: BigInt = 0

  override def receive: Receive = onMessage(balance)

  private def onMessage(balance: BigInt): Receive = {
    case Deposit(amount: BigInt) =>
      context.become(onMessage(balance + amount))
      sender() ! Done
    case Withdraw(amount: BigInt) if amount <= balance =>
      context.become(onMessage(balance - amount))
      sender() ! Done
    case _ => sender() ! Failed
  }
}
*/

object BankAccount {
  case class Deposit(amount: BigInt)

  case class Withdraw(amount: BigInt)

  case object Done

  case object Failed

}
