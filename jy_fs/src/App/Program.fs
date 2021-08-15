open System
open Argu

open JY
open JY.Lang


let VERSION = "0.2.0-20210815"

type Arguments =
    | [<AltCommandLine("-p")>] Parallel
    | [<AltCommandLine("-s")>] Sequence
    | [<MainCommand; Unique>] ConfigPath of configPath: string list 

    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Parallel _ -> "open urls in parallel mode."
            | Sequence _ -> "open urls in sequence mode."
            | ConfigPath _ -> "config path"

let errorHandler =
    ProcessExiter(
        colorizer =
            function
            | ErrorCode.HelpText -> None
            | _ -> Some ConsoleColor.Red
    )

let private openUrls mode configUrls =
    configUrls
    |> Util.pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                           @"/home/itang/bin/jiayou.toml" ]
    |> OptionExt.fold (Util.urlsFromTomlPath >> (Biz.openUrls mode)) Biz.onNonePath

[<EntryPoint>]
let main argv =
    printfn $"jy_fs-v%s{VERSION}"

    let parser =
        ArgumentParser.Create<Arguments>(programName = "jy_fs", errorHandler = errorHandler)
    //parser.PrintUsage() |> printfn "%s"

    let results = parser.ParseCommandLine argv
    printfn $"DEBUG: %A{results}"

    let mode =
        if results.Contains Sequence then
            Biz.OpenMode.Sequence
        else
            Biz.OpenMode.Parallel

    let configUrls = results.GetResult(ConfigPath, defaultValue = []) |> List.toArray 

    openUrls mode configUrls
