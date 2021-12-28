package lib.html

import java.time.*
import java.time.format.DateTimeFormatter

import lib.util.Util.*

class JifenHtmlGen(startDate: LocalDate, endDate: LocalDate):
  val columns = List("#", "月份", "星期", "日期", "任务完成情况", "自评得分", "审核得分", "备注")

  def html(): String =
    def _dates: LazyList[LocalDate] =
      def loop(start: LocalDate): LazyList[LocalDate] =
        start #:: loop(start.plusDays(1))
      loop(startDate)
    val theDates = _dates.takeWhile(it => !it.isAfter(endDate))
    s"""<!-- create time: ${LocalDateTime.now.format(
      DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss")
    )} -->
<!doctype html>
 <html lang="en">
 <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
    <style>
    * {
    font-size: 11px;
    line-height: 1;
    }
    </style>
 </head>
 <body>
  <div class="container">
  <table class="table table-bordered table-sm caption-top">
    <caption>晨曦积分计划 ${startDate.f}~${endDate.f}</caption>
    <thead>
      <tr>
      ${(for column <- columns
    yield s"<td align=\"center\" valign='middle'>${column}</td>")
      .mkString("\n")}
      </tr>
    </thead>
  <tbody>
  ${(for (date, index) <- theDates.zipWithIndex
    yield htmlRows(date, index)).mkString("\n")}
 </tbody></table>
</div>
</body>
</html>
"""

  private def htmlRows(date: LocalDate, index: Int): String =
    s"""<tr height='42px' valign='middle'>
      <td width='30px' align='center'>${index + 1}</td>
      ${if date.firstDayOfMonth(startDate) then
      s"<td width='60px' align='center' rowspan='${date.daysToEndMonth}'>${date.getMonth.f}</td>"
    else ""}
      <td width='60px' align='center'>${date.getDayOfWeek.f}</td>
      <td width='100px' align='center'>${date.f}</td>
      <td></td>
      <td width='40px'></td>
      <td width='40px'></td>
      <td width='80px'></td>
      </tr>
      ${if date.lastDayOfMongth then """<tr height='28px'><td colspan='5'></td>
      <td align="right">月结:</td>
      <td></td>
      <td></td>
      </tr>"""
    else ""}"""
