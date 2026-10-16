import { describe, expect, test } from "bun:test";
import { alignTranslations, parseTranslationResponse, translateBatch } from "../src/batch.js";

describe("batch contract", () => {
  test("parses fenced JSON and surrounding prose", () => {
    expect(parseTranslationResponse('```json\n{"0":"A","1":"B"}\n```')).toEqual({ 0: "A", 1: "B" });
    expect(parseTranslationResponse('Here you go: {"0":"A"} — done')).toEqual({ 0: "A" });
  });

  test("throws when no JSON object is present", () => {
    expect(() => parseTranslationResponse("no json here")).toThrow();
  });

  test("aligns exact, shufflable and extra keys", () => {
    const segments = [
      { index: 0, text: "a" },
      { index: 1, text: "b" },
      { index: 2, text: "c" },
    ];

    const aligned = alignTranslations(segments, { 0: "A", 1: "B", 2: "C" });
    expect(aligned.results.map((r) => r.text)).toEqual(["A", "B", "C"]);
    expect(aligned.missing).toEqual([]);

    const partial = alignTranslations(segments, { 0: "A", 2: "C", 9: "X" });
    expect(partial.missing.map((s) => s.index)).toEqual([1]);
    expect(partial.extra).toEqual(["9"]);
  });

  test("retries missing segments once and marks unrecovered ones", async () => {
    const segments = [
      { index: 0, text: "a" },
      { index: 1, text: "b" },
    ];
    let calls = 0;
    const results = await translateBatch(segments, async () => {
      calls += 1;
      return calls === 1 ? { raw: JSON.stringify({ 0: "A" }) } : { raw: JSON.stringify({}) };
    });

    expect(results.find((r) => r.index === 0)?.text).toBe("A");
    expect(results.find((r) => r.index === 1)?.error).toBe("INCOMPLETE");
    expect(calls).toBe(2);
  });

  test("recovers omitted segments on retry", async () => {
    const segments = [
      { index: 0, text: "a" },
      { index: 1, text: "b" },
    ];
    let calls = 0;
    const results = await translateBatch(segments, async () => {
      calls += 1;
      return calls === 1 ? { raw: JSON.stringify({ 0: "A" }) } : { raw: JSON.stringify({ 1: "B" }) };
    });

    expect(results).toEqual([
      { index: 0, text: "A" },
      { index: 1, text: "B" },
    ]);
  });
});
