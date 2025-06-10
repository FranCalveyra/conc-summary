package org.edu.austral
package bank

import bank.BankAccount
import bank.BankAccount.Deposit
import bank.WireTransfer.{Done, Failed, Transfer}

import akka.actor.{Actor, ActorRef}

object WireTransfer {
  case class Transfer(from: BankAccount, to: BankAccount, amount: BigInt)

  case object Done

  case object Failed
}

/**
 * Objeto intermedio para realizar transferencias entre cuentas bancarias
 */
class WireTransfer extends Actor {
  def receive: Receive = {
    case Transfer(from: BankAccount, to: BankAccount, amount: BigInt) =>
      // Manera falopa de resolverlo, no sé por qué no me compilaba sino.
      // Intuyo que como BankAccount es un actor y necesito el actorRef,
      // me obliga a agarrar su self
      from.self ! BankAccount.Withdraw(amount)
      context.become(awaitWithdraw(to, amount, sender()))
  }

  private def awaitWithdraw(to: Actor, amount: BigInt, client: ActorRef): Receive = {
    case BankAccount.Done =>
      // Mismo caso que en el receive default
      to.self ! Deposit(amount: BigInt)
      context.become(awaitDeposit(client))
    case BankAccount.Failed =>
      client ! Failed
      context.stop(self)
  }

  private def awaitDeposit(client: ActorRef): Receive = {
    case BankAccount.Done =>
      client ! Done
      context.stop(self)
    case BankAccount.Failed =>
      client ! Failed
      context.stop(self)
  }
}
