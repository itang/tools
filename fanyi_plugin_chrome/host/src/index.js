/**
 * Native messaging host entry point.
 *
 * Reads length-prefixed JSON frames from stdin, dispatches them, and writes
 * response frames to stdout. Exits when Chrome closes the channel (stdin EOF).
 */

import { FrameDecoder, encodeFrame } from "./framing.js";
import { OpencodeRunner } from "./opencode.js";
import { TranslationEngine } from "./engine.js";
import { TaskQueue } from "./queue.js";
import { LruCache } from "./cache.js";
import { createDispatcher } from "./server.js";
import { createHealthCheck } from "./health.js";
import { DEFAULT_MODEL, ERROR_CODES } from "./protocol.js";

function intFromEnv(name, fallback) {
  const value = Number.parseInt(process.env[name] ?? "", 10);
  return Number.isFinite(value) && value > 0 ? value : fallback;
}

const timeoutMs = intFromEnv("TRANSLATE_TIMEOUT_MS", 120_000);
const concurrency = intFromEnv("TRANSLATE_CONCURRENCY", 2);
const cacheMax = intFromEnv("TRANSLATE_CACHE_MAX", 2000);

// `TRANSLATE_MODEL` overrides the built-in default; set it to an empty string
// to let opencode pick its own default model instead.
const defaultModel = process.env.TRANSLATE_MODEL ?? DEFAULT_MODEL;

const binArgs = (() => {
  const raw = process.env.OPENCODE_BIN_ARGS;
  if (!raw) return [];
  try {
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed.map(String) : [];
  } catch {
    return [];
  }
})();

const runner = new OpencodeRunner({ binArgs, timeoutMs });
const engine = new TranslationEngine({
  runner,
  queue: new TaskQueue(concurrency),
  cache: new LruCache(cacheMax),
  timeoutMs,
  defaultModel,
});
const health = createHealthCheck(runner);

function emit(message) {
  try {
    process.stdout.write(encodeFrame(message));
  } catch {
    // Chrome closed the channel; nothing left to do.
  }
}

const dispatch = createDispatcher({ engine, health, emit });
const decoder = new FrameDecoder();

/** Track in-flight requests so the host never exits before replying. */
const pending = new Set();

for await (const chunk of Bun.stdin.stream()) {
  for (const request of decoder.push(chunk)) {
    if (!request) continue;
    const task = Promise.resolve(dispatch(request))
      .catch((error) => {
        emit({
          id: typeof request?.id === "string" ? request.id : "",
          seq: 0,
          last: true,
          type: "error",
          error: { code: ERROR_CODES.HOST_ERROR, message: String(error?.message ?? error) },
        });
      })
      .finally(() => pending.delete(task));
    pending.add(task);
  }
}

// Chrome closed the channel: drain outstanding requests, then exit.
await Promise.allSettled([...pending]);
process.exit(0);
