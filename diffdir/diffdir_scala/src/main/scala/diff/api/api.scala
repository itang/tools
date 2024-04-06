package diff.api

import java.io.File

/// Loader
trait Loader[A, B]:
    def load(a: A): B

/// 差异化对比特质
trait Diff[A, B]:
    def diff(left: A, right: A): B

/// 格式化 type class
trait Formatter[T]:
    extension (t: T)
        def formatForConsole(level: Int = 0): String

abstract class AbstractFileTreeLoader extends Loader[File, FileTree]

abstract class AbstractFileTreeDiff extends Diff[FileTree, DiffResult]

trait FileTreeFormatter extends Formatter[FileTree]

trait DiffResultFormatter extends Formatter[DiffResult]
