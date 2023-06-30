import lib.{convertToJustfile, getDenoJsonFiles}
import tang.{ignore, time, |>, |>!}

object Main:
  def main(args: Array[String]): Unit = time {
    import scala.language.unsafeNulls
    try {
      args
        |> getDirFromArgs |> (_.getOrElse(tang.panic_!("input dir")))
        |> getDenoJsonFiles |>! (paths => if paths.isEmpty then tang.panic_!("未找到deno.json文件"))
        |> (_.foreach { path =>
          val justfileContent = convertToJustfile(path) |>! println
          val targetPath      = path / os.up / "justfile" |>! println

          justfileContent writeTo targetPath
        })
    } catch {
      case e: Exception => println(s"ERROR: ${e.getMessage} ${Option(e.getCause).map(_.getMessage).getOrElse("")}")
    }
  }.ignore()

  private def getDirFromArgs(args: Array[String]): Option[String] = args.headOption

  extension (s: String) inline private infix def writeTo(path: os.Path): Unit = os.write.over(path, s)
