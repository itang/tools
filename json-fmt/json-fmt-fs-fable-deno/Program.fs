open System

open Cli
open Native

let error_input () =
    eprintfn "ERROR: please Input the content for format"

[<EntryPoint>]
let main _ =
    // printfn "//args: %A" Deno.args
    // printfn "//%s" <| "*" * 100

    let arg1 = Deno.args |> Array.tryHead |> Option.defaultValue ""

    match arg1 with
    | "-h" -> printfn """help: jsonfmt-fs -f <File> ['<json string>']"""
    | "-v"
    | "--version" -> printfn "0.1"
    | "" -> error_input ()
    | _ ->
        let options = Options.Parse(Deno.args)
        //eprintfn "INFO: options: %A" options
        //eprintfn ""

        if options.File.IsNone && (options.Values |> Array.isEmpty) then
            error_input ()
        else
            let content =
                match options.File with
                | Some file -> Deno.readTextFileSync file
                | None -> String.Join("", options.Values)

            try
                let ret = JsonFormatter.prettyFormat content
                printfn "%s" ret

            with e ->
                printfn "ERROR: %A" e

    0
