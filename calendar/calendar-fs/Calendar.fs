module Calendar

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

    member this.IsWeekend =
        match this with
        | DayOfWeek.Saturday
        | DayOfWeek.Sunday -> true
        | _ -> false

    member this.isLastDayOfWeek =
        match this with
        | DayOfWeek.Sunday -> true
        | _ -> false


type DateTime with
    member this.Formated = this.ToString("yyyy-MM-dd")
    member this.FormatedLong = this.ToString("yyyy-MM-dd HH:mm:ss")

let displayDay (days: int) =
    let startDate = DateTime.Now

    for i in 0 .. (days - 1) do
        let d = startDate.AddDays i
        printfn $"{i + 1}: {d.DayOfWeek.Formated} - {d.Formated}"

        if d.DayOfWeek.isLastDayOfWeek then
            printfn "-"
