module Util

open System
open System.IO

let readText path = File.ReadAllText path

let nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

let tipNameFromArgv defaultTip argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultTip

let log =
    let lockObj = obj ()

    fun color s ->
        lock lockObj (fun _ ->
            Console.ForegroundColor <- color
            printfn "%s" s
            Console.ResetColor())

let complete = log ConsoleColor.Magenta
let ok = log ConsoleColor.Green
let info = log ConsoleColor.Cyan
let warn = log ConsoleColor.Yellow
let error = log ConsoleColor.Red
