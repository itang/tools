module Whois

open System.Diagnostics

[<Literal>]
let private NoMatch = "No match for domain"

let private whois name =
    let pi =
        ProcessStartInfo(fileName = "whois", UseShellExecute = false, RedirectStandardOutput = true, Arguments = name)

    let p = new Process(StartInfo = pi)
    p.Start() |> ignore

    let out = p.StandardOutput.ReadToEnd()

    do
        printfn "out: %s" out
        p.WaitForExit()

    out

let whoisYes name =
    let out = whois name
    out.Contains(NoMatch)
