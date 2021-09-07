open System
open FSharp.Data

[<EntryPoint>]
let main argv =
    let url = argv |> Array.tryHead |> Option.defaultValue "http://localhost:8080"

    let num = System.Environment.GetEnvironmentVariable("NUM")
    let num = if String.IsNullOrEmpty(num) then 1000 else (int num)

    let timeout = System.Environment.GetEnvironmentVariable("TIMEOUT") 
    let timeout = if String.IsNullOrEmpty(timeout) then 5000 else (int timeout)

    printfn $"url={url}, num={num}, timeout={timeout}."
    let ts =
        seq {
            for i = 0 to num do
                yield
                    Http.AsyncRequestString(url, timeout = timeout)
                    |> Async.Catch
        }

    ts
    |> Async.Parallel
    |> Async.RunSynchronously
    |> Seq.iter (printfn "%A")

    0 // return an integer exit code
