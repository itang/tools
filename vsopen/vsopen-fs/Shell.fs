﻿module Shell

open System

/// Process Result
type ProcessResult =
    { ExitCode: int
      StdOut: string
      StdErr: string }

let private executeProcess (processName: string) (processArgs: string) =
    let psi = new Diagnostics.ProcessStartInfo(processName, processArgs)
    psi.UseShellExecute <- false
    psi.RedirectStandardOutput <- true
    psi.RedirectStandardError <- true
    psi.CreateNoWindow <- true

    let proc = Diagnostics.Process.Start(psi)
    let output = new Text.StringBuilder()
    let error = new Text.StringBuilder()
    proc.OutputDataReceived.Add(fun args -> output.Append(args.Data) |> ignore)
    proc.ErrorDataReceived.Add(fun args -> error.Append(args.Data) |> ignore)
    proc.BeginErrorReadLine()
    proc.BeginOutputReadLine()
    proc.WaitForExit()

    { ExitCode = proc.ExitCode
      StdOut = output.ToString()
      StdErr = error.ToString() }

/// Launch visual studio
let devenv (name: string) = executeProcess "devenv" name
