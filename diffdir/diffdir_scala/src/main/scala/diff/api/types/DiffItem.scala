package diff.api.types


/// 差异化对比项
case class DiffItem(left: Option[FileTree], right: Option[FileTree]):
    def isSizeEq: Option[Boolean] = (left, right) match
        case (Some(l), Some(r)) => Some(l.node.length() == r.node.length())
        case _                  => None
end DiffItem