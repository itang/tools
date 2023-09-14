open FSharp.SystemCommandLine

let mainHandler =
    fun name ->
        let nameOpt = name |> Option.orElseWith ProjectFounder.tryFindSlnOrProjectName

        match nameOpt with
        | Some name ->
            Shell.devenvAsync name |> printfn "%A"
            0
        | None ->
            //TODO: try find down / up
            eprintfn "NO FOUND project"
            -1

[<EntryPoint>]
let main argv =
    let name = Input.ArgumentMaybe<string>("The file(.sln or .fsproj) for the project")

    rootCommand argv {
        description "visual studio project opener"
        inputs (name)
        setHandler mainHandler
    }
