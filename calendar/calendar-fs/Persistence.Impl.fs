module Persistence.Impl

open System.IO

open Persistence.Api

type FilePersistence() =
    interface IPersistence with
        member _.Save (key: string) (content: string) : unit =
            let file =
                match key with
                | "html" -> "c.html"
                | "task" -> "t.html"
                | _ -> failwithf "Persistence, unknown key: %s" key

            printfn $"write to file: '{file}'..."
            File.WriteAllText(file, content)
