//! Batch translation contract: prompt building, response parsing, alignment.

use crate::errors::HostError;
use crate::protocol::{ErrorCode, TranslateSegment, TranslationResult};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};

/// Instruction sent to the model. The source segments are supplied on stdin and
/// the model must answer with a JSON object keyed by segment index so that
/// results stay aligned even if the model reorders them.
pub fn build_instruction(target_lang: &str) -> String {
    [
        "You are a translation engine.".to_string(),
        format!(
            "Translate the \"text\" field of every item in the JSON array provided on stdin into {target_lang}."
        ),
        "Some texts contain markup tokens: [[n]] and [[/n]] wrap an inline element, {{n}} is an opaque node, {{br}} is a line break.".to_string(),
        "Keep every token byte-for-byte, in the same order; never translate, drop, renumber, duplicate or add tokens.".to_string(),
        "Translate only the natural language between tokens.".to_string(),
        "Preserve proper nouns, @handles, #hashtags, URLs, numbers and code as-is.".to_string(),
        "Do not translate text that is already in the target language.".to_string(),
        "Respond with ONLY a JSON object mapping each item index (as a string) to its translation, e.g. {\"0\":\"...\",\"1\":\"...\"}.".to_string(),
        "No prose, no markdown, no code fences.".to_string(),
    ]
    .join(" ")
}

/// Serialize segments to the stdin payload.
pub fn build_stdin_payload(segments: &[TranslateSegment]) -> String {
    let items: Vec<Value> = segments
        .iter()
        .map(|segment| json!({ "index": segment.index, "text": segment.text }))
        .collect();
    serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
}

/// Strip a Markdown fenced block, if present, and return its inner text.
fn extract_fence(text: &str) -> Option<String> {
    let start = text.to_lowercase().find("```")?;
    let rest = &text[start + 3..];
    let close = rest.find("```")?;
    let mut content = rest[..close].trim();
    if content.to_lowercase().starts_with("json") {
        content = content[4..].trim_start();
    }
    Some(content.to_string())
}

/// Pull a JSON object out of a raw model answer, tolerating markdown fences and
/// surrounding prose.
pub fn parse_translation_response(raw: &str) -> Result<Map<String, Value>, HostError> {
    let mut text = raw.trim().to_string();
    if let Some(fenced) = extract_fence(&text) {
        text = fenced;
    }

    let start = text.find('{');
    let end = text.rfind('}');
    let (Some(start), Some(end)) = (start, end) else {
        return Err(HostError::new(ErrorCode::HostError, "模型未返回 JSON 对象"));
    };
    if end < start {
        return Err(HostError::new(ErrorCode::HostError, "模型未返回 JSON 对象"));
    }

    let parsed: Value = serde_json::from_str(&text[start..=end])
        .map_err(|_| HostError::new(ErrorCode::HostError, "模型未返回 JSON 对象"))?;
    match parsed {
        Value::Object(map) => Ok(map),
        _ => Err(HostError::new(
            ErrorCode::HostError,
            "模型返回的 JSON 不是对象",
        )),
    }
}

/// Result of aligning a parsed `index -> translation` mapping onto segments.
pub struct Alignment {
    pub results: Vec<TranslationResult>,
    pub missing: Vec<TranslateSegment>,
    #[allow(dead_code)]
    pub extra: Vec<String>,
}

/// Align a parsed `index -> translation` mapping back onto the original
/// segments.
pub fn align_translations(
    segments: &[TranslateSegment],
    mapping: &Map<String, Value>,
) -> Alignment {
    let mut results = Vec::new();
    let mut missing = Vec::new();

    for segment in segments {
        let key = segment.index.to_string();
        match mapping.get(&key).and_then(Value::as_str) {
            Some(text) if !text.is_empty() => {
                results.push(TranslationResult::ok(segment.index, text))
            }
            _ => missing.push(segment.clone()),
        }
    }

    let wanted: HashSet<String> = segments
        .iter()
        .map(|segment| segment.index.to_string())
        .collect();
    let extra = mapping
        .keys()
        .filter(|key| !wanted.contains(*key))
        .cloned()
        .collect();
    Alignment {
        results,
        missing,
        extra,
    }
}

/// Turn a batch response into per-segment results, retrying the segments that
/// the model omitted exactly once.
pub async fn translate_batch<F, Fut>(
    segments: &[TranslateSegment],
    call_model: F,
) -> Result<Vec<TranslationResult>, HostError>
where
    F: Fn(Vec<TranslateSegment>) -> Fut,
    Fut: std::future::Future<Output = Result<String, HostError>>,
{
    let raw = call_model(segments.to_vec()).await?;
    let mapping = parse_translation_response(&raw)?;
    let aligned = align_translations(segments, &mapping);

    if aligned.missing.is_empty() {
        return Ok(aligned.results);
    }

    // One retry for the segments the model skipped.
    let retry_raw = call_model(aligned.missing.clone()).await?;
    let retry_mapping = parse_translation_response(&retry_raw)?;
    let retry_aligned = align_translations(&aligned.missing, &retry_mapping);

    let mut by_index: HashMap<i64, TranslationResult> = aligned
        .results
        .into_iter()
        .map(|result| (result.index, result))
        .collect();
    for item in retry_aligned.results {
        by_index.insert(item.index, item);
    }
    for item in retry_aligned.missing {
        by_index.insert(
            item.index,
            TranslationResult::failed(item.index, ErrorCode::Incomplete),
        );
    }

    Ok(segments
        .iter()
        .map(|segment| {
            by_index
                .get(&segment.index)
                .cloned()
                .unwrap_or_else(|| TranslationResult::failed(segment.index, ErrorCode::Incomplete))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segments() -> Vec<TranslateSegment> {
        vec![
            TranslateSegment {
                index: 0,
                text: "a".into(),
            },
            TranslateSegment {
                index: 1,
                text: "b".into(),
            },
        ]
    }

    #[test]
    fn parses_fenced_json_and_surrounding_prose() {
        let fenced = parse_translation_response("```json\n{\"0\":\"A\",\"1\":\"B\"}\n```").unwrap();
        assert_eq!(fenced.get("0").and_then(Value::as_str), Some("A"));

        let prose = parse_translation_response("Here you go: {\"0\":\"A\"} — done").unwrap();
        assert_eq!(prose.get("0").and_then(Value::as_str), Some("A"));
    }

    #[test]
    fn errors_when_no_json_object_is_present() {
        assert!(parse_translation_response("no json here").is_err());
    }

    #[test]
    fn aligns_exact_shuffled_and_extra_keys() {
        let segments = vec![
            TranslateSegment {
                index: 0,
                text: "a".into(),
            },
            TranslateSegment {
                index: 1,
                text: "b".into(),
            },
            TranslateSegment {
                index: 2,
                text: "c".into(),
            },
        ];

        let full: Map<String, Value> =
            serde_json::from_str("{\"0\":\"A\",\"1\":\"B\",\"2\":\"C\"}").unwrap();
        let aligned = align_translations(&segments, &full);
        assert_eq!(
            aligned
                .results
                .iter()
                .filter_map(|r| r.text.clone())
                .collect::<Vec<_>>(),
            vec!["A", "B", "C"]
        );
        assert!(aligned.missing.is_empty());

        let partial: Map<String, Value> =
            serde_json::from_str("{\"0\":\"A\",\"2\":\"C\",\"9\":\"X\"}").unwrap();
        let partial_aligned = align_translations(&segments, &partial);
        assert_eq!(
            partial_aligned
                .missing
                .iter()
                .map(|s| s.index)
                .collect::<Vec<_>>(),
            vec![1]
        );
        assert_eq!(partial_aligned.extra, vec!["9"]);
    }

    #[tokio::test]
    async fn retries_missing_segments_once_and_marks_unrecovered_ones() {
        let segments = segments();
        let calls = std::cell::Cell::new(0);
        let results = translate_batch(&segments, |_batch| {
            let call = calls.get() + 1;
            calls.set(call);
            async move {
                Ok(if call == 1 {
                    "{\"0\":\"A\"}".to_string()
                } else {
                    "{}".to_string()
                })
            }
        })
        .await
        .unwrap();

        assert_eq!(
            results
                .iter()
                .find(|r| r.index == 0)
                .and_then(|r| r.text.clone()),
            Some("A".to_string())
        );
        assert_eq!(
            results.iter().find(|r| r.index == 1).and_then(|r| r.error),
            Some(ErrorCode::Incomplete)
        );
        assert_eq!(calls.get(), 2);
    }

    #[tokio::test]
    async fn recovers_omitted_segments_on_retry() {
        let segments = segments();
        let calls = std::cell::Cell::new(0);
        let results = translate_batch(&segments, |_batch| {
            let call = calls.get() + 1;
            calls.set(call);
            async move {
                Ok(if call == 1 {
                    "{\"0\":\"A\"}".to_string()
                } else {
                    "{\"1\":\"B\"}".to_string()
                })
            }
        })
        .await
        .unwrap();

        let rendered: Vec<Value> = results.iter().map(TranslationResult::to_value).collect();
        assert_eq!(
            rendered,
            vec![json!({"index":0,"text":"A"}), json!({"index":1,"text":"B"})]
        );
    }
}
