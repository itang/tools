package cli

sealed abstract class Command(val name: String, val alias: String*):
  lazy val nameAndAlias: List[String] = name :: alias.toList

  def contains(command: String): Boolean = nameAndAlias.contains(command)

object MonthCommand extends Command("month", "m", "-m")

object WeekCommand extends Command("week", "w", "-w")

object QuarterCommand extends Command("quarter", "q", "-q")

object DaysCommand extends Command("days", "d", "-d")

object Command:
  val all: List[Command] = List(MonthCommand, WeekCommand, QuarterCommand, DaysCommand)

  private type OnIgnore = String => Unit

  /** from: Command from a string arg.
    *
    * @param arg
    *   Command line arg
    * @return
    *   The Command
    */
  def from(arg: String): Option[Command] =
    arg match
      case _ if MonthCommand.contains(arg)   => Some(MonthCommand)
      case _ if WeekCommand.contains(arg)    => Some(WeekCommand)
      case _ if QuarterCommand.contains(arg) => Some(QuarterCommand)
      case _ if DaysCommand.contains(arg)    => Some(DaysCommand)
      case _                                 => None

  def fromList(args: List[String], onIgnore: OnIgnore = defaultOnIgnore): List[(String, Command)] =
    args
      .flatMap: arg =>
        from(arg) match
          case Some(it) =>
            Some((arg, it))
          case None =>
            onIgnore(arg)
            None
      .distinct

  private def defaultOnIgnore: OnIgnore =
    (x: String) => System.err.println(s"> Unknown '$x' command, just ignore")
