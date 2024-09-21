//! Util module
//!

open Webapi

open Types
open FFI

module DictWithArrayValue = {
  type t<'v> = Js.Dict.t<array<'v>>

  let push = (dict: t<'v>, key: string, value: 'v): t<'v> => {
    let oldValue = Dict.get(dict, key)

    switch oldValue {
    | None => dict->Dict.set(key, [value])
    | Some(arr) => arr->Array.push(value)
    }

    dict
  }
}

module URLSearchParamsOps = {
  let fromQS = (searchParams: Url.URLSearchParams.t): DictWithArrayValue.t<string> => {
    let dict = Js.Dict.empty()

    let entries = searchParams->Url.URLSearchParams.entries

    let arr = entries->Array.fromArrayLike

    ArrayOps.reduce(
      arr,
      (d, (key, value)) => {
        d->DictWithArrayValue.push(key, value)
      },
      dict,
    )
  }
}

module UrlOps = {
  %%private(
    let toObj0 = (url: Url.t, ~tf: Url.URLSearchParams.t => 'b) =>
      {
        "protocol": url->Url.protocol,
        "host": url->Url.host,
        "port": url->Url.port,
        "pathname": url->Url.pathname,
        "searchParams": url->Url.searchParams->tf,
        "hash": url->Url.hash,
      }
  )

  let toObj = (url: Url.t, ~mode: Mode.t=Qs) =>
    switch mode {
    | Qsl => url->toObj0(~tf=ObjectOps.fromEntries)
    | Qs => url->toObj0(~tf=URLSearchParamsOps.fromQS)
    }
}
