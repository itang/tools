import { describe, expect, test } from "bun:test";
import { TranslationEngine } from "../src/engine.js";
import { TaskQueue } from "../src/queue.js";
import { LruCache } from "../src/cache.js";
import { DEFAULT_MODEL } from "../src/protocol.js";

/** Build an engine whose runner is a stub controlled by `handler`. */
function engineWith(handler, engineOptions = {}) {
  let calls = 0;
  const models = [];
  const runner = {
    runOnce: async ({ stdin, model }) => {
      calls += 1;
      models.push(model);
      const items = JSON.parse(stdin);
      return { spawnError: null, timedOut: false, ...handler(items, calls) };
    },
  };
  const engine = new TranslationEngine({
    runner,
    queue: new TaskQueue(2),
    cache: new LruCache(100),
    ...engineOptions,
  });
  return { engine, calls: () => calls, models };
}

const okHandler = (items) => ({
  exitCode: 0,
  errorEvents: [],
  text: JSON.stringify(Object.fromEntries(items.map((item) => [String(item.index), `T:${item.text}`]))),
});

describe("TranslationEngine", () => {
  test("serves a repeated segment from cache without a second model call", async () => {
    const { engine, calls } = engineWith(okHandler);

    const first = await engine.translate([{ index: 0, text: "hello" }], {});
    const second = await engine.translate([{ index: 5, text: "hello" }], {});

    expect(calls()).toBe(1);
    expect(first[0].text).toBe("T:hello");
    expect(second[0].text).toBe("T:hello");
    expect(second[0].index).toBe(5);
  });

  test("deduplicates identical text inside a single request", async () => {
    const { engine, calls } = engineWith(okHandler);
    const results = await engine.translate(
      [
        { index: 0, text: "same" },
        { index: 1, text: "same" },
      ],
      {},
    );
    expect(calls()).toBe(1);
    expect(results.map((r) => r.text)).toEqual(["T:same", "T:same"]);
  });

  test("surfaces region errors as MODEL_UNAVAILABLE", async () => {
    const { engine } = engineWith(() => ({
      exitCode: 1,
      errorEvents: [{ type: "provider.auth", message: "This model is not available in your country" }],
      text: "",
    }));

    let caught;
    try {
      await engine.translate([{ index: 0, text: "x" }], {});
    } catch (error) {
      caught = error;
    }
    expect(caught?.code).toBe("MODEL_UNAVAILABLE");
  });

  test("marks segments the model omits as INCOMPLETE", async () => {
    const { engine } = engineWith(() => ({ exitCode: 0, errorEvents: [], text: "{}" }));
    const results = await engine.translate([{ index: 0, text: "a" }], {});
    expect(results[0].error).toBe("INCOMPLETE");
  });

  test("falls back to the default model when the request omits one", async () => {
    const { engine, models } = engineWith(okHandler, { defaultModel: DEFAULT_MODEL });
    await engine.translate([{ index: 0, text: "hello" }], {});
    expect(models[0]).toBe(DEFAULT_MODEL);
  });

  test("prefers an explicit request model over the default", async () => {
    const { engine, models } = engineWith(okHandler, { defaultModel: "fallback/model" });
    await engine.translate([{ index: 0, text: "hello" }], { model: "prov/explicit#high" });
    expect(models[0]).toBe("prov/explicit#high");
  });

  test("reports progress and preserves input order", async () => {
    const { engine } = engineWith(okHandler);
    const phases = [];
    const results = await engine.translate(
      [
        { index: 10, text: "a" },
        { index: 20, text: "b" },
      ],
      {},
      (progress) => phases.push(progress.phase),
    );

    expect(results.map((r) => r.index)).toEqual([10, 20]);
    expect(phases).toContain("done");
  });
});
