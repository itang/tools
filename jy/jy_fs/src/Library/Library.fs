namespace JY

open JY.Lang
open JY.Util

module Biz =
    type OpenMode =
        | Sequence
        | Parallel

    let openUrls mode urls =
        let doFunc =
            match mode with
            | Parallel -> AsyncExt.AwaitParallelAll
            | Sequence -> AsyncExt.AwaitSequentialAll

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
