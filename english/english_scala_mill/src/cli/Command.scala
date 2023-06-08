package cli

sealed case class Command(name: String, alias: List[String]):
  lazy val nameAndAlias: List[String] = name :: alias

  def contains(command: String): Boolean = nameAndAlias.contains(command)

object MonthCommand extends Command("month", List("m", "-m"))

object WeekCommand extends Command("week", List("w", "-w"))

object QuarterCommand extends Command("quarter", List("q", "-q"))

object DaysCommand extends Command("days", List("d", "-d"))

object Command:
  val all: List[Command] = List(MonthCommand, WeekCommand, QuarterCommand, DaysCommand)

  private val allCommandNames: List[String] = all.flatMap(_.nameAndAlias)

  private type OnIgnore = String => Unit

  def from(args: List[String], onIgnore: OnIgnore = defaultOnIgnore): List[Command] =
    def filter(args: List[String]): List[String] =
      args.flatMap:
        case x if !allCommandNames.contains(x) =>
          onIgnore(x)
          None
        case x =>
          Some(x)

    filter(args).flatMap {
      case it if MonthCommand.contains(it)   => Some(MonthCommand)
      case it if WeekCommand.contains(it)    => Some(WeekCommand)
      case it if QuarterCommand.contains(it) => Some(QuarterCommand)
      case it if DaysCommand.contains(it)    => Some(DaysCommand)
      case _                                 => None
    }.distinct

  private def defaultOnIgnore: OnIgnore =
    (x: String) => System.err.println(s"> Unknown '$x' command, just ignore")
