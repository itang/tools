module Args

open System
open Argu

type Arguments =
    | [<AltCommandLine("-p")>] Parallel
    | [<AltCommandLine("-s")>] Sequence
    | [<MainCommand; Unique>] ConfigPath of configPath: string list

    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Parallel _ -> "open urls in parallel mode."
            | Sequence _ -> "open urls in sequence mode."
            | ConfigPath _ -> "config path"

let errorHandler =
    ProcessExiter(
        colorizer =
            function
            | ErrorCode.HelpText -> None
            | _ -> Some ConsoleColor.Red
    )
