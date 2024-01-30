open System

open Coder

let print_help () =
    printfn
        """icoder-fs [command] <input>
  --help -v             help
  --version -v          version
  base64 <input>        base64 encode
  base64 -d <input>     base64 decode
  hex       <input>     hex encode
  hex    -d <input>     hex decode
  i2hex     <input>     10 进制 转 16 进制
  i2hex  -d <input>     16 进制 转 10 进制
  uuid                  uuid字符串
  upcase    <input>     转大写
  lowcase   <input>     转小写
  random    <length>    随机字符串. length 指定长度, 默认8
"""

let handleUnknownCommand argv =
    let command = argv |> Array.tryHead |> Option.defaultValue ""
    printfn $"Unknown command '{command}'"
    "*" |> String.replicate 50 |> printfn "%s"

    print_help ()

[<EntryPoint>]
let main argv =

    match argv |> List.ofArray with
    | "--help" :: _tail
    | "-h" :: _tail -> print_help ()
    | "--version" :: _tail
    | "-v" :: _tail -> printfn "v0.1-20240129.1"
    | "base64" :: tail ->
        let base64Coder = new Base64Coder() :> ICoder

        match tail with
        | "-d" :: input :: _tail -> input |> base64Coder.decode |> printfn "%s"
        | input :: _tail -> input |> base64Coder.encode |> printfn "%s"
        | [] -> Console.ReadLine() |> base64Coder.encode |> printfn "%s"

    | "hex" :: tail ->
        let hexCoder = new HexCoder() :> ICoder

        match tail with
        | "-d" :: input :: _tail -> input |> hexCoder.decode |> printfn "%s"
        | input :: _tail -> input |> hexCoder.encode |> printfn "%s"
        | [] -> Console.ReadLine() |> hexCoder.encode |> printfn "%s"

    | "i2hex" :: tail ->
        let i2hexCoder = new I2HexCoder() :> ICoder

        match tail with
        | "-d" :: input :: _tail -> input |> i2hexCoder.decode |> printfn "%s"
        | input :: _tail -> input |> i2hexCoder.encode |> printfn "%s"
        | [] -> Console.ReadLine() |> i2hexCoder.encode |> printfn "%s"

    | "upcase" :: input :: _tail -> input.ToUpper() |> printfn "%s"
    | "lowcase" :: input :: _tail -> input.ToLower() |> printfn "%s"

    | "uuid" :: _tail -> Guid.NewGuid().ToString() |> printfn "%s"

    | "random" :: length ->
        let len = length |> List.tryHead |> Option.map int |> Option.defaultValue 8
        len |> randomStr |> printfn "%s"

    | _ -> handleUnknownCommand argv

    0
