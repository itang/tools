///| Error classification for opencode failures.

///| Error carrying one of the stable protocol error codes.
pub(all) suberror HostError {
  HostError(ErrorCode, String)
}

///| The stable code carried by a `HostError`.
pub fn HostError::code(self : HostError) -> ErrorCode {
  match self {
    HostError(code, _) => code
  }
}

///| The human readable message carried by a `HostError`.
pub fn HostError::message(self : HostError) -> String {
  match self {
    HostError(_, message) => message
  }
}

///| Lowercased haystack built from an error event's message, body and type.
fn error_haystack(error : Json) -> String {
  let parts : Array[String] = []
  for key in ["message", "type"] {
    match get_str_field(error, key) {
      Some(value) => parts.push(value)
      None => ()
    }
  }
  match get_field(error, "response") {
    Some(response) =>
      match get_str_field(response, "body") {
        Some(body) => parts.push(body)
        None => ()
      }
    None => ()
  }
  parts.join(" ").to_lower()
}

///| Match a message against a substring, case-insensitively.
fn contains_ci(haystack : String, needle : String) -> Bool {
  haystack.to_lower().contains(needle.to_lower())
}

///| Decide whether an opencode error event describes a region restriction.
pub fn looks_like_region_error(error : Json) -> Bool {
  let haystack = error_haystack(error)
  contains_ci(haystack, "not available in your country") ||
  contains_ci(haystack, "regionerror") ||
  contains_ci(haystack, "not available in your region") ||
  contains_ci(haystack, "unsupported_country")
}

///| Map the outcome of an opencode invocation to a `HostError`, or `None` when
/// the invocation succeeded.
pub fn map_opencode_failure(result : RunResult) -> HostError? {
  match result.spawn_error {
    Some(message) =>
      return Some(
        HostError(OpencodeMissing, "无法启动 opencode：\{message}"),
      )
    None => ()
  }
  if result.timed_out {
    return Some(HostError(Timeout, "opencode 调用超时"))
  }
  for event in result.error_events {
    if looks_like_region_error(event) {
      let message = match get_str_field(event, "message") {
        Some(message) => message
        None => "当前模型在你的区域不可用，请在设置中更换模型"
      }
      return Some(HostError(ModelUnavailable, message))
    }
  }
  for event in result.error_events {
    let event_type = match get_str_field(event, "type") {
      Some(value) => value
      None => ""
    }
    if event_type.contains("auth") {
      let message = match get_str_field(event, "message") {
        Some(message) => message
        None => "opencode 认证失败，请先登录或配置 provider"
      }
      return Some(HostError(AuthRequired, message))
    }
  }
  if !result.error_events.is_empty() {
    let message = match get_str_field(result.error_events[0], "message") {
      Some(message) => message
      None => "opencode 返回了错误"
    }
    return Some(HostError(HostFailure, message))
  }
  match result.exit_code {
    Some(0) => None
    _ => {
      let detail = result.stderr.trim().to_owned()
      let detail = if detail.length() > 500 {
        detail[:500].to_owned()
      } else {
        detail
      }
      let message = if detail.length() > 0 {
        detail
      } else {
        "opencode 以非零状态退出（\{result.exit_code.unwrap_or(-1)}）"
      }
      Some(HostError(HostFailure, message))
    }
  }
}
