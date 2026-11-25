// Host and Node live in the ffi package. These aliases keep the app code free
// of `@ffi` qualifiers; methods defined on them still work. `Signal`, `Scanner`,
// `Clock` and `TableState` are inferred from Host accessors, so no aliases are
// needed for them.

///|
type Host = @ffi.Host

///|
type Node = @ffi.Node
