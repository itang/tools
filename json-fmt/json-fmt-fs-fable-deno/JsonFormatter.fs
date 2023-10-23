module JsonFormatter

open Fable.Core

let prettyFormat value =
    let value = JS.JSON.parse value
    JS.JSON.stringify (value, (fun _ obj -> obj), Some 2)
