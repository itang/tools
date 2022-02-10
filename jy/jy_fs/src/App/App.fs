module App

open Argu

open JY
open JY.Lang
open Args

[<Literal>]
let Version = "0.2.0-20220210"

[<Literal>]
let Name = "jy_fs"

let private openUrls mode configUrls =
    configUrls
    |> Util.pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                           @"/home/itang/bin/jiayou.toml" ]
    |> OptionExt.fold (Util.urlsFromTomlPath >> (Biz.openUrls mode)) Biz.onNonePath

let run (argv: string array) =
    let parser =
        ArgumentParser.Create<Arguments>(programName = Name, errorHandler = errorHandler)

    let results = parser.ParseCommandLine argv
    printfn $"DEBUG: arguments=%A{results}"

    let mode =
        if results.Contains Sequence then
            Biz.OpenMode.Sequence
        else
            Biz.OpenMode.Parallel

    let configUrls =
        results.GetResult(ConfigPath, defaultValue = [])
        |> List.toArray

    openUrls mode configUrls
