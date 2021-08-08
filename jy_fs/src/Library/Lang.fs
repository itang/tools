namespace JY

module Lang =
    let fold onSomeFunc onNoneAction =
        function
        | Some value -> onSomeFunc value
        | None -> onNoneAction ()

    let arrayHead defaultValue argv =
        argv
        |> Array.tryHead
        |> Option.defaultValue defaultValue


    let containsIgnoreCase (it: string) (source: string) = source.ToLower().Contains(it.ToLower())
