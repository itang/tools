module Util

open System
open System.Runtime.InteropServices

type Logger() =
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
        Logger.Log(ConsoleColor.Magenta, s, newline)

    static member Ok(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Logger.Log(ConsoleColor.Green, s, newline)

    static member Info(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Logger.Log(ConsoleColor.Cyan, s, newline)

    static member Warn(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Logger.Log(ConsoleColor.Yellow, s, newline)

    static member Error(s, [<Optional; DefaultParameterValue(true)>] newline: bool) =
        Logger.Log(ConsoleColor.Red, s, newline)
