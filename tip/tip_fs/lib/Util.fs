module Util

open System.IO

let readText path = File.ReadAllText path

let nameToPath rootDir name = Path.Combine(rootDir, $"%s{name}.md")

let tipNameFromArgv defaultTip argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultTip
