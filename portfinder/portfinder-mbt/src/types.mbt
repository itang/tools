// These types live in the ffi package. The aliases keep app code free of
// `@ffi` qualifiers; methods defined on them still work.

///|
type NshHost = @ffi.NshHost

///|
type Node = @ffi.Node

///|
type Signal = @ffi.Signal

///|
type Clock = @ffi.Clock

///|
type TableState = @ffi.TableState

///|
type Scanner = @ffi.Scanner

// The four injected ffi capabilities, grouped once at the entry point so the
// state and view modules do not each need four parameters.

///|
priv struct Runtime {
  host : NshHost
  clock : Clock
  table : TableState
  scanner : Scanner
}
