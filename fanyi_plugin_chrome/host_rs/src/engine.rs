//! Translation engine: cache lookup, batching, queueing, error mapping.

use crate::batch::{build_instruction, build_stdin_payload, translate_batch};
use crate::cache::{cache_key, LruCache};
use crate::errors::{map_opencode_failure, HostError};
use crate::opencode::Runner;
use crate::protocol::{
    BoxFuture, ErrorCode, Progress, TranslateOptions, TranslateSegment, TranslationResult,
};
use crate::queue::TaskQueue;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// The translation workhorse: cache + queue + batching over an opencode runner.
pub struct TranslationEngine<R: Runner> {
    runner: R,
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

impl<R: Runner> TranslationEngine<R> {
    pub fn new(
        runner: R,
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
                build_instruction(target_lang),
                build_stdin_payload(batch),
                if model.is_empty() {
                    None
                } else {
                    Some(model.to_string())
                },
                agent.map(String::from),
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

impl<R: Runner> Translator for TranslationEngine<R> {
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
    use crate::opencode::{RunOnceResult, RunResult};
    use serde_json::{json, Value};
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Mutex};

    /// A `Runner` driven by a closure, for in-process engine tests.
    struct StubRunner {
        calls: Arc<AtomicUsize>,
        models: Arc<Mutex<Vec<Option<String>>>>,
        handler: StubHandler,
    }

    impl Runner for StubRunner {
        fn bin(&self) -> &str {
            "stub"
        }

        fn run_raw<'a>(
            &'a self,
            _args: Vec<String>,
            _stdin: String,
            _timeout_ms: Option<u64>,
        ) -> BoxFuture<'a, RunResult> {
            Box::pin(async { RunResult::default() })
        }

        fn run_once<'a>(
            &'a self,
            _prompt: String,
            stdin: String,
            model: Option<String>,
            _agent: Option<String>,
            _timeout_ms: Option<u64>,
        ) -> BoxFuture<'a, RunOnceResult> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            self.models.lock().unwrap().push(model);
            let items: Vec<Value> = serde_json::from_str(&stdin).unwrap_or_default();
            let (exit_code, text, error_events) = (self.handler)(call, &items);
            let result = RunOnceResult {
                run: RunResult {
                    exit_code,
                    ..RunResult::default()
                },
                events: Vec::new(),
                text,
                error_events,
            };
            Box::pin(async move { result })
        }
    }

    type Counters = (Arc<AtomicUsize>, Arc<Mutex<Vec<Option<String>>>>);
    type StubOutcome = (Option<i32>, String, Vec<Value>);
    type StubHandler = Box<dyn Fn(usize, &[Value]) -> StubOutcome + Send + Sync>;

    fn engine_with(
        handler: impl Fn(usize, &[Value]) -> StubOutcome + Send + Sync + 'static,
        default_model: &str,
    ) -> (TranslationEngine<StubRunner>, Counters) {
        let calls = Arc::new(AtomicUsize::new(0));
        let models = Arc::new(Mutex::new(Vec::new()));
        let runner = StubRunner {
            calls: calls.clone(),
            models: models.clone(),
            handler: Box::new(handler),
        };
        let engine = TranslationEngine::new(
            runner,
            TaskQueue::new(2),
            LruCache::new(100),
            120_000,
            default_model.to_string(),
        );
        (engine, (calls, models))
    }

    /// A model answer that translates every item, prefixed with `T:`.
    fn ok_handler(_call: usize, items: &[Value]) -> (Option<i32>, String, Vec<Value>) {
        let mut map = serde_json::Map::new();
        for item in items {
            let index = item
                .get("index")
                .map(Value::to_string)
                .unwrap_or_else(|| "0".to_string());
            let text = item.get("text").and_then(Value::as_str).unwrap_or("");
            map.insert(index, Value::String(format!("T:{text}")));
        }
        (Some(0), Value::Object(map).to_string(), Vec::new())
    }

    fn seg(index: i64, text: &str) -> TranslateSegment {
        TranslateSegment {
            index,
            text: text.to_string(),
        }
    }

    #[tokio::test]
    async fn serves_a_repeated_segment_from_cache_without_a_second_model_call() {
        let (engine, (calls, _models)) = engine_with(ok_handler, "");
        let first = engine
            .translate(vec![seg(0, "hello")], TranslateOptions::default(), None)
            .await
            .unwrap();
        let second = engine
            .translate(vec![seg(5, "hello")], TranslateOptions::default(), None)
            .await
            .unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(first[0].text.as_deref(), Some("T:hello"));
        assert_eq!(second[0].text.as_deref(), Some("T:hello"));
        assert_eq!(second[0].index, 5);
    }

    #[tokio::test]
    async fn deduplicates_identical_text_within_a_request() {
        let (engine, (calls, _models)) = engine_with(ok_handler, "");
        let results = engine
            .translate(
                vec![seg(0, "same"), seg(1, "same")],
                TranslateOptions::default(),
                None,
            )
            .await
            .unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            results
                .iter()
                .filter_map(|r| r.text.clone())
                .collect::<Vec<_>>(),
            vec!["T:same", "T:same"]
        );
    }

    #[tokio::test]
    async fn surfaces_region_errors_as_model_unavailable() {
        let (engine, _) = engine_with(
            |_call, _items| {
                (
                    Some(1),
                    String::new(),
                    vec![
                        json!({ "type": "provider.auth", "message": "This model is not available in your country" }),
                    ],
                )
            },
            "",
        );
        let error = engine
            .translate(vec![seg(0, "x")], TranslateOptions::default(), None)
            .await
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::ModelUnavailable);
    }

    #[tokio::test]
    async fn marks_segments_the_model_omits_as_incomplete() {
        let (engine, _) = engine_with(|_call, _items| (Some(0), "{}".to_string(), Vec::new()), "");
        let results = engine
            .translate(vec![seg(0, "a")], TranslateOptions::default(), None)
            .await
            .unwrap();
        assert_eq!(results[0].error, Some(ErrorCode::Incomplete));
    }

    #[tokio::test]
    async fn falls_back_to_the_default_model_when_the_request_omits_one() {
        let (engine, (_calls, models)) = engine_with(ok_handler, "fallback/model");
        engine
            .translate(vec![seg(0, "hello")], TranslateOptions::default(), None)
            .await
            .unwrap();
        assert_eq!(
            models.lock().unwrap()[0],
            Some("fallback/model".to_string())
        );
    }

    #[tokio::test]
    async fn an_empty_default_model_lets_opencode_choose() {
        let (engine, (_calls, models)) = engine_with(ok_handler, "");
        engine
            .translate(vec![seg(0, "hello")], TranslateOptions::default(), None)
            .await
            .unwrap();
        assert_eq!(models.lock().unwrap()[0], None);
    }

    #[tokio::test]
    async fn prefers_an_explicit_request_model_over_the_default() {
        let (engine, (_calls, models)) = engine_with(ok_handler, "fallback/model");
        engine
            .translate(
                vec![seg(0, "hello")],
                TranslateOptions {
                    model: Some("prov/explicit#high".to_string()),
                    ..TranslateOptions::default()
                },
                None,
            )
            .await
            .unwrap();
        assert_eq!(
            models.lock().unwrap()[0],
            Some("prov/explicit#high".to_string())
        );
    }

    #[tokio::test]
    async fn reports_progress_phases_and_preserves_input_order() {
        let (engine, _) = engine_with(ok_handler, "");
        let phases = Mutex::new(Vec::new());
        let collect = |progress: Progress| phases.lock().unwrap().push(progress.phase);
        let results = engine
            .translate(
                vec![seg(10, "a"), seg(20, "b")],
                TranslateOptions::default(),
                Some(&collect),
            )
            .await
            .unwrap();

        assert_eq!(
            results.iter().map(|r| r.index).collect::<Vec<_>>(),
            vec![10, 20]
        );
        let phases = phases.into_inner().unwrap();
        assert!(phases.contains(&"cache".to_string()));
        assert!(phases.contains(&"batch".to_string()));
        assert!(phases.contains(&"done".to_string()));
    }
}
