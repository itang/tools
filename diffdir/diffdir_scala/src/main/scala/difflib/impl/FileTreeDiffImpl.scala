package difflib.impl

import difflib.types.{DiffItem, DiffResult, FileTree}
import difflib.FileTreeDiff

/// 差异化对比实现
class FileTreeDiffImpl extends FileTreeDiff:
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
