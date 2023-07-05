open FSharp.SystemCommandLine
open System

open Calendar.Api
open Calendar.Impl
open Formater.Api
open Formater.Impl
open Persistence.Api
open Persistence.Impl

let mainHandle (days: option<int>, format: option<string>) =
    let days = Option.defaultValue 10 days

    try
        let formater: IFormater<string> =
            match format with
            | Some "html" -> HtmlFormater()
            | Some "task" -> TaskFormater()
            | Some "tui" -> TuiFormater()
            | Some v -> failwithf "unknown format: %s" v
            | _ -> TuiFormater()

        let calendar: ICalendar = Calendar(DateTime.Now, days)

        let content = formater.Format calendar

        let persistence: IPersistence = FilePersistence()

        persistence.Save formater.Name content

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
