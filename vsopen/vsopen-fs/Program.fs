open FSharp.SystemCommandLine

let mainHandler =
    fun (name) ->
        try
            match name with
            | Some name -> Shell.devenv name
            | None ->
                match ProjectFounder.tryFindSlnOrProjectName () with
                | Some name -> Shell.devenv name
                | None -> failwith "NO FOUND project"
            |> printfn "%A"

            0
        with ex ->
            eprintfn $"{ex.Message}"
            -1

[<EntryPoint>]
let main argv =
    let name = Input.ArgumentMaybe<string>("The file(.sln or .fsproj) for the project")

    rootCommand argv {
        description "visual studio project opener"
        inputs (name)
        setHandler mainHandler
    }
