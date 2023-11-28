package app

import java.io.File
import java.nio.file.Paths
import app.Util.*

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
                    if files == null then Array.empty[File] else files.nn
                catch case _: Throwable => Array.empty[File]
            )
            .filter(_.nn.isExe)
            .map(_.nn.getName.nn)

        if searchKeys.isEmpty then names
        else names.filter(name => searchKeys.exists(name.noExtension.contains))
    end getNames

    private def getDirs(all: Boolean): List[File] =
        val home = System.getProperty("user.home")
        val files =
            if all then
                val path = System.getenv("Path").nn
                path.split(File.pathSeparator).nn.map(_.nn.trim.nn).filterNot(_.isBlank.nn).map(File(_)).toList
            else
                List(
                  Paths.get("D:/dev-env/bin"),
                  Paths.get(home, ".cargo", "bin"),
                  Paths.get(home, ".deno", "bin"),
                  Paths.get(home, "AppData/Roaming/npm")
                ).map(_.nn.toFile.nn)

        files.filter(_.isDirectory)
    end getDirs

    extension (name: String)
        private inline def display(full_name: Boolean): String =
            if full_name then name else name.noExtension
end AppLogic
