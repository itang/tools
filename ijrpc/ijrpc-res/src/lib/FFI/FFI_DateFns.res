//! module FFI_DateFns
//!

@module("date-fns")
external format: (Js.Date.t, string) => string = "format"
