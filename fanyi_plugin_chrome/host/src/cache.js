/** Translation result cache with LRU eviction. */

import { createHash } from "node:crypto";

/** Collapse whitespace so cosmetic differences share a cache entry. */
export function normalizeText(text) {
  return String(text ?? "")
    .replace(/\s+/g, " ")
    .trim();
}

/** Stable content hash for a normalized string. */
export function hashText(text) {
  return createHash("sha256").update(normalizeText(text)).digest("hex");
}

/** Cache key for one (language, model, text) triple. */
export function cacheKey(targetLang, model, text) {
  return `${targetLang}\u0000${model || ""}\u0000${hashText(text)}`;
}

export class LruCache {
  /** @param {number} [max] */
  constructor(max = 1000) {
    this.max = Math.max(1, Number(max) || 1);
    /** @type {Map<string, string>} */
    this.map = new Map();
  }

  get size() {
    return this.map.size;
  }

  has(key) {
    return this.map.has(key);
  }

  get(key) {
    if (!this.map.has(key)) return undefined;
    const value = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, value);
    return value;
  }

  set(key, value) {
    if (this.map.has(key)) this.map.delete(key);
    this.map.set(key, value);
    while (this.map.size > this.max) {
      const oldest = this.map.keys().next().value;
      this.map.delete(oldest);
    }
  }
}
