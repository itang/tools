package diff.api.types

import java.io.File

/// 树形结构
trait TreeNode[N, T <: TreeNode[N, T]]:
    /// 当前节点
    val node: N
    /// 顶点
    val root: N
    /// 子集
    val children: Array[T]
end TreeNode

object FileTree:
    private val _1024: Int     = 1024
    private val _1024d: Double = _1024.toDouble

/// 带层级的文件树结构
case class FileTree(node: File, root: File, children: Array[FileTree]) extends TreeNode[File, FileTree]:

    import FileTree.{_1024d, _1024}

    // NOTICE: 需确保children的totalSize先计算好
    private val totalSize: Long            = if node.isDirectory then children.map(_.totalSize).sum else node.length()
    private inline def totalSizeKB: Double = totalSize / _1024d
    private inline def totalSizeMB: Double = totalSize / _1024d / _1024d
    private inline def totalSizeGB: Double = totalSize / _1024d / _1024d / _1024d

    val totalSizeHuman: String =
        if totalSize > _1024 * _1024 * _1024 then f"$totalSizeGB%.3fGB"
        else if totalSize > _1024 * _1024 then f"$totalSizeMB%.3fMB"
        else if totalSize > _1024 then f"$totalSizeKB%.3fKB"
        else s"${totalSize}B"
    end totalSizeHuman

    val relatePath: String =
        import language.unsafeNulls
        node.getAbsolutePath.substring(root.getAbsolutePath.length)

    def findByRelatePath(relatePath: String): Option[FileTree] =
        find { file => file.relatePath == relatePath }

    private type WalkFn[T] = (FileTree, Int) => T

    // TODO: 树搜索性能优化
    private def find(fn: FileTree => Boolean): Option[FileTree] =
        def _find(fileSize: FileTree, fn: FileTree => Boolean): Option[FileTree] =
            if fn(fileSize) then Some(fileSize)
            else
                var ret: Option[FileTree] = None
                for f <- fileSize.children if ret.isEmpty do
                    _find(f, fn) match
                        case None =>
                        case v    => ret = v
                ret
        end _find

        _find(this, fn)
    end find

    def walk(maxLevel: Int | Option[Int] = None)(fn: WalkFn[Unit]): Unit =
        def _walk(file: FileTree, level: Int): Unit =
            maxLevel match
                case Some(v) => if level > v then return
                case v: Int  => if level > v then return
                case _       =>

            fn(file, level)

            for f <- file.children do
                _walk(f, level + 1)
        end _walk

        _walk(this, 0)
    end walk

end FileTree
