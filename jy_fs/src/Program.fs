open Util
open Lang

let VERSION = "0.1.1-20210808"

[<EntryPoint>]
let main argv =
    printfn $"jy_fs-v%s{VERSION}"

    argv
    |> pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                      @"/home/itang/bin/jiayou.toml" ]
    |> fold (urlsFromTomlPath >> JY.openUrls) JY.onNonePath