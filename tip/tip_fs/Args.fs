module Args

open System.Runtime.CompilerServices
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

    static member ParseArgs(args) : ParseResults<CliArguments> =
        let parser =
            ArgumentParser.Create<CliArguments>(programName = "tip_fs.exe")

        try
            parser.ParseCommandLine(inputs = args, ignoreMissing = true, raiseOnUsage = true, ignoreUnrecognized = true)
        with
        | e ->
            printfn "%s" e.Message
            exit 0

[<Extension>]
type ParseResultsForCliArgumentsExtensions =
    [<Extension>]
    static member inline Name(it: ParseResults<CliArguments>) : string option =
        it.GetResult(CliArguments.NameCommand, [])
        |> List.tryHead

let inline (|FVersion|FListTips|Success|) (args: ParseResults<CliArguments>) =
    if args.Contains Version then
        FVersion
    else if args.Contains ListTips then
        FListTips
    else
        Success args
