import tang.*
import scala.language.unsafeNulls

val ROOT_DIR = "D:/workspace/donew_rs/templates"

import upickle.default.*

case class DenoConfig(tasks: Map[String, String])

object DenoConfig:
  given rw: ReadWriter[DenoConfig] = macroRW

object Main:
  def main(args: Array[String]): Unit = time {
    val path = os.Path(ROOT_DIR)
    os.walk(path).foreach: path =>
      val file = path.toIO
      val name = file.getName
      if file.isFile && name.startsWith("deno.json") then
        println(path)

        try
          val justfileContent = convertToJustfile(file) |>! println
          os.write.over(os.Path(file.getParentFile) / "justfile", justfileContent)
        catch
          case e: Exception => e.printStackTrace()

  }.ignore()


def convertToJustfile(file: java.io.File): String =
  val sb = StringBuilder()
  val iter = os.read(os.Path(file)).linesIterator
    .map(_.trim)
    .filter(!_.isBlank)
    .filter(it => !(it.startsWith("//") || it.startsWith("#")))
    .toList.iterator


  while iter.hasNext do
    val curr = iter.next()
    if iter.hasNext then
      val next = iter.next()
      if next == "}" then
        sb.append(curr.replace(",", "")).ignore()
      else
        sb.append(curr).ignore()
      sb.append("\n").append(next).append('\n').ignore()
    else
      sb.append(curr).append('\n').ignore()


  sb |>! println

  println("-" * 100)

  val denoConfig = read[DenoConfig](sb.toString())
  denoConfig.toString
  val j = denoConfig.tasks.map { case (k, v) =>
    val key = k.replaceAll(":", "_")
    val value = v.replaceAll("&&", ";")
    s"$key:\n  $value\n"
  }.mkString("\n")

  s"""|set shell := ["nu", "-c"]
      |
      |default:
      |  just --list
      |
      |$j
      |""".stripMargin

