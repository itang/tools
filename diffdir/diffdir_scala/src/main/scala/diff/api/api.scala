package diff.api

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
