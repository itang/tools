import java.io.File
import java.nio.file.Paths

object AppLogic:
  def getNames(all: Boolean, searchKeys: Seq[String]): List[String] =
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

    if searchKeys.isEmpty then names
    else names.filter(name => searchKeys.exists(key => name.noExtension.contains(key)))

  end getNames

  def printNames(names: List[String], full_name: Boolean): Unit =
    for (_, names) <- names.groupBy(_.head) do
      for
        name <- names
        nameWrap = if full_name then name else name.noExtension
      do print(nameWrap + " ")

      println()
  end printNames

  extension (s: String)
    private def noExtension: String =
      if s.contains(".") then
        val pos = s.lastIndexOf(".")
        s.substring(0, pos)
      else s
