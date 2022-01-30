//> using scala "3.1.1"
// https://scala-cli.virtuslab.org/docs/reference/directives#compiler-options
//> using javaOpt "-Xmx2g", "-Dfile.encoding=utf8"

import java.nio.file.{Paths, Files}
import java.nio.charset.StandardCharsets
import java.io.File

import lib.util.*

object Main:
  def main(args: Array[String]): Unit =
    println("tip_scala-V0.1-20220130\n")

    args.headOption match
      case None | Some("--help") | Some("-h") =>
        println("Please input name!")
        println("-" * 80)
        displayFiles(dataDir())
      case Some(n) =>
        doMain(n)

  private def doMain(name: String): Unit =
    val d = dataDir()
    val f = d / s"$name.md"

    if File(f).exists then
      f |>! println
        |> (Paths.get(_))
        |> Files.readAllBytes
        |> (String(_, "UTF-8"))
        //|>! (Files.writeString(Paths.get("a.txt"), _, StandardCharsets.UTF_8))
        |> println
    else displayFiles(d)

  private def dataDir(): String =
    "TIP_DATA_ROOT".E or ("HOME".E / "bin" / "data" / "tip")

  private def displayFiles(d: String): Unit =
    println("$ tip --help")
    println("$ tip mvn")
    File(d).listFiles
      .sortBy(_.getName)
      .grouped(6)
      .foreach(fs =>
        fs.map(file => f"${file.getName.noFileExtension}%-16s")
          .mkString(" ")
          |> println
      )
