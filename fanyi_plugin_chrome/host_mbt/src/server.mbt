///| Request dispatcher: turns protocol requests into response frames.

///|
/// Build an error response frame.
fn error_frame(
  id : String,
  seq : Int32,
  code : ErrorCode,
  message : String,
) -> Json {
  {
    "id": id,
    "seq": seq,
    "last": true,
    "type": "error",
    "error": { "code": code.name(), "message": message },
  }
}

///|
/// Render translation results as a JSON array.
pub fn translations_json(results : Array[TranslationResult]) -> Json {
  Json::array(
    results.map(result => {
      match result.text {
        Some(text) => { "index": result.index, "text": text }
        None =>
          {
            "index": result.index,
            "error": result.error.unwrap_or(HostFailure).name(),
          }
      }
    }),
  )
}

///|
/// Normalize a request's `segments` field into usable segments.
pub fn normalize_segments(value : Json?) -> Array[TranslateSegment] {
  let segments : Array[TranslateSegment] = []
  guard value is Some(Array(items)) else { return segments }
  for item in items {
    guard item is { "text": String(text), .. } && text.length() > 0 else {
      continue
    }
    let index = if item is { "index": Number(value, ..), .. } {
      value.to_int()
    } else {
      segments.length()
    }
    segments.push({ index, text, })
  }
  segments
}

///|
/// Read the optional `options` object of a translate request.
fn parse_options(options : Json?) -> TranslateOptions {
  let target_lang = match options {
    Some({ "targetLang": String(target_lang), .. }) => target_lang
    _ => ""
  }
  let model = match options {
    Some({ "model": String(model), .. }) => model
    _ => ""
  }
  let agent = match options {
    Some({ "agent": String(agent), .. }) if agent.length() > 0 => Some(agent)
    _ => None
  }
  let timeout_ms = match options {
    Some({ "timeoutMs": Number(timeout_ms, ..), .. }) =>
      Some(timeout_ms.to_int())
    _ => None
  }
  { target_lang, model, agent, timeout_ms, }
}

///|
/// Turns protocol requests into response frames.
pub struct Dispatcher {
  engine : TranslationEngine
  runner : &Runner
  bin : String
  emit : async (Json) -> Unit
}

///|
/// Create a dispatcher.
pub fn Dispatcher::new(
  engine : TranslationEngine,
  runner : &Runner,
  bin : String,
  emit : async (Json) -> Unit,
) -> Dispatcher {
  { engine, runner, bin, emit, }
}

///|
/// Dispatch one request, emitting one or more response frames.
pub async fn Dispatcher::dispatch(self : Dispatcher, request : Json) -> Unit {
  let id = match request {
    { "id": String(id), .. } => id
    _ => ""
  }
  let mut seq = 0
  match request {
    { "type": "health", .. } => {
      let data = check_health(self.runner, self.bin)
      (self.emit)({
        "id": id,
        "seq": seq,
        "last": true,
        "type": "health",
        "data": data,
      })
    }
    { "type": "translate", "segments"? : segments, "options"? : options, .. } => {
      let segments = normalize_segments(segments)
      if segments.is_empty() {
        (self.emit)({
          "id": id,
          "seq": seq,
          "last": true,
          "type": "translate",
          "translations": Json::array([]),
        })
        return
      }
      let options = parse_options(options)
      let outcome : Array[TranslationResult]? = Some(
        self.engine.translate(segments, options, progress => {
          (self.emit)({
            "id": id,
            "seq": seq,
            "last": false,
            "type": "progress",
            "phase": progress.phase,
            "completed": progress.completed,
            "total": progress.total,
          })
          seq = seq + 1
        }),
      ) catch {
        HostError::HostError(code, message) => {
          (self.emit)(error_frame(id, seq, code, message))
          None
        }
        err => {
          (self.emit)(error_frame(id, seq, HostFailure, err.to_string()))
          None
        }
      }
      match outcome {
        Some(translations) => {
          let render : (Array[TranslationResult]) -> Json = chunk => {
            "id": id,
            "seq": 0,
            "last": false,
            "type": "translate",
            "translations": translations_json(chunk),
          }
          let chunks = chunk_items(translations, render, MAX_FRAME_BYTES)
          for position = 0; position < chunks.length(); position = position + 1 {
            (self.emit)({
              "id": id,
              "seq": seq,
              "last": position == chunks.length() - 1,
              "type": "translate",
              "translations": translations_json(chunks[position]),
            })
            seq = seq + 1
          }
        }
        None => ()
      }
    }
    _ => {
      let message = match request {
        { "type": String(kind), .. } if kind.length() > 0 =>
          "未知请求类型：\{kind}"
        _ => "请求格式不正确"
      }
      (self.emit)(error_frame(id, seq, BadRequest, message))
    }
  }
}
