package difflib.types


/// 差异化对比项
case class DiffItem(left: Option[FileTree], right: Option[FileTree]):
    def isSizeEq: Option[Boolean] = (left, right) match
        case (Some(l), Some(r)) => Some(l.node.length() == r.node.length())
        case _                  => None
        
    def isSizeEqToString: String = isSizeEq match
        case Some(b) => b.toString
        case _       => ""
end DiffItem