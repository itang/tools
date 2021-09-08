namespace wrkfs

module Wrk =

    open FSharp.Data

    type Options = 
        {Threads: int; Duration: int; Connections: int; Url: string}

        static member Default =
            {Connections = 10
             Threads =  2
             Duration = 10
             Url = "http://localhost:8080"}
        
        member this.ToPrettyString (?indent: int) =
            let result = $"%A{this}"
            let indent = defaultArg indent 4

            if indent > 0 then
                let indentAllLines (s: string) indent =
                    s.Split("\n") |> Array.map ((+) (String.replicate indent " ")) |> String.concat "\n"
                indentAllLines result indent
            else result

    //TODO: 参考wrk 实现
    let run (options: Options) =
        printfn $"options: \n{options.ToPrettyString()}"
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

