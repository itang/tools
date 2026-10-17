//! Native messaging host entry point.
//!
//! Reads length-prefixed JSON frames from stdin, dispatches them, and writes
//! response frames to stdout. Exits when Chrome closes the channel (stdin EOF).

mod batch;
mod cache;
mod engine;
mod errors;
mod framing;
mod health;
mod opencode;
mod protocol;
mod queue;
mod server;

use std::io::Write;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tokio::io::AsyncReadExt;

use engine::TranslationEngine;
use framing::{encode_frame, FrameDecoder};

/// Serializes frame writes so concurrent request tasks never interleave bytes.
struct FrameWriter {
    inner: Mutex<std::io::Stdout>,
}

impl FrameWriter {
    fn new() -> Self {
        Self {
            inner: Mutex::new(std::io::stdout()),
        }
    }

    fn write(&self, message: &Value) {
        let bytes = encode_frame(message);
        if let Ok(mut out) = self.inner.lock() {
            let _ = out.write_all(&bytes);
            let _ = out.flush();
        }
    }
}

/// Read a positive integer environment variable, falling back on missing or
/// malformed values.
fn int_from_env(name: &str, fallback: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(fallback)
}

/// Parse the `OPENCODE_BIN_ARGS` JSON array, ignoring malformed values.
fn parse_bin_args(raw: Option<String>) -> Vec<String> {
    let Some(raw) = raw else {
        return Vec::new();
    };
    match serde_json::from_str::<Value>(&raw) {
        Ok(Value::Array(items)) => items
            .into_iter()
            .map(|item| {
                item.as_str()
                    .map(String::from)
                    .unwrap_or_else(|| item.to_string())
            })
            .collect(),
        _ => Vec::new(),
    }
}

#[tokio::main]
async fn main() {
    let timeout_ms = int_from_env("TRANSLATE_TIMEOUT_MS", 120_000);
    let concurrency = int_from_env("TRANSLATE_CONCURRENCY", 2) as usize;
    let cache_max = int_from_env("TRANSLATE_CACHE_MAX", 2000) as usize;

    // `TRANSLATE_MODEL` overrides the built-in default; set it to an empty
    // string to let opencode pick its own default model instead.
    let default_model =
        std::env::var("TRANSLATE_MODEL").unwrap_or_else(|_| protocol::DEFAULT_MODEL.to_string());

    let bin = std::env::var("OPENCODE_BIN")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "opencode".to_string());
    let bin_args = parse_bin_args(std::env::var("OPENCODE_BIN_ARGS").ok());

    let runner = opencode::OpencodeRunner::new(bin, bin_args, timeout_ms);
    let health_runner = runner.clone();
    let engine = Arc::new(TranslationEngine::new(
        runner,
        queue::TaskQueue::new(concurrency),
        cache::LruCache::new(cache_max),
        timeout_ms,
        default_model,
    ));

    let writer = Arc::new(FrameWriter::new());
    let emit_writer = writer.clone();
    let emit = Arc::new(move |message: Value| emit_writer.write(&message));

    let health: Arc<dyn Fn() -> protocol::BoxFuture<'static, Value> + Send + Sync> =
        Arc::new(move || {
            let runner = health_runner.clone();
            Box::pin(async move { health::check_health(&runner).await })
        });

    let mut tasks = tokio::task::JoinSet::new();
    let mut decoder = FrameDecoder::new();
    let mut stdin = tokio::io::stdin();
    let mut buffer = [0u8; 8192];

    loop {
        let read = match stdin.read(&mut buffer).await {
            Ok(0) => break,
            Ok(read) => read,
            Err(_) => break,
        };
        for request in decoder.push(&buffer[..read]) {
            if request.is_null() {
                continue;
            }
            let engine = engine.clone();
            let health = health.clone();
            let emit = emit.clone();
            tasks.spawn(async move {
                server::dispatch(&*engine, &*health, &*emit, request).await;
            });
        }
    }

    // Chrome closed the channel: drain outstanding requests, then exit.
    while tasks.join_next().await.is_some() {}
}
