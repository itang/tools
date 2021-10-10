[<EntryPoint>]
let main args =
    if args |> Array.isEmpty then
        printfn "please input keys."
    else
        App.doMain args

    0
