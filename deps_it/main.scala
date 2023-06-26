//> using scala "3.3.0"
//> using option "-Wunused:all" "-Wvalue-discard" "-Yexplicit-nulls"

//> using nativeVersion 0.4.14
//> using nativeGc "none"
//> using nativeLto "thin"
//> using nativeMode "release"

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
        val s = s"""ivy:
                  |  ${dependency.toIvy}
                  |  import $$ivy.`${dependency.toIvy}`
                  |
                  |scala-cli repl:
                  |  scala-cli repl -S 3.3.0 --dep ${dependency.toIvy}
                  |
                  |mill:
                  |  override def ivyDeps = Agg(ivy"${dependency.toIvy}")
                  |
                  |sbt:
                  |  ${dependency.toSbt}
                  |
                  |maven:
                  |${dependency.toMaven.linesIterator.map(it => s"  ${it}").mkString("\n")}
                  """.stripMargin
        println(s)

      case None => println("unknown")

    println()
  }
}
