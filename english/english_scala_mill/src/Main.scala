import cli.*
import data.*
import tang.{panic_!, |>}
import types.Group

object Main:

  def main(args: Array[String]): Unit = args match
    case Array()                               => Command.all |> commandsToGroups |> printGroups
    case Array("--help" | "-h" | "help")       => help()
    case Array("--version" | "-v" | "version") => version()
    case _                                     => args.toList |> (Command.from(_)) |> commandsToGroups |> printGroups

  private def commandsToGroups(commands: List[Command]): List[Group] =
    commands.flatMap:
      case MonthCommand   => Some(GroupRepository.monthGroup)
      case WeekCommand    => Some(GroupRepository.weekGroup)
      case QuarterCommand => Some(GroupRepository.quarterGroup)
      case DaysCommand    => Some(GroupRepository.daysGroup)

  private def printGroups(groups: Iterable[Group]): Unit =
    val s = groups
      .map { case Group(name, items) => s"$name\n${"-" * 60}\n${items.mkString("\n")}" }
      .mkString("\n\n\n")

    println(s)

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
    "v0.1-20230608.1" |> println
