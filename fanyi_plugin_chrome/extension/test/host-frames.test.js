import { describe, expect, test } from "bun:test";
import { createFrameAccumulator } from "../src/shared/host-frames.js";

describe("host frame accumulator", () => {
  test("merges chunked translate frames and resolves on the terminal frame", () => {
    const accumulator = createFrameAccumulator();
    accumulator.add({ type: "translate", translations: [{ index: 0, text: "A" }] });
    accumulator.add({ type: "translate", translations: [{ index: 1, text: "B" }] });
    const result = accumulator.resolve({ type: "translate", translations: [{ index: 2, text: "C" }] });
    expect(result.translations).toEqual([
      { index: 0, text: "A" },
      { index: 1, text: "B" },
      { index: 2, text: "C" },
    ]);
  });

  test("resolves error frames", () => {
    const result = createFrameAccumulator().resolve({ type: "error", error: { code: "TIMEOUT", message: "slow" } });
    expect(result.error.code).toBe("TIMEOUT");
  });

  test("resolves health frames", () => {
    const result = createFrameAccumulator().resolve({ type: "health", data: { host: { ok: true } } });
    expect(result.health.host.ok).toBe(true);
  });

  test("unknown terminal frame yields HOST_ERROR", () => {
    const result = createFrameAccumulator().resolve({ type: "mystery" });
    expect(result.error.code).toBe("HOST_ERROR");
  });
});
