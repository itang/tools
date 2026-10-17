///| White-box tests for the health check.

///|
/// A runner whose `--version` invocation answers with `version` on stdout.
fn version_runner(version : String) -> StubRunner {
  StubRunner::new((stdin, _model) => translate_all_result(stdin), (
    _args,
    _stdin,
  ) => raw_result(Some(0), version, "", None))
}

///|
async test "health reports the opencode version on success" {
  let health = check_health(version_runner("opencode v2.0.29\n"), "opencode")
  assert_true(
    health
    is {
      "host": { "ok": True, .. },
      "opencode": { "ok": True, "version": "opencode v2.0.29", .. },
      ..
    },
  )
}

///|
async test "health reports a missing opencode binary" {
  let runner = StubRunner::new((stdin, _model) => translate_all_result(stdin), (
    _args,
    _stdin,
  ) => raw_result(None, "", "", Some("ENOENT")))
  let health = check_health(runner, "opencode")
  assert_true(
    health
    is { "opencode": { "ok": False, "error": "OPENCODE_MISSING", .. }, .. },
  )
}

///|
async test "health reports a non-zero version exit" {
  let runner = StubRunner::new((stdin, _model) => translate_all_result(stdin), (
    _args,
    _stdin,
  ) => raw_result(Some(3), "", "broken", None))
  let health = check_health(runner, "opencode")
  assert_true(
    health is { "opencode": { "ok": False, "error": "HOST_ERROR", .. }, .. },
  )
}
