namespace JY

module Util =
    open System
    open System.Diagnostics
    open System.IO
    open Tomlyn
    open Tomlyn.Model

    let private parseToml path = path |> File.ReadAllText |> Toml.Parse

    let private isWindows () =
        Environment.OSVersion.Platform.ToString()
        |> Lang.StringExt.containsIgnoreCase "win"


    let private getBrowserCmd () =
        if isWindows () then
            "start"
        else
            "x-www-browser"


    let pathFromArgv (defaultValues: string list) argv =
        argv
        |> Array.tryHead
        |> Option.orElseWith (fun _ -> List.tryFind File.Exists defaultValues)

    let urlsFromTomlPath path =
        let urlTomlArray =
            path
            |> parseToml
            |> (fun it -> it.ToModel().Item("urls") :?> TomlArray)

        seq { for it in urlTomlArray -> (it :?> string) }

    let openBrowser url =
        if isWindows () then
            let psi = ProcessStartInfo()
            psi.UseShellExecute <- true
            psi.FileName <- url
            Process.Start(psi) |> ignore
        else
            let cmd = getBrowserCmd ()
            Process.Start(cmd, [ url ]) |> ignore
