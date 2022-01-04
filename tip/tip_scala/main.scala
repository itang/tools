//using scala 3.1.0
import java.nio.file.{Paths, Files}
import java.io.File

import lib.util.*

@main
def main(name: String): Unit =
  println("tip_scala-V0.1-20220103_01\n")

  val d = dataDir()
  val f = d / s"$name.md"

  if File(f).exists then
    f |>! println
      |> (Paths.get(_))
      |> Files.readAllBytes
      |> (String(_, "UTF-8"))
      |> println
  else displayFiles(d)

def dataDir(): String = "TIP_DATA_ROOT".E or ("HOME".E / "data" / "tip")

def displayFiles(d: String): Unit =
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
