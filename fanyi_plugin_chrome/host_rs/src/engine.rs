//! Translation engine: cache lookup, batching, queueing, error mapping.

use crate::batch::{build_instruction, build_stdin_payload, translate_batch};
use crate::cache::{cache_key, LruCache};
use crate::errors::{map_opencode_failure, HostError};
use crate::opencode::OpencodeRunner;
use crate::protocol::{
    BoxFuture, ErrorCode, Progress, TranslateOptions, TranslateSegment, TranslationResult,
};
use crate::queue::TaskQueue;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// The translation workhorse: cache + queue + batching over the opencode CLI.
pub struct TranslationEngine {
    runner: OpencodeRunner,
    queue: TaskQueue,
    cache: Mutex<LruCache>,
    timeout_ms: u64,
    default_model: String,
}

/// A thing that can translate a batch of segments (implemented by the engine
/// and by test stubs).
pub trait Translator: Send + Sync {
    fn translate<'a>(
        &'a self,
        segments: Vec<TranslateSegment>,
        options: TranslateOptions,
        on_progress: Option<&'a (dyn Fn(Progress) + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Vec<TranslationResult>, HostError>>;
}

impl TranslationEngine {
    pub fn new(
        runner: OpencodeRunner,
        queue: TaskQueue,
        cache: LruCache,
        timeout_ms: u64,
        default_model: String,
    ) -> Self {
        Self {
            runner,
            queue,
            cache: Mutex::new(cache),
            timeout_ms,
            default_model,
        }
    }

    async fn call_model_once(
        &self,
        batch: &[TranslateSegment],
        target_lang: &str,
        model: &str,
        agent: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> Result<String, HostError> {
        let result = self
            .runner
            .run_once(
                &build_instruction(target_lang),
                &build_stdin_payload(batch),
                if model.is_empty() { None } else { Some(model) },
                agent,
                timeout_ms.or(Some(self.timeout_ms)),
            )
            .await;

        if let Some(failure) = map_opencode_failure(
            result.run.spawn_error.as_deref(),
            result.run.timed_out,
            &result.error_events,
            result.run.exit_code,
            &result.run.stderr,
        ) {
            return Err(failure);
        }
        Ok(result.text)
    }

    async fn translate_impl(
        &self,
        segments: Vec<TranslateSegment>,
        options: TranslateOptions,
        on_progress: Option<&(dyn Fn(Progress) + Send + Sync)>,
    ) -> Result<Vec<TranslationResult>, HostError> {
        let target_lang = options
            .target_lang
            .clone()
            .unwrap_or_else(|| "zh-CN".to_string());
        let model = options
            .model
            .clone()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| self.default_model.clone());
        let total = segments.len();
        let mut results: Vec<Option<TranslationResult>> = vec![None; total];

        // Group positions by cache key so identical text is translated once.
        let mut key_to_positions: HashMap<String, Vec<usize>> = HashMap::new();
        let mut unique: Vec<(String, TranslateSegment)> = Vec::new();
        for (position, segment) in segments.iter().enumerate() {
            let key = cache_key(&target_lang, &model, &segment.text);
            if !key_to_positions.contains_key(&key) {
                key_to_positions.insert(key.clone(), Vec::new());
                unique.push((key.clone(), segment.clone()));
            }
            key_to_positions.get_mut(&key).unwrap().push(position);
        }

        let mut misses: Vec<(String, TranslateSegment)> = Vec::new();
        let mut completed = 0usize;
        for (key, segment) in &unique {
            let cached = self.cache.lock().unwrap().get(key);
            let positions = key_to_positions.get(key).unwrap();
            if let Some(text) = cached {
                for position in positions {
                    results[*position] = Some(TranslationResult::ok(
                        segments[*position].index,
                        text.clone(),
                    ));
                }
                completed += positions.len();
            } else {
                misses.push((key.clone(), segment.clone()));
            }
        }
        if let Some(callback) = on_progress {
            callback(Progress {
                phase: "cache".to_string(),
                completed,
                total,
            });
        }

        if !misses.is_empty() {
            let done = AtomicUsize::new(completed);
            let miss_segments: Vec<TranslateSegment> =
                misses.iter().map(|(_, segment)| segment.clone()).collect();

            let call_model = |batch: Vec<TranslateSegment>| {
                let target_lang = target_lang.clone();
                let model = model.clone();
                let agent = options.agent.clone();
                let timeout_ms = options.timeout_ms;
                let done = &done;
                async move {
                    self.queue
                        .add(|| async {
                            let raw = self
                                .call_model_once(
                                    &batch,
                                    &target_lang,
                                    &model,
                                    agent.as_deref(),
                                    timeout_ms,
                                )
                                .await?;
                            let current =
                                done.fetch_add(batch.len(), Ordering::SeqCst) + batch.len();
                            if let Some(callback) = on_progress {
                                callback(Progress {
                                    phase: "batch".to_string(),
                                    completed: current.min(total),
                                    total,
                                });
                            }
                            Ok::<String, HostError>(raw)
                        })
                        .await
                }
            };

            let batch_results = translate_batch(&miss_segments, call_model).await?;
            let by_index: HashMap<i64, TranslationResult> = batch_results
                .into_iter()
                .map(|result| (result.index, result))
                .collect();

            for (key, segment) in &misses {
                let positions = key_to_positions.get(key).unwrap();
                let text = by_index
                    .get(&segment.index)
                    .and_then(|result| result.text.clone());
                match text {
                    Some(text) if !text.is_empty() => {
                        self.cache.lock().unwrap().set(key, text.clone());
                        for position in positions {
                            results[*position] = Some(TranslationResult::ok(
                                segments[*position].index,
                                text.clone(),
                            ));
                        }
                    }
                    _ => {
                        for position in positions {
                            results[*position] = Some(TranslationResult::failed(
                                segments[*position].index,
                                ErrorCode::Incomplete,
                            ));
                        }
                    }
                }
            }
        }

        if let Some(callback) = on_progress {
            callback(Progress {
                phase: "done".to_string(),
                completed: total,
                total,
            });
        }

        Ok(results
            .into_iter()
            .enumerate()
            .map(|(position, result)| {
                result.unwrap_or_else(|| {
                    TranslationResult::failed(
                        segments
                            .get(position)
                            .map(|segment| segment.index)
                            .unwrap_or(0),
                        ErrorCode::Incomplete,
                    )
                })
            })
            .collect())
    }
}

impl Translator for TranslationEngine {
    fn translate<'a>(
        &'a self,
        segments: Vec<TranslateSegment>,
        options: TranslateOptions,
        on_progress: Option<&'a (dyn Fn(Progress) + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Vec<TranslationResult>, HostError>> {
        Box::pin(self.translate_impl(segments, options, on_progress))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn renders_results_as_expected_json() {
        assert_eq!(
            TranslationResult::ok(3, "你好").to_value(),
            json!({ "index": 3, "text": "你好" })
        );
        assert_eq!(
            TranslationResult::failed(4, ErrorCode::Incomplete).to_value(),
            json!({ "index": 4, "error": "INCOMPLETE" })
        );
    }

    #[test]
    fn builds_an_engine_with_the_configured_default_model() {
        let engine = TranslationEngine::new(
            OpencodeRunner::new("opencode", Vec::new(), 120_000),
            TaskQueue::new(2),
            LruCache::new(100),
            120_000,
            crate::protocol::DEFAULT_MODEL.to_string(),
        );
        assert_eq!(engine.default_model, crate::protocol::DEFAULT_MODEL);
    }
}
