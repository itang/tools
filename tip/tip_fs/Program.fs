// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open Biz

[<EntryPoint>]
let main argv =
    let dataDir = GetDataDir()

    printfn $"INFO: tip data dir: %s{dataDir}"

    let isListFlag = (Set.ofList [ "--list"; "-l" ]).Contains

    if Array.exists isListFlag argv then
        ListTips dataDir |> Handle
    else
        DisplayTip(dataDir, argv) |> Handle
