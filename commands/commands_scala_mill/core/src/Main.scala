import mainargs.{main, arg, ParserForMethods, Flag, Leftover}

object Main:

  def main(args: Array[String]): Unit = ParserForMethods(this).runOrExit(args)

  @main
  def commands(
      @arg(short = 'a', doc = "print all commands")
      all: Flag,
      @arg(short = 'f', doc = "print full name")
      full_name: Flag,
      @arg(doc = "search keys")
      searchKeys: Leftover[String]
  ): Unit = _run(all.value, full_name.value, searchKeys.value)

  private def _run(all: Boolean, full_name: Boolean, searchKeys: Seq[String]): Unit =
    val names = AppLogic.getNames(all, searchKeys)
    names.prettyPrint(full_name)
