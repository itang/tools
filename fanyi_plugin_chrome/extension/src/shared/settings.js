/** Settings model, defaults, validation and persistence. */

import { DEFAULT_BATCH_SIZE, DEFAULT_MODEL, DEFAULT_TARGET_LANG } from "./constants.js";

export const DEFAULT_SETTINGS = Object.freeze({
  model: DEFAULT_MODEL,
  agent: "",
  targetLang: DEFAULT_TARGET_LANG,
  batchSize: DEFAULT_BATCH_SIZE,
  sites: Object.freeze({ "x.com": true, "twitter.com": true }),
});

const MAX_BATCH_SIZE = 200;

/**
 * Merge stored values over defaults, coercing and ignoring invalid input.
 * @param {unknown} stored
 */
export function normalizeSettings(stored) {
  const base = {
    ...DEFAULT_SETTINGS,
    sites: { ...DEFAULT_SETTINGS.sites },
  };
  if (!stored || typeof stored !== "object") return base;

  if (typeof stored.model === "string") base.model = stored.model.trim();
  if (typeof stored.agent === "string") base.agent = stored.agent.trim();
  if (typeof stored.targetLang === "string" && stored.targetLang.trim()) {
    base.targetLang = stored.targetLang.trim();
  }
  const batch = Number(stored.batchSize);
  if (Number.isInteger(batch) && batch > 0 && batch <= MAX_BATCH_SIZE) base.batchSize = batch;

  if (stored.sites && typeof stored.sites === "object" && !Array.isArray(stored.sites)) {
    base.sites = { ...base.sites };
    for (const [host, enabled] of Object.entries(stored.sites)) {
      if (typeof enabled === "boolean") base.sites[host.toLowerCase()] = enabled;
    }
  }
  return base;
}

/** Whether the extension is enabled for the given hostname (default: off). */
export function isSiteEnabled(settings, host) {
  if (!host) return false;
  const normalized = normalizeSettings(settings);
  return normalized.sites[String(host).toLowerCase()] === true;
}

/** Match patterns for every enabled site. */
export function enabledOrigins(settings) {
  const normalized = normalizeSettings(settings);
  return Object.entries(normalized.sites)
    .filter(([, enabled]) => enabled === true)
    .map(([host]) => `*://${host}/*`);
}

/** Hostname of a URL, or "" when it cannot be parsed. */
export function hostOf(url) {
  try {
    return new URL(url).hostname.toLowerCase();
  } catch {
    return "";
  }
}

/** Load settings from chrome.storage.sync, falling back to defaults. */
export async function loadSettings() {
  const stored = await chrome.storage.sync.get("settings");
  return normalizeSettings(stored.settings);
}

/** Persist a settings patch and return the merged result. */
export async function saveSettings(patch) {
  const current = await loadSettings();
  const next = normalizeSettings({ ...current, ...patch });
  await chrome.storage.sync.set({ settings: next });
  return next;
}
