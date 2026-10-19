/**
 * Youdao web translate client (fanyi.youdao.com's own endpoint), used by the
 * service worker as an alternative to the native opencode host.
 *
 * Flow per request batch:
 *   1. POST /translate/key (signed with the public static key-getter secret)
 *      -> { token, secretKey } (cached, refreshed on auth failures)
 *   2. POST /webtranslate/sse with the segment as form field `i`, signed with
 *      the returned secretKey; the SSE body carries the translation as
 *      incremental `transIncre` chunks.
 *
 * Kept free of chrome APIs so it is unit-testable.
 */

import { md5 } from "./md5.js";

export const YOUDAO_BASE = "https://dict-trans.youdao.com";

/** Static secret published in fanyi.youdao.com's bundle for the key getter. */
const KEY_GETTER_SECRET = "kSy5gtKA4yRUxAVPJPrdYKZ0jBKyd3t1";
const KEY_ID = "translate-webfanyi-webmain";
const COMMON_PARAMS = Object.freeze({
  product: "webfanyi",
  client: "webmain",
  mid: "1",
  vendor: "web",
  screen: "1",
  model: "1",
  imei: "1",
  network: "wifi",
  keyfrom: "webfanyi.webmain",
});

const KEY_TTL_MS = 10 * 60 * 1000;
const REQUEST_TIMEOUT_MS = 30_000;
const CONCURRENCY = 4;
const CACHE_MAX = 2000;

/**
 * Youdao signs every request as md5(sorted "k=v&" params + "key=<secretKey>"),
 * empty values dropped; the joined key list is echoed back as `pointParam`.
 */
export function genSign(params, secretKey) {
  const values = { ...params };
  for (const key of Object.keys(values)) {
    if (values[key] === "") delete values[key];
  }
  const keys = Object.keys(values)
    .filter((key) => values[key] !== undefined)
    .sort();
  keys.push("key");
  const line = keys.map((key) => `${key}=${key === "key" ? secretKey : values[key]}`).join("&");
  return { sign: md5(line), pointParam: keys.join(",") };
}

const LANG_MAP = new Map([
  ["zh", "zh-CHS"],
  ["zh-cn", "zh-CHS"],
  ["zh-hans", "zh-CHS"],
  ["zh-tw", "zh-CHT"],
  ["zh-hant", "zh-CHT"],
  ["en", "en"],
  ["ja", "ja"],
  ["jp", "ja"],
  ["ko", "ko"],
  ["fr", "fr"],
  ["de", "de"],
  ["es", "es"],
  ["ru", "ru"],
  ["pt", "pt"],
  ["it", "it"],
  ["ar", "ar"],
  ["th", "th"],
  ["vi", "vi"],
  ["id", "id"],
  ["ms", "ms"],
  ["tr", "tr"],
  ["nl", "nl"],
  ["pl", "pl"],
]);

/** Map a BCP-47-ish target language to a youdao language code. */
export function normalizeLang(targetLang) {
  const raw = String(targetLang || "").toLowerCase().trim();
  if (!raw) return "zh-CHS";
  const primary = raw.split(/[-_]/)[0];
  return LANG_MAP.get(raw) || LANG_MAP.get(primary) || raw;
}

function errorResult(code, message) {
  return { error: { code, message: String(message) } };
}

/**
 * @param {{ uuid: string, fetchImpl?: typeof fetch }} options
 *   `uuid` is a stable per-install device id sent as `yduuid`.
 */
export function createYoudaoClient({ uuid, fetchImpl = (...args) => fetch(...args) }) {
  /** @type {{secretKey: string, token: string, fetchedAt: number} | null} */
  let key = null;
  let keyPromise = null;
  const cache = new Map();

  async function fetchJson(url, init) {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetchImpl(url, { ...init, signal: controller.signal });
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      return await response.json();
    } finally {
      clearTimeout(timer);
    }
  }

  async function fetchKey(force) {
    if (!force && key && Date.now() - key.fetchedAt < KEY_TTL_MS) return key;
    if (keyPromise) return keyPromise;
    keyPromise = (async () => {
      const params = {
        ...COMMON_PARAMS,
        appVersion: "12.0.0",
        keyid: "translate-webmain-key-getter",
        mysticTime: String(Date.now()),
        yduuid: uuid,
        abtest: "0",
        targetKeyid: KEY_ID,
      };
      if (force && key?.token) params.token = key.token;
      const { sign, pointParam } = genSign(params, KEY_GETTER_SECRET);
      const query = Object.entries({ ...params, sign, pointParam })
        .map(([k, v]) => `${k}=${encodeURIComponent(v)}`)
        .join("&");
      const json = await fetchJson(`${YOUDAO_BASE}/translate/key?${query}`, { method: "POST" });
      if (json.code !== 0 || !json.data?.secretKey) {
        throw new Error(`key 接口返回异常 code=${json.code} msg=${json.msg}`);
      }
      key = { secretKey: json.data.secretKey, token: json.data.token, fetchedAt: Date.now() };
      return key;
    })().finally(() => {
      keyPromise = null;
    });
    return keyPromise;
  }

  /** Concatenate the `transIncre` chunks of an SSE payload. */
  function readSse(body) {
    let out = "";
    for (const line of body.split("\n")) {
      if (!line.startsWith("data:")) continue;
      try {
        const json = JSON.parse(line.slice(5));
        if (typeof json.transIncre === "string") out += json.transIncre;
      } catch {
        // Ignore non-JSON frames (begin/end carry metadata we don't need).
      }
    }
    return out;
  }

  async function requestTranslate(text, to, attemptedRefresh) {
    const active = await fetchKey(false);
    const params = {
      ...COMMON_PARAMS,
      appVersion: "1",
      keyid: KEY_ID,
      mysticTime: String(Date.now()),
      yduuid: uuid,
      modelName: "llmLite",
      useTerm: "false",
      i: encodeURIComponent(text),
      from: "auto",
      to,
      signSecretKey: active.secretKey,
      keyId: KEY_ID,
      token: active.token,
      source: "webmain",
    };
    const { sign, pointParam } = genSign(params, active.secretKey);
    const form = new FormData();
    for (const [k, v] of Object.entries({ ...params, sign, pointParam })) form.append(k, String(v));

    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    let response;
    try {
      response = await fetchImpl(`${YOUDAO_BASE}/webtranslate/sse`, { method: "POST", body: form, signal: controller.signal });
    } finally {
      clearTimeout(timer);
    }
    if (response.status === 401 || response.status === 403) {
      if (!attemptedRefresh) {
        await fetchKey(true);
        return requestTranslate(text, to, true);
      }
      throw new Error(`鉴权失败 HTTP ${response.status}`);
    }
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const translated = readSse(await response.text());
    if (!translated) throw new Error("返回空译文");
    return translated;
  }

  /** Translate one segment; resolves to "" when the segment failed. */
  async function translateOne(text, to) {
    const cacheKey = `${to}|${text}`;
    if (cache.has(cacheKey)) {
      const hit = cache.get(cacheKey);
      cache.delete(cacheKey);
      cache.set(cacheKey, hit);
      return hit;
    }
    const translated = await requestTranslate(text, to, false).catch(() => "");
    if (translated) {
      if (cache.size >= CACHE_MAX) cache.delete(cache.keys().next().value);
      cache.set(cacheKey, translated);
    }
    return translated;
  }

  return {
    /**
     * Translate `segments` ([{index, text}]) into `targetLang`.
     * Resolves to a host-shaped result: {translations:[{index,text}]} or {error}.
     */
    async translateSegments(segments, targetLang, onProgress) {
      const to = normalizeLang(targetLang);
      const queue = segments.filter((s) => typeof s?.text === "string" && s.text.trim() !== "");
      const total = queue.length;
      if (total === 0) return { translations: [] };

      let completed = 0;
      let succeeded = 0;
      let networkFailure = 0;
      const translations = [];

      const worker = async () => {
        for (let segment = queue.shift(); segment; segment = queue.shift()) {
          let text = "";
          try {
            text = await translateOne(segment.text, to);
          } catch {
            networkFailure += 1;
          }
          if (text) {
            translations.push({ index: segment.index, text });
            succeeded += 1;
          }
          completed += 1;
          onProgress?.({ phase: "batch", completed, total });
        }
      };

      await Promise.all(Array.from({ length: Math.min(CONCURRENCY, total) }, worker));

      if (succeeded === 0 && networkFailure > 0) {
        return errorResult("YOUDAO_FAILED", `有道接口请求失败（${networkFailure} 次请求均出错）`);
      }
      return { translations };
    },

    /** Lightweight availability probe for the options page. */
    async health() {
      try {
        await fetchKey(false);
        return { health: { backend: "youdao", youdao: { ok: true } } };
      } catch (error) {
        return errorResult("YOUDAO_UNAVAILABLE", error?.message ?? error);
      }
    },
  };
}
