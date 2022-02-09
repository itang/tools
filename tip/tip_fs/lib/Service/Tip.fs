namespace Service

module Tip =

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
                Console.Ok($"%-16s{chunk} ", newline = false)

            printfn ""

        0

    let private handleDisplayTip dataDir name =
        try
            name
            |> nameToPath dataDir
            |> readText
            |> Console.Ok

            0
        with
        | :? FileNotFoundException as ex ->
            Console.Error $"ERROR: %s{ex.Message}"

            handleListTips dataDir

    let getDataDir () =
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
        | DisplayTipCommand of dataDir: string * name: string

    let handleCommand =
        function
        | ListTipsCommand dataDir -> handleListTips dataDir
        | DisplayTipCommand (dataDir, name) -> handleDisplayTip dataDir name
