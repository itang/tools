open Webapi

module RpcResult = {
  type t<'a> = {id: string, jsonrpc: string, result: option<'a>}
}

%%private(
  let get_json = async (url: string): Js.Json.t => {
    let resp = await Fetch.fetch(url)
    let json = await resp->Webapi.Fetch.Response.json
    json
  }
)

external convertToRpcResult: Js.Json.t => RpcResult.t<'a> = "%identity"

let get = async (url: string): RpcResult.t<'a> => {
  {await get_json(url)}->convertToRpcResult
}
