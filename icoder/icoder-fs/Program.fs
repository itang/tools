open System
open System.Text

let print_help () =
    printfn
        """icoder-fs [command] <input>
  --help -v         help
  --version -v      version
  base64 <input>    base64 encode
  base64 -d <input> base64 decode
  uuid              UUID
"""

let handleUnknownCommand argv =
    let command = argv |> Array.tryHead |> Option.defaultValue ""
    printfn $"Unknown command '{command}'"
    "*" |> String.replicate 50 |> printfn "%s"

    print_help ()

type ICoder =
    abstract member encode: input: string -> string
    abstract member decode: input: string -> string

type Base64Coder() =
    interface ICoder with
        member _.encode input =
            input |> Encoding.UTF8.GetBytes |> Convert.ToBase64String

        member _.decode input : string =
            input |> Convert.FromBase64String |> Encoding.UTF8.GetString

[<EntryPoint>]
let main argv =
    let base64Coder = new Base64Coder() :> ICoder

    match argv with
    | [| "--help" |]
    | [| "-h" |] -> print_help ()
    | [| "--version" |]
    | [| "-v" |] -> printfn "v0.1-20240129.1"
    | [| "base64"; input |] -> input |> base64Coder.encode |> printfn "%s"
    | [| "base64"; "-d"; input |] -> input |> base64Coder.decode |> printfn "%s"
    | [| "uuid" |] -> Guid.NewGuid().ToString() |> printfn "%s"
    | _ -> handleUnknownCommand argv

    0
