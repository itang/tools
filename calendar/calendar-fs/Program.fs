open FSharp.SystemCommandLine
open System

open Calendar
open Formater

let mainHandle (days: option<int>, format: option<string>) =
    let days = Option.defaultValue 10 days

    try
        let formater: Formater =
            match format with
            | Some "html" -> HtmlFormater()
            | Some "task" -> TaskFormater()
            | Some "tui" -> TuiFormater()
            | Some v -> failwithf "unknown format: %s" v
            | _ -> TuiFormater()

        let calendar = Calendar(DateTime.Now, days)

        formater.Format(calendar) |> ignore

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
