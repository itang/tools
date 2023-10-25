module Native

open Fable.Core

[<Global>]
module Deno =
    let args: array<string> = jsNative
    let readTextFileSync (path: string) = jsNative
    let exit () = jsNative
