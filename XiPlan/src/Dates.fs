module Dates

open System

type Dates(?start: string) =
    member this.GetSlice(lowBound: int option, upperBound: int option) =
        let startDate =
            match start with
            | Some it -> DateTime.Parse it
            | _ -> DateTime.Now

        match lowBound, upperBound with
        | None, None -> Seq.initInfinite startDate.AddDays
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
