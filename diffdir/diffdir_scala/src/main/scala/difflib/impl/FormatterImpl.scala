package difflib.impl

import difflib.{DiffResultFormatter, FileTreeFormatter}
import difflib.types.{DiffResult, FileTree}

import java.io.File

class FileTreeConsoleFormatter extends FileTreeFormatter:
    extension (f: FileTree)
        override def formatForConsole(level: Int): String =
            extension (f: File)
                private inline def asFileDisplay: String = if f.isDirectory then "+" else "-"

            val path   = if level == 0 then f.node.getAbsolutePath else f.node.getName
            val prefix = s"""${" " * level * 2}${f.node.asFileDisplay} $path"""

            f"$prefix%-80s ${f.totalSizeHuman}%10s"

end FileTreeConsoleFormatter

class DiffResultConsoleFormatter extends DiffResultFormatter:
    extension (t: DiffResult)
        override def formatForConsole(_level: Int = 0): String =
            t.items.map: item =>
                (item.left, item.right) match
                    case (Some(l), Some(r)) =>
                        f"${l.relatePath}%-138s > [both ], eq: ${item.isSizeEqToString}%-5s, left:size: ${l.totalSizeHuman}%-10s, right:size: ${r.totalSizeHuman}%-10s"
                    case (Some(l), None) =>
                        f"${l.relatePath}%-138s > [left ], left:size : ${l.totalSizeHuman}%-10s"
                    case (None, Some(r)) =>
                        f"${r.relatePath}%-138s > [right], right:size: ${r.totalSizeHuman}%-10s"
                    case _ => throw IllegalStateException("illegal state")
            .mkString("\n")
end DiffResultConsoleFormatter

class DiffResultTableConsoleFormatter extends DiffResultFormatter:
    extension (t: DiffResult)
        override def formatForConsole(_level: Int = 0): String =
            import de.vandermeer.asciitable.AsciiTable
            import de.vandermeer.skb.interfaces.transformers.textformat.TextAlignment

            val asciiTable = new AsciiTable()
            asciiTable.addRule()
            asciiTable.addRow("path", "M", "eq", "left", "right")
            asciiTable.addRule()

            t.items.foreach: item =>
                (item.left, item.right) match
                    case (Some(l), Some(r)) =>
                        asciiTable.addRow(
                          l.relatePath,
                          "both",
                          item.isSizeEqToString,
                          l.totalSizeHuman,
                          r.totalSizeHuman
                        )
                    case (Some(l), None) =>
                        asciiTable.addRow(l.relatePath, "left", item.isSizeEqToString, l.totalSizeHuman, "")
                    case (None, Some(r)) =>
                        asciiTable.addRow(r.relatePath, "right", item.isSizeEqToString, "", r.totalSizeHuman)
                    case _ => throw IllegalStateException("illegal state")
                end match

                asciiTable.addRule()

            asciiTable.setTextAlignment(TextAlignment.CENTER)
            asciiTable.render
end DiffResultTableConsoleFormatter
