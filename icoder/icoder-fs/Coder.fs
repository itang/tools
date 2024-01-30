module Coder

open System
open System.Text

type ICoder =
    abstract member encode: input: string -> string
    abstract member decode: input: string -> string

type Base64Coder() =
    interface ICoder with
        member _.encode input =
            input |> Encoding.UTF8.GetBytes |> Convert.ToBase64String

        member _.decode input : string =
            input |> Convert.FromBase64String |> Encoding.UTF8.GetString

type HexCoder() =
    interface ICoder with
        member _.encode input =
            input |> Encoding.UTF8.GetBytes |> Convert.ToHexString

        member _.decode input : string =
            input |> Convert.FromHexString |> Encoding.UTF8.GetString

type I2HexCoder() =
    interface ICoder with
        member _.encode input =
            let v = input |> Int64.Parse
            let hex = v.ToString("x")
            $"0x{hex}"

        member _.decode input : string =
            let v = if input.StartsWith("0x") then input.Substring(2) else input
            Int64.Parse(v, Globalization.NumberStyles.HexNumber) |> string

let randomStr n =
    let r = Random()
    let chars = Array.concat ([ [| 'a' .. 'z' |]; [| 'A' .. 'Z' |]; [| '0' .. '9' |] ])
    let sz = Array.length chars in
    String(Array.init n (fun _ -> chars.[r.Next sz]))
