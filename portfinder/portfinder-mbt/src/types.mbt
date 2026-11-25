// These types live in the ffi package. The aliases keep app code free of
// `@ffi` qualifiers; methods defined on them still work. `Scanner`, `Clock` and
// `TableState` are inferred from NshHost accessors, so no aliases are needed.

///|
type NshHost = @ffi.NshHost

///|
type Node = @ffi.Node

///|
type Signal = @ffi.Signal
