import cli.Command
import cli.Commands.*
import data.{daysGroup, monthGroup, quarterGroup, weekGroup}
import tang.{panic_!, |>}
import types.Group

object Main:

  def main(args: Array[String]): Unit = args match
    case Array()                         => allCommands |> distinctCommands |> commandsToGroups |> print
    case Array("--help" | "-h" | "help") => help()
    case _                               => args.toList |> filterArgs |> distinctCommands |> commandsToGroups |> print

  private def filterArgs(args: List[String]): List[String] =
    args.flatMap:
      case x if !allCommands.contains(x) =>
        System.err.println(s"> Unknown '$x' command, just ignore")
        None
      case x => Some(x)

  private def distinctCommands(args: List[String]): Set[Command] =
    args.flatMap {
      case it if monthCommand.contains(it)   => Some(monthCommand)
      case it if weekCommand.contains(it)    => Some(weekCommand)
      case it if quarterCommand.contains(it) => Some(quarterCommand)
      case it if daysCommand.contains(it)    => Some(daysCommand)
      case _                                 => None
    }.toSet

  private def commandsToGroups(commands: Set[Command]): Set[Group] =
    commands.flatMap:
      case Command("month", _)   => Some(monthGroup)
      case Command("week", _)    => Some(weekGroup)
      case Command("quarter", _) => Some(quarterGroup)
      case Command("days", _)    => Some(daysGroup)
      case _                     => panic_!("unknown command")

  private def print(groups: Iterable[Group]): Unit =
    val s = groups.map { case Group(name, items) => s"$name\n${"-" * 60}\n${items.mkString("\n")}" }.mkString("\n\n\n")
    s |> println

  private def help(): Unit =
    println(s"""|$$ english [command] ... [command]
                |    sub commands:
                |      ${monthCommand.all_commands.mkString(", ")}
                |      ${weekCommand.all_commands.mkString((", "))}
                |      ${quarterCommand.all_commands.mkString(", ")}
                |      ${daysCommand.all_commands.mkString(", ")}
                |
                |    e.g.
                |      english month
                |
                |""".stripMargin)
