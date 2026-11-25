// Host and Node live in the ffi package. These aliases keep the app code free
// of `@ffi` qualifiers; methods defined on them still work. `Signal` values
// are inferred from `Host::new_signal`, so no alias is needed for them.
type Host = @ffi.Host
type Node = @ffi.Node
