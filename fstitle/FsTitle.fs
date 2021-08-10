module FsTitle

open FsHttp
open FsHttp.DslCE

let private START_TAG, END_TAG = "<title>", "</title>"

let getBodyFromUrl url = http { GET url } |> Response.toText

let extractTitle (body: string) =
    let startPos = body.IndexOf(START_TAG)
    let endPos = body.IndexOf(END_TAG)
    let startIndex = startPos + START_TAG.Length
    let endIndex = endPos - 1

    let title = body.[startIndex..endIndex].Trim()
    title
