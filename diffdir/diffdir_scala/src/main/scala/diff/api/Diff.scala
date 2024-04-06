package diff.api

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

/// 带层级的文件树结构
case class FileTree(node: File, root: File, children: Array[FileTree]) extends TreeNode[File, FileTree]:

    // NOTICE: 需确保children的totalSize先计算好
    private val totalSize: Long            = if node.isDirectory then children.map(_.totalSize).sum else node.length()
    private inline def totalSizeKB: Double = totalSize / 1024.0
    private inline def totalSizeMB: Double = totalSize / 1024.0 / 1024.0
    private inline def totalSizeGB: Double = totalSize / 1024.0 / 1024.0 / 1024.0

    val totalSizeHuman: String =
        if totalSize > 1024 * 1024 * 1024 then f"$totalSizeGB%.3fGB"
        else if totalSize > 1024 * 1024 then f"$totalSizeMB%.3fMB"
        else if totalSize > 1024 then f"$totalSizeKB%.3fKB"
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

/// 差异化对比项
case class DiffItem(left: Option[FileTree], right: Option[FileTree]):
    def isSizeEq: Option[Boolean] = (left, right) match
        case (Some(l), Some(r)) => Some(l.node.length() == r.node.length())
        case _                  => None
end DiffItem

/// 对比结果表达对象
case class DiffResult(items: List[DiffItem])

/// 差异化对比特质
trait Diff:
    def loadFileTree(file: File): FileTree

    def diff(left: FileTree, right: FileTree): DiffResult

/// 格式化 type class
trait Formatter[T]:
    extension (t: T)
        def formatForConsole(level: Int = 0): String
