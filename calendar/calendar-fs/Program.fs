open FSharp.SystemCommandLine

let mainHandle (days: option<int>) =
    Calendar.displayDay (days |> Option.defaultValue 10)

    0


[<EntryPoint>]
let main argv =
    let days = Input.OptionMaybe<int>([ "--days"; "-d" ], "display days")

    rootCommand argv {
        description "calendar something"
        inputs (days) // must be set before setHandler
        setHandler mainHandle
    }
