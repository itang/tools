package diff.impl

import java.io.File

import diff.api.{Diff, Side, FileTree, DiffResult, DiffItem}

/// 差异化对比实现
class DiffImpl extends Diff:
    override def diff(left: Side, right: Side): DiffResult =
        val leftFileTree = loadFileTree(left.rootFile)

        val rightFileTree = loadFileTree(right.rootFile)

        val items = diffFileTree(leftFileTree, rightFileTree)

        DiffResult(items)
    end diff

    override def loadFileTree(root: File): FileTree =
        // @rec
        def _walk(file: File): FileTree =
            if file.isDirectory then
                import language.unsafeNulls
                val children = file.listFiles().map(_walk)
                FileTree(file, root, children)
            else
                FileTree(file, root, Array.empty)

        _walk(root)
    end loadFileTree

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

end DiffImpl
