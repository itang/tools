// Host and Node live in the ffi package. These aliases keep the app code free
// of `@ffi` qualifiers; methods defined on them still work. `Signal` and the
// native `Scanner` are inferred from `Host::new_signal` / `Host::scanner`, so
// no aliases are needed for them.

///|
type Host = @ffi.Host

///|
type Node = @ffi.Node
