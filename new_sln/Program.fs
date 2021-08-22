open Library

let Version = "0.1-20210822"

[<EntryPoint>]
let main argv =
    printfn $"new_sln-v%s{Version}"

    let name =
        argv
        |> Array.tryHead
        |> Option.defaultValue "MyFSharpSln"

    let dir = name

    newFSharpSln name dir

    printfn $"cd %s{dir} ..."

    0 // return an integer exit code
