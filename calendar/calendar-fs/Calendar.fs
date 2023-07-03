module Calendar

open System
open PrettyTable

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


type DateTime with
    member this.Formated = this.ToString("yyyy-MM-dd")

let displayDay (days: int) =
    let startDate = DateTime.Now
    let headers = [ "no"; "week_f"; "day_f" ]

    let rows =
        seq {
            for i in 0 .. (days - 1) do
                let day = startDate.AddDays i

                yield
                    [ $"{i + 1}"
                      day.DayOfWeek.Formated
                      day.Formated ]

                if day.DayOfWeek.isLastDayOfWeek then
                    yield [ ""; "-"; "-" ]
        }

    rows
    |> Seq.toList
    |> prettyTable
    |> withHeaders headers
    |> printTable
