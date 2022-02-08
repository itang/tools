// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open Argu

open Biz
open Util

type CliArguments =
    | [<AltCommandLine("-v")>] Version
    | [<AltCommandLine("-l")>] List
    | [<MainCommand; Last>] NameCommand of names: string list

    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Version -> " Version"
            | List -> "List tips"
            | NameCommand _ -> "The name for tip"

let private parseArgs args =
    let parser =
        ArgumentParser.Create<CliArguments>(programName = "fff.exe")

    try
        parser.ParseCommandLine(inputs = args, ignoreMissing = true, raiseOnUsage = true, ignoreUnrecognized = true)
    with
    | e ->
        printfn "%s" e.Message
        exit 0

[<Literal>]
let version = "0.1-20220208"

[<EntryPoint>]
let main argv =
    let parseResults = parseArgs argv

    if parseResults.Contains Version then
        Console.info $"V%s{version}"

        0
    else
        let dataDir = GetDataDir()

        Console.info $"INFO: tip data dir: %s{dataDir}"

        let isListFlag = parseResults.Contains(CliArguments.List)

        if isListFlag then
            ListTips dataDir |> Handle
        else
            let argv =
                parseResults.GetResult(CliArguments.NameCommand, [])

            DisplayTip(dataDir, argv) |> Handle
