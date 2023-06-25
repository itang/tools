package types

case class Item(name: String, nameEN: String, abbr: String):
  override def toString: String = f"$name%-6s $nameEN%-16s $abbr%-10s"

case class Group(name: String, items: List[Item])

extension (s: String)
  def toItems: List[Item] =
    s.linesIterator
      .map(_.split(" "))
      .toList
      .flatMap:
        case Array(a, rest*) => Some(Item(a.nn, rest.mkString(" "), ""))
        case _               => None

given stringToListItem: Conversion[String, List[Item]] with
  override def apply(x: String): List[Item] = x.toItems
