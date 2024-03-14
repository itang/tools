import scala.jdk.CollectionConverters.*
import java.nio.file.Files
import java.nio.file.Paths
import language.unsafeNulls

case class Item(requestTime: Double, responseTime: Double)
case class Kv(name: String, value: Double)
case class SubResult(name: String, avg: Double, max: Double, min: Double, ps: List[Kv])
case class Result(count: Int, items: List[SubResult])

object Main:
    private val plist = List(0.995, 0.99, 0.98, 0.97, 0.96, 0.95, 0.90, 0.75, 0.50)

    def main(args: Array[String]): Unit =
        val file    = if args.length >= 1 then args(0) else "D:\\authine\\项目\\建发\\more\\access.log-20240304(1)"
        val apiName = if args.length >= 2 then args(1) else "/getClient"
        println(s"""file: $file\napiname:$apiName""")

        val content = Files.readString(Paths.get(file))

        val result = getResult(content, apiName)

        println(s"count: ${result.count}")

        for item <- result.items do
            println(item.name)
            println(s"\tavg: ${item.avg}")
            println(s"\tmax: ${item.max}")
            for kv <- item.ps do
                println(s"\t${kv.name}: ${kv.value}")
            println(s"\tmin: ${item.min}")
            println()
        end for
    end main

    private def subResult(name: String, valueFn: Item => Double, items: List[Item]): SubResult =
        val values = items.map(valueFn)
        SubResult(
          name,
          avg = values.sum / items.length,
          max = values.max,
          min = values.min,
          ps = plist.map(pv => Kv(s"p${(pv * 1000.0).toInt/10.0}", p(values, pv)))
        )
    end subResult

    private def getResult(content: String, apiName: String): Result =
        val clientLines = content.linesIterator.filter(_.contains(apiName)).toList
        clientLines.foreach: line =>
            println(line)

        val items = clientLines.map: line =>
            val array = line.split("\\s+")
            val reqT = {
                val it = array(0); it.substring(1, it.length - 1)
            }.toDouble
            val respT = {
                val it = array(1); if it == "-" then "0" else it.substring(1, it.length - 1)
            }.toDouble
            Item(reqT, respT)

        Result(
          count = items.length,
          items = List(
            subResult("request time", _.requestTime, items),
            subResult("response time", _.responseTime, items)
          )
        )
    end getResult

    private def p(list: List[Double], value: Double): Double =
        val sorted = list.sorted
        val index =
            val it = (list.length * value).toInt - 1;
            if it < 0 then 0 else if it >= list.length then list.length else it
        sorted(index)
    end p

end Main
