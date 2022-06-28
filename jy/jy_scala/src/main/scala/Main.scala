import scala.jdk.CollectionConverters.*
import utils.*
import com.moandjiezana.toml.Toml

extension (url: String) def open(): Unit = java.awt.Desktop.getDesktop.browse(java.net.URI(url))
extension (toml: Toml) def getStringList(key: String): List[String] = toml.getList[String](key).asScala.toList

object Main:
  val DefaultFile = "D:\\ProgramData\\jy_rs\\jiayou.toml"
  def main(args: Array[String]): Unit =
    val toml = args.headOption.getOrElse(DefaultFile) |> (java.io.File(_)) |> Toml().read

    val urls: List[String] = toml.getStringList("urls")

    for (url, index) <- urls.zipWithIndex do
      println(s"$index: $url")
      url.open()
