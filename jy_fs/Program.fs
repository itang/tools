open Util


let fold onSome onNone opt =
    match opt with
    | Some it -> onSome it
    | None -> onNone ()

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

[<EntryPoint>]
let main argv =
    argv
    |> pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                      @"/home/itang/bin/jiayou.toml" ]
    |> fold (urlsFromTomlPath >> openUrls) onNonePath
