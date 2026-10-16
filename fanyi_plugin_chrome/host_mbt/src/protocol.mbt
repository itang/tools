///| Protocol constants and shared shapes for the native messaging host.
///
/// Frames are length-prefixed JSON (Chrome native messaging stdio format).
/// Chrome limits host -> extension frames to 1 MB, so responses may be split
/// across multiple frames (see `framing.mbt` / `server.mbt`).

///| Maximum size of a single frame the host may send to the extension (1 MB).
pub const MAX_FRAME_BYTES : Int = 1024 * 1024

///| Host protocol version; bumped on incompatible frame changes.
pub const PROTOCOL_VERSION : Int = 1

///| Model used when a request does not carry one (override with the
/// `TRANSLATE_MODEL` environment variable; set it to an empty string to let
/// opencode pick its own default).
pub let default_model : String = "opencode/deepseek-v4.1-flash-free"

///| Stable set of error codes returned to the extension.
pub(all) enum ErrorCode {
  OpencodeMissing
  AuthRequired
  ModelUnavailable
  Timeout
  Incomplete
  HostFailure
  BadRequest
}

///| The wire spelling of an error code (also used in response frames).
pub fn ErrorCode::name(self : ErrorCode) -> String {
  match self {
    OpencodeMissing => "OPENCODE_MISSING"
    AuthRequired => "AUTH_REQUIRED"
    ModelUnavailable => "MODEL_UNAVAILABLE"
    Timeout => "TIMEOUT"
    Incomplete => "INCOMPLETE"
    HostFailure => "HOST_ERROR"
    BadRequest => "BAD_REQUEST"
  }
}

///| A single source segment to translate.
pub(all) struct TranslateSegment {
  index : Int
  text : String
}

///| Options attached to a `translate` request.
pub(all) struct TranslateOptions {
  target_lang : String
  model : String
  agent : String?
  timeout_ms : Int?
}

///| Progress reported while a batch request runs.
pub(all) struct Progress {
  phase : String
  completed : Int
  total : Int
}

///| One per-segment translation outcome: either `text` or `error` is set.
pub(all) struct TranslationResult {
  index : Int
  text : String?
  error : ErrorCode?
}

///| A successful translation result.
pub fn TranslationResult::ok(index : Int, text : String) -> TranslationResult {
  { index, text: Some(text), error: None }
}

///| A failed translation result carrying a stable error code.
pub fn TranslationResult::failed(
  index : Int,
  error : ErrorCode,
) -> TranslationResult {
  { index, text: None, error: Some(error) }
}
