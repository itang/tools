//! Types module
//!

module Mode = {
  type t =
    | Qs
    | Qsl

  let fromString = (s: string): option<t> =>
    switch s {
    | "qsl" => Some(Qsl)
    | "qs" => Some(Qs)
    | _ => None
    }
}

module AppArgs = {
  type t = {
    mutable mode: option<Mode.t>,
    mutable urls: array<string>,
  }

  let parseArgs = (raw_args: array<string>): t => {
    let args = raw_args->Array.sliceToEnd(~start=2)

    let ret: t = {mode: None, urls: []}

    let index = ref(0)
    while index.contents < args->Array.length {
      let curr = args[index.contents]->Option.getOr("")
      if curr == "--mode" || curr == "-m" {
        ret.mode = args[index.contents + 1]->Option.flatMap(Mode.fromString)

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

  let setDefault = (args: t, ~mode: Mode.t=Qsl, ~url: string): t => {
    if args.mode == None {
      args.mode = Some(mode)
    }
    if args.urls->Array.length == 0 {
      args.urls->Array.push(url)
    }

    args
  }
}
