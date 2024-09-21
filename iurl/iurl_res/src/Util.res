//! Util module
//!

open Webapi

open Types
open FFI

type dictArray<'a> = Js.Dict.t<array<'a>>

%%private(
  let addToDictValueArray = (dict: dictArray<'v>, key: string, value: 'v): dictArray<'v> => {
    let oldValue = Dict.get(dict, key)

    switch oldValue {
    | None => dict->Dict.set(key, [value])
    | Some(arr) => arr->Array.push(value)
    }

    dict
  }

  @inline
  let fromQS = (searchParams: Url.URLSearchParams.t): dictArray<string> => {
    let dict = Js.Dict.empty()

    let entries = searchParams->Url.URLSearchParams.entries

    let arr = entries->Array.fromArrayLike

    ArrayOps.reduce(
      arr,
      (d, (key, value)) => {
        d->addToDictValueArray(key, value)
      },
      dict,
    )
  }

  let urlToObj0 = (url: Url.t, ~tf: Url.URLSearchParams.t => 'b) =>
    {
      "protocol": url->Url.protocol,
      "host": url->Url.host,
      "port": url->Url.port,
      "pathname": url->Url.pathname,
      "searchParams": url->Url.searchParams->tf,
      "hash": url->Url.hash,
    }
)

let urlToObj = (url: Url.t, ~mode: mode_t=Qs) =>
  switch mode {
  | Qsl => url->urlToObj0(~tf=ObjectOps.fromEntries)
  | Qs => url->urlToObj0(~tf=fromQS)
  }
