open FSharp.SystemCommandLine
open Util
open Tip


let mainHandle (list, name) =
    match (list, name) with
    | (true, _) -> newTiper().ListTips()
    | (false, Some name) -> newTiper().DisplayTip(name)
    | (false, None) ->
        Logger.Warn "Please input the tip name:"
        newTiper().ListTips()


[<EntryPoint>]
let main argv =
    let list = Input.Option<bool>([ "--list"; "-l" ], "List tips")
    let name = Input.ArgumentMaybe<string>("The name for tip")

    rootCommand argv {
        description "tip something"
        inputs (list, name) // must be set before setHandler
        setHandler mainHandle
    }
