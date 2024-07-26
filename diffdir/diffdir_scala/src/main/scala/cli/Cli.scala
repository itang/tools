package cli

import java.io.File
import mainargs.{Leftover, arg, main}
import tang.{ignore, time, |>}

import diff.api.types.DiffResult
import diff.api.{FileTreeDiff, FileTreeLoader}
import diff.impl.{FileTreeDiffImpl, FileTreeLoaderImpl, given}

/// 命令行界面
object Cli:
    private lazy val loader: FileTreeLoader = FileTreeLoaderImpl()
    private lazy val diff: FileTreeDiff     = FileTreeDiffImpl()

    @main
    def diff(
        @arg(short = 'l', doc = "left file")
        leftFile: String,
        @arg(short = 'r', doc = "right file")
        rightFile: String
    ): Unit = time {
        val left  = leftFile |> (File(_)) |> loader.load
        val right = rightFile |> (File(_)) |> loader.load

        diff.diff(left, right).formatForConsole() |> println

    }.ignore()

    @main
    def walk(
        @arg(short = 'm', doc = "max level")
        maxLevel: Option[Int],
        @arg(doc = "files")
        files: Leftover[String]
    ): Unit = time {
        println(s"DEBUG: $maxLevel $files")

        val filesP =doCheckAndPreprocessingFiles(files)

        val fileSizes = filesP.value.map(File(_)).map(loader.load)
        for fileSize <- fileSizes do
            fileSize.walk(): (file, level) =>
                maxLevel match
                    case Some(v) if level <= v =>
                        file.formatForConsole(level) |> println
                    case None =>
                        file.formatForConsole(level) |> println
                    case _ =>
            println()
        end for
    }.ignore()

    private def doCheckAndPreprocessingFiles(files: Leftover[String]): Leftover[String] =
        if files.value.isEmpty then
            println(s"INFO: Unspecified directory, use the current work directory")
            Leftover(".")

        else files

end Cli
