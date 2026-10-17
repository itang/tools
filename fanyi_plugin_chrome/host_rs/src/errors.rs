//! Error classification for opencode failures.

use crate::protocol::ErrorCode;
use serde_json::Value;
use std::fmt;

/// Error carrying one of the stable protocol error codes.
#[derive(Debug, Clone)]
pub struct HostError {
    pub code: ErrorCode,
    pub message: String,
}

impl HostError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HostError {}

/// Matches region / country restrictions reported by providers.
const REGION_PATTERNS: [&str; 4] = [
    "not available in your country",
    "regionerror",
    "not available in your region",
    "unsupported_country",
];

/// Decide whether an opencode error event describes a region restriction.
pub fn looks_like_region_error(error: &Value) -> bool {
    let mut parts: Vec<&str> = Vec::new();
    if let Some(message) = error.get("message").and_then(Value::as_str) {
        parts.push(message);
    }
    if let Some(body) = error
        .get("response")
        .and_then(|r| r.get("body"))
        .and_then(Value::as_str)
    {
        parts.push(body);
    }
    if let Some(kind) = error.get("type").and_then(Value::as_str) {
        parts.push(kind);
    }
    let haystack = parts.join(" ").to_lowercase();
    REGION_PATTERNS
        .iter()
        .any(|pattern| haystack.contains(pattern))
}

/// Map the outcome of an opencode invocation to a [`HostError`], or return
/// `None` when the invocation succeeded.
pub fn map_opencode_failure(
    spawn_error: Option<&str>,
    timed_out: bool,
    error_events: &[Value],
    exit_code: Option<i32>,
    stderr: &str,
) -> Option<HostError> {
    if let Some(message) = spawn_error {
        return Some(HostError::new(
            ErrorCode::OpencodeMissing,
            format!("无法启动 opencode：{message}"),
        ));
    }
    if timed_out {
        return Some(HostError::new(ErrorCode::Timeout, "opencode 调用超时"));
    }

    for event in error_events {
        if looks_like_region_error(event) {
            let message = event
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("当前模型在你的区域不可用，请在设置中更换模型");
            return Some(HostError::new(ErrorCode::ModelUnavailable, message));
        }
    }
    for event in error_events {
        let kind = event.get("type").and_then(Value::as_str).unwrap_or("");
        if kind.contains("auth") {
            let message = event
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("opencode 认证失败，请先登录或配置 provider");
            return Some(HostError::new(ErrorCode::AuthRequired, message));
        }
    }
    if let Some(event) = error_events.first() {
        let message = event
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("opencode 返回了错误");
        return Some(HostError::new(ErrorCode::HostError, message));
    }

    if exit_code != Some(0) {
        let detail: String = stderr.trim().chars().take(500).collect();
        let message = if detail.is_empty() {
            format!(
                "opencode 以非零状态退出（{}）",
                exit_code.map_or("unknown".into(), |c| c.to_string())
            )
        } else {
            detail
        };
        return Some(HostError::new(ErrorCode::HostError, message));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn spawn_failure_is_opencode_missing() {
        let error = map_opencode_failure(Some("spawn opencode ENOENT"), false, &[], None, "");
        assert_eq!(error.unwrap().code, ErrorCode::OpencodeMissing);
    }

    #[test]
    fn region_error_is_model_unavailable() {
        let events = vec![json!({
            "type": "provider.auth",
            "message": "This model is not available in your country",
            "status": 403
        })];
        let error = map_opencode_failure(None, false, &events, Some(1), "");
        assert_eq!(error.unwrap().code, ErrorCode::ModelUnavailable);
    }

    #[test]
    fn authentication_error_is_auth_required() {
        let events = vec![json!({ "type": "provider.auth", "message": "Unauthorized" })];
        let error = map_opencode_failure(None, false, &events, Some(1), "");
        assert_eq!(error.unwrap().code, ErrorCode::AuthRequired);
    }

    #[test]
    fn timeout_wins() {
        let error = map_opencode_failure(None, true, &[], None, "");
        assert_eq!(error.unwrap().code, ErrorCode::Timeout);
    }

    #[test]
    fn non_zero_exit_is_host_error_with_stderr_detail() {
        let error = map_opencode_failure(None, false, &[], Some(2), "boom").unwrap();
        assert_eq!(error.code, ErrorCode::HostError);
        assert!(error.message.contains("boom"));
    }

    #[test]
    fn successful_run_maps_to_none() {
        assert!(map_opencode_failure(None, false, &[], Some(0), "").is_none());
    }
}
