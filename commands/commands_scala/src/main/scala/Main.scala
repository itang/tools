import tang.*

import java.nio.file.Paths
import java.io.File
import scala.util.Try

import mainargs.{main, arg, ParserForMethods, Flag}

object Main {

  def main(args: Array[String]): Unit = ParserForMethods(this).runOrExit(args)

  @main
  def run(
      @arg(short = 'a', doc = "print all commands")
      all: Flag,
      @arg(short = 'f', doc = "print full name")
      full_name: Flag
  ): Unit = _run(all.value, full_name.value)

  private def _run(all: Boolean, full_name: Boolean): Unit =
    val home = System.getProperty("user.home")

    val dirList =
      if all then System.getenv("Path").split(File.pathSeparator).map(_.trim).filterNot(_.isBlank).map(File(_)).toList
      else
        List(
          Paths.get("D:/dev-env/bin"),
          Paths.get(home, ".cargo", "bin"),
          Paths.get(home, ".deno", "bin"),
          Paths.get(home, "AppData/Roaming/npm")
        ).map(_.toFile)

    val names = dirList
      .flatMap(f =>
        try
          val files = f.listFiles()
          if files == null then Array.empty[File] else files
        catch case _: Throwable => Array.empty[File]
      )
      .filter(it =>
        val name = it.getName
        // TODO: win / linux
        name.endsWith(".exe") || name.endsWith(".bat") || name.endsWith(".cmd") || !name.contains(".")
      )
      .filter(_.isFile)
      .map(_.getName)

    val names2 = names.groupBy(_.head)

    for (_, names) <- names2 do
      for
        name <- names
        nameWrap = if full_name then name else name.noExtension
      do print(nameWrap + " ")

      println()

  extension (s: String)
    private def noExtension: String =
      if s.contains(".") then
        val pos = s.lastIndexOf(".")
        s.substring(0, pos)
      else s
}
