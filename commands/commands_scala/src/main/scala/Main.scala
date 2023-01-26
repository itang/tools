import tang.*

import mainargs.{main, arg, ParserForMethods, Flag}
import AppLogic.*

object Main:

  def main(args: Array[String]): Unit = ParserForMethods(this).runOrExit(args)

  @main
  def run(
      @arg(short = 'a', doc = "print all commands")
      all: Flag,
      @arg(short = 'f', doc = "print full name")
      full_name: Flag
  ): Unit = _run(all.value, full_name.value)

  private def _run(all: Boolean, full_name: Boolean): Unit =
    val names = getNames(all)
    printNames(names, full_name)
