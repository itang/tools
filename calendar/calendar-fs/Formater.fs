module Formater

open System
open PrettyTable

open Calendar
open Common

type Formater =
    abstract member Format: ICalendar -> string

type TuiFormater() =
    interface Formater with
        member _.Format(cal: ICalendar) : string =

            let columns = [ "no"; "week_f"; "day_f" ]

            let rows =
                seq {
                    for (i, day) in (cal.Dates() |> Seq.toList |> List.indexed) do
                        yield [ $"{i + 1}"; day.DayOfWeek.Formated; day.Formated ]

                        if day.DayOfWeek.isLastDayOfWeek then
                            yield [ ""; "-"; "-" ]
                }

            rows |> Seq.toList |> prettyTable |> withHeaders columns |> printTable
            ""

type HtmlFormater() =
    interface Formater with
        member _.Format(cal: ICalendar) : string =
            let rows =
                seq {
                    for (i, day) in (cal.Dates() |> Seq.toList |> List.indexed) do
                        yield $"<tr><td>{i + 1}</td><td>{day.DayOfWeek.Formated}</td><td>{day.Formated}</td>"

                        if day.DayOfWeek.isLastDayOfWeek then
                            yield $"""<tr><td colspan="3">-</td></tr>"""
                }

            let html = $"""<table>{String.concat "" rows}</table>"""
            IO.File.WriteAllText("c.html", html)
            printfn "%s" html
            html

type TaskFormater() =
    interface Formater with
        member _.Format(cal: ICalendar) : string =
            let columns = [ "日期"; "星期"; "工作项"; "工时"; "备注" ]
            let rowNum = columns |> List.length

            let rows =
                seq {
                    for (i, day) in (cal.Dates() |> Seq.toList |> List.indexed) do
                        let rowspan = if day.DayOfWeek.isWeekend then 2 else 4

                        yield
                            $"""<tr><td rowspan="{rowspan}">{day.Formated}</td><td rowspan="{rowspan}">{day.DayOfWeek.Formated}</td><td></td><td></td>"""

                        for _ in 1 .. (rowspan - 1) do
                            yield $"""<tr><td></td><td></td><td></td>"""

                        if day.DayOfWeek.isLastDayOfWeek then
                            yield $"""<tr><td colspan="{rowNum}">-</td></tr>"""
                }

            let header = seq { for c in columns -> $"<td>{c}</td>" } |> String.concat ""
            let columns = seq { for c in columns -> $"<td>{c}</td>" } |> String.concat ""

            let html =
                $"""<style>
table, th, td {{
  border: 1px solid black;
  border-collapse: collapse;
}}
</style>
<table>
  <thead><tr>{columns}</tr></thead>
  <body>{String.concat "" rows}<body>
</table>
"""

            IO.File.WriteAllText("t.html", html)
            printfn "%s" html
            html