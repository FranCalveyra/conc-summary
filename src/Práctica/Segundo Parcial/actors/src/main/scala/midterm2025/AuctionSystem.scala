package org.edu.austral
package midterm2025

import midterm2025.AuctionProtocol.{StartAuction, Winner}

import akka.actor.{Actor, ActorRef, ActorSystem, Props}

import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.{Duration, DurationInt, FiniteDuration}
import scala.util.Random

object AuctionSystem extends App {
  val system = ActorSystem("AuctionSystem")

  private val organizer = system.actorOf(Props[Organizer](), "organizer")
  private val auctionDuration = 15.seconds

  private val auction = system.actorOf(Props(new Auction(auctionDuration, organizer)), "auction")

  auction ! StartAuction(organizer)

  // Bidder A places bids up to 50 for 8 seconds
  system.actorOf(Props(new Bidder(auction, "A", 50, 8.seconds)), "bidderA")

  // Bidder B places bids up to 100 for 10 seconds
  system.actorOf(Props(new Bidder(auction, "B", 100, 10.seconds)), "bidderB")

  // Bidder C places bids up to 75 for 6 seconds
  system.actorOf(Props(new Bidder(auction, "C", 75, 6.seconds)), "bidderC")
}

object AuctionProtocol {
  case class Winner(id: String, amount: Int) // Se lo manda al Organizer

  case object AnnounceWinner

  case class StartAuction(organizer: ActorRef)
}


class Organizer extends Actor {
  override def receive: Receive = {
    case Winner(id: String, amount: Int) =>
      println(s"The winner is $id with the incredible amount of $amount !")
  }
}

class Auction(duration: FiniteDuration, organizer: ActorRef) extends Actor {
  // Cuando se me termina el tiempo, anuncio el ganador
  context.system.scheduler.scheduleOnce(duration, self, AuctionProtocol.AnnounceWinner)
  private var currentMaxBid: (String, Int) = ("", 0)

  override def receive: Receive = {

    case AuctionProtocol.StartAuction(organizer: ActorRef) =>
      println(s"The auction has started! It's being organized by ${organizer.toString}")

    case BidderProtocol.Bid(id: String, amount: Int) =>
      if (currentMaxBid._2 < amount) { // Si es una puja mayor, la actualizo
        currentMaxBid = (id, amount)
      }

    case AuctionProtocol.AnnounceWinner =>
      organizer ! Winner(currentMaxBid._1, currentMaxBid._2) // Le mando el ganador al organizador
      context.stop(self) // Freno este actor
  }
}

class Bidder(auction: ActorRef, id: String, maxBid: Int, duration: FiniteDuration) extends Actor {
  // Programo para pujar cada cierto tiempo
  context.system.scheduler.scheduleWithFixedDelay(Duration.Zero, 50.millis, self, BidderProtocol.SendBid)
  private val random = Random()
  private var currentBid: Int = 0

  // Tengo que dejar de pujar después de un cierto tiempo
  context.system.scheduler.scheduleOnce(duration, self, BidderProtocol.Stop)

  override def receive: Receive = {

    case BidderProtocol.Stop =>
      context.stop(self) // Freno este actor cuando se me termina el tiempo (es decir, cuando dejo de pujar)

    case BidderProtocol.SendBid =>
      auction ! BidderProtocol.Bid(id, getCurrentMaxBid)
  }

  private def getCurrentMaxBid: Int = {
    val newBid = random.nextInt(maxBid)
    if (newBid > currentBid) {
      currentBid = newBid
    }
    currentBid
  }
}

object BidderProtocol {
  case object SendBid

  case class Bid(id: String, amount: Int)

  case object Stop
}