namespace JY

module Biz =
    open JY.Util
    open JY.Lang

    let openUrls urls =
        urls
        |> Seq.mapi
            (fun index url ->
                async {
                    printfn $"%d{index} open %s{url}"
                    openBrowser url
                })
        |> AsyncExt.AwaitAll
        |> ignore

        0

    let onNonePath () =
        printfn $"ERROR: path is invalid."

        1
