/** Thin wrapper around the local `opencode` CLI. */

/**
 * Parse newline-delimited JSON (the `--format json` output) into events.
 * Non-JSON lines are ignored so stray logs never break parsing.
 * @param {string} text
 * @returns {any[]}
 */
export function parseJsonl(text) {
  const events = [];
  for (const line of String(text ?? "").split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    try {
      events.push(JSON.parse(trimmed));
    } catch {
      // ignore non-JSON output
    }
  }
  return events;
}

/** Concatenate the `text` parts of the event stream into the model answer. */
export function extractText(events) {
  let out = "";
  for (const event of events) {
    if (event?.type === "text" && typeof event?.part?.text === "string") {
      out += event.part.text;
    }
  }
  return out;
}

/** Collect structured `type: "error"` events. */
export function extractErrorEvents(events) {
  return events.filter((event) => event?.type === "error" && event?.error).map((event) => event.error);
}

/** Spawns the `opencode` binary and captures its output. */
export class OpencodeRunner {
  /**
   * @param {object} [config]
   * @param {string} [config.bin]      Executable, defaults to `OPENCODE_BIN` or `opencode`.
   * @param {string[]} [config.binArgs] Arguments inserted before the subcommand.
   * @param {number} [config.timeoutMs] Default per-invocation timeout.
   * @param {Record<string,string>} [config.env] Extra environment for the child.
   */
  constructor({ bin, binArgs = [], timeoutMs = 120_000, env } = {}) {
    this.bin = bin || process.env.OPENCODE_BIN || "opencode";
    this.binArgs = binArgs;
    this.timeoutMs = timeoutMs;
    this.env = env;
  }

  /**
   * Run an arbitrary `opencode` argument list, writing `stdin` and enforcing a
   * timeout that kills the child process.
   *
   * @param {string[]} args
   * @param {{stdin?: string, timeoutMs?: number}} [options]
   * @returns {Promise<{exitCode: number|null, stdout: string, stderr: string, spawnError: any, timedOut: boolean}>}
   */
  async runRaw(args, { stdin = "", timeoutMs } = {}) {
    const limit = timeoutMs ?? this.timeoutMs;
    let proc;
    try {
      proc = Bun.spawn([this.bin, ...args], {
        stdin: "pipe",
        stdout: "pipe",
        stderr: "pipe",
        env: this.env ? { ...process.env, ...this.env } : undefined,
      });
    } catch (error) {
      return { exitCode: null, stdout: "", stderr: "", spawnError: error, timedOut: false };
    }

    let timedOut = false;
    const timer = setTimeout(() => {
      timedOut = true;
      try {
        proc.kill();
      } catch {
        // process already gone
      }
    }, limit);

    try {
      proc.stdin.write(stdin ?? "");
      await proc.stdin.end();
    } catch {
      // stdin may already be closed
    }

    let stdout = "";
    let stderr = "";
    let exitCode = null;
    try {
      [stdout, stderr] = await Promise.all([
        new Response(proc.stdout).text(),
        new Response(proc.stderr).text(),
      ]);
      exitCode = await proc.exited;
    } catch (error) {
      return { exitCode, stdout, stderr, spawnError: error, timedOut };
    } finally {
      clearTimeout(timer);
    }

    return { exitCode, stdout, stderr, spawnError: null, timedOut };
  }

  /**
   * Invoke `opencode run --format json` for one translation prompt.
   * @param {object} request
   * @param {string} request.prompt    Instruction passed as the message argument.
   * @param {string} request.stdin     Text body (source segments) written to stdin.
   * @param {string} [request.model]
   * @param {string} [request.agent]
   * @param {number} [request.timeoutMs]
   * @returns {Promise<{exitCode: number|null, stdout: string, stderr: string, spawnError: any, timedOut: boolean, events: any[], text: string, errorEvents: any[]}>}
   */
  async runOnce({ prompt, stdin, model, agent, timeoutMs }) {
    const args = [...this.binArgs, "run", "--format", "json"];
    if (model) args.push("--model", model);
    if (agent) args.push("--agent", agent);
    if (prompt) args.push(prompt);

    const result = await this.runRaw(args, { stdin, timeoutMs });
    const events = parseJsonl(result.stdout);
    return {
      ...result,
      events,
      text: extractText(events),
      errorEvents: extractErrorEvents(events),
    };
  }
}
