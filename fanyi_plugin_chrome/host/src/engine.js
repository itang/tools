/** Translation engine: cache lookup, batching, queueing, error mapping. */

import { ERROR_CODES } from "./protocol.js";
import { HostError, mapOpencodeFailure } from "./errors.js";
import { buildInstruction, buildStdinPayload, translateBatch } from "./batch.js";
import { cacheKey } from "./cache.js";

export class TranslationEngine {
  /**
   * @param {object} deps
   * @param {import("./opencode.js").OpencodeRunner} deps.runner
   * @param {import("./queue.js").TaskQueue} deps.queue
   * @param {import("./cache.js").LruCache} deps.cache
   * @param {number} [deps.timeoutMs]
   * @param {string} [deps.defaultModel] Model used when a request omits one.
   */
  constructor({ runner, queue, cache, timeoutMs = 120_000, defaultModel = "" }) {
    this.runner = runner;
    this.queue = queue;
    this.cache = cache;
    this.timeoutMs = timeoutMs;
    this.defaultModel = defaultModel;
  }

  /**
   * @param {import("./protocol.js").TranslateSegment[]} segments
   * @param {import("./protocol.js").TranslateOptions} [options]
   * @param {(progress: {phase: string, completed: number, total: number}) => void} [onProgress]
   * @returns {Promise<import("./protocol.js").TranslationResult[]>}
   */
  async translate(segments, options = {}, onProgress) {
    const targetLang = options.targetLang || "zh-CN";
    const model = options.model || this.defaultModel || "";
    const total = segments.length;
    /** @type {(import("./protocol.js").TranslationResult|undefined)[]} */
    const results = new Array(total);

    // Group positions by cache key so identical text is translated once.
    const keyToPositions = new Map();
    const unique = [];
    segments.forEach((segment, position) => {
      const key = cacheKey(targetLang, model, segment.text);
      if (!keyToPositions.has(key)) {
        keyToPositions.set(key, []);
        unique.push({ key, segment });
      }
      keyToPositions.get(key).push(position);
    });

    const misses = [];
    let completed = 0;
    for (const { key, segment } of unique) {
      const cached = this.cache.get(key);
      if (cached !== undefined) {
        for (const position of keyToPositions.get(key)) {
          results[position] = { index: segments[position].index, text: cached };
        }
        completed += keyToPositions.get(key).length;
      } else {
        misses.push({ key, segment });
      }
    }
    onProgress?.({ phase: "cache", completed, total });

    if (misses.length > 0) {
      let done = completed;
      const callModel = (batch) =>
        this.queue.add(async () => {
          const raw = await this.callModelOnce(batch, {
            targetLang,
            model,
            agent: options.agent,
            timeoutMs: options.timeoutMs,
          });
          done += batch.length;
          onProgress?.({ phase: "batch", completed: Math.min(done, total), total });
          return { raw };
        });

      const missSegments = misses.map((m) => m.segment);
      const batchResults = await translateBatch(missSegments, callModel);
      const byIndex = new Map(batchResults.map((r) => [r.index, r]));

      for (const { key, segment } of misses) {
        const result = byIndex.get(segment.index);
        const positions = keyToPositions.get(key);
        if (result?.text) {
          this.cache.set(key, result.text);
          for (const position of positions) {
            results[position] = { index: segments[position].index, text: result.text };
          }
        } else {
          for (const position of positions) {
            results[position] = { index: segments[position].index, error: ERROR_CODES.INCOMPLETE };
          }
        }
      }
      onProgress?.({ phase: "done", completed: total, total });
    } else {
      onProgress?.({ phase: "done", completed: total, total });
    }

    return /** @type {import("./protocol.js").TranslationResult[]} */ (results);
  }

  /**
   * Run one model call for a batch.
   * @param {import("./protocol.js").TranslateSegment[]} batch
   * @param {{targetLang: string, model: string, agent?: string, timeoutMs?: number}} options
   * @returns {Promise<string>}
   */
  async callModelOnce(batch, options) {
    const result = await this.runner.runOnce({
      prompt: buildInstruction(options.targetLang),
      stdin: buildStdinPayload(batch),
      model: options.model || undefined,
      agent: options.agent || undefined,
      timeoutMs: options.timeoutMs ?? this.timeoutMs,
    });
    const failure = mapOpencodeFailure(result);
    if (failure) throw failure;
    return result.text;
  }
}

export { HostError };
