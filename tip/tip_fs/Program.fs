open Args
open Util
open Tip

[<Literal>]
let version = "0.1-20220209"


[<EntryPoint>]
let main argv =
    let args = CliArguments.ParseArgs(argv)

    match args with
    | FVersion -> Logger.Info $"V%s{version}"
    | FListTips -> newTiper().ListTips()
    | Success _ ->
        let name =
            args.GetResult(CliArguments.NameCommand, [])
            |> List.tryHead

        match name with
        | Some name -> newTiper().DisplayTip(name)
        | None ->
            Logger.Warn "Please input the tip name:"
            newTiper().ListTips()


    0
