import language.unsafeNulls

import java.io.File
import mainargs.{Leftover, ParserForMethods, arg, main}
import tang.*
import diff.{Diff, DiffImpl, FileSize, Side}

object Cli:
    @main
    def diff(
        @arg(short = 'l', doc = "left file")
        leftFile: String,
        @arg(short = 'r', doc = "right file")
        rightFile: String
    ): Unit = time {
        val leftSide  = Side("left", File(leftFile))
        val rightSide = Side("right", File(rightFile))

        val diff: Diff = DiffImpl()

        diff.diff(leftSide, rightSide)
            .outputToConsole()

    }.ignore()

    @main
    def walk(
        @arg(short = 'm', doc = "max level")
        maxLevel: Option[Int],
        @arg(doc = "files")
        files: Leftover[String]
    ): Unit = time {
        println(s"DEBUG: $maxLevel $files")
        val diff: Diff = DiffImpl()
        for fileSize <- files.value.map(File(_) |> diff.walkFile) do
            fileSize.walk(): (file: FileSize, level: Int) =>
                maxLevel match
                    case Some(v) if level <= v =>
                        println(file.toStringWithLevel(level))
                    case None =>
                        println(file.toStringWithLevel(level))
                    case _ =>
            println()
        end for
    }.ignore()

end Cli

object Main:
    def main(args: Array[String]): Unit = ParserForMethods(Cli).runOrExit(args).ignore()
