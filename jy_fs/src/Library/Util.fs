namespace JY

module Util =
    open System
    open System.Diagnostics
    open System.IO
    open Tomlyn
    open Tomlyn.Model

    let pathFromArgv (defaultValues: string list) argv =
        argv
        |> Array.tryHead
        |> Option.orElseWith (fun _ -> List.tryFind File.Exists defaultValues)


    let parseToml path = path |> File.ReadAllText |> Toml.Parse

    let isWindows () =
        Environment.OSVersion.Platform.ToString()
        |> Lang.StringExt.containsIgnoreCase "win"

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
