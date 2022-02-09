namespace Service

module Tip =

    open System
    open System.IO
    open Util

    let private nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

    let handleListTips dir =
        let names =
            dir
            |> Directory.GetFiles
            |> Array.map Path.GetFileNameWithoutExtension

        for chunks in (names |> Array.chunkBySize 6) do
            for chunk in chunks do
                Logger.Ok($"%-16s{chunk} ", newline = false)

            printfn ""

    let handleDisplayTip dataDir name =
        try
            name
            |> nameToPath dataDir
            |> File.ReadAllText
            |> Logger.Ok
        with
        | :? FileNotFoundException as ex ->
            Logger.Error $"ERROR: %s{ex.Message}"
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
