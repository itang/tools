module Cli

open System
open Native

type Options =
    { File: option<string>
      Values: array<string> }

    static member Parse(argv: array<string>) =
        let arg1 = argv |> Array.tryHead |> Option.defaultValue ""

        match arg1 with
        | "-h"
        | "--help" ->
            printfn """help: jsonfmt-fs -f <File> ['<json string>']"""
            Deno.exit ()
        | "-v"
        | "--version" ->
            printfn "0.1"
            Deno.exit ()
        | _ -> ()

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
                        Values = Array.append (options.Values) [| arg |] }

        options
