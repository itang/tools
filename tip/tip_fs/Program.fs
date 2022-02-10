open Args
open Util
open Tip

[<Literal>]
let version = "0.2-20220210"

[<EntryPoint>]
let main argv =
    match CliArguments.ParseArgs(argv) with
    | FVersion -> Logger.Info $"V%s{version}"
    | FListTips -> newTiper().ListTips()
    | Success args ->
        match args.Name() with
        | Some name -> newTiper().DisplayTip(name)
        | None ->
            Logger.Warn "Please input the tip name:"
            newTiper().ListTips()

    0
