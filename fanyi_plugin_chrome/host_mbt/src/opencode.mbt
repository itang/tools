///| Thin wrapper around the local `opencode` CLI.

///|
/// Parse newline-delimited JSON (the `--format json` output) into events.
/// Non-JSON lines are ignored so stray logs never break parsing.
pub fn parse_jsonl(text : String) -> Array[Json] {
  let events : Array[Json] = []
  for line in text.split("\n") {
    let trimmed = line.trim().to_owned()
    if trimmed.is_empty() {
      continue
    }
    match parse_json_option(trimmed) {
      Some(event) => events.push(event)
      None => ()
    }
  }
  events
}

///|
/// Concatenate the `text` parts of the event stream into the model answer.
pub fn extract_text(events : Array[Json]) -> String {
  let out = StringBuilder()
  for event in events {
    guard event is { "type": "text", "part": { "text": String(text), .. }, .. } else {
      continue
    }
    out.write_string(text)
  }
  out.to_string()
}

///|
/// Collect structured `type: "error"` events.
pub fn extract_error_events(events : Array[Json]) -> Array[Json] {
  events.filter_map(event => {
    match event {
      { "type": "error", "error": error, .. } => Some(error)
      _ => None
    }
  })
}

///|
/// The captured outcome of one `opencode` invocation.
pub(all) struct RunResult {
  exit_code : Int32?
  stdout : String
  stderr : String
  spawn_error : String?
  timed_out : Bool
  events : Array[Json]
  text : String
  error_events : Array[Json]
}

///|
/// Spawns the `opencode` binary and captures its output.
pub struct OpencodeRunner {
  bin : String
  bin_args : Array[String]
  timeout_ms : Int32
  env : Map[String, String]
}

///|
/// Create a runner. `bin` defaults to the `OPENCODE_BIN` environment
/// variable or `opencode`.
pub fn OpencodeRunner::new(
  bin? : String,
  bin_args~ : Array[String] = [],
  timeout_ms~ : Int32 = 120_000,
  env~ : Map[String, String] = Map([]),
) -> OpencodeRunner {
  let resolved = match bin {
    Some(bin) => bin
    None =>
      match @env.get_env_var("OPENCODE_BIN") {
        Some(bin) => bin
        None => "opencode"
      }
  }
  { bin: resolved, bin_args, timeout_ms, env, }
}

///|
/// The executable this runner invokes.
pub fn OpencodeRunner::bin(self : OpencodeRunner) -> String {
  self.bin
}

///|
/// Run an arbitrary `opencode` argument list, writing `stdin` and enforcing a
/// timeout that kills the child process.
pub async fn OpencodeRunner::call_raw(
  self : OpencodeRunner,
  args : Array[String],
  stdin : String,
  timeout_ms : Int32?,
) -> RunResult {
  let limit = match timeout_ms {
    Some(value) => value
    None => self.timeout_ms
  }
  let argv : Array[String] = []
  argv.append(self.bin_args[:])
  argv.append(args[:])
  // Stream the request to the child through a pipe instead of a temp file:
  // nothing is written to disk, and the child sees EOF as soon as the writer
  // closes its end.
  let (child_in, child_stdin) = @process.write_to_process()
  let (out_read, out_write) = @process.read_from_process()
  let (err_read, err_write) = @process.read_from_process()
  let mut timed_out = false
  let mut spawn_error : String? = None
  let mut exit_code : Int32? = None
  let (stdout, stderr) = @async.with_task_group(group => {
    let out_task = group.spawn(() => {
      defer out_read.close()
      @utf8.decode_lossy(out_read.read_all().binary()[:])
    })
    let err_task = group.spawn(() => {
      defer err_read.close()
      @utf8.decode_lossy(err_read.read_all().binary()[:])
    })
    group.spawn_bg(() => {
      defer child_stdin.close()
      if stdin.length() > 0 {
        child_stdin.write(@utf8.encode(stdin)) catch {
          // The child may exit before reading all of its input; a broken pipe
          // here is expected and harmless.
          _ => ()
        }
      }
    })
    try {
      let code = @async.with_timeout(limit, () => {
        @process.run(
          self.bin,
          argv,
          stdin=child_in,
          stdout=out_write,
          stderr=err_write,
          extra_env=self.env,
          no_console_window=true,
          cancel_handler=@process.hard_cancel(),
        )
      })
      exit_code = Some(code)
    } catch {
      @async.TimeoutError => timed_out = true
      err => spawn_error = Some(err.to_string())
    }
    out_write.close()
    err_write.close()
    (out_task.wait(), err_task.wait())
  })
  {
    exit_code,
    stdout,
    stderr,
    spawn_error,
    timed_out,
    events: [],
    text: "",
    error_events: [],
  }
}

///|
/// Invoke `opencode run --format json` for one translation prompt.
pub async fn OpencodeRunner::call_once(
  self : OpencodeRunner,
  prompt : String,
  stdin : String,
  model : String?,
  agent : String?,
  timeout_ms : Int32?,
) -> RunResult {
  let args : Array[String] = ["run", "--format", "json"]
  match model {
    Some(model) =>
      if model.length() > 0 {
        args.push("--model")
        args.push(model)
      }
    None => ()
  }
  match agent {
    Some(agent) =>
      if agent.length() > 0 {
        args.push("--agent")
        args.push(agent)
      }
    None => ()
  }
  if prompt.length() > 0 {
    args.push(prompt)
  }
  let result = self.call_raw(args, stdin, timeout_ms)
  let events = parse_jsonl(result.stdout)
  {
    exit_code: result.exit_code,
    stdout: result.stdout,
    stderr: result.stderr,
    spawn_error: result.spawn_error,
    timed_out: result.timed_out,
    events,
    text: extract_text(events),
    error_events: extract_error_events(events),
  }
}

///|
/// A runner able to execute `opencode` invocations. The trait lets tests
/// inject a stub without spawning a process.
pub(open) trait Runner {
  fn bin(Self) -> String
  async fn run_raw(Self, Array[String], String, Int32?) -> RunResult
  async fn run_once(Self, String, String, String?, String?, Int32?) -> RunResult
}

///|
pub extend OpencodeRunner with Runner::{run_raw, run_once}

///|
pub impl Runner for OpencodeRunner with fn bin(self) -> String {
  self.bin
}

///|
pub impl Runner for OpencodeRunner with fn run_raw(
  self,
  args,
  stdin,
  timeout_ms,
) {
  self.call_raw(args, stdin, timeout_ms)
}

///|
pub impl Runner for OpencodeRunner with fn run_once(
  self,
  prompt,
  stdin,
  model,
  agent,
  timeout_ms,
) {
  self.call_once(prompt, stdin, model, agent, timeout_ms)
}
