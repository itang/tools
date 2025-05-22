module Extensions

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
        | _ -> Exception("Unsupported day of week") |> raise

    member this.IsWeekend =
        match this with
        | DayOfWeek.Saturday
        | DayOfWeek.Sunday -> true
        | _ -> false


type DateTime with

    member this.Formated = this.ToString("yyyy-MM-dd")

    member this.FormatedLong = this.ToString("yyyy-MM-dd HH:mm:ss")


type String with

    member this.Indent(?num: int) =
        (String.replicate (defaultArg num 2) " ") + this

    member this.IndentLevel1 = this.Indent(2)

    member this.IndentLevel2 = this.Indent(4)


module Seq =
    let dates (starts: string) (ends: string) =
        let start_datetime, end_datetime = DateTime.Parse starts, DateTime.Parse ends

        start_datetime
        |> Seq.unfold (fun it ->
            if (it >= end_datetime) then
                None
            else
                Some((it, it.AddDays 1)))
