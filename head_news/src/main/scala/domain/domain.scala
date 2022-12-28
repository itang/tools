package domain

import org.scalajs.dom.fetch

import scala.concurrent.{ExecutionContext, Future}
import scala.scalajs.js

case class HeadNews(title: String, href: String)

trait HeadNewsProvider:
  def name: String
  def fetchHeadNews(using ctx: ExecutionContext): Future[Option[HeadNews]]

object SinaNews extends HeadNewsProvider {

  private val url = "https://news.sina.com.cn/"

  private val startTag = """<h1 data-client="headline">"""

  private val R = """.+href="(.+)"\sclass.+>(.+)""".r

  override val name = "新浪新闻"

  override def fetchHeadNews(using
      ctx: ExecutionContext
  ): Future[Option[HeadNews]] = {
    import js.Thenable.Implicits.*
    for
      resp <- fetch(url)
      text <- resp.text()
    yield
      val start = text.indexOf(startTag) + startTag.length()
      val end = text.indexOf("</a>", start)
      val line = text.substring(start, end).trim()

      line match
        case R(href, title) => Some(HeadNews(title, href))
        case _              => None
  }
  /*
  val s =
    """<h1 data-client="headline"><a target="_blank" href="https://news.sina.com.cn/c/xl/2022-11-16/doc-imqmmthc4760493.shtml" class="linkNewsTopBold">G2峰会第一天 习主席重点谈了什么？</a></h1>"""
   */
}
