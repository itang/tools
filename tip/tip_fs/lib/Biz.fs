module Biz

open System
open System.IO
open Util

let private handleListTips dir =
    let names =
        dir
        |> Directory.GetFiles
        |> Array.map Path.GetFileNameWithoutExtension

    for chunks in (names |> Array.chunkBySize 6) do
        for chunk in chunks do
            Console.ok ($"%-16s{chunk} ", newline = false)

        printfn ""

    0

let private handleDisplayTip dataDir argv =
    try
        match argv with
        | name :: _ ->
            name
            |> nameToPath dataDir
            |> readText
            |> Console.ok

            0
        | _ ->
            Console.warn "Please input the tip name:"
            handleListTips dataDir
    with
    | :? FileNotFoundException as ex ->
        Console.error $"ERROR: %s{ex.Message}"

        handleListTips dataDir

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
    | ListTipsCommand of dataDir: string
    | DisplayTipCommand of dataDir: string * argv: string list

let handleCommand =
    function
    | ListTipsCommand dataDir -> handleListTips dataDir
    | DisplayTipCommand (dataDir, argv) -> handleDisplayTip dataDir argv
