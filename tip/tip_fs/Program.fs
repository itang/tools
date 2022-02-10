open Args
open Util
open Service.Tip


[<Literal>]
let version = "0.1-20220209"

[<EntryPoint>]
let main argv =
    let args = CliArguments.ParseArgs(argv)

    if args.Contains Version then
        Logger.Info $"V%s{version}"
    else
        let dataDir = getDataDir ()
        Logger.Info $"INFO: tip data dir: %s{dataDir}"

        if args.Contains(ListTips) then
            listTips dataDir
        else
            let name =
                args.GetResult(CliArguments.NameCommand, [])
                |> List.tryHead

            match name with
            | Some name -> displayTip dataDir name
            | None ->
                Logger.Warn "Please input the tip name:"
                listTips dataDir

    0
