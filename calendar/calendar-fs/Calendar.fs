module Calendar

open System
open PrettyTable

type Format =
    | HtmlView
    | TuiView
    | TaskView

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

let headers = [ "no"; "week_f"; "day_f" ]

let private displayDayTui (dates: seq<DateTime>) =
    let rows =
        seq {
            for (i, day) in (dates |> Seq.toList |> List.indexed) do
                yield [ $"{i + 1}"; day.DayOfWeek.Formated; day.Formated ]

                if day.DayOfWeek.isLastDayOfWeek then
                    yield [ ""; "-"; "-" ]
        }

    rows |> Seq.toList |> prettyTable |> withHeaders headers |> printTable

let private displayDayHtml (dates: seq<DateTime>) =
    let rows =
        seq {
            for (i, day) in (dates |> Seq.toList |> List.indexed) do
                yield $"<tr><td>{i + 1}</td><td>{day.DayOfWeek.Formated}</td><td>{day.Formated}</td>"

                if day.DayOfWeek.isLastDayOfWeek then
                    yield $"""<tr><td colspan="3">-</td></tr>"""
        }

    let html = $"""<table>{rows |> String.concat ""}</table>"""
    IO.File.WriteAllText("c.html", html)
    html |> printfn "%s"

let private displayDayTask (dates: seq<DateTime>) =
    let headers = [ "日期"; "星期"; "工作项"; "工时"; "备注" ]
    let rowNum = headers |> List.length

    let rows =
        seq {
            for (i, day) in (dates |> Seq.toList |> List.indexed) do
                let rowspan = if day.DayOfWeek.isWeekend then 2 else 4

                yield
                    $"""<tr><td rowspan="{rowspan}">{day.Formated}</td><td rowspan="{rowspan}">{day.DayOfWeek.Formated}</td><td></td><td></td>"""

                for _ in 1 .. (rowspan - 1) do
                    yield $"""<tr><td></td><td></td><td></td>"""

                if day.DayOfWeek.isLastDayOfWeek then
                    yield $"""<tr><td colspan="{rowNum}">-</td></tr>"""
        }

    let columns = seq { for c in headers -> $"<td>{c}</td>" } |> String.concat ""

    let html =
        $"""
<style>
table, th, td {{
  border: 1px solid black;
  border-collapse: collapse;
}}
</style>
<table>
  <tr>{columns}</tr>
  <body>{rows |> String.concat ""}<body>
</table>
"""

    IO.File.WriteAllText("t.html", html)
    html |> printfn "%s"

let displayDay (days: int) (format: Format) =
    let startDate = DateTime.Now
    let dates = startDate.Dates(days)

    let handle =
        match format with
        | TuiView -> displayDayTui
        | HtmlView -> displayDayHtml
        | TaskView -> displayDayTask

    handle dates
