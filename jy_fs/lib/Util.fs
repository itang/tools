module Util

open System
open System.Diagnostics
open System.IO
open Tomlyn
open Tomlyn.Model

let private arrayHead defaultValue argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultValue


let private containsIgnoreCase (it: string) (source: string) = source.ToLower().Contains(it.ToLower())


let pathFromArgv (defaultValues: string list) argv =
    match Array.tryHead argv with
    | Some (path) -> Some(path)
    | None ->
        defaultValues
        |> List.tryFind (fun it -> File.Exists(it))


let parseToml path = path |> File.ReadAllText |> Toml.Parse

let isWindows () =
    Environment.OSVersion.Platform.ToString()
    |> containsIgnoreCase "win"

let urlsFromTomlPath path =
    let urlTomlArray =
        path
        |> parseToml
        |> (fun it -> it.ToModel().Item("urls") :?> TomlArray)

    seq { for it in urlTomlArray -> (it :?> string) }


let getBrowserCmd () =
    if isWindows () then
        "start"
    else
        "x-www-browser"


let openBrowser url =
    if isWindows () then
        let psi = ProcessStartInfo()
        psi.UseShellExecute <- true
        psi.FileName <- url
        Process.Start(psi) |> ignore
    else
        let cmd = getBrowserCmd ()
        Process.Start(cmd, [ url ]) |> ignore
