module Tip

open System
open System.IO
open System.Runtime.InteropServices
open Util

type Tiper(dataDir: string) =
    let nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

    member _.ListTips([<Optional; DefaultParameterValue(6)>] lineTipsNum: int) : unit =
        let names =
            dataDir
            |> Directory.GetFiles
            |> Array.map Path.GetFileNameWithoutExtension

        for chunks in (names |> Array.chunkBySize lineTipsNum) do
            for chunk in chunks do
                Logger.Ok($"%-16s{chunk} ", newline = false)

            printfn ""

    member this.DisplayTip(name: string) : unit =
        try
            name
            |> nameToPath dataDir
            |> File.ReadAllText
            |> Logger.Ok
        with
        | :? FileNotFoundException as ex ->
            Logger.Error $"ERROR: %s{ex.Message}"
            this.ListTips()

    static member GetDataDir() : string =
        let dataDir = Environment.GetEnvironmentVariable "TIP_DATA_ROOT"

        if String.IsNullOrEmpty(dataDir) then
            let homeDir = Environment.GetEnvironmentVariable "HOME"

            Path.Combine(homeDir, "bin", "data", "tip")
        else
            dataDir

[<AutoOpen>]
module Factory =
    let newTiper () : Tiper =
        let dataDir = Tiper.GetDataDir()
        Logger.Info $"INFO: tip data dir: %s{dataDir}"

        new Tiper(dataDir)
