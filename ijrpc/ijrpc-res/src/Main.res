open FFI

let get_app_args = (args: array<string>): array<string> => args->Array.sliceToEnd(~start=2)

let run = async (args: array<string>): unit => {
  let url = args->Array.get(0)->Option.getOr("http://127.0.0.1:8080/api/ops/getServerInfo")
  Console.log3(">>get", "url       =", url)

  let ret = await JsonRpc.get(url)

  Console.log()
  Console.log3("<<response", "data =", `${ret->JSONOps.stringifyUnsafe(~space=2)}`)
}

let main = async (): unit => {
  let args = Process.argv->get_app_args
  await run(args)
}

await main()
