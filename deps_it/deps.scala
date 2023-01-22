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
          if itt.startsWith("\"") || itt.startsWith("'") then itt.substring(1, it.length - 1)
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
