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

//TODO: 使用tabled样式输出
given diffResultConsoleFormatter: DiffResultFormatter with
    extension (t: DiffResult)
        override def formatForConsole(_level: Int = 0): String =
            t.items.map: item =>
                (item.left, item.right) match
                    case (Some(l), Some(r)) =>
                        f"${l.relatePath}%-100s => on both side, size:eq: ${item.isSizeEqToString}%-5s, left:size: ${l.totalSizeHuman}%-10s, right:size: ${r.totalSizeHuman}%-10s"
                    case (Some(l), None) =>
                        f"${l.relatePath}%-100s => just on left side, left:size: ${l.totalSizeHuman}%-10s"
                    case (None, Some(r)) =>
                        f"${r.relatePath}%-100s => just on right side, right:size: ${r.totalSizeHuman}%-10s"
                    case _ => throw IllegalStateException("illegal state")
            .mkString("\n")

end diffResultConsoleFormatter
