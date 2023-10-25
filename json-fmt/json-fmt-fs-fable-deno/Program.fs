open Fable.Core
open Tang.String

open Cli

[<Global>]
module Deno =
    let args: array<string> = jsNative
    let readTextFileSync (path: string) = jsNative

[<EntryPoint>]
let main _ =
    // printfn "//args: %A" Deno.args
    // printfn "//%s" <| "*" * 100

    let arg1 = Deno.args |> Array.tryHead |> Option.defaultValue ""

    //TODO: 支持--file 指定要格式化的json文件
    match arg1 with
    | "-h" -> printfn """help: jsonfmt-fs -f <File> ['<json string>']"""
    | "-v"
    | "--version" -> printfn "0.1"
    | _ ->
        let options = Options.Parse(Deno.args)
        eprintfn "INFO: options: %A" options
        eprintfn ""

        if options.File.IsNone && options.Values |> Array.isEmpty then
            eprintfn "please Input the content for format"
        else
            let content =
                match options.File with
                | Some file -> Deno.readTextFileSync file
                | None -> System.String.Join("", options.Values)

            try
                let ret = JsonFormatter.prettyFormat content
                printfn "%s" ret

            with e ->
                printfn "ERROR: %A" e

    0
