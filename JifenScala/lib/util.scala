package lib.util

import java.time.*
import java.time.format.DateTimeFormatter

object Util:
  val zn =
    Array("零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二")
  extension (i: Int)
    def w: String = s"星期${if i == 7 then "日" else zn(i)}"
    def m: String = s"${zn(i)}月"
  extension (t: DayOfWeek) def f: String = t.getValue.w
  extension (m: Month) def f: String = m.getValue.m
  extension (d: LocalDate)
    def f: String = d.format(DateTimeFormatter.ISO_DATE)
    def firstDayOfMonth(startDate: LocalDate): Boolean =
      d.getDayOfMonth() == 1 || d == startDate
    def endOfMonth: LocalDate = d.withDayOfMonth(d.lengthOfMonth())
    def lastDayOfMongth: Boolean = d == endOfMonth
    def daysToEndMonth: Int = endOfMonth.getDayOfMonth - d.getDayOfMonth + 1
