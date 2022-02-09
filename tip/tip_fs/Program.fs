open Args
open Util
open Service

[<Literal>]
let version = "0.1-20220209"

[<EntryPoint>]
let main argv =
    let parseResults = CliArguments.ParseArgs(argv)

    if parseResults.Contains Version then
        Logger.Info $"V%s{version}"
    else
        let dataDir = Tip.getDataDir ()
        Logger.Info $"INFO: tip data dir: %s{dataDir}"

        let isListFlag = parseResults.Contains(CliArguments.List)

        if isListFlag then
            Tip.handleListTips dataDir
        else
            let name =
                parseResults.GetResult(CliArguments.NameCommand, [])
                |> List.tryHead

            match name with
            | Some name -> Tip.handleDisplayTip dataDir name
            | None ->
                Logger.Warn "Please input the tip name:"
                Tip.handleListTips dataDir

    0
