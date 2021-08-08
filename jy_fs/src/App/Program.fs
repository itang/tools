open JY

let VERSION = "0.1.1-20210808"

[<EntryPoint>]
let main argv =
    printfn $"jy_fs-v%s{VERSION}"

    argv
    |> Util.pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                           @"/home/itang/bin/jiayou.toml" ]
    |> Lang.fold (Util.urlsFromTomlPath >> Biz.openUrls) Biz.onNonePath
