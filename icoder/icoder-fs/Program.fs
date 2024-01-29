open System
open System.Text

let print_help () =
    printfn
        """icoder-fs [command] <input>
  --help -v         help
  --version -v      version
  base64 <input>    base64 encode
  base64 -d <input> base64 decode
"""

[<EntryPoint>]
let main argv =
    match argv with
    | [| "--help" |]
    | [| "-h" |] -> print_help ()
    | [| "--version" |]
    | [| "-v" |] -> printfn "v0.1-20240129.1"
    | [| "base64"; input |] -> Encoding.UTF8.GetBytes(input) |> Convert.ToBase64String |> printfn "%s"
    | [| "base64"; "-d"; input |] -> input |> Convert.FromBase64String |> Encoding.UTF8.GetString |> printfn "%s"
    | v ->
        let command = v |> Array.tryHead |> Option.defaultValue ""
        printfn $"unknown command '{command}'"
        "*" |> String.replicate 50 |> printfn "%s"
        print_help ()

    0
