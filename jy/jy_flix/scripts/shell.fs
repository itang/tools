module Shell0

open System
open Fake.Core

open System.Text.RegularExpressions
open System.Runtime.InteropServices

let sh0 cmd (args: list<string>) =
    let (cmd, args) =
        if RuntimeInformation.IsOSPlatform(OSPlatform.Windows) then
            let cargs = (cmd :: args) |> String.concat " "
            ("cmd", [ "/c"; cargs ])
        else
            (cmd, args)


    CreateProcess.fromRawCommand cmd args
    |> Proc.run
    |> ignore

let sh (cmdline: string) =
    match Regex.Split(cmdline, "\\s") |> List.ofArray with

    | cmd :: args -> sh0 cmd args
    | _ -> raise (System.ArgumentException("命令不合法"))
