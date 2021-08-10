module FsTitle

open FsHttp
open FsHttp.DslCE

let private START_TAG, END_TAG = "<title>", "</title>"

// DUS 用作 单大小写可区分联合，以帮助对基元类型进行域建模
type Body = Body of string
type Title = Title of string

let getBodyFromUrl url =
    http { GET url } |> Response.toText |> Body

let extractTitle (Body body) =
    let startPos = body.IndexOf(START_TAG)
    let endPos = body.IndexOf(END_TAG)
    let startIndex = startPos + START_TAG.Length
    let endIndex = endPos - 1

    let title = body.[startIndex..endIndex].Trim()
    Title title

let unwrapTitle (Title title) = title
