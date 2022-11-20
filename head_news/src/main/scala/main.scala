import scala.scalajs.js

import domain.*

@main def main(): Unit =
  val providers = List(SinaNews)

  import scala.concurrent.ExecutionContext.Implicits.global
  import js.Thenable.Implicits.*
  for
    provider <- providers
    headNewsOpt <- provider.fetchHeadNews
    HeadNews(title, href) <- headNewsOpt
  do
    println(s"${provider.name}头条(${js.Date()}")
    println(s"=>《$title》 : $href")
    println("\n")
