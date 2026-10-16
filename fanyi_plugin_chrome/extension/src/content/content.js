/**
 * Content script entry point. Runs in the page and orchestrates: collect
 * translation units, request translations from the service worker, render
 * them inline, watch for dynamically loaded content and support reverting.
 */

import { MSG } from "../shared/messages.js";
import { messageForCode } from "../shared/constants.js";
import { isSiteEnabled, loadSettings } from "../shared/settings.js";
import { isTranslatableText, normalizeText } from "./extract.js";
import { hasTranslations, insertTranslation, revertAll } from "./render.js";
import { collectUnits, hashText, unitForElement } from "./units.js";
import { createStatusUI } from "./status-ui.js";

const ANCHOR_SELECTOR = "p,li,div,blockquote,article,section,h1,h2,h3,h4,h5,h6,span,td";
const OBSERVER_DEBOUNCE_MS = 400;

const status = createStatusUI(document);

const state = {
  settings: null,
  enabled: false,
  translating: false,
  processed: new WeakMap(),
  observer: null,
  debounce: null,
  lastContextTarget: null,
};

/** Send a request to the host through the service worker, tracking progress. */
function requestHost(request) {
  return new Promise((resolve) => {
    const id = crypto.randomUUID ? crypto.randomUUID() : `${Date.now()}-${Math.random()}`;
    let settled = false;

    const onProgress = (message) => {
      if (message?.type === MSG.HOST_PROGRESS && message.id === id && message.progress) {
        status.progress(message.progress.completed, message.progress.total);
      }
    };

    const finish = (result) => {
      if (settled) return;
      settled = true;
      chrome.runtime.onMessage.removeListener(onProgress);
      resolve(result);
    };

    chrome.runtime.onMessage.addListener(onProgress);
    try {
      chrome.runtime.sendMessage({ type: MSG.HOST_REQUEST, id, request }, (response) => {
        if (chrome.runtime.lastError) {
          finish({ error: { code: "HOST_UNAVAILABLE", message: chrome.runtime.lastError.message } });
        } else {
          finish(response || { error: { code: "HOST_ERROR", message: "空响应" } });
        }
      });
    } catch (error) {
      finish({ error: { code: "HOST_UNAVAILABLE", message: String(error?.message ?? error) } });
    }
  });
}

async function ensureSettings() {
  if (!state.settings) state.settings = await loadSettings();
  state.enabled = isSiteEnabled(state.settings, location.hostname);
  return state.settings;
}

function buildOptions() {
  const settings = state.settings;
  return {
    targetLang: settings.targetLang,
    model: settings.model || undefined,
    agent: settings.agent || undefined,
  };
}

/** Translate a list of units, batching and rendering as results arrive. */
async function translateUnits(units) {
  const batchSize = state.settings.batchSize;
  let translated = 0;

  for (let offset = 0; offset < units.length; offset += batchSize) {
    const batch = units.slice(offset, offset + batchSize);
    const segments = batch.map((unit, index) => ({ index, text: unit.text }));
    status.progress(offset, units.length);

    const response = await requestHost({ type: "translate", segments, options: buildOptions() });
    if (response.error) {
      status.error(messageForCode(response.error.code));
      return;
    }

    const byIndex = new Map((response.translations || []).map((item) => [item.index, item]));
    for (let index = 0; index < batch.length; index += 1) {
      const unit = batch[index];
      if (!unit.element.isConnected) continue;
      const result = byIndex.get(index);
      if (result?.text) {
        insertTranslation(document, unit.element, result.text, { inline: unit.inline });
        translated += 1;
      }
      // Record success and failure alike so the observer never loops on a
      // segment that cannot be translated.
      state.processed.set(unit.element, unit.key);
    }
  }

  status.show(`已翻译 ${translated}/${units.length} 处`);
}

async function translatePage() {
  if (state.translating) return;
  state.translating = true;
  try {
    const units = collectUnits(document, document.body, state.processed);
    if (units.length === 0) {
      status.show(hasTranslations(document) ? "本页已全部翻译。" : "没有可翻译的内容。");
    } else {
      await translateUnits(units);
    }
    startObserver();
  } finally {
    state.translating = false;
  }
}

async function translateSelection() {
  const selection = document.getSelection();
  const text = selection ? normalizeText(selection.toString()) : "";
  if (!text) {
    await translateRegion();
    return;
  }
  if (!isTranslatableText(text)) {
    status.show("选中的内容无需翻译。");
    return;
  }

  const range = selection.getRangeAt(0);
  const container = range.commonAncestorContainer;
  const element = container.nodeType === 1 ? container : container.parentElement;
  if (!element) return;

  state.translating = true;
  try {
    const response = await requestHost({
      type: "translate",
      segments: [{ index: 0, text }],
      options: buildOptions(),
    });
    if (response.error) {
      status.error(messageForCode(response.error.code));
      return;
    }
    const translation = response.translations?.[0]?.text;
    if (!translation) {
      status.error(messageForCode("INCOMPLETE"));
      return;
    }
    const anchor = element.closest?.(ANCHOR_SELECTOR) || element;
    insertTranslation(document, anchor, translation, { inline: false });
    state.processed.set(anchor, hashText(normalizeText(anchor.textContent)));
    status.show("已翻译选中内容");
  } finally {
    state.translating = false;
  }
}

async function translateRegion() {
  const target = state.lastContextTarget;
  if (!target || !target.isConnected) {
    status.error("请在要翻译的区域上右键后再试。");
    return;
  }

  state.translating = true;
  try {
    let units = collectUnits(document, target, state.processed);
    if (units.length === 0) {
      const anchor = target.closest?.(ANCHOR_SELECTOR) || target;
      const single = unitForElement(anchor);
      if (single && state.processed.get(anchor) !== single.key) units = [single];
    }
    if (units.length === 0) {
      status.show("该区域没有可翻译的内容。");
      return;
    }
    await translateUnits(units);
  } finally {
    state.translating = false;
  }
}

function revert() {
  stopObserver();
  const removed = revertAll(document);
  state.processed = new WeakMap();
  status.show(removed > 0 ? `已撤销 ${removed} 处翻译` : "没有可撤销的翻译。");
}

async function togglePage() {
  if (hasTranslations(document)) {
    revert();
    return;
  }
  await translatePage();
}

function startObserver() {
  if (state.observer) return;
  state.observer = new MutationObserver(() => {
    if (state.translating || !state.enabled) return;
    clearTimeout(state.debounce);
    state.debounce = setTimeout(runObserverPass, OBSERVER_DEBOUNCE_MS);
  });
  state.observer.observe(document.body, { childList: true, subtree: true });
}

function stopObserver() {
  if (state.observer) {
    state.observer.disconnect();
    state.observer = null;
  }
  clearTimeout(state.debounce);
}

async function runObserverPass() {
  if (state.translating || !state.enabled) return;
  state.translating = true;
  try {
    const units = collectUnits(document, document.body, state.processed);
    if (units.length > 0) await translateUnits(units);
  } finally {
    state.translating = false;
  }
}

async function handleCommand(message) {
  if (!message?.type || message.type === MSG.HOST_PROGRESS) return;
  await ensureSettings();

  if (!state.enabled && message.type !== MSG.REVERT_PAGE) {
    status.error("该站点未启用翻译，请在扩展设置中启用。");
    return;
  }

  switch (message.type) {
    case MSG.TOGGLE_PAGE:
      await togglePage();
      break;
    case MSG.TRANSLATE_PAGE:
      if (hasTranslations(document)) status.show("本页已翻译。");
      else await translatePage();
      break;
    case MSG.REVERT_PAGE:
      revert();
      break;
    case MSG.TRANSLATE_SELECTION:
      await translateSelection();
      break;
    case MSG.TRANSLATE_REGION:
      await translateRegion();
      break;
    default:
      break;
  }
}

chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (!message?.type || message.type === MSG.HOST_PROGRESS) return false;
  handleCommand(message)
    .then(() => sendResponse({ ok: true }))
    .catch((error) => sendResponse({ ok: false, error: String(error?.message ?? error) }));
  return true;
});

document.addEventListener(
  "contextmenu",
  (event) => {
    state.lastContextTarget = event.target;
  },
  true,
);

try {
  chrome.storage.onChanged.addListener((changes, area) => {
    if (area === "sync" && changes.settings) {
      state.settings = null;
      ensureSettings();
    }
  });
} catch {
  // storage may be unavailable in some contexts
}

ensureSettings();
