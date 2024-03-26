import language.unsafeNulls
import java.io.File
import mainargs.{Leftover, ParserForMethods, arg, main}
import tang.*
import diff.{Diff, DiffImpl, DiffResult, FileSize, Side}

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

        diff.diff(leftSide, rightSide) |> printTheResult

    }.ignore()

    @main
    def walk(
        @arg(short = 'm', doc = "max level")
        maxLevel: Option[Int],
        @arg(doc = "files")
        files: Leftover[String]
    ): Unit =
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
    end walk

    private def printTheResult(result: DiffResult): Unit =
        for item <- result.items do
            val line = (item.left, item.right) match
                case (Some(l), Some(r)) =>
                    f"${l.relatePath}%-80s, 两边都在 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}, right:size: ${r.totalSizeHuman}"
                case (Some(l), None) =>
                    f"${l.relatePath}%-80s, 只在左边 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}"
                case (None, Some(r)) =>
                    f"${r.relatePath}%-80s, 只在右边 size:eq: ${item.isSizeEq}, right:size: ${r.totalSizeHuman}"
                case _ => throw IllegalStateException("illegal state")

            println(line)

        end for
    end printTheResult
end Cli

object Main:
    def main(args: Array[String]): Unit = ParserForMethods(Cli).runOrExit(args).ignore()
