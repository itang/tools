import language.unsafeNulls

import java.io.File

import mainargs.{/*Flag, Leftover, */ ParserForMethods, arg, main}

import tang.*

import diff.{Diff, DiffImpl, DiffResult, FileSize, Side}

object Main:
    def main(args: Array[String]): Unit = ParserForMethods(this).runOrExit(args).ignore()

    @main
    def run(
        @arg(short = 'l', doc = "left file")
        leftFile: String,
        @arg(short = 'r', doc = "right file")
        rightFile: String
    ): Unit = time {
        val leftSide  = Side("left", File(leftFile))
        val rightSide = Side("right", File(rightFile))

        val diff: Diff = DiffImpl()

        for side <- List(leftSide, rightSide) do
            println(side)

            val fileSize = side.rootFile |> diff.walkFile
            fileSize.walk(): (file: FileSize, level: Int) =>
                println(file.toStringWithLevel(level))

            println()
        end for

        diff.diff(leftSide, rightSide) |> printTheResult

    }.ignore()

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

end Main
