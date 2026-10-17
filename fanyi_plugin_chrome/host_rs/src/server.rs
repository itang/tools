//! Request dispatcher: turns protocol requests into response frames.

use crate::engine::Translator;
use crate::framing::chunk_items;
use crate::protocol::{
    BoxFuture, ErrorCode, Progress, TranslateOptions, TranslateSegment, TranslationResult,
    MAX_FRAME_BYTES,
};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Normalize a request's `segments` array into a clean list.
fn normalize_segments(value: Option<&Value>) -> Vec<TranslateSegment> {
    let Some(Value::Array(items)) = value else {
        return Vec::new();
    };
    let mut segments = Vec::new();
    for item in items {
        let Some(text) = item.get("text").and_then(Value::as_str) else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        let index = item
            .get("index")
            .and_then(|value| value.as_i64().or_else(|| value.as_u64().map(|n| n as i64)))
            .unwrap_or(segments.len() as i64);
        segments.push(TranslateSegment {
            index,
            text: text.to_string(),
        });
    }
    segments
}

/// Parse a request's `options` object.
fn parse_options(value: Option<&Value>) -> TranslateOptions {
    let mut options = TranslateOptions::default();
    let Some(value) = value else {
        return options;
    };
    options.target_lang = value
        .get("targetLang")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(String::from);
    options.model = value
        .get("model")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(String::from);
    options.agent = value
        .get("agent")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(String::from);
    options.timeout_ms = value
        .get("timeoutMs")
        .and_then(Value::as_u64)
        .filter(|n| *n > 0);
    options
}

/// Dispatch one request, emitting zero or more response frames.
///
/// `health` is a boxed future-producing closure so the dispatcher stays
/// decoupled from the concrete runner.
pub async fn dispatch<E, H, F>(engine: &E, health: &H, emit: &F, request: Value)
where
    E: Translator + ?Sized,
    H: Fn() -> BoxFuture<'static, Value> + ?Sized,
    F: Fn(Value) + Send + Sync + ?Sized,
{
    let id = request
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let seq = AtomicUsize::new(0);
    let next_seq = || seq.fetch_add(1, Ordering::SeqCst);

    if !request.is_object() {
        emit(json!({
            "id": id,
            "seq": next_seq(),
            "last": true,
            "type": "error",
            "error": { "code": ErrorCode::BadRequest.name(), "message": "请求格式不正确" }
        }));
        return;
    }

    match request.get("type").and_then(Value::as_str) {
        Some("health") => {
            let data = health().await;
            emit(
                json!({ "id": id, "seq": next_seq(), "last": true, "type": "health", "data": data }),
            );
        }
        Some("translate") => {
            let segments = normalize_segments(request.get("segments"));
            if segments.is_empty() {
                emit(
                    json!({ "id": id, "seq": next_seq(), "last": true, "type": "translate", "translations": [] }),
                );
                return;
            }

            let options = parse_options(request.get("options"));
            let progress = |progress: Progress| {
                emit(json!({
                    "id": id,
                    "seq": next_seq(),
                    "last": false,
                    "type": "progress",
                    "phase": progress.phase,
                    "completed": progress.completed,
                    "total": progress.total,
                }));
            };

            match engine.translate(segments, options, Some(&progress)).await {
                Ok(translations) => {
                    let render = |chunk: &[TranslationResult]| {
                        json!({
                            "id": id,
                            "seq": 0,
                            "last": false,
                            "type": "translate",
                            "translations": chunk.iter().map(TranslationResult::to_value).collect::<Vec<_>>(),
                        })
                    };
                    let chunks = chunk_items(&translations, render, MAX_FRAME_BYTES);
                    let last = chunks.len().saturating_sub(1);
                    for (position, chunk) in chunks.iter().enumerate() {
                        emit(json!({
                            "id": id,
                            "seq": next_seq(),
                            "last": position == last,
                            "type": "translate",
                            "translations": chunk.iter().map(TranslationResult::to_value).collect::<Vec<_>>(),
                        }));
                    }
                }
                Err(error) => {
                    emit(json!({
                        "id": id,
                        "seq": next_seq(),
                        "last": true,
                        "type": "error",
                        "error": { "code": error.code.name(), "message": error.message }
                    }));
                }
            }
        }
        _ => {
            let rendered = request
                .get("type")
                .map(|value| {
                    value
                        .as_str()
                        .map(String::from)
                        .unwrap_or_else(|| value.to_string())
                })
                .unwrap_or_else(|| "undefined".to_string());
            emit(json!({
                "id": id,
                "seq": next_seq(),
                "last": true,
                "type": "error",
                "error": { "code": ErrorCode::BadRequest.name(), "message": format!("未知请求类型：{rendered}") }
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::HostError;
    use std::sync::Mutex;

    struct StubEngine {
        result: Mutex<Option<Result<Vec<TranslationResult>, HostError>>>,
    }

    impl Translator for StubEngine {
        fn translate<'a>(
            &'a self,
            segments: Vec<TranslateSegment>,
            _options: TranslateOptions,
            on_progress: Option<&'a (dyn Fn(Progress) + Send + Sync)>,
        ) -> BoxFuture<'a, Result<Vec<TranslationResult>, HostError>> {
            Box::pin(async move {
                let result = self.result.lock().unwrap().take().unwrap_or_else(|| {
                    Ok(segments
                        .iter()
                        .map(|segment| TranslationResult::ok(segment.index, "T"))
                        .collect())
                });
                if result.is_ok() {
                    if let Some(callback) = on_progress {
                        callback(Progress {
                            phase: "done".to_string(),
                            completed: segments.len(),
                            total: segments.len(),
                        });
                    }
                }
                result
            })
        }
    }

    fn collector() -> std::sync::Arc<Mutex<Vec<Value>>> {
        std::sync::Arc::new(Mutex::new(Vec::new()))
    }

    async fn run(engine: &dyn Translator, request: Value) -> Vec<Value> {
        let frames = collector();
        let sink = frames.clone();
        let emit = move |value: Value| sink.lock().unwrap().push(value);
        let health: Box<dyn Fn() -> BoxFuture<'static, Value> + Send + Sync> = Box::new(|| {
            Box::pin(async {
                json!({ "host": { "ok": true }, "opencode": { "ok": true, "version": "v" } })
            })
        });
        dispatch(engine, &health, &emit, request).await;
        let out = frames.lock().unwrap().clone();
        out
    }

    #[tokio::test]
    async fn health_request_emits_one_terminal_frame() {
        let engine = StubEngine {
            result: Mutex::new(None),
        };
        let frames = run(&engine, json!({ "id": "1", "type": "health" })).await;
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0]["last"], json!(true));
        assert_eq!(frames[0]["type"], json!("health"));
        assert_eq!(frames[0]["data"]["opencode"]["version"], json!("v"));
    }

    #[tokio::test]
    async fn unknown_request_type_is_bad_request() {
        let engine = StubEngine {
            result: Mutex::new(None),
        };
        let frames = run(&engine, json!({ "id": "2", "type": "nope" })).await;
        assert_eq!(frames[0]["error"]["code"], json!("BAD_REQUEST"));
    }

    #[tokio::test]
    async fn translate_emits_progress_then_terminal_frame() {
        let engine = StubEngine {
            result: Mutex::new(None),
        };
        let frames = run(
            &engine,
            json!({ "id": "3", "type": "translate", "segments": [{ "index": 0, "text": "a" }] }),
        )
        .await;
        assert_eq!(
            frames
                .iter()
                .filter(|frame| frame["type"] == json!("progress"))
                .count(),
            1
        );
        let terminal = frames
            .iter()
            .find(|frame| frame["type"] == json!("translate") && frame["last"] == json!(true))
            .unwrap();
        assert_eq!(
            terminal["translations"],
            json!([{ "index": 0, "text": "T" }])
        );
    }

    #[tokio::test]
    async fn host_error_becomes_error_frame_with_its_code() {
        let engine = StubEngine {
            result: Mutex::new(Some(Err(HostError::new(
                ErrorCode::ModelUnavailable,
                "no model",
            )))),
        };
        let frames = run(
            &engine,
            json!({ "id": "4", "type": "translate", "segments": [{ "index": 0, "text": "a" }] }),
        )
        .await;
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0]["type"], json!("error"));
        assert_eq!(frames[0]["error"]["code"], json!("MODEL_UNAVAILABLE"));
    }

    #[tokio::test]
    async fn empty_segment_list_yields_empty_terminal_frame() {
        let engine = StubEngine {
            result: Mutex::new(None),
        };
        let frames = run(
            &engine,
            json!({ "id": "5", "type": "translate", "segments": [] }),
        )
        .await;
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0]["type"], json!("translate"));
        assert_eq!(frames[0]["last"], json!(true));
        assert_eq!(frames[0]["translations"], json!([]));
    }
}
