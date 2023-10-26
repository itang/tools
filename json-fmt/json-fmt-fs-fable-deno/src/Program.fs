open System

open Cli
open Infra

let error_input () =
    eprintfn "ERROR: please Input the content for format"

[<EntryPoint>]
let main _ =
    // printfn "//args: %A" Deno.args
    // printfn "//%s" <| "*" * 100

    let options = Options.Parse(Target.GetArgs())
    //eprintfn "INFO: options: %A" options
    //eprintfn ""

    if options.File.IsNone && (options.Values |> Array.isEmpty) then
        error_input ()
    else
        let content =
            match options.File with
            | Some file -> Target.ReadTextFileSync(file)
            | None -> String.Join("", options.Values)

        try
            let ret = Target.JsonPrettyFormat content
            printfn "%s" ret

        with e ->
            printfn "ERROR: %A" e

    0
