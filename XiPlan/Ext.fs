module Ext

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

type DateTime with
    member this.Formated = this.ToString("yyyy-MM-dd")
    member this.FormatedLong = this.ToString("yyyy-MM-dd HH:mm:ss")

type String with
    member this.Indent(?num: int) =
        (String.replicate (defaultArg num 2) " ") + this

    member this.I1 = this.Indent(2)
    member this.I2 = this.Indent(4)

module Seq =
    let dates (starts: string) (ends: string) =
        let s, e =
            DateTime.Parse starts, DateTime.Parse ends

        s
        |> Seq.unfold
            (fun s ->
                if (s >= e) then
                    None
                else
                    Some((s, s.AddDays 1)))

//let zipWithIndex (s: 't seq) = Seq.zip s (Seq.initInfinite id)

type Dates(?start: string) =
    member this.GetSlice(lowBound: int option, upperBound: int option) =
        let startDate =
            match start with
            | Some it -> DateTime.Parse it
            | _ -> DateTime.Now

        match lowBound, upperBound with
        | None, None -> Seq.initInfinite (fun index -> startDate.AddDays(index))
        | None, Some upper ->
            seq {
                for index in 0 .. (upper - 1) do
                    yield startDate.AddDays(index)
            }
        | Some low, Some upper ->
            seq {
                for index in low .. (upper - 1) do
                    yield startDate.AddDays(index)
            }
        | Some low, None -> Seq.initInfinite (fun index -> startDate.AddDays(index + low |> float))
