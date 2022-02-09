module Util

open System
open System.IO
open System.Runtime.InteropServices

let readText path = File.ReadAllText path

let nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

let tipNameFromArgv defaultTip argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultTip


type Console() =
    static let lockObj = obj ()

    static member Log(color, s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        lock
            lockObj
            (fun _ ->
                Console.ForegroundColor <- color
                let fn = if newline then printfn else printf
                fn "%s" s
                Console.ResetColor())

    static member Complete(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.Log(ConsoleColor.Magenta, s, newline)

    static member Ok(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.Log(ConsoleColor.Green, s, newline)

    static member Info(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.Log(ConsoleColor.Cyan, s, newline)

    static member Warn(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.Log(ConsoleColor.Yellow, s, newline)

    static member Error(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.Log(ConsoleColor.Red, s, newline)
