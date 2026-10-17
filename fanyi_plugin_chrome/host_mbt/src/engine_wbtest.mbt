///| White-box tests for the translation engine.

///|
/// Options with no explicit model, language or agent.
fn engine_options() -> TranslateOptions {
  { target_lang: "", model: "", agent: None, timeout_ms: None, }
}

///|
async test "engine serves a repeated segment from cache" {
  let calls = TestCounter::new()
  let runner = StubRunner::new(
    (stdin, _model) => {
      calls.bump()
      translate_all_result(stdin)
    },
    (_args, _stdin) => ok_result(""),
  )
  let engine = TranslationEngine::new(
    runner,
    concurrency=2,
    cache_max=100,
    default_model="",
  )
  let first = engine.translate(
    [{ index: 0, text: "hello", }],
    engine_options(),
    _progress => (),
  )
  let second = engine.translate(
    [{ index: 5, text: "hello", }],
    engine_options(),
    _progress => (),
  )
  debug_inspect(calls.value, content="1")
  debug_inspect(first[0].text, content="Some(\"T:hello\")")
  debug_inspect(second[0].text, content="Some(\"T:hello\")")
  debug_inspect(second[0].index, content="5")
}

///|
async test "engine deduplicates identical text within one request" {
  let calls = TestCounter::new()
  let runner = StubRunner::new(
    (stdin, _model) => {
      calls.bump()
      translate_all_result(stdin)
    },
    (_args, _stdin) => ok_result(""),
  )
  let engine = TranslationEngine::new(runner, default_model="")
  let results = engine.translate(
    [{ index: 0, text: "same", }, { index: 1, text: "same", }],
    engine_options(),
    _progress => (),
  )
  debug_inspect(calls.value, content="1")
  debug_inspect(
    results.map(result => result.text.unwrap()),
    content="[\"T:same\", \"T:same\"]",
  )
}

///|
async test "engine surfaces region errors as MODEL_UNAVAILABLE" {
  let runner = StubRunner::new(
    (_stdin, _model) => failed_result(Some(1), "", [region_event()]),
    (_args, _stdin) => ok_result(""),
  )
  let engine = TranslationEngine::new(runner, default_model="")
  let mut caught : ErrorCode? = None
  ignore(
    engine.translate([{ index: 0, text: "x", }], engine_options(), _progress => {
      ()
    }),
  ) catch {
    HostError::HostError(code, _) => caught = Some(code)
    _ => ()
  }
  debug_inspect(caught.unwrap().name(), content="\"MODEL_UNAVAILABLE\"")
}

///|
async test "engine marks segments the model omits as INCOMPLETE" {
  let runner = StubRunner::new((_stdin, _model) => ok_result("{}"), (
    _args,
    _stdin,
  ) => ok_result(""))
  let engine = TranslationEngine::new(runner, default_model="")
  let results = engine.translate([{ index: 0, text: "a", }], engine_options(), _progress => {
    ()
  })
  debug_inspect(results[0].error.unwrap().name(), content="\"INCOMPLETE\"")
}

///|
async test "engine falls back to the default model" {
  let models : Array[String] = []
  let runner = StubRunner::new(
    (stdin, model) => {
      models.push(model.unwrap_or(""))
      translate_all_result(stdin)
    },
    (_args, _stdin) => ok_result(""),
  )
  let engine = TranslationEngine::new(runner, default_model="fallback/model")
  ignore(
    engine.translate([{ index: 0, text: "hello", }], engine_options(), _progress => {
      ()
    }),
  )
  debug_inspect(models[0], content="\"fallback/model\"")
}

///|
async test "engine prefers an explicit request model over the default" {
  let models : Array[String] = []
  let runner = StubRunner::new(
    (stdin, model) => {
      models.push(model.unwrap_or(""))
      translate_all_result(stdin)
    },
    (_args, _stdin) => ok_result(""),
  )
  let engine = TranslationEngine::new(runner, default_model="fallback/model")
  ignore(
    engine.translate(
      [{ index: 0, text: "hello", }],
      {
        target_lang: "",
        model: "prov/explicit#high",
        agent: None,
        timeout_ms: None,
      },
      _progress => (),
    ),
  )
  debug_inspect(models[0], content="\"prov/explicit#high\"")
}

///|
async test "engine reports progress and preserves input order" {
  let phases : Array[String] = []
  let runner = StubRunner::new((stdin, _model) => translate_all_result(stdin), (
    _args,
    _stdin,
  ) => ok_result(""))
  let engine = TranslationEngine::new(runner, default_model="")
  let results = engine.translate(
    [{ index: 10, text: "a", }, { index: 20, text: "b", }],
    engine_options(),
    progress => phases.push(progress.phase),
  )
  debug_inspect(results.map(result => result.index), content="[10, 20]")
  assert_true(phases.contains("done"))
}
