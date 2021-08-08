open JY
open JY.Lang

let VERSION = "0.1.2-20210808"

[<EntryPoint>]
let main argv =
    printfn $"jy_fs-v%s{VERSION}"

    argv
    |> Util.pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                           @"/home/itang/bin/jiayou.toml" ]
    |> OptionExt.fold (Util.urlsFromTomlPath >> Biz.openUrls) Biz.onNonePath
