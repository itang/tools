open Webapi

open Types
open Util

let defaultUrl = "https://www.baidu.com/hello?a=1&b=2&b=3#/main"

let run = (args: array<string>): unit => {
  let appArgs = args->AppArgs.parseArgs
  Js.log2("INFO: appArgs=", appArgs)

  let appArgs = appArgs->AppArgs.setDefault(~mode=Mode.Qsl, ~url=defaultUrl)

  Js.log3("INFO: after set default, appArgs=", appArgs, "\n")

  switch appArgs.mode {
  | Some(mode) =>
    appArgs.urls->Array.forEachWithIndex((url, index) => {
      Js.log(`[${Int.toString(index + 1)}]INFO: url=${url}`)
      url->Url.make->UrlOps.toObj(~mode)->JSON.stringifyAny(~space=2)->Js.log2("\n")
    })
  | None => Js.log("ERROR: unknown mode")
  }
}

let main = (): unit => {
  let args = FFI.NodeProcess.argv
  Js.log(`DEBUG: args=${args->Array.toString}`)

  run(args)
}

main()
