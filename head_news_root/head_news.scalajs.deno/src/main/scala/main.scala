import domain.*

import scala.scalajs.js

@main def main(): Unit =
    val providers = List(SinaNews)

    import js.Thenable.Implicits.*
    import scala.concurrent.ExecutionContext.Implicits.global
    for
        provider              <- providers
        headNewsOpt           <- provider.fetchHeadNews
        HeadNews(title, href) <- headNewsOpt
    do
        println(s"${provider.name}头条(${js.Date()}")
        println(s"=>《$title》 : $href")
        println("\n")
    end for
end main
