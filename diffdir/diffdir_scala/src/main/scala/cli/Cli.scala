package cli

import java.io.File
import mainargs.{Leftover, arg, main}
import tang.{ignore, time, |>}
import diff.api.{Diff, DiffResult, FileTree, Loader}
import diff.impl.{FileTreeDiffImpl, FileTreeLoader, given}

/// 命令行界面
object Cli:
    private lazy val loader: Loader[File, FileTree]   = FileTreeLoader()
    private lazy val diff: Diff[FileTree, DiffResult] = FileTreeDiffImpl()

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

        val fileSizes = files.value.map(File(_)).map(loader.load)
        for fileSize <- fileSizes do
            fileSize.walk(): (file: FileTree, level: Int) =>
                maxLevel match
                    case Some(v) if level <= v =>
                        file.formatForConsole(level) |> println
                    case None =>
                        file.formatForConsole(level) |> println
                    case _ =>
            println()
        end for
    }.ignore()

end Cli
