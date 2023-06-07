package data

import types.{*, given}

def monthGroup = Group(
  "十二月份英文及英文缩写",
  List(
    Item("一月", "January", "Jan"),
    Item("二月", "February", "Feb"),
    Item("三月", "March", "Mar"),
    Item("四月", "April", "Apr"),
    Item("五月", "May", "May"),
    Item("六月", "June", "Jun"),
    Item("七月", "July", "Jul"),
    Item("八月", "August", "Aug"),
    Item("九月", "September", "Sep/Sept"),
    Item("十月", "October", "Oct"),
    Item("十一月", "November", "Nov"),
    Item("十二月", "December", "Dec")
  )
)

def weekGroup = Group(
  "星期",
  """|星期一 Monday
     |星期二 Tuesday
     |星期三 Wednesday
     |星期四 Thursday
     |星期五 Friday
     |星期六 Saturday
     |星期天 Sunday"""
)

def daysGroup = Group(
  "日期",
  """|昨天 Yesterday
     |今天 Today
     |后天 The day after tomorrow
     |前天 The day before yesterday
     |明天 Tomorrow"""
)

def quarterGroup = Group(
  "季度",
  """|春天 Spring
     |夏天 Summer
     |秋天 Autumn/Fall
     |冬天 Winter"""
)
