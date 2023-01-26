import java.io.File
import java.nio.file.Paths

object AppLogic:

  opaque type CommandNames = List[String]

  def getNames(all: Boolean, searchKeys: Seq[String]): CommandNames =
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

  extension (names: CommandNames)
    def prettyPrint(full_name: Boolean): Unit =
      val namesGrouped = names.groupBy(_.head)
      for (_, names) <- namesGrouped do
        val s = s"""> ${names.map(_.display(full_name)).mkString(" ")}"""
        println(s)
    end prettyPrint

  extension (name: String)
    private inline def display(full_name: Boolean): String =
      if full_name then name else name.noExtension

    private inline def noExtension: String =
      if name.contains(".") then
        val pos = name.lastIndexOf(".")
        name.substring(0, pos)
      else name
