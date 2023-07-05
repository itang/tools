module Persistence.Api

type IPersistence =
    abstract member Save: string -> string -> unit
