import sbt.Keys.resolvers

ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.7.1"

lazy val root = (project in file("."))
  .settings(
    name := "bank",
    idePackagePrefix := Some("org.edu.austral")
  )
resolvers += "Akka library repository".at("https://repo.akka.io/maven")
libraryDependencies += "com.lightbend.akka" %% "akka-projection-core" % "1.6.13"
