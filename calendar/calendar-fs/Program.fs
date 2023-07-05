open FSharp.SystemCommandLine
open Calendar

let mainHandle (days: option<int>, format: option<string>) =
    let days = Option.defaultValue 10 days

    try
        let format =
            match format with
            | Some "html" -> HtmlView
            | Some "task" -> TaskView
            | Some "tui" -> TuiView
            | Some v -> failwithf "unknown format: %s" v
            | _ -> TuiView

        Calendar.displayDay days format

        0
    with e ->
        printfn "ERROR: %s" e.Message
        -1

[<EntryPoint>]
let main argv =
    let days = Input.OptionMaybe<int>([ "--days"; "-d" ], "display days")
    let format = Input.OptionMaybe<string>([ "--format"; "-f" ], "format")

    rootCommand argv {
        description "calendar something"
        inputs (days, format) // must be set before setHandler
        setHandler mainHandle
    }
