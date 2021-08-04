module Util

open System
open System.IO
open Tomlyn
open Tomlyn.Model

let private arrayHead defaultValue argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultValue


let private containsIgnoreCase (it: string) (source: string) = source.ToLower().Contains(it.ToLower())


let pathFromArgv (defaultValue: string) argv = arrayHead defaultValue argv


let parseToml path = path |> File.ReadAllText |> Toml.Parse


let urlsFromTomlPath path =
    let urlTomlArray =
        path
        |> parseToml
        |> (fun it -> it.ToModel().Item("urls") :?> TomlArray)

    seq { for it in urlTomlArray -> (it :?> string) }


let getBrowserCmd () =
    if Environment.OSVersion.Platform.ToString()
       |> containsIgnoreCase "win" then
        "explorer"
    else
        "x-www-browser"


let openBrowser cmd url =
    Diagnostics.Process.Start(cmd, [ url ]) |> ignore
