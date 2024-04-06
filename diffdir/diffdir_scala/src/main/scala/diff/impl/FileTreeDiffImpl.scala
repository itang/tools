package diff.impl

import java.io.File
import diff.api.{
    AbstractFileTreeDiff,
    AbstractFileTreeLoader,
    DiffItem,
    DiffResult,
    DiffResultFormatter,
    FileTree,
    FileTreeFormatter
}

/// Loader
class FileTreeLoader extends AbstractFileTreeLoader:
    override def load(root: File): FileTree =
        // @rec
        def _walk(file: File): FileTree =
            if file.isDirectory then
                import language.unsafeNulls
                val children = file.listFiles().map(_walk)
                FileTree(file, root, children)
            else
                FileTree(file, root, Array.empty)

        _walk(root)
    end load
end FileTreeLoader

/// 差异化对比实现
class FileTreeDiffImpl extends AbstractFileTreeDiff:
    override def diff(left: FileTree, right: FileTree): DiffResult =
        val items = diffFileTree(left, right)

        DiffResult(items)
    end diff

    // TODO: 树比较输出结构和优化性能
    private def diffFileTree(left: FileTree, right: FileTree): List[DiffItem] =
        var list: List[DiffItem] = Nil
        left.walk(): (file, _) =>
            val leftPath = file.relatePath
            list = DiffItem(Some(file), right.findByRelatePath(leftPath)) :: list

        right.walk(): (file, _) =>
            val rightPath    = file.relatePath
            val leftFileSize = left.findByRelatePath(rightPath)
            leftFileSize match
                case None => list = DiffItem(None, Some(file)) :: list
                case _    =>

        list.reverse

    end diffFileTree

end FileTreeDiffImpl

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
