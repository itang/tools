open Util

[<EntryPoint>]
let main argv =
    argv
    |> pathFromArgv @"D:\ProgramData\bin\jiayou.toml"
    |> urlsFromTomlPath
    |> Seq.iteri
        (fun index url ->
            printfn $"%d{index} open %s{url}"
            openBrowser url)

    0 // return an integer exit code
