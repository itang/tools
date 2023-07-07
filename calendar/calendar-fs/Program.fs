open FSharp.SystemCommandLine
open System

open Calendar.Api
open Calendar.Impl
open Formatter.Api
open Formatter.Impl
open Persistence.Api
open Persistence.Impl

let mainHandle (days: option<int>, format: option<string>) =
    let days = Option.defaultValue 10 days

    try
        let formatter: IFormatter<string> =
            match format with
            | Some "html" -> HtmlFormatter()
            | Some "task" -> TaskFormatter()
            | Some "tui" -> TuiFormatter()
            | Some v -> failwithf "unknown format: %s" v
            | _ -> TuiFormatter()

        let calendar: ICalendar = Calendar(DateTime.Now, days)

        let content = formatter.Format calendar

        printfn "%s" content

        if formatter.Name <> "tui" then
            let persistence: IPersistence = FilePersistence()
            persistence.Save formatter.Name content

        0
    with e ->
        printfn "[ERROR] %s" e.Message
        -1

[<EntryPoint>]
let main argv =
    let days = Input.OptionMaybe<int>([ "--days"; "-d" ], "display days")

    let format =
        Input.OptionMaybe<string>([ "--format"; "-f" ], "format: tui | html | task")

    rootCommand argv {
        description "calendar something"
        inputs (days, format) // must be set before setHandler
        setHandler mainHandle
    }
