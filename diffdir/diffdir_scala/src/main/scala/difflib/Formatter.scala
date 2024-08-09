package difflib

import difflib.types.{DiffResult, FileTree}

/// 格式化 type class
trait Formatter[T]:
    extension (t: T)
        def formatForConsole(level: Int = 0): String

trait FileTreeFormatter extends Formatter[FileTree]

trait DiffResultFormatter extends Formatter[DiffResult]
