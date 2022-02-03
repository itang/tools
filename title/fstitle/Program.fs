open FsTitle

let VERSION = "0.1.0-20210807"

[<EntryPoint>]
let main argv =
    try
        match argv |> Array.tryHead with
        | Some url ->
            let title =
                url
                |> getBodyFromUrl
                |> extractTitle
                |> unwrapTitle

            printfn $"title: %s{title}"
        | _ -> printfn "Please input the url."
    with
    | e -> printfn $"ERROR: %s{e.Message}"

    0 // return an integer exit code
