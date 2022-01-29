//> using scala "3.1.1"
import java.time.*
import java.nio.file.{Files, Paths}
import lib.html.JifenHtmlGen

@main
def main(): Unit =
  val startDate = LocalDate.of(2021, 12, 27)
  val endDate = LocalDate.of(2022, 12, 31)

  println("start gen...")
  val content: String = JifenHtmlGen(startDate, endDate).html()
  Files.write(Paths.get("plan.html"), content.getBytes("utf-8"))
  println("finished.")
