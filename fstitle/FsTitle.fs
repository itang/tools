module FsTitle

open FsHttp
open FsHttp.DslCE

let private START_TAG, END_TAG = "<title>", "</title>"

let getBodyFromUrl url = http { GET url } |> Response.toText

let extractTitle (body: string) =
    let getIndex () =
        let pos = body.IndexOf(START_TAG)
        let endPos = body.IndexOf(END_TAG)
        let start = pos + START_TAG.Length
        let length = endPos - start
        (start, length)

    let start, length = getIndex ()
    let title = body.Substring(start, length).Trim()
    title
