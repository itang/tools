import data.{daysGroup, monthGroup, quarterGroup, weekGroup}
import tang.*
import types.Group

object Main:
  def main(args: Array[String]): Unit =
    val groups: Array[Group] = argsToGroups(args)

    val s =
      (for Group(name, items) <- groups yield s"$name\n${"-" * 60}\n${items.mkString("\n")}")
        .mkString("\n\n\n")

    s |> println

  private def argsToGroups(args: Array[String]): Array[Group] =
    if args.isEmpty then Array(monthGroup, weekGroup, quarterGroup, daysGroup)
    else
      args.distinct.flatMap:
        case "month"   => Array(monthGroup)
        case "week"    => Array(weekGroup)
        case "quarter" => Array(quarterGroup)
        case "days"    => Array(daysGroup)
        case _         => Array.empty[Group]
