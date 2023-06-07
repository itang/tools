package types

case class Item(name: String, nameEN: String, abbr: String):
  override def toString: String = f"$name%-6s $nameEN%-16s $abbr%-10s"

case class Group(name: String, items: List[Item])

extension (s: String)
  def toItems: List[Item] =
    s.stripMargin.linesIterator
      .map(it =>
        val Array(a, rest @ _*) = it.split(" ")
        Item(a, rest.mkString(" "), "")
      )
      .toList

given Conversion[String, List[Item]] with
  override def apply(x: String): List[Item] = x.toItems
