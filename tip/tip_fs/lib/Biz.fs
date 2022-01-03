module Biz

open System
open System.IO
open Util

let private handleListTips dir =
    let names =
        dir
        |> Directory.GetFiles
        |> Array.map Path.GetFileNameWithoutExtension

    for chunks in (names |> Array.chunkBySize 10) do
        for chunk in chunks do
            printf $"%-16s{chunk} "

        printfn ""

    0

let private handleDisplayTip dataDir argv =
    try
        argv
        |> tipNameFromArgv "tip"
        |> nameToPath dataDir
        |> readText
        |> printfn "%s"

        0
    with
    | :? FileNotFoundException as ex ->
        printfn $"ERROR: %s{ex.Message}"

        1

let GetDataDir () =
    let dataDir =
        Environment.GetEnvironmentVariable "TIP_DATA_ROOT"

    if String.IsNullOrEmpty(dataDir) then
        let homeDir =
            Environment.GetEnvironmentVariable "HOME"

        Path.Combine(homeDir, "bin", "data", "tip")
    else
        dataDir


type BizCommand =
    | ListTips of dataDir: string
    | DisplayTip of dataDir: string * argv: string []

let Handle =
    function
    | ListTips dataDir -> handleListTips dataDir
    | DisplayTip (dataDir, argv) -> handleDisplayTip dataDir argv
