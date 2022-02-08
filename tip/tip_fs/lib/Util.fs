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

    static member log(color, s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        lock lockObj (fun _ ->
            Console.ForegroundColor <- color
            let fn = if newline then printfn else printf
            fn "%s" s
            Console.ResetColor())

    static member complete(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.log (ConsoleColor.Magenta, s, newline)

    static member ok(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.log (ConsoleColor.Green, s, newline)

    static member info(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.log (ConsoleColor.Cyan, s, newline)

    static member warn(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.log (ConsoleColor.Yellow, s, newline)

    static member error(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Console.log (ConsoleColor.Red, s, newline)
