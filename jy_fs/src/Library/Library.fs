namespace JY

module Biz =
    open JY.Util
    open JY.Lang

    type OpenMode =
        | Sequence
        | Parallel
        member this.FromString =
            function
            | "sequence"
            | "s" -> Some(Sequence)
            | "parallel"
            | "p" -> Some(Parallel)
            | _ -> None

    let openUrls mode urls =
        let doFunc =
            match mode with
            | Parallel -> AsyncExt.AwaitAll
            | Sequence -> AsyncExt.AwaitSeqAll

        urls
        |> Seq.mapi
            (fun index url ->
                async {
                    printfn $"%d{index} open %s{url}"
                    openBrowser url
                })
        |> doFunc
        |> ignore

        0

    let onNonePath () =
        printfn $"ERROR: path is invalid."

        1
