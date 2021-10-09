module Whois

open System.Diagnostics

let private whois name =
    let p = new Process()
    p.StartInfo.FileName <- "whois"
    p.StartInfo.UseShellExecute <- false
    p.StartInfo.RedirectStandardOutput <- true
    p.StartInfo.Arguments <- name
    p.Start() |> ignore

    let out = p.StandardOutput.ReadToEnd()
    printfn "out: %s" out
    p.WaitForExit()

    out

let whoisYes name =
    let out = whois name
    out.Contains("No match for domain")
