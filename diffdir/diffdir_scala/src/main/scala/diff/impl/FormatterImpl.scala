package diff.impl

import diff.api.{DiffResultFormatter, FileTreeFormatter}
import diff.api.types.{DiffResult, FileTree}

import java.io.File

given fileTreeConsoleFormatter: FileTreeFormatter with
    extension (f: FileTree)
        override def formatForConsole(level: Int): String =
            extension (f: File)
                private inline def asFileDisplay: String = if f.isDirectory then "+" else "-"

            val path   = if level == 0 then f.node.getAbsolutePath else f.node.getName
            val prefix = s"""${" " * level * 2}${f.node.asFileDisplay} $path"""

            f"$prefix%-80s ${f.totalSizeHuman}%10s"

end fileTreeConsoleFormatter

given diffResultConsoleFormatter: DiffResultFormatter with
    extension (t: DiffResult)
        override def formatForConsole(_level: Int = 0): String =
            t.items.map: item =>
                (item.left, item.right) match
                    case (Some(l), Some(r)) =>
                        f"${l.relatePath}%-80s, 两边都在 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}, right:size: ${r.totalSizeHuman}"
                    case (Some(l), None) =>
                        f"${l.relatePath}%-80s, 只在左边 size:eq: ${item.isSizeEq}, left:size: ${l.totalSizeHuman}"
                    case (None, Some(r)) =>
                        f"${r.relatePath}%-80s, 只在右边 size:eq: ${item.isSizeEq}, right:size: ${r.totalSizeHuman}"
                    case _ => throw IllegalStateException("illegal state")
            .mkString("\n")

end diffResultConsoleFormatter
