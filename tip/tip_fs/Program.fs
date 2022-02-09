open Args
open Util
open Service

[<Literal>]
let version = "0.1-20220208"

[<EntryPoint>]
let main argv =
    let parseResults = CliArguments.ParseArgs(argv)

    if parseResults.Contains Version then
        Console.Info $"V%s{version}"

        0
    else
        let dataDir = Tip.getDataDir ()
        Console.Info $"INFO: tip data dir: %s{dataDir}"

        let isListFlag = parseResults.Contains(CliArguments.List)

        if isListFlag then
            Tip.handleCommand <| Tip.ListTipsCommand dataDir
        else
            let name =
                parseResults.GetResult(CliArguments.NameCommand, [])
                |> List.tryHead

            match name with
            | Some name ->
                Tip.handleCommand
                <| Tip.DisplayTipCommand(dataDir, name)
            | None ->
                Console.Warn "Please input the tip name:"
                Tip.handleCommand <| Tip.ListTipsCommand dataDir
