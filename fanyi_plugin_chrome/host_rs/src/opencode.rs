//! Thin wrapper around the local `opencode` CLI.

use crate::protocol::BoxFuture;
use serde_json::Value;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

/// Parse newline-delimited JSON (the `--format json` output) into events.
/// Non-JSON lines are ignored so stray logs never break parsing.
pub fn parse_jsonl(text: &str) -> Vec<Value> {
    let mut events = Vec::new();
    for raw_line in text.split('\n') {
        let line = raw_line.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(value) = serde_json::from_str::<Value>(line) {
            events.push(value);
        }
    }
    events
}

/// Concatenate the `text` parts of the event stream into the model answer.
pub fn extract_text(events: &[Value]) -> String {
    let mut out = String::new();
    for event in events {
        if event.get("type").and_then(Value::as_str) != Some("text") {
            continue;
        }
        if let Some(text) = event
            .get("part")
            .and_then(|part| part.get("text"))
            .and_then(Value::as_str)
        {
            out.push_str(text);
        }
    }
    out
}

/// Collect structured `type: "error"` events.
pub fn extract_error_events(events: &[Value]) -> Vec<Value> {
    events
        .iter()
        .filter(|event| {
            event.get("type").and_then(Value::as_str) == Some("error")
                && event.get("error").is_some()
        })
        .filter_map(|event| event.get("error").cloned())
        .collect()
}

/// Outcome of one raw `opencode` invocation.
#[derive(Debug, Clone, Default)]
pub struct RunResult {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub spawn_error: Option<String>,
    pub timed_out: bool,
}

/// Outcome of one `opencode run` translation invocation.
#[derive(Debug, Clone, Default)]
pub struct RunOnceResult {
    pub run: RunResult,
    /// Parsed event stream (kept for diagnostics / tests).
    #[allow(dead_code)]
    pub events: Vec<Value>,
    pub text: String,
    pub error_events: Vec<Value>,
}

/// Abstraction over the opencode CLI. It lets the engine and health check run
/// against a stub in tests without spawning a process.
pub trait Runner: Send + Sync {
    /// The executable this runner invokes.
    fn bin(&self) -> &str;

    /// Run an arbitrary argument list, writing `stdin` and enforcing a timeout
    /// that kills the child process.
    fn run_raw<'a>(
        &'a self,
        args: Vec<String>,
        stdin: String,
        timeout_ms: Option<u64>,
    ) -> BoxFuture<'a, RunResult>;

    /// Invoke `opencode run --format json` for one translation prompt.
    fn run_once<'a>(
        &'a self,
        prompt: String,
        stdin: String,
        model: Option<String>,
        agent: Option<String>,
        timeout_ms: Option<u64>,
    ) -> BoxFuture<'a, RunOnceResult>;
}

/// Spawns the `opencode` binary and captures its output.
#[derive(Clone)]
pub struct OpencodeRunner {
    bin: String,
    bin_args: Vec<String>,
    timeout_ms: u64,
}

impl OpencodeRunner {
    /// `bin` defaults to `OPENCODE_BIN` or `opencode` at the call site.
    pub fn new(bin: impl Into<String>, bin_args: Vec<String>, timeout_ms: u64) -> Self {
        Self {
            bin: bin.into(),
            bin_args,
            timeout_ms,
        }
    }

    /// Run an arbitrary `opencode` argument list, writing `stdin` and enforcing
    /// a timeout that kills the child process.
    pub async fn call_raw(
        &self,
        args: &[String],
        stdin: &str,
        timeout_ms: Option<u64>,
    ) -> RunResult {
        let limit = timeout_ms.unwrap_or(self.timeout_ms);
        let mut command = Command::new(&self.bin);
        command
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        // Avoid flashing a console window when Chrome (a GUI process) launches
        // this host and the host in turn spawns the console-mode opencode.
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.as_std_mut().creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                return RunResult {
                    spawn_error: Some(error.to_string()),
                    ..RunResult::default()
                };
            }
        };

        // Feed stdin on a side task so large stdout cannot deadlock the pipe.
        if let Some(mut pipe) = child.stdin.take() {
            let data = stdin.to_string();
            tokio::spawn(async move {
                let _ = pipe.write_all(data.as_bytes()).await;
                let _ = pipe.shutdown().await;
            });
        }

        let mut stdout_pipe = child.stdout.take();
        let mut stderr_pipe = child.stderr.take();

        let read_stdout = async {
            let mut buffer = Vec::new();
            if let Some(pipe) = stdout_pipe.as_mut() {
                let _ = pipe.read_to_end(&mut buffer).await;
            }
            buffer
        };
        let read_stderr = async {
            let mut buffer = Vec::new();
            if let Some(pipe) = stderr_pipe.as_mut() {
                let _ = pipe.read_to_end(&mut buffer).await;
            }
            buffer
        };

        let waited = tokio::time::timeout(Duration::from_millis(limit), async {
            tokio::join!(read_stdout, read_stderr, child.wait())
        })
        .await;

        match waited {
            Ok((stdout, stderr, status)) => RunResult {
                exit_code: status.ok().and_then(|status| status.code()),
                stdout: String::from_utf8_lossy(&stdout).into_owned(),
                stderr: String::from_utf8_lossy(&stderr).into_owned(),
                spawn_error: None,
                timed_out: false,
            },
            Err(_) => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                RunResult {
                    timed_out: true,
                    ..RunResult::default()
                }
            }
        }
    }

    /// Invoke `opencode run --format json` for one translation prompt.
    pub async fn call_once(
        &self,
        prompt: &str,
        stdin: &str,
        model: Option<&str>,
        agent: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> RunOnceResult {
        let mut args = self.bin_args.clone();
        args.push("run".to_string());
        args.push("--format".to_string());
        args.push("json".to_string());
        if let Some(model) = model.filter(|value| !value.is_empty()) {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if let Some(agent) = agent.filter(|value| !value.is_empty()) {
            args.push("--agent".to_string());
            args.push(agent.to_string());
        }
        if !prompt.is_empty() {
            args.push(prompt.to_string());
        }

        let run = self.call_raw(&args, stdin, timeout_ms).await;
        let events = parse_jsonl(&run.stdout);
        let text = extract_text(&events);
        let error_events = extract_error_events(&events);
        RunOnceResult {
            run,
            events,
            text,
            error_events,
        }
    }
}

impl Runner for OpencodeRunner {
    fn bin(&self) -> &str {
        &self.bin
    }

    fn run_raw<'a>(
        &'a self,
        args: Vec<String>,
        stdin: String,
        timeout_ms: Option<u64>,
    ) -> BoxFuture<'a, RunResult> {
        Box::pin(async move { self.call_raw(&args, &stdin, timeout_ms).await })
    }

    fn run_once<'a>(
        &'a self,
        prompt: String,
        stdin: String,
        model: Option<String>,
        agent: Option<String>,
        timeout_ms: Option<u64>,
    ) -> BoxFuture<'a, RunOnceResult> {
        Box::pin(async move {
            self.call_once(
                &prompt,
                &stdin,
                model.as_deref(),
                agent.as_deref(),
                timeout_ms,
            )
            .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_ndjson_and_ignores_noise() {
        let events = parse_jsonl("{\"type\":\"a\"}\nnot json\n\n{\"type\":\"b\"}\r\n");
        assert_eq!(events, vec![json!({"type":"a"}), json!({"type":"b"})]);
    }

    #[test]
    fn concatenates_text_parts() {
        let events = vec![
            json!({ "type": "text", "part": { "text": "Hello" } }),
            json!({ "type": "step_start" }),
            json!({ "type": "text", "part": { "text": " world" } }),
        ];
        assert_eq!(extract_text(&events), "Hello world");
    }

    #[test]
    fn collects_error_events() {
        let events = vec![
            json!({ "type": "error", "error": { "type": "provider.auth" } }),
            json!({ "type": "text", "part": { "text": "x" } }),
        ];
        assert_eq!(
            extract_error_events(&events),
            vec![json!({ "type": "provider.auth" })]
        );
    }
}
