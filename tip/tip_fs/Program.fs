open Args
open Util
open Tip

[<Literal>]
let version = "0.1-20220209"

[<EntryPoint>]
let main argv =
    match CliArguments.ParseArgs(argv) with
    | FVersion -> Logger.Info $"V%s{version}"
    | FListTips -> newTiper().ListTips()
    | Success args ->
        let name =
            args.GetResult(CliArguments.NameCommand, [])
            |> List.tryHead

        match name with
        | Some name -> newTiper().DisplayTip(name)
        | None ->
            Logger.Warn "Please input the tip name:"
            newTiper().ListTips()

    0
