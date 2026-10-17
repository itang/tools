/** Batch translation contract: prompt building, response parsing, alignment. */

import { ERROR_CODES } from "./protocol.js";

/**
 * Instruction sent to the model. The source segments are supplied on stdin and
 * the model must answer with a JSON object keyed by segment index so that
 * results stay aligned even if the model reorders them.
 * @param {string} targetLang
 */
export function buildInstruction(targetLang) {
  return [
    "You are a translation engine.",
    `Translate the "text" field of every item in the JSON array provided on stdin into ${targetLang}.`,
    "Some texts contain markup tokens: [[n]] and [[/n]] wrap an inline element, {{n}} is an opaque node, {{br}} is a line break.",
    "Keep every token byte-for-byte, in the same order; never translate, drop, renumber, duplicate or add tokens.",
    "Translate only the natural language between tokens.",
    "Preserve proper nouns, @handles, #hashtags, URLs, numbers and code as-is.",
    "Do not translate text that is already in the target language.",
    'Respond with ONLY a JSON object mapping each item index (as a string) to its translation, e.g. {"0":"...","1":"..."}.',
    "No prose, no markdown, no code fences.",
  ].join(" ");
}

/** Serialize segments to the stdin payload. */
export function buildStdinPayload(segments) {
  return JSON.stringify(segments.map((s) => ({ index: s.index, text: s.text })));
}

/**
 * Pull a JSON object out of a raw model answer, tolerating markdown fences and
 * surrounding prose.
 * @param {string} raw
 * @returns {Record<string, unknown>}
 */
export function parseTranslationResponse(raw) {
  let text = String(raw ?? "").trim();
  const fence = text.match(/```(?:json)?\s*([\s\S]*?)```/i);
  if (fence) text = fence[1].trim();

  const start = text.indexOf("{");
  const end = text.lastIndexOf("}");
  if (start === -1 || end === -1 || end < start) {
    throw new Error("模型未返回 JSON 对象");
  }
  const parsed = JSON.parse(text.slice(start, end + 1));
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("模型返回的 JSON 不是对象");
  }
  return parsed;
}

/**
 * Align a parsed `index -> translation` mapping back onto the original
 * segments.
 *
 * @param {import("./protocol.js").TranslateSegment[]} segments
 * @param {Record<string, unknown>} mapping
 * @returns {{results: {index: number, text: string}[], missing: import("./protocol.js").TranslateSegment[], extra: string[]}}
 */
export function alignTranslations(segments, mapping) {
  const results = [];
  const missing = [];
  const wanted = new Set(segments.map((s) => String(s.index)));

  for (const segment of segments) {
    const key = String(segment.index);
    const value = mapping[key];
    if (typeof value === "string" && value.length > 0) {
      results.push({ index: segment.index, text: value });
    } else {
      missing.push(segment);
    }
  }

  const extra = Object.keys(mapping).filter((key) => !wanted.has(key));
  return { results, missing, extra };
}

/**
 * Turn a batch response into per-segment results, retrying the segments that
 * the model omitted exactly once.
 *
 * @param {import("./protocol.js").TranslateSegment[]} segments
 * @param {(batch: import("./protocol.js").TranslateSegment[]) => Promise<{raw: string}>} callModel
 * @returns {Promise<import("./protocol.js").TranslationResult[]>}
 */
export async function translateBatch(segments, callModel) {
  const { raw } = await callModel(segments);
  const mapping = parseTranslationResponse(raw);
  const { results, missing } = alignTranslations(segments, mapping);

  if (missing.length === 0) {
    return results;
  }

  // One retry for the segments the model skipped.
  const retry = await callModel(missing);
  const retryMapping = parseTranslationResponse(retry.raw);
  const retryAligned = alignTranslations(missing, retryMapping);
  const byIndex = new Map(results.map((r) => [r.index, r]));

  for (const item of retryAligned.results) {
    byIndex.set(item.index, item);
  }
  for (const item of retryAligned.missing) {
    byIndex.set(item.index, { index: item.index, error: ERROR_CODES.INCOMPLETE });
  }

  return segments.map((s) => byIndex.get(s.index) ?? { index: s.index, error: ERROR_CODES.INCOMPLETE });
}
