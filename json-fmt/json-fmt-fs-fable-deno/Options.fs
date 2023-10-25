module Cli

open System

type Options =
    { File: option<string>
      Values: array<string> }

    static member Parse(argv: array<string>) =
        let mutable options = { File = None; Values = [||] }
        let iter = argv.GetEnumerator()

        while iter.MoveNext() do
            let arg = iter.Current |> string

            if arg = "-f" || arg = "--file" then
                if iter.MoveNext() then
                    options <-
                        { options with
                            File = Some(iter.Current :?> string) }
                else
                    raise <| ArgumentException("--file value is empty")

            if not (arg.StartsWith("-")) then
                options <-
                    { options with
                        Values = options.Values |> Array.append [| arg |] }

        options
