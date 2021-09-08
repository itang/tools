namespace wrkfs

module Wrk =

    open FSharp.Data

    type Options = 
        {Threads: int; Duration: int; Connections: int; Url: string}

        static member Default =
            {Connections = 10
             Threads =  2
             Duration = 10
             Url ="http://localhost:8080"}

    //TODO: 参考wrk 实现
    let run (options: Options) =
        printfn $"options: %A{options}"
        let ts =
            seq {
                 for i = 0 to options.Connections do
                     yield
                         Http.AsyncRequestString(options.Url, timeout = 10 * 1000)
                         |> Async.Catch
             }
        ts
        |> Async.Parallel
        |> Async.RunSynchronously
        |> Seq.iter (printfn "%A")

