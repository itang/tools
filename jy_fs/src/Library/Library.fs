namespace JY

module Biz =
    open JY.Util

    let openUrls urls =
        Seq.iteri
            (fun index url ->
                printfn $"%d{index} open %s{url}"
                do openBrowser url)
            urls

        0

    let onNonePath () =
        printfn $"ERROR: path is invalid."

        1
