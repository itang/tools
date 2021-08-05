open Util

[<EntryPoint>]
let main argv =
    let path =
        pathFromArgv
            [ @"D:\ProgramData\bin\jiayou.toml"
              @"/home/itang/bin/jiayou.toml" ]
            argv

    match path with
    | Some (x) ->
        x
        |> urlsFromTomlPath
        |> Seq.iteri
            (fun index url ->
                printfn $"%d{index} open %s{url}"
                do openBrowser url)
    | _ -> printfn $"ERROR: path is invalid."

    0 // return an integer exit code
