open FSharp.SystemCommandLine

open System

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

let devenv name = executeProcess "devenv" name

let getNameByExt (files: string seq) (ext: string) =
    files |> Seq.filter (fun f -> f.EndsWith(ext)) |> Seq.tryHead

let getSlnOrProjectName () : string option =
    let files = IO.Directory.EnumerateFiles "."
    let getNameInFilesByExt = getNameByExt files

    match getNameInFilesByExt ".sln" with
    | Some name ->
        printfn $"found sln file: {name}"
        Some name
    | None ->
        match getNameInFilesByExt ".fsproj" with
        | Some name ->
            printfn $"found fsproj file: {name}"
            Some name
        | None -> None

let mainHandle (name) =
    try
        match name with
        | Some name -> devenv name
        | None ->
            match getSlnOrProjectName () with
            | Some name -> devenv name
            | None -> failwith "NO FOUND project"
        |> printfn "%A"

        0
    with ex ->
        eprintfn $"{ex.Message}"
        -1

[<EntryPoint>]
let main argv =
    let name = Input.ArgumentMaybe<string>("The file for the project")

    rootCommand argv {
        description "visual studio project opener"
        inputs (name) // must be set before setHandler
        setHandler mainHandle
    }
