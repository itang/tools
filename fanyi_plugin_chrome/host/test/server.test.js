import { describe, expect, test } from "bun:test";
import { createDispatcher } from "../src/server.js";
import { HostError } from "../src/errors.js";

function collector() {
  const frames = [];
  return { frames, emit: (message) => frames.push(message) };
}

describe("dispatcher", () => {
  test("health request emits one terminal frame", async () => {
    const { frames, emit } = collector();
    const dispatch = createDispatcher({
      engine: {},
      health: async () => ({ host: { ok: true }, opencode: { ok: true, version: "opencode v2.0.29" } }),
      emit,
    });

    await dispatch({ id: "1", type: "health" });

    expect(frames).toHaveLength(1);
    expect(frames[0]).toMatchObject({ id: "1", last: true, type: "health" });
    expect(frames[0].data.opencode.version).toBe("opencode v2.0.29");
  });

  test("unknown request type -> BAD_REQUEST", async () => {
    const { frames, emit } = collector();
    const dispatch = createDispatcher({ engine: {}, health: async () => ({}), emit });
    await dispatch({ id: "2", type: "nope" });
    expect(frames[0].error.code).toBe("BAD_REQUEST");
  });

  test("translate emits progress frames then a terminal frame", async () => {
    const { frames, emit } = collector();
    const engine = {
      translate: async (segments, _options, onProgress) => {
        onProgress?.({ phase: "batch", completed: 1, total: 1 });
        onProgress?.({ phase: "done", completed: 1, total: 1 });
        return segments.map((s) => ({ index: s.index, text: "T" }));
      },
    };
    const dispatch = createDispatcher({ engine, health: async () => ({}), emit });

    await dispatch({ id: "3", type: "translate", segments: [{ index: 0, text: "a" }] });

    expect(frames.filter((f) => f.type === "progress")).toHaveLength(2);
    const terminal = frames.find((f) => f.type === "translate" && f.last);
    expect(terminal.translations).toEqual([{ index: 0, text: "T" }]);
  });

  test("HostError from the engine becomes an error frame with its code", async () => {
    const { frames, emit } = collector();
    const engine = { translate: async () => { throw new HostError("MODEL_UNAVAILABLE", "no model"); } };
    const dispatch = createDispatcher({ engine, health: async () => ({}), emit });

    await dispatch({ id: "4", type: "translate", segments: [{ index: 0, text: "a" }] });

    expect(frames).toHaveLength(1);
    expect(frames[0]).toMatchObject({ type: "error", last: true });
    expect(frames[0].error.code).toBe("MODEL_UNAVAILABLE");
  });

  test("empty segment list yields an empty terminal frame", async () => {
    const { frames, emit } = collector();
    const engine = { translate: async () => [] };
    const dispatch = createDispatcher({ engine, health: async () => ({}), emit });
    await dispatch({ id: "5", type: "translate", segments: [] });
    expect(frames).toHaveLength(1);
    expect(frames[0]).toMatchObject({ type: "translate", last: true, translations: [] });
  });
});
