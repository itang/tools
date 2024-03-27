package diff

import diff.FType.{FDir, FFile}

import language.unsafeNulls
import java.io.File

case class Side(name: String, rootFile: File)

enum FType:
    case FFile, FDir
    def name: String = this match
        case FFile => "文件"
        case FDir  => "目录"
end FType

extension (f: File)
    def asFileDisplay: String = if f.isDirectory then "+" else "-"

case class FileSize(file: File, root: File, children: Array[FileSize]):

    private val totalSize: Long     = if file.isDirectory then children.map(_.totalSize).sum else file.length()
    private def totalSizeKB: Double = totalSize / 1024.0
    private def totalSizeMB: Double = totalSize / 1024.0 / 1024.0
    private def totalSizeGB: Double = totalSize / 1024.0 / 1024.0 / 1024.0

    val totalSizeHuman: String =
        if totalSize > 1024 * 1024 * 1024 then f"$totalSizeGB%.3fGB"
        else if totalSize > 1024 * 1024 then f"$totalSizeMB%.3fMB"
        else if totalSize > 1024 then f"$totalSizeKB%.3fKB"
        else s"${totalSize}B"
    end totalSizeHuman

    val relatePath: String = file.getAbsolutePath.substring(root.getAbsolutePath.length)

    def toStringWithLevel(level: Int): String =
        val path   = if level == 0 then file.getAbsolutePath else file.getName
        val prefix = f"""${" " * level * 2}${file.asFileDisplay} $path"""
        f"$prefix%-80s $totalSizeHuman%10s"
    end toStringWithLevel

    def findByRelatePath(relatePath: String): Option[FileSize] =
        find { file => file.relatePath == relatePath }

    private type WalkFn[T] = (FileSize, Int) => T

    // TODO: 树搜索性能优化
    private def find(fn: FileSize => Boolean): Option[FileSize] =
        def _find(fileSize: FileSize, fn: FileSize => Boolean): Option[FileSize] =
            if fn(fileSize) then Some(fileSize)
            else
                var ret: Option[FileSize] = None
                for f <- fileSize.children if ret.isEmpty do
                    _find(f, fn) match
                        case None =>
                        case v    => ret = v
                ret
        end _find

        _find(this, fn)
    end find

    def walk(maxLevel: Int | Option[Int] = None)(fn: WalkFn[Unit]): Unit =
        def _walk(file: FileSize, level: Int): Unit =
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

end FileSize

case class Item(left: Option[FileSize], right: Option[FileSize]):
    def isSizeEq: Option[Boolean] = (left, right) match
        case (Some(l), Some(r)) => Some(l.file.length() == r.file.length())
        case _                  => None
end Item

case class DiffResult(items: List[Item])

trait Walk:
    def walkFile(file: File): FileSize

trait Diff extends Walk:
    def diff(leftSide: Side, rightSide: Side): DiffResult

class DiffImpl extends Diff:
    override def diff(leftSide: Side, rightSide: Side): DiffResult =
        val leftFiles = loadFileSize(leftSide)

        val rightFiles = loadFileSize(rightSide)

        val items = diffTheFiles(leftFiles, rightFiles)

        DiffResult(items)
    end diff

    private def loadFileSize(side: Side): FileSize = walkFile(side.rootFile)

    override def walkFile(root: File): FileSize =
        // @rec
        def _walk(file: File): FileSize =
            if file.isDirectory then
                val children = file.listFiles().map(_walk)
                FileSize(file, root, children)
            else
                FileSize(file, root, Array.empty)

        _walk(root)
    end walkFile

    // TODO: 树比较输出结构和优化性能
    private def diffTheFiles(left: FileSize, right: FileSize): List[Item] =
        var list: List[Item] = Nil
        left.walk(): (file, _) =>
            val leftPath = file.relatePath
            list = Item(Some(file), right.findByRelatePath(leftPath)) :: list

        right.walk(): (file, _) =>
            val rightPath    = file.relatePath
            val leftFileSize = left.findByRelatePath(rightPath)
            leftFileSize match
                case None => list = Item(None, Some(file)) :: list
                case _    =>

        list.reverse

    end diffTheFiles

end DiffImpl
