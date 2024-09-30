//! module FFI_Json

@raises @val
external stringifyUnsafe: ('a, ~replacer: JSON.replacer=?, ~space: int=?) => string =
  "JSON.stringify"
