package lib

import scala.language.unsafeNulls

import upickle.default.*

import tang.{|>, ignore}

case class DenoConfig(tasks: Map[String, String]) derives ReadWriter

def getDenoJsonFiles(dir: String): List[os.Path] = {
  val path = os.Path(java.io.File(dir).getAbsoluteFile)
  os
    .walk(path)
    .filter { path =>
      val file = path.toIO
      file.isFile && file.getName.startsWith("deno.json")
    }
    .toList
}

object convertToJustfile {
  def apply(path: os.Path): String = path |> toJsonString |> jsonToJustfile

  private def jsonToJustfile(json: String): String = {
    val denoConfig = read[DenoConfig](json)
    val j = denoConfig.tasks
      .map { case (k, v) =>
        val key   = k.replaceAll(":", "_")
        val value = v.replaceAll("&&", ";")
        s"$key:\n  $value\n"
      }
      .mkString("\n")

    s"""|set shell := ["nu", "-c"]
        |
        |default:
        |  just --list
        |
        |$j
        |""".stripMargin
  }

  private def toJsonString(path: os.Path): String = {
    if path.toIO.getName.endsWith(".json") then os.read(path)
    else {
      val iter = os
        .read(path)
        .linesIterator
        .map(_.trim)
        .filter(!_.isBlank)
        .filter(it => !(it.startsWith("//") || it.startsWith("#")))
        .toList
        .iterator

      val sb = StringBuilder()
      while iter.hasNext do
        val curr = iter.next()
        if iter.hasNext then
          val next = iter.next()
          if next == "}" then sb.append(curr.replace(",", "")).ignore()
          else sb.append(curr).ignore()
          sb.append("\n").append(next).append('\n').ignore()
        else sb.append(curr).append('\n').ignore()

      sb.toString()
    }
  }
}
