//> using scala "3.1.3"
////> using lib "org.scalameta:svm-subs_2.13:20.2.0"

import scala.util.Try

case class Deps(
    groupId: String,
    artifactId: String,
    version: String
)

object Deps:
  def from(s: String)(using parsers: List[Parser]): Option[Deps] =
    def _from(parsers: List[Parser]): Option[Deps] =
      parsers match
        case Nil => None
        case parser :: tail =>
          parser.unapply(s) match
            case Some(d) => Some(d)
            case None    => _from(tail)

    Try(_from(parsers)).toOption.flatten
trait Parser:
  def unapply(s: String): Option[Deps]

object SbtParser extends Parser:
  def unapply(s: String): Option[Deps] =
    if s.contains("%") then
      val Array(a, b, c) = s.trim
        .replaceAll("%", "")
        .split("\\s+")
        .map(it =>
          val itt = it.trim
          if itt.startsWith("\"") || itt.startsWith("'") then
            itt.substring(1, it.length - 1)
          else itt
        )
      Some(Deps(a, b, c))
    else None

object IvyParser extends Parser:
  def unapply(s: String): Option[Deps] =
    if s.contains(":") then
      val Array(a, b, c) = s.trim.replaceAll(":", " ").split("\\s+").map(_.trim)
      Some(Deps(a, b, c))
    else None

extension (t: Deps)
  def toIvy: String =
    s"${t.groupId}::${t.artifactId}:${t.version}"

  def toSbt: String =
    s""""${t.groupId}" %% "${t.artifactId}" % "${t.version}""""

  def toMaven: String =
    s"""|<dependency>
        |  <groupId>${t.groupId}</groupId>
        |  <artifactId>${t.artifactId}</artifactId>
        |  <version>${t.version}</version>
        |</dependency>""".stripMargin

val VERSION = "0.1.0-20220702.1"
object Main:
  def main(args: Array[String]): Unit =
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
          s"scala-cli repl:\n\tscala-cli repl -S 3.1.3 --dep ${deps.toIvy}\n"
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
