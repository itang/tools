import lib.*
import tang.{ignore, time, |>, |>!}

import scala.language.unsafeNulls

object Main:
  def main(args: Array[String]): Unit = time {
    try {
      val dir = args.headOption.getOrElse(tang.panic_!("input dir"))
      dir
        |> getDenoJsonFiles |> (it => if it.isEmpty then tang.panic_!("未找到deno.json文件") else it)
        |> (_.foreach { path =>
          val justfileContent = convertToJustfile(path) |>! println
          val targetPath      = os.Path(path.toIO.getParentFile) / "justfile" |>! println

          os.write.over(targetPath, justfileContent)
        })
    } catch {
      case e: Exception => println(s"ERROR: ${e.getMessage} ${Option(e.getCause).map(_.getMessage).getOrElse("")}")
    }
  }.ignore()
