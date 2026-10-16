///| White-box tests for the batch translation contract.

///| A fixed three-segment batch for alignment tests.
fn test_segments() -> Array[TranslateSegment] {
  [{ index: 0, text: "a" }, { index: 1, text: "b" }, { index: 2, text: "c" }]
}

///|
test "parse translation response tolerates fences and prose" {
  let fenced = parse_translation_response(
    "```json\n{\"0\":\"A\",\"1\":\"B\"}\n```",
  )
  debug_inspect(as_string(fenced.get("0").unwrap()), content="Some(\"A\")")
  debug_inspect(as_string(fenced.get("1").unwrap()), content="Some(\"B\")")
  let prose = parse_translation_response("Here you go: {\"0\":\"A\"} — done")
  debug_inspect(as_string(prose.get("0").unwrap()), content="Some(\"A\")")
}

///|
test "parse translation response fails without a JSON object" {
  let mut failed = false
  try {
    let _ = parse_translation_response("no json here")
  } catch {
    _ => failed = true
  }
  assert_true(failed)
}

///|
test "align translations handles exact, partial and extra keys" {
  let segments = test_segments()
  let aligned = align_translations(
    segments,
    Map::from_array([
      ("0", Json::string("A")),
      ("1", Json::string("B")),
      ("2", Json::string("C")),
    ]),
  )
  debug_inspect(
    aligned.results.map(result => result.text.unwrap()),
    content="[\"A\", \"B\", \"C\"]",
  )
  debug_inspect(aligned.missing.length(), content="0")
  let partial = align_translations(
    segments,
    Map::from_array([
      ("0", Json::string("A")),
      ("2", Json::string("C")),
      ("9", Json::string("X")),
    ]),
  )
  debug_inspect(partial.missing.map(segment => segment.index), content="[1]")
  debug_inspect(partial.extra, content="[\"9\"]")
}

///|
async test "translate batch retries missing segments once and marks unrecovered" {
  let segments : Array[TranslateSegment] = [
    { index: 0, text: "a" },
    { index: 1, text: "b" },
  ]
  let mut calls = 0
  let results = translate_batch(segments, _batch => {
    calls = calls + 1
    if calls == 1 {
      "{\"0\":\"A\"}"
    } else {
      "{}"
    }
  })
  debug_inspect(results[0].text, content="Some(\"A\")")
  debug_inspect(results[1].error.unwrap().name(), content="\"INCOMPLETE\"")
  debug_inspect(calls, content="2")
}

///|
async test "translate batch recovers omitted segments on retry" {
  let segments : Array[TranslateSegment] = [
    { index: 0, text: "a" },
    { index: 1, text: "b" },
  ]
  let mut calls = 0
  let results = translate_batch(segments, _batch => {
    calls = calls + 1
    if calls == 1 {
      "{\"0\":\"A\"}"
    } else {
      "{\"1\":\"B\"}"
    }
  })
  debug_inspect(results[0].text, content="Some(\"A\")")
  debug_inspect(results[1].text, content="Some(\"B\")")
}
