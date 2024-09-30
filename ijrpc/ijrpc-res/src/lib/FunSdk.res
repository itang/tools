//! module FunSdk

module ServerInfo = {
  type t = {pid: int, tid: int, machine_id: string}
}

module Ops = {
  type t = {subPath: string, urlPrefix: string}

  let make = (subPath: string, urlPrefix: string): t => {subPath, urlPrefix}

  let getServerInfo = async (self: t): JsonRpc.RpcResult.t<ServerInfo.t> => {
    await JsonRpc.get(`${self.urlPrefix}/${self.subPath}/getServerInfo`)
  }
}

module Sdk = {
  //TODO: remove urlPrefix?
  type t = {urlPrefix: string, ops: Ops.t}
}

let make = (urlPrefix: string): Sdk.t => {urlPrefix, ops: Ops.make("ops", urlPrefix)}
