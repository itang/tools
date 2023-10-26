module Native

type INative =
    abstract member GetArgs: unit -> array<string>
    abstract member ReadTextFileSync: path: string -> string
    abstract member Exit: int -> unit

#if FABLE_COMPILER
open Fable.Core

[<Global>]
module Deno =
    let args: array<string> = jsNative
    let readTextFileSync (path: string) = jsNative
    let exit () = jsNative

let native =
    { new INative with
        member _.GetArgs() : array<string> = Deno.args
        member _.ReadTextFileSync(path: string) : string = Deno.readTextFileSync (path)
        member _.Exit(_: int) : unit = Deno.exit () }

#else

open System
open System.IO

let native =
    { new INative with
        member _.GetArgs() : array<string> = Environment.GetCommandLineArgs()[1..]
        member _.ReadTextFileSync(path: string) : string = File.ReadAllText(path)
        member _.Exit(code: int) : unit = Environment.Exit(code) }
#endif
