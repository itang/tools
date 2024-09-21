//! FFI ArrayOps Module
//!

@send external reduce: (array<'b>, ('a, 'b) => 'a, 'a) => 'a = "reduce"
