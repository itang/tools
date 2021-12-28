// using scala 3.1.0
import java.time.*
import java.nio.file.{Files, Paths}
import lib.html.HtmlGen

@main def main(): Unit =
  val startDate = LocalDate.of(2021, 12, 27)
  val endDate = LocalDate.of(2022, 12, 31)

  val content: String = HtmlGen(startDate, endDate).html()
  Files.write(Paths.get("plan.html"), content.getBytes("utf-8"))
