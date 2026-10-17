//! Protocol constants and shared shapes for the native messaging host.
//!
//! Frames are length-prefixed JSON (Chrome native messaging stdio format).
//! Chrome limits host -> extension frames to 1 MB, so responses may be split
//! across multiple frames (see `framing` / `server`).

use serde_json::{json, Map, Value};
use std::future::Future;
use std::pin::Pin;

/// Maximum size of a single frame the host may send to the extension.
pub const MAX_FRAME_BYTES: usize = 1024 * 1024; // 1 MB (Chrome host -> extension limit)

/// Host protocol version; bumped on incompatible frame changes.
pub const PROTOCOL_VERSION: u64 = 1;

/// Model used when a request does not carry one (override with the
/// `TRANSLATE_MODEL` environment variable; set it to an empty string to let
/// opencode pick its own default).
pub const DEFAULT_MODEL: &str = "opencode/deepseek-v4.1-flash-free";

/// A boxed, `Send` future used at async trait boundaries.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Stable set of error codes returned to the extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    OpencodeMissing,
    AuthRequired,
    ModelUnavailable,
    Timeout,
    Incomplete,
    HostError,
    BadRequest,
}

impl ErrorCode {
    /// The wire spelling of an error code (also used in response frames).
    pub fn name(self) -> &'static str {
        match self {
            ErrorCode::OpencodeMissing => "OPENCODE_MISSING",
            ErrorCode::AuthRequired => "AUTH_REQUIRED",
            ErrorCode::ModelUnavailable => "MODEL_UNAVAILABLE",
            ErrorCode::Timeout => "TIMEOUT",
            ErrorCode::Incomplete => "INCOMPLETE",
            ErrorCode::HostError => "HOST_ERROR",
            ErrorCode::BadRequest => "BAD_REQUEST",
        }
    }
}

/// A single source segment to translate.
#[derive(Clone, Debug)]
pub struct TranslateSegment {
    /// Caller assigned index, echoed back in the response.
    pub index: i64,
    /// Source text to translate.
    pub text: String,
}

/// Options attached to a `translate` request.
#[derive(Clone, Debug, Default)]
pub struct TranslateOptions {
    pub target_lang: Option<String>,
    pub model: Option<String>,
    pub agent: Option<String>,
    pub timeout_ms: Option<u64>,
}

/// Progress reported while a batch request runs.
#[derive(Clone, Debug)]
pub struct Progress {
    pub phase: String,
    pub completed: usize,
    pub total: usize,
}

/// One per-segment translation outcome: either `text` or `error` is set.
#[derive(Clone, Debug)]
pub struct TranslationResult {
    pub index: i64,
    pub text: Option<String>,
    pub error: Option<ErrorCode>,
}

impl TranslationResult {
    /// A successful translation result.
    pub fn ok(index: i64, text: impl Into<String>) -> Self {
        Self {
            index,
            text: Some(text.into()),
            error: None,
        }
    }

    /// A failed translation result carrying a stable error code.
    pub fn failed(index: i64, error: ErrorCode) -> Self {
        Self {
            index,
            text: None,
            error: Some(error),
        }
    }

    /// Render the result as the JSON object the extension expects.
    pub fn to_value(&self) -> Value {
        let mut map = Map::new();
        map.insert("index".to_string(), json!(self.index));
        if let Some(text) = &self.text {
            map.insert("text".to_string(), json!(text));
        }
        if let Some(error) = self.error {
            map.insert("error".to_string(), json!(error.name()));
        }
        Value::Object(map)
    }
}
