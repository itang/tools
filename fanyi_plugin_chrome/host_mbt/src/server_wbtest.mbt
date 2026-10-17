///| White-box tests for the request dispatcher.

///|
/// Build a dispatcher over a stub runner, collecting emitted frames.
fn test_dispatcher(
  frames : Array[Json],
  on_once : (String, String?) -> RunResult,
  on_raw : (Array[String], String) -> RunResult,
) -> Dispatcher {
  let runner = StubRunner::new(on_once, on_raw)
  let engine = TranslationEngine::new(runner, default_model="")
  let emit : async (Json) -> Unit = frame => frames.push(frame)
  Dispatcher::new(engine, runner, emit)
}

///|
/// A single-segment translate request with the given id.
fn translate_request(id : String) -> Json {
  { "id": id, "type": "translate", "segments": [{ "index": 0, "text": "a" }] }
}

///|
async test "dispatcher health emits one terminal frame" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (stdin, _model) => translate_all_result(stdin),
    (_args, _stdin) => raw_result(Some(0), "opencode v2.0.29\n", "", None),
  )
  dispatcher.dispatch(({ "id": "1", "type": "health" } : Json))
  debug_inspect(frames.length(), content="1")
  assert_true(
    frames[0]
    is {
      "type": "health",
      "last": True,
      "data": { "opencode": { "version": "opencode v2.0.29", .. }, .. },
      ..
    },
  )
}

///|
async test "dispatcher rejects an unknown request type" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (stdin, _model) => translate_all_result(stdin),
    (_args, _stdin) => ok_result(""),
  )
  dispatcher.dispatch(({ "id": "2", "type": "nope" } : Json))
  debug_inspect(frames.length(), content="1")
  assert_true(
    frames[0] is { "type": "error", "error": { "code": "BAD_REQUEST", .. }, .. },
  )
}

///|
async test "dispatcher emits progress then a terminal translate frame" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (stdin, _model) => translate_all_result(stdin),
    (_args, _stdin) => ok_result(""),
  )
  dispatcher.dispatch(translate_request("3"))
  let progress = frames.filter(frame => frame is { "type": "progress", .. })
  assert_true(progress.length() >= 1)
  let terminal = frames.filter(frame => {
    frame is { "type": "translate", "last": True, .. }
  })
  debug_inspect(terminal.length(), content="1")
  assert_true(
    terminal[0] is { "translations": [{ "text": "T:a", .. }, ..], .. },
  )
}

///|
async test "dispatcher turns an engine HostError into an error frame" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (_stdin, _model) => failed_result(Some(1), "", [region_event()]),
    (_args, _stdin) => ok_result(""),
  )
  dispatcher.dispatch(translate_request("4"))
  let errors = frames.filter(frame => frame is { "type": "error", .. })
  debug_inspect(errors.length(), content="1")
  assert_true(
    errors[0]
    is { "last": True, "error": { "code": "MODEL_UNAVAILABLE", .. }, .. },
  )
}

///|
async test "dispatcher yields an empty terminal frame for no segments" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (stdin, _model) => translate_all_result(stdin),
    (_args, _stdin) => ok_result(""),
  )
  dispatcher.dispatch(
    ({ "id": "5", "type": "translate", "segments": [] } : Json),
  )
  debug_inspect(frames.length(), content="1")
  assert_true(frames[0] is { "type": "translate", "translations": [], .. })
}
