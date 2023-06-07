package cli

case class Command(name: String, alias: List[String]):
  lazy val all_commands: List[String] = name :: alias

  def contains(command: String): Boolean = all_commands.contains(command)

object Commands:
  val monthCommand: Command     = Command("month", List("m", "-m"))
  val weekCommand: Command      = Command("week", List("w", "-w"))
  val quarterCommand: Command   = Command("quarter", List("q", "-q"))
  val daysCommand: Command      = Command("days", List("d", "-d"))
  val allCommands: List[String] = List(monthCommand, weekCommand, quarterCommand, daysCommand).flatMap(_.all_commands)
