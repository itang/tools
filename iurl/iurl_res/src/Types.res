//! Types module
//!

type mode_t =
  | Qs
  | Qsl

let modeFromString = (s: string): option<mode_t> =>
  switch s {
  | "qsl" => Some(Qsl)
  | "qs" => Some(Qs)
  | _ => None
  }

type appArgs = {
  mutable mode: string,
  mutable urls: array<string>,
}

let parseArgs = (raw_args: array<string>): appArgs => {
  let args = raw_args->Array.slice(~start=2, ~end=raw_args->Array.length)

  let ret: appArgs = {mode: "", urls: []}
  let index = ref(0)
  while index.contents < args->Array.length {
    let curr = args[index.contents]->Option.getOr("")
    if curr == "--mode" || curr == "-m" {
      ret.mode = args[index.contents + 1]->Option.getOr("")

      index := index.contents + 1
    } else if !(curr->String.startsWith("-")) {
      ret.urls->Array.push(curr)
    } else {
      Js.log(`WARN: unknown ${curr}`)
    }

    index := index.contents + 1
  }

  ret
}
