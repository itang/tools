open System

open Cli
open Native

let error_input () =
    eprintfn "ERROR: please Input the content for format"

[<EntryPoint>]
let main _ =
    let options = Options.Parse(native.GetArgs())

    if options.File.IsNone && (Array.isEmpty options.Values) then
        error_input ()
    else
        let content =
            match options.File with
            | Some file -> native.ReadTextFileSync(file)
            | None -> String.Join("", options.Values)

        try
            content |> JsonFormatter.prettyFormat |> printfn "%s"

        with e ->
            printfn "ERROR: %A" e

    0
