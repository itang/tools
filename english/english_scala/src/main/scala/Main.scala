import cli.*
import data.*
import tang.|>
import types.Group

object Main:

  def main(args: Array[String]): Unit = args match
    case Array()                               => Command.all |> commandsToGroups |> printGroups
    case Array("--help" | "-h" | "help")       => help()
    case Array("--version" | "-v" | "version") => version()
    case _ => args.toList |> (Command.fromList(_).map(_._2)) |> commandsToGroups |> printGroups

  private def commandsToGroups(commands: List[Command]): List[Group] =
    commands.flatMap:
      case MonthCommand   => Some(GroupRepository.monthGroup)
      case WeekCommand    => Some(GroupRepository.weekGroup)
      case QuarterCommand => Some(GroupRepository.quarterGroup)
      case DaysCommand    => Some(GroupRepository.daysGroup)

  private def printGroups(groups: Iterable[Group]): Unit =
    groups
      .map:
        case Group(name, items) => s"$name\n${"-" * 60}\n${items.mkString("\n")}"
      .mkString("\n\n\n")
      |> println

  private def help(): Unit =
    s"""|$$ english [command] ... [command]
        |    sub commands:
        |      ${MonthCommand.nameAndAlias.mkString(", ")}
        |      ${WeekCommand.nameAndAlias.mkString(", ")}
        |      ${QuarterCommand.nameAndAlias.mkString(", ")}
        |      ${DaysCommand.nameAndAlias.mkString(", ")}
        |
        |      help, --help, -h
        |      version, --version, -v
        |
        |    e.g.
        |      english month
        |
        |""".stripMargin
      |> println

  private def version(): Unit =
    "v0.2-20241011.1" |> println
