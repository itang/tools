import scala.jdk.CollectionConverters.*

import java.nio.charset.Charset
import tang.*

object Main:
    def main(args: Array[String]): Unit = time {
        println("available charsets:")

        val items = Charset.availableCharsets().asScala.zipWithIndex
        for ((key, value), index) <- items do
            println(f"$index%-3d: $key, $value")

    }.ignore()

end Main
