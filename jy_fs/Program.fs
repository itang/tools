open Util
open Lang

let openUrls urls =
    Seq.iteri
        (fun index url ->
            printfn $"%d{index} open %s{url}"
            do openBrowser url)
        urls

    0

let onNonePath () =
    printfn $"ERROR: path is invalid."

    1

let VERSION = "0.1.0-20210808"

[<EntryPoint>]
let main argv =
    printfn $"jy_fs-v%s{VERSION}"

    argv
    |> pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                      @"/home/itang/bin/jiayou.toml" ]
    |> fold (urlsFromTomlPath >> openUrls) onNonePath
