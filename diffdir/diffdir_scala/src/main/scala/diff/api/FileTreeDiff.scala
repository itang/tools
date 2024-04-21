package diff.api

import diff.api.types.{DiffResult, FileTree}

/// 差异化对比特质
trait Diff[A, B]:
    def diff(left: A, right: A): B

trait FileTreeDiff extends Diff[FileTree, DiffResult]
