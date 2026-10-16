///| White-box tests for opencode output parsing and the process runner.

///|
test "parse jsonl ignores non-JSON lines" {
  let events = parse_jsonl(
    "log line\n{\"type\":\"text\",\"part\":{\"text\":\"A\"}}\n{\"type\":\"step_start\"}\n",
  )
  debug_inspect(events.length(), content="2")
  debug_inspect(extract_text(events), content="\"A\"")
}

///|
test "extract error events picks structured errors" {
  let events = parse_jsonl(
    "{\"type\":\"error\",\"error\":{\"type\":\"x\",\"message\":\"y\"}}",
  )
  debug_inspect(
    get_str_field(extract_error_events(events)[0], "message"),
    content="Some(\"y\")",
  )
}

///| The fake opencode fixture, relative to the module root.
let fixture_path : String = "test/fixtures/fake-opencode.js"

///| Environment map for driving the fake fixture.
fn fake_env(pairs : Array[(String, String)]) -> Map[String, String] {
  Map::from_array(pairs)
}

///|
async test "runner concatenates text and forwards model/agent/prompt args" {
  let dir = @fs.tmpdir(prefix="oc-host-test")
  let args_file = dir + "/args.json"
  let runner = OpencodeRunner::new(
    bin="bun",
    bin_args=[fixture_path],
    env=fake_env([("FAKE_MODE", "aligned"), ("FAKE_ARGS_FILE", args_file)]),
  )
  let segments : Array[TranslateSegment] = [
    { index: 0, text: "hello" },
    { index: 1, text: "world" },
  ]
  let result = runner.run_once(
    "PROMPT",
    build_stdin_payload(segments),
    Some("prov/model#high"),
    Some("translator"),
    None,
  )
  debug_inspect(result.exit_code, content="Some(0)")
  assert_true(result.text.contains("hello"))
  assert_true(result.text.contains("world"))
  let args = as_array(
    parse_json_option(@fs.read_file(args_file).text()).unwrap(),
  ).unwrap()
  let expected = [
    "run",
    "--format",
    "json",
    "--model",
    "prov/model#high",
    "--agent",
    "translator",
    "PROMPT",
  ]
  debug_inspect(args.length(), content="8")
  for i = 0; i < expected.length(); i = i + 1 {
    assert_true(as_string(args[i]) == Some(expected[i]))
  }
  try {
    @fs.remove(dir)
  } catch {
    _ => ()
  }
}

///|
async test "runner surfaces a region error from the fake opencode" {
  let runner = OpencodeRunner::new(
    bin="bun",
    bin_args=[fixture_path],
    env=fake_env([("FAKE_MODE", "region")]),
  )
  let result = runner.run_once("p", "[]", None, None, None)
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"MODEL_UNAVAILABLE\"",
  )
}

///|
async test "runner kills a hanging invocation and reports a timeout" {
  let runner = OpencodeRunner::new(
    bin="bun",
    bin_args=[fixture_path],
    env=fake_env([("FAKE_MODE", "hang")]),
    timeout_ms=300,
  )
  let result = runner.run_once("p", "[]", None, None, None)
  assert_true(result.timed_out)
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"TIMEOUT\"",
  )
}

///|
async test "runner reports a missing executable" {
  let runner = OpencodeRunner::new(bin="definitely-not-a-real-binary-xyz")
  let result = runner.run_once("p", "[]", None, None, None)
  debug_inspect(
    map_opencode_failure(result).unwrap().code().name(),
    content="\"OPENCODE_MISSING\"",
  )
}
