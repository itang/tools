import language.unsafeNulls
import tang.*

import java.io.File
import diff.{Diff, DiffImpl, DiffResult, FileSize, Side}

object Main:
    def main(args: Array[String]): Unit = time {
        assert(args.length >= 2)

        val leftDir  = args(0)
        val rightDir = args(1)

        val leftSide  = Side("left", File(leftDir))
        val rightSide = Side("right", File(rightDir))

        println(leftSide)
        println(rightSide)

        val diff: Diff = DiffImpl()

        val leftFileSize = leftSide.rootFile |> diff.walkFile
        leftFileSize.walk() { (file: FileSize, level: Int) =>
            println(file.toStringWithLevel(level))
        }

        println()

        val rightFileSize = rightSide.rootFile |> diff.walkFile
        rightFileSize.walk() { (file: FileSize, level: Int) =>
            println(file.toStringWithLevel(level))
        }

        println()

        val diffResult = diff.diff(leftSide, rightSide)
        printTheResult(diffResult)

    }.ignore()

    private def printTheResult(result: DiffResult): Unit =
        for item <- result.items do
            (item.left, item.right) match
                case (Some(l), Some(r)) =>
                    if item.isSizeEq.contains(false) then
                        println(
                          f"${l.relatePath}%-80s, 两边都在 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}, right:size: ${r.totalSizeHuman}"
                        )
                case (Some(l), None) =>
                    println(f"${l.relatePath}%-80s, 只在左边 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}")
                case (None, Some(r)) =>
                    println(f"${r.relatePath}%-80s, 只在右边 size:eq: ${item.isSizeEq}, right:size: ${r.totalSizeHuman}")
                case _ => throw IllegalStateException("illegal state")
end Main
