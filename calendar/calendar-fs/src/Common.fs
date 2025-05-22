module Common

open System

type DayOfWeek with

    member this.Formated =
        match this with
        | DayOfWeek.Monday -> "星期一"
        | DayOfWeek.Tuesday -> "星期二"
        | DayOfWeek.Wednesday -> "星期三"
        | DayOfWeek.Thursday -> "星期四"
        | DayOfWeek.Friday -> "星期五"
        | DayOfWeek.Saturday -> "星期六"
        | DayOfWeek.Sunday -> "星期日"
        | _ -> "Unknown"

    member this.isLastDayOfWeek =
        match this with
        | DayOfWeek.Sunday -> true
        | _ -> false

    member this.isWeekend =
        match this with
        | DayOfWeek.Sunday -> true
        | DayOfWeek.Saturday -> true
        | _ -> false

type DateTime with

    member this.Formated = this.ToString("yyyy-MM-dd")

    member this.Dates(days: int) : seq<DateTime> =
        seq { for i in 0 .. (days - 1) -> this.AddDays(i) }
