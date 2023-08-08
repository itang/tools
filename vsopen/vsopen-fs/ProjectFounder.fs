module ProjectFounder

open System

let private getNameByExt (files: string seq) (ext: string) =
    files |> Seq.tryFind (fun f -> f.EndsWith(ext))

/// Get sln or project file name in current work directory
let tryFindSlnOrProjectName () : string option =
    let files = IO.Directory.EnumerateFiles "."
    let getNameInFilesByExt = getNameByExt files

    let searchExtensions =
        seq {
            ".sln"
            ".fsproj"
            ".csproj"
        }

    let searchProjectFiles =
        searchExtensions
        |> Seq.map (fun ext ->
            printfn $"try find {ext} file ..."
            (ext, getNameInFilesByExt ext))

    let found = searchProjectFiles |> Seq.tryFind (fun (_, p) -> Option.isSome p)

    match found with
    | Some(ext, Some name) ->
        printfn $"found {ext} file: {name}"
        Some name
    | _ -> None
