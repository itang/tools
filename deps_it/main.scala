val VERSION = "0.1.0-20230122.1"
object Main {
  def main(args: Array[String]): Unit = {
    println(s"deps-v$VERSION")

    val input =
      if args.isEmpty then """ "org.scala-sbt" % "sbt" % "1.6.2" """
      else args.mkString(" ")
    println(s"input: $input\n")

    given listParser: List[Parser] = List(SbtParser, IvyParser)

    Deps.from(input) match
      case Some(deps) =>
        println(s"ivy:\n\t${deps.toIvy}")
        println(s"\timport $$ivy.`${deps.toIvy}`\n")
        println(
          s"scala-cli repl:\n\tscala-cli repl -S 3.2.2 --dep ${deps.toIvy}\n"
        )
        println(
          s"""mill:\n\toverride def ivyDeps = Agg(ivy"${deps.toIvy}")\n"""
        )

        println(s"sbt:\n\t${deps.toSbt}\n")

        println(
          s"maven:\n${deps.toMaven.linesIterator.map(it => s"\t${it}").mkString("\n")}\n"
        )
      case None => println("unknown")

    println()
  }
}
