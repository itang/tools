module ProjectFounder

open System

let private getNameByExt (files: string seq) (ext: string) =
    //files |> Seq.filter (fun f -> f.EndsWith(ext)) |> Seq.tryHead
    printfn $"try find {ext} file ..."
    files |> Seq.tryFind (fun f -> f.EndsWith(ext))

/// Get sln or project file name in current work directory
let getSlnOrProjectName () : string option =
    let files = IO.Directory.EnumerateFiles "."
    let getNameInFilesByExt = getNameByExt files

    let searchExts =
        seq {
            ".sln"
            ".fsproj"
        }

    let pseq = searchExts |> Seq.map (fun ext -> (ext, getNameInFilesByExt ext))
    let found = pseq |> Seq.tryFind (fun (_, p) -> Option.isSome p)

    match found with
    | Some(ext, Some name) ->
        printfn $"found {ext} file: {name}"
        Some name
    | _ -> None

(*        match getNameInFilesByExt ".sln" with
    | Some name ->
        printfn $"found sln file: {name}"
        Some name
    | None ->
        match getNameInFilesByExt ".fsproj" with
        | Some name ->
            printfn $"found fsproj file: {name}"
            Some name
        | None -> None*)
