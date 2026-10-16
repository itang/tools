///| White-box tests for the request dispatcher.

///| Build a dispatcher over a stub runner, collecting emitted frames.
fn test_dispatcher(
  frames : Array[Json],
  on_once : (String, String?) -> RunResult,
  on_raw : (Array[String], String) -> RunResult,
) -> Dispatcher {
  let runner = StubRunner::new(on_once, on_raw)
  let engine = TranslationEngine::new(runner, default_model="")
  let emit : async (Json) -> Unit = frame => frames.push(frame)
  Dispatcher::new(engine, runner, "opencode", emit)
}

///| A single-segment translate request with the given id.
fn translate_request(id : String) -> Json {
  Json::object({
    "id": Json::string(id),
    "type": Json::string("translate"),
    "segments": Json::array([
      Json::object({ "index": json_int(0), "text": Json::string("a") }),
    ]),
  })
}

///|
async test "dispatcher health emits one terminal frame" {
  let frames : Array[Json] = []
  let dispatcher = test_dispatcher(
    frames,
    (stdin, _model) => translate_all_result(stdin),
    (_args, _stdin) => raw_result(Some(0), "opencode v2.0.29\n", "", None),
  )
  dispatcher.dispatch(
    Json::object({ "id": Json::string("1"), "type": Json::string("health") }),
  )
  debug_inspect(frames.length(), content="1")
  debug_inspect(get_str_field(frames[0], "type"), content="Some(\"health\")")
  debug_inspect(
    get_field(frames[0], "last").unwrap().stringify(),
    content="\"true\"",
  )
  let opencode = get_field(get_field(frames[0], "data").unwrap(), "opencode").unwrap()
  debug_inspect(
    get_str_field(opencode, "version"),
    content="Some(\"opencode v2.0.29\")",
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
  dispatcher.dispatch(
    Json::object({ "id": Json::string("2"), "type": Json::string("nope") }),
  )
  debug_inspect(frames.length(), content="1")
  debug_inspect(
    get_str_field(get_field(frames[0], "error").unwrap(), "code"),
    content="Some(\"BAD_REQUEST\")",
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
  let progress = frames.filter(
    frame => get_str_field(frame, "type") == Some("progress"),
  )
  assert_true(progress.length() >= 1)
  let terminal = frames.filter(
    frame =>
      get_str_field(frame, "type") == Some("translate") &&
      get_field(frame, "last").unwrap().stringify() == "true",
  )
  debug_inspect(terminal.length(), content="1")
  let translations = as_array(
    get_field(terminal[0], "translations").unwrap(),
  ).unwrap()
  debug_inspect(translations.length(), content="1")
  debug_inspect(get_str_field(translations[0], "text"), content="Some(\"T:a\")")
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
  let errors = frames.filter(
    frame => get_str_field(frame, "type") == Some("error"),
  )
  debug_inspect(errors.length(), content="1")
  debug_inspect(get_str_field(errors[0], "type"), content="Some(\"error\")")
  debug_inspect(
    get_field(errors[0], "last").unwrap().stringify(),
    content="\"true\"",
  )
  debug_inspect(
    get_str_field(get_field(errors[0], "error").unwrap(), "code"),
    content="Some(\"MODEL_UNAVAILABLE\")",
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
    Json::object({
      "id": Json::string("5"),
      "type": Json::string("translate"),
      "segments": Json::array([]),
    }),
  )
  debug_inspect(frames.length(), content="1")
  debug_inspect(get_str_field(frames[0], "type"), content="Some(\"translate\")")
  let translations = as_array(
    get_field(frames[0], "translations").unwrap(),
  ).unwrap()
  debug_inspect(translations.length(), content="0")
}
