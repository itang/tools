///| Request dispatcher: turns protocol requests into response frames.

///| Build an outbound frame object with `id`/`seq`/`last` plus `fields`.
fn make_frame(
  id : String,
  seq : Int,
  last : Bool,
  fields : Array[(String, Json)],
) -> Json {
  let map : Map[String, Json] = Map([])
  map.set("id", Json::string(id))
  map.set("seq", json_int(seq))
  map.set("last", Json::boolean(last))
  for field in fields {
    map.set(field.0, field.1)
  }
  Json::object(map)
}

///| Build an error response frame.
fn error_frame(
  id : String,
  seq : Int,
  code : ErrorCode,
  message : String,
) -> Json {
  make_frame(id, seq, true, [
    ("type", Json::string("error")),
    (
      "error",
      Json::object({
        "code": Json::string(code.name()),
        "message": Json::string(message),
      }),
    ),
  ])
}

///| Render translation results as a JSON array.
pub fn translations_json(results : Array[TranslationResult]) -> Json {
  Json::array(
    results.map(result => {
      match result.text {
        Some(text) =>
          Json::object({
            "index": json_int(result.index),
            "text": Json::string(text),
          })
        None =>
          Json::object({
            "index": json_int(result.index),
            "error": Json::string(
              match result.error {
                Some(code) => code.name()
                None => HostFailure.name()
              },
            ),
          })
      }
    }),
  )
}

///| Normalize a request's `segments` field into usable segments.
pub fn normalize_segments(value : Json?) -> Array[TranslateSegment] {
  let segments : Array[TranslateSegment] = []
  match value {
    Some(json) =>
      match as_array(json) {
        Some(items) =>
          for item in items {
            match get_str_field(item, "text") {
              Some(text) =>
                if text.length() > 0 {
                  let index = match get_int_field(item, "index") {
                    Some(index) => index
                    None => segments.length()
                  }
                  segments.push({ index, text })
                }
              None => ()
            }
          }
        None => ()
      }
    None => ()
  }
  segments
}

///| Read the `options` field of a translate request.
fn parse_options(request : Json) -> TranslateOptions {
  let options = get_field(request, "options")
  let target_lang = match options {
    Some(options) => get_str_field(options, "targetLang").unwrap_or("")
    None => ""
  }
  let model = match options {
    Some(options) => get_str_field(options, "model").unwrap_or("")
    None => ""
  }
  let agent = match options {
    Some(options) =>
      match get_str_field(options, "agent") {
        Some(agent) if agent.length() > 0 => Some(agent)
        _ => None
      }
    None => None
  }
  let timeout_ms = match options {
    Some(options) => get_int_field(options, "timeoutMs")
    None => None
  }
  { target_lang, model, agent, timeout_ms }
}

///| Turns protocol requests into response frames.
pub struct Dispatcher {
  engine : TranslationEngine
  runner : &Runner
  bin : String
  emit : async (Json) -> Unit
}

///| Create a dispatcher.
pub fn Dispatcher::new(
  engine : TranslationEngine,
  runner : &Runner,
  bin : String,
  emit : async (Json) -> Unit,
) -> Dispatcher {
  { engine, runner, bin, emit }
}

///| Dispatch one request, emitting one or more response frames.
pub async fn Dispatcher::dispatch(
  self : Dispatcher,
  request : Json,
) -> Unit {
  let id = get_str_field(request, "id").unwrap_or("")
  let mut seq = 0
  let kind = get_str_field(request, "type").unwrap_or("")
  match kind {
    "health" => {
      let data = check_health(self.runner, self.bin)
      (self.emit)(
        make_frame(id, seq, true, [
          ("type", Json::string("health")),
          ("data", data),
        ]),
      )
    }
    "translate" => {
      let segments = normalize_segments(get_field(request, "segments"))
      if segments.is_empty() {
        (self.emit)(
          make_frame(id, seq, true, [
            ("type", Json::string("translate")),
            ("translations", Json::array([])),
          ]),
        )
        return
      }
      let options = parse_options(request)
      let outcome : Array[TranslationResult]? = try {
        Some(
          self
            .engine
            .translate(segments, options, progress => {
              let frame = make_frame(id, seq, false, [
                ("type", Json::string("progress")),
                ("phase", Json::string(progress.phase)),
                ("completed", json_int(progress.completed)),
                ("total", json_int(progress.total)),
              ])
              seq = seq + 1
              (self.emit)(frame)
            }),
        )
      } catch {
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
          let render = chunk => {
            make_frame(id, 0, false, [
              ("type", Json::string("translate")),
              ("translations", translations_json(chunk)),
            ])
          }
          let chunks = chunk_items(translations, render, MAX_FRAME_BYTES)
          for position = 0; position < chunks.length(); position = position + 1 {
            let chunk = chunks[position]
            (self.emit)(
              make_frame(id, seq, position == chunks.length() - 1, [
                ("type", Json::string("translate")),
                ("translations", translations_json(chunk)),
              ]),
            )
            seq = seq + 1
          }
        }
        None => ()
      }
    }
    _ => {
      let message = if kind.length() > 0 {
        "未知请求类型：\{kind}"
      } else {
        "请求格式不正确"
      }
      (self.emit)(error_frame(id, seq, BadRequest, message))
    }
  }
}
