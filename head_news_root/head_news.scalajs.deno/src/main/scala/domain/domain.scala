package domain

import org.scalajs.dom

import scala.concurrent.{ExecutionContext, Future}
import scala.scalajs.js

case class HeadNews(title: String, href: String)

trait HeadNewsProvider:
    def name: String

    def fetchHeadNews(using ctx: ExecutionContext): Future[Option[HeadNews]]

object SinaNews extends HeadNewsProvider:

    private val url = "https://news.sina.com.cn/"

    private val startTag = """<h1 data-client="headline">"""

    private val R = """.+href="(.+)"\sclass.+>(.+)""".r

    override val name: String = "新浪新闻"

    override def fetchHeadNews(using ctx: ExecutionContext): Future[Option[HeadNews]] =
        import js.Thenable.Implicits.*
        for
            resp <- dom.fetch(url)
            text <- resp.text()
        yield
            // println(text)
            val start = text.indexOf(startTag) + startTag.length()
            val end   = text.indexOf("</a>", start)
            val line  = text.substring(start, end).trim().replaceAll("\n", "").replaceAll("\r", "")

            line match
                case R(href, title) => Some(HeadNews(title, href))
                case _              => None
        end for
    end fetchHeadNews
end SinaNews
