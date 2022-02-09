module Args

open Argu

type CliArguments =
    | [<AltCommandLine("-v")>] Version
    | [<AltCommandLine("-l")>] ListTips
    | [<MainCommand; Last>] NameCommand of name: string list

    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Version -> "Show Version"
            | ListTips -> "List tips"
            | NameCommand _ -> "The name for tip"

    static member ParseArgs(args) =
        let parser =
            ArgumentParser.Create<CliArguments>(programName = "tip_fs.exe")

        try
            parser.ParseCommandLine(inputs = args, ignoreMissing = true, raiseOnUsage = true, ignoreUnrecognized = true)
        with
        | e ->
            printfn "%s" e.Message
            exit 0
