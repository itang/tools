// NshHost and Node live in the ffi package. These aliases keep the app code free
// of `@ffi` qualifiers; methods defined on them still work. `Signal`, `Scanner`,
// `Clock` and `TableState` are inferred from NshHost accessors, so no aliases are
// needed for them.

///|
type NshHost = @ffi.NshHost

///|
type Node = @ffi.Node
