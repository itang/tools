open Fable.Core
open Tang.String

[<Global>]
module Deno =
    let args: array<string> = jsNative

module JsonFormater =
    let fmt value =
        let value = JS.JSON.parse value
        JS.JSON.stringify (value, (fun _ obj -> obj), Some 2)

[<EntryPoint>]
let main _ =
    printfn "//args: %A" Deno.args
    printfn "//%s" <| "*" * 100

    let arg1 = Deno.args |> Array.tryHead |> Option.defaultValue ""

    match arg1 with
    | "-h" -> printfn """help: fmt-json '<json string>'"""
    | "-v"
    | "--version" -> printfn "0.1"
    | "" -> printfn "please Input the content for format"
    | _ ->
        let content = System.String.Join("", Deno.args)

        try
            let ret = JsonFormater.fmt content
            printfn "%s" ret

        with e ->
            printfn "ERROR: %A" e

    0
