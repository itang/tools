module Tip

open System
open System.IO
open Util


type Tiper(dataDir) =
    let nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

    member _.ListTips() =
        let names =
            dataDir
            |> Directory.GetFiles
            |> Array.map Path.GetFileNameWithoutExtension

        for chunks in (names |> Array.chunkBySize 6) do
            for chunk in chunks do
                Logger.Ok($"%-16s{chunk} ", newline = false)

        printfn ""

    member self.DisplayTip(name) =
        try
            name
            |> nameToPath dataDir
            |> File.ReadAllText
            |> Logger.Ok
        with
        | :? FileNotFoundException as ex ->
            Logger.Error $"ERROR: %s{ex.Message}"
            self.ListTips()


    static member GetDataDir() =
        let dataDir =
            Environment.GetEnvironmentVariable "TIP_DATA_ROOT"

        if String.IsNullOrEmpty(dataDir) then
            let homeDir =
                Environment.GetEnvironmentVariable "HOME"

            Path.Combine(homeDir, "bin", "data", "tip")
        else
            dataDir


[<AutoOpen>]
module Factory =
    let newTiper () =
        let dataDir = Tiper.GetDataDir()
        Logger.Info $"INFO: tip data dir: %s{dataDir}"

        new Tiper(dataDir)
