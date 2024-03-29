package cli

import java.io.File
import mainargs.{Leftover, arg, main}
import tang.{ignore, time}
import diff.api.{Diff, Side, FileTree}
import diff.impl.DiffImpl

/// 命令行界面
object Cli:
    private lazy val diff: Diff = DiffImpl()

    @main
    def diff(
        @arg(short = 'l', doc = "left file")
        leftFile: String,
        @arg(short = 'r', doc = "right file")
        rightFile: String
    ): Unit = time {
        val leftSide  = Side("left", File(leftFile))
        val rightSide = Side("right", File(rightFile))

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

        val fileSizes = files.value.map(File(_)).map(diff.loadFileTree)
        for fileSize <- fileSizes do
            fileSize.walk(): (file: FileTree, level: Int) =>
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
