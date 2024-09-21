open Webapi

open Types
open Util

let defaultUrl = "https://www.baidu.com/hello?a=1&b=2&b=3#/main"

let run = (args: array<string>): unit => {
  let appArgs = args->parseArgs

  Js.log2("INFO: appArgs=", appArgs)
  if appArgs.mode == "" {
    appArgs.mode = "qsl"
  }
  if appArgs.urls->Array.length == 0 {
    appArgs.urls->Array.push(defaultUrl)
  }
  Js.log2("INFO: fill default, appArgs=", appArgs)
  Js.log()

  let mode = appArgs.mode->modeFromString
  switch mode {
  | None => Js.log("ERROR: unknown mode")
  | Some(mode) =>
    appArgs.urls->Array.forEachWithIndex((url, index) => {
      Js.log(`[${Int.toString(index + 1)}]INFO: url=${url}`)
      url->Url.make->urlToObj(~mode)->JSON.stringifyAny(_, ~space=2)->Js.log
      Js.log()
    })
  }
}

let main = (): unit => {
  let args = FFI.NodeProcess.argv
  Js.log(`DEBUG: args=${args->Array.toString}`)

  run(args)
}

main()
