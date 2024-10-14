open Webapi

open Types
open Util

let defaultUrl = "https://user1:pass@www.baidu.com/portal?a=1&b=2&c=3&c=4#/apps/main"

let run = (appArgs: AppArgs.t): unit => {
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

  run(args->AppArgs.parseArgs)
}

main()
