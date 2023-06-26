import scala.annotation.tailrec
import scala.util.Try

case class Dependency(
    groupId: String,
    artifactId: String,
    version: String
)

object Dependency:
  def from(s: String)(using parsers: List[Parser]): Option[Dependency] =
    @tailrec
    def _from(parsers: List[Parser]): Option[Dependency] =
      parsers match
        case Nil => None
        case parser :: tail =>
          parser.unapply(s) match
            case Some(d) => Some(d)
            case None    => _from(tail)

    Try(_from(parsers)).toOption.flatten

trait Parser:
  def unapply(s: String): Option[Dependency]

object SbtParser extends Parser:
  def unapply(s: String): Option[Dependency] =
    import scala.language.unsafeNulls
    if s.contains("%") then
      val Array(a, b, c) = s.trim
        .replaceAll("%", "")
        .split("\\s+")
        .map(it =>
          val itt = it.trim
          if itt.startsWith("\"") || itt.startsWith("'") then itt.substring(1, it.length - 1)
          else itt
        )
      Some(Dependency(a, b, c))
    else None

object IvyParser extends Parser:
  def unapply(s: String): Option[Dependency] =
    import scala.language.unsafeNulls
    if s.contains(":") then
      val Array(a, b, c) = s.trim.replaceAll(":", " ").split("\\s+").map(_.trim)
      Some(Dependency(a, b, c))
    else None

extension (t: Dependency)
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
