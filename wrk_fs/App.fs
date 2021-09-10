namespace wrkfs

module App =

    open System.Runtime.CompilerServices

    open Argu
    open wrkfs.Args
    open wrkfs.Wrk

    [<Literal>]
    let AppName = "wrk_fs"

    [<Literal>]
    let AppVersion = "0.1-20210908"

    //see https://stackoverflow.com/questions/1531580/extension-methods-for-specific-generic-types
    [<Extension>]
    type Extenions() =
        [<Extension>]
        static member inline toOptions(result: ParseResults<Arguments>) =
            { Connections = result.GetResult(Arguments.Connections, Options.Default.Connections)
              Threads = result.GetResult(Arguments.Threads, Options.Default.Threads)
              Duration = result.GetResult(Arguments.Duration, Options.Default.Duration)
              Url = result.GetResult(Arguments.Url, Options.Default.Url) }

    let run (argv: string []) =
        try
            let parser =
                ArgumentParser.Create<Arguments>(programName = AppName)

            let results =
                parser.ParseCommandLine(inputs = argv, raiseOnUsage = true)

            results.toOptions () |> Wrk.run

            0
        with
        | e ->
            printfn "%s" e.Message

            1
