module Lang

let fold onSome onNone opt =
    match opt with
    | Some it -> onSome it
    | None -> onNone ()

let arrayHead defaultValue argv =
    argv
    |> Array.tryHead
    |> Option.defaultValue defaultValue


let containsIgnoreCase (it: string) (source: string) = source.ToLower().Contains(it.ToLower())
