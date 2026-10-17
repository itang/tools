///| White-box tests for opencode failure classification.

///|
test "spawn failure maps to OPENCODE_MISSING" {
  let result = make_run_result(
    None,
    "",
    "",
    Some("spawn opencode ENOENT"),
    false,
    "",
    [],
  )
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"OPENCODE_MISSING\"",
  )
}

///|
test "region error maps to MODEL_UNAVAILABLE" {
  let result = failed_result(Some(1), "", [region_event()])
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"MODEL_UNAVAILABLE\"",
  )
}

///|
test "authentication error maps to AUTH_REQUIRED" {
  let event : Json = { "type": "provider.auth", "message": "Unauthorized" }
  let result = failed_result(Some(1), "", [event])
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"AUTH_REQUIRED\"",
  )
}

///|
test "timeout maps to TIMEOUT" {
  let result = make_run_result(None, "", "", None, true, "", [])
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"TIMEOUT\"",
  )
}

///|
test "non-zero exit maps to HOST_ERROR with stderr detail" {
  let result = failed_result(Some(2), "boom", [])
  let error = map_opencode_failure(result).unwrap()
  debug_inspect(error.code().name(), content="\"HOST_ERROR\"")
  assert_true(error.message().contains("boom"))
}

///|
test "successful run maps to None" {
  assert_true(map_opencode_failure(ok_result("x")) is None)
}
