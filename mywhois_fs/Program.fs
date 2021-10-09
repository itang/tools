[<EntryPoint>]
let main args =
    if args |> Array.isEmpty then
        printfn "INPUT keys"
    else
        App.doMain args

    0
