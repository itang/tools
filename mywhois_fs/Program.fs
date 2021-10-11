open app

[<EntryPoint>]
let main args =
    if Array.isEmpty args then
        printfn "please input keys."
    else
        args |> Array.toList |> MyWhoisApp.main

    0
