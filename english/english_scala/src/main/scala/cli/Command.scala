package cli

sealed case class Command(name: String, alias: List[String]):
  lazy val nameAndAlias: List[String] = name :: alias

  def contains(command: String): Boolean = nameAndAlias.contains(command)

object Commands:
  object MonthCommand   extends Command("month", List("m", "-m"))
  object WeekCommand    extends Command("week", List("w", "-w"))
  object QuarterCommand extends Command("quarter", List("q", "-q"))
  object DaysCommand    extends Command("days", List("d", "-d"))

  val allCommands: List[Command] =
    List(MonthCommand, WeekCommand, QuarterCommand, DaysCommand)

  val allCommandNames: List[String] =
    allCommands.flatMap(_.nameAndAlias)
