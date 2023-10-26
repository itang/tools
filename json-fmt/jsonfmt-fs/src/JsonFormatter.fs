module JsonFormatter

#if FABLE_COMPILER

open Fable.Core.JS

let prettyFormat (value: string) =
    let value = JSON.parse value
    JSON.stringify (value, (fun _ obj -> obj), Some 2)

#else

open System.Text

let prettyFormat (value: string) =
    let value = Json.JsonSerializer.Deserialize(value)
    let options = Json.JsonSerializerOptions(WriteIndented = true)
    Json.JsonSerializer.Serialize(value, options)

#endif
