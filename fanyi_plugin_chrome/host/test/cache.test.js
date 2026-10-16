import { describe, expect, test } from "bun:test";
import { LruCache, cacheKey, normalizeText } from "../src/cache.js";

describe("cache", () => {
  test("normalizes whitespace", () => {
    expect(normalizeText("  a\n\t b  ")).toBe("a b");
  });

  test("cacheKey distinguishes target language and model", () => {
    expect(cacheKey("zh-CN", "m1", "hello")).not.toBe(cacheKey("zh-CN", "m2", "hello"));
    expect(cacheKey("zh-CN", "m1", "hello")).not.toBe(cacheKey("en", "m1", "hello"));
    expect(cacheKey("zh-CN", "m1", "a  b")).toBe(cacheKey("zh-CN", "m1", "a b"));
  });

  test("evicts the least recently used entry", () => {
    const cache = new LruCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    expect(cache.get("a")).toBe(1); // "a" becomes newest
    cache.set("c", 3); // evicts "b"
    expect(cache.has("b")).toBe(false);
    expect(cache.has("a")).toBe(true);
    expect(cache.has("c")).toBe(true);
    expect(cache.size).toBe(2);
  });
});
