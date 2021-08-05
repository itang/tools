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

[<EntryPoint>]
let main argv =
    argv
    |> pathFromArgv [ @"D:\ProgramData\bin\jiayou.toml"
                      @"/home/itang/bin/jiayou.toml" ]
    |> fold (urlsFromTomlPath >> openUrls) (fun () -> printfn $"ERROR: path is invalid.")

    0 // return an integer exit code
