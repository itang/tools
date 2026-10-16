import { describe, expect, test } from "bun:test";
import { FrameDecoder, chunkItems, encodeFrame, messageBytes } from "../src/framing.js";

describe("framing", () => {
  test("round-trips a message", () => {
    const decoder = new FrameDecoder();
    const frame = encodeFrame({ id: "a", type: "health" });
    expect(decoder.push(new Uint8Array(frame))).toEqual([{ id: "a", type: "health" }]);
  });

  test("reassembles a frame split across chunks", () => {
    const frame = encodeFrame({ hello: "world", n: 42 });
    const decoder = new FrameDecoder();
    expect(decoder.push(frame.subarray(0, 2))).toEqual([]);
    expect(decoder.push(frame.subarray(2))).toEqual([{ hello: "world", n: 42 }]);
  });

  test("decodes several frames from one chunk", () => {
    const decoder = new FrameDecoder();
    const joined = Buffer.concat([encodeFrame({ a: 1 }), encodeFrame({ b: 2 })]);
    expect(decoder.push(joined)).toEqual([{ a: 1 }, { b: 2 }]);
  });

  test("chunkItems splits oversized payloads and keeps items intact", () => {
    const items = Array.from({ length: 10 }, (_, index) => ({ index, text: "x".repeat(100) }));
    const chunks = chunkItems(items, (chunk) => ({ type: "translate", translations: chunk }), 300);
    expect(chunks.length).toBeGreaterThan(1);
    expect(chunks.flat()).toEqual(items);
    for (const chunk of chunks) {
      expect(messageBytes({ type: "translate", translations: chunk })).toBeLessThanOrEqual(300);
    }
  });

  test("chunkItems keeps a single item that cannot be split further", () => {
    const chunks = chunkItems([{ index: 0, text: "y".repeat(5000) }], (c) => ({ translations: c }), 100);
    expect(chunks.length).toBe(1);
    expect(chunks[0].length).toBe(1);
  });
});
