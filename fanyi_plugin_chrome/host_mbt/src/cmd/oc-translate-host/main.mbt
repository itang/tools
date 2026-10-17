///| Native messaging host entry point.
///
/// Reads length-prefixed JSON frames from stdin, dispatches them, and writes
/// response frames to stdout. Exits when Chrome closes the channel (stdin EOF).

///|
/// Parse a positive integer from a string, or `None`.
fn parse_positive_int(raw : String) -> Int32? {
  let text = raw.trim().to_owned()
  if text.is_empty() {
    return None
  }
  let mut index = 0
  let mut value = 0
  let mut any = false
  while index < text.length() {
    match text.get_char(index) {
      Some(c) =>
        if c.is_ascii_digit() {
          value = value * 10 + (c.to_int() - '0'.to_int())
          any = true
        } else {
          return None
        }
      None => ()
    }
    index = index + 1
  }
  if any && value > 0 {
    Some(value)
  } else {
    None
  }
}

///|
/// Read a positive integer environment variable, falling back on missing or
/// malformed values.
fn int_from_env(name : String, fallback : Int32) -> Int32 {
  match @env.get_env_var(name) {
    Some(raw) =>
      match parse_positive_int(raw) {
        Some(value) => value
        None => fallback
      }
    None => fallback
  }
}

///|
/// Parse the `OPENCODE_BIN_ARGS` JSON array, ignoring malformed values.
fn parse_bin_args() -> Array[String] {
  guard @env.get_env_var("OPENCODE_BIN_ARGS") is Some(raw) else { return [] }
  guard @host.parse_json_option(raw) is Some(Array(items)) else { return [] }
  items.filter_map(item => if item is String(text) { Some(text) } else { None })
}

///|
async fn main {
  let timeout_ms = int_from_env("TRANSLATE_TIMEOUT_MS", 120_000)
  let concurrency = int_from_env("TRANSLATE_CONCURRENCY", 2)
  let cache_max = int_from_env("TRANSLATE_CACHE_MAX", 2000)
  // `TRANSLATE_MODEL` overrides the built-in default; set it to an empty
  // string to let opencode pick its own default model instead.
  let model = match @env.get_env_var("TRANSLATE_MODEL") {
    Some(value) => value
    None => @host.default_model
  }
  let runner = @host.OpencodeRunner::new(bin_args=parse_bin_args(), timeout_ms~)
  let engine = @host.TranslationEngine::new(
    runner,
    concurrency~,
    cache_max~,
    timeout_ms~,
    default_model=model,
  )
  let write_lock = @async.Mutex::Mutex()
  let emit : async (Json) -> Unit = frame => {
    write_lock.acquire()
    defer write_lock.release()
    @stdio.stdout.write(@host.encode_frame(frame))
  }
  let dispatcher = @host.Dispatcher::new(engine, runner, runner.bin(), emit)
  @async.with_task_group(group => {
    let decoder = @host.FrameDecoder::new()
    let mut eof = false
    while !eof {
      match @stdio.stdin.read_some() {
        Some(chunk) =>
          if chunk.length() == 0 {
            eof = true
          } else {
            for message in decoder.push(chunk[:]) {
              match message {
                Some(request) =>
                  group.spawn_bg(() => dispatcher.dispatch(request))
                None => ()
              }
            }
          }
        None => eof = true
      }
    }
  })
}
