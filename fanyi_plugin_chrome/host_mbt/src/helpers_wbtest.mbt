///| Shared test helpers: a closure-driven `Runner` stub, `RunResult` builders
/// and a tiny mutable counter. Only compiled in test mode.

///|
/// Build a `RunResult` from its observable fields; `events` are derived from
/// `stdout` so callers only supply what the assertions care about.
fn make_run_result(
  exit_code : Int32?,
  stdout : String,
  stderr : String,
  spawn_error : String?,
  timed_out : Bool,
  text : String,
  error_events : Array[Json],
) -> RunResult {
  {
    exit_code,
    stdout,
    stderr,
    spawn_error,
    timed_out,
    events: parse_jsonl(stdout),
    text,
    error_events,
  }
}

///|
/// A successful run whose model answer is `text`.
fn ok_result(text : String) -> RunResult {
  make_run_result(Some(0), "", "", None, false, text, [])
}

///|
/// A run defined by its raw process outcome (used for `--version` probes).
fn raw_result(
  exit_code : Int32?,
  stdout : String,
  stderr : String,
  spawn_error : String?,
) -> RunResult {
  make_run_result(exit_code, stdout, stderr, spawn_error, false, "", [])
}

///|
/// A run that exited non-zero with the given stderr and error events.
fn failed_result(
  exit_code : Int32?,
  stderr : String,
  error_events : Array[Json],
) -> RunResult {
  make_run_result(exit_code, "", stderr, None, false, "", error_events)
}

///|
/// Translate every item on stdin, prefixing each result with `T:`.
fn translate_all_result(stdin : String) -> RunResult {
  let items = match parse_json_option(stdin) {
    Some(Array(items)) => items
    _ => []
  }
  let map : Map[String, Json] = Map([])
  for item in items {
    let index = match item {
      { "index": Number(index, ..), .. } => index.to_int()
      _ => 0
    }
    let text = match item {
      { "text": String(text), .. } => text
      _ => ""
    }
    map.set(index.to_string(), Json("T:" + text))
  }
  ok_result(Json(map).stringify())
}

///|
/// The provider-agnostic region error event used across tests.
fn region_event() -> Json {
  {
    "type": "provider.auth",
    "message": "This model is not available in your country",
    "status": 403,
  }
}

///|
/// A simple mutable counter shared by closures.
struct TestCounter {
  mut value : Int32
}

///|
/// Create a zeroed counter.
fn TestCounter::new() -> TestCounter {
  { value: 0, }
}

///|
/// Increment the counter.
fn TestCounter::bump(self : TestCounter) -> Unit {
  self.value = self.value + 1
}

///|
/// A runner driven by two closures, for injecting behavior in tests.
struct StubRunner {
  on_once : (String, String?) -> RunResult
  on_raw : (Array[String], String) -> RunResult
}

///|
/// Create a stub runner.
fn StubRunner::new(
  on_once : (String, String?) -> RunResult,
  on_raw : (Array[String], String) -> RunResult,
) -> StubRunner {
  { on_once, on_raw, }
}

///|
impl Runner for StubRunner with fn bin(_self) -> String {
  "opencode"
}

///|
impl Runner for StubRunner with fn run_once(
  self,
  _prompt,
  stdin,
  model,
  _agent,
  _timeout_ms,
) {
  (self.on_once)(stdin, model)
}

///|
impl Runner for StubRunner with fn run_raw(self, args, stdin, _timeout_ms) {
  (self.on_raw)(args, stdin)
}
