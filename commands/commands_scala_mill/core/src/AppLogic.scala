import java.io.File
import java.nio.file.Paths

object AppLogic:

  opaque type CommandNames = List[String]

  extension (names: CommandNames)
    def prettyPrint(full_name: Boolean): Unit =
      val s = names
        .groupBy(_.head)
        .toList
        .sortBy(_._1)
        .map { case (_, names) => s"""> ${names.map(_.display(full_name)).mkString(" ")}""" }
        .mkString("\n")
      println(s)

  def getNames(all: Boolean, searchKeys: Seq[String]): CommandNames =
    val names = getDirs(all)
      .flatMap(dir =>
        try
          val files = dir.listFiles()
          if files == null then Array.empty[File] else files
        catch case _: Throwable => Array.empty[File]
      )
      .filter(_.isExe)
      .map(_.getName)

    if searchKeys.isEmpty then names
    else names.filter(name => searchKeys.exists(name.noExtension.contains))

  //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

  private def getDirs(all: Boolean): List[File] =
    val home = System.getProperty("user.home")
    val files =
      if all then
        val path = System.getenv("Path")
        path.split(File.pathSeparator).map(_.trim).filterNot(_.isBlank).map(File(_)).toList
      else
        List(
          Paths.get("D:/dev-env/bin"),
          Paths.get(home, ".cargo", "bin"),
          Paths.get(home, ".deno", "bin"),
          Paths.get(home, "AppData/Roaming/npm")
        ).map(_.toFile)

    files.filter(_.isDirectory)
  end getDirs

  extension (name: String)
    private inline def display(full_name: Boolean): String =
      if full_name then name else name.noExtension

    private inline def noExtension: String =
      if name.contains(".") then
        val pos = name.lastIndexOf(".")
        name.substring(0, pos)
      else name

  extension (file: File)
    private def isExe: Boolean = file.isFile && {
      val name = file.getName
      // TODO: win / linux
      name.endsWith(".exe") || name.endsWith(".bat") || name.endsWith(".cmd") || !name.contains(".")
    }
