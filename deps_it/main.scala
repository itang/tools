val VERSION = "0.1.0-20230122.2"

object Main {
  def main(args: Array[String]): Unit = {
    println(s"deps-v$VERSION")

    val input =
      if args.isEmpty then """ "org.scala-sbt" % "sbt" % "1.6.2" """
      else args.mkString(" ")
    println(s"input: $input\n")

    given listParser: List[Parser] = List(SbtParser, IvyParser)

    Dependency.from(input) match
      case Some(dependency) =>
        println(s"ivy:\n\t${dependency.toIvy}")
        println(s"\timport $$ivy.`${dependency.toIvy}`\n")
        println(
          s"scala-cli repl:\n\tscala-cli repl -S 3.2.2 --dep ${dependency.toIvy}\n"
        )
        println(
          s"""mill:\n\toverride def ivyDeps = Agg(ivy"${dependency.toIvy}")\n"""
        )

        println(s"sbt:\n\t${dependency.toSbt}\n")

        println(
          s"maven:\n${dependency.toMaven.linesIterator.map(it => s"\t${it}").mkString("\n")}\n"
        )
      case None => println("unknown")

    println()
  }
}
