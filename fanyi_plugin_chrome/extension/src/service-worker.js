/**
 * MV3 service worker: owns the native messaging port, routes translation
 * requests from content scripts, forwards progress, and wires up the toolbar
 * action, context menus and keyboard commands.
 */

import { HOST_NAME } from "./shared/constants.js";
import { MSG } from "./shared/messages.js";
import { createFrameAccumulator } from "./shared/host-frames.js";
import { enabledOrigins, loadSettings } from "./shared/settings.js";

const CONTENT_SCRIPT_ID = "oc-translate-content";
const CONTENT_FILE = "content.js";

// --- Native messaging client -------------------------------------------------

/** @type {chrome.runtime.Port | null} */
let port = null;
/** @type {Map<string, {resolve: Function, onProgress?: Function, accumulator: ReturnType<typeof createFrameAccumulator>}>} */
const handlers = new Map();

function ensurePort() {
  if (port) return port;
  port = chrome.runtime.connectNative(HOST_NAME);
  port.onMessage.addListener(onHostMessage);
  port.onDisconnect.addListener(onHostDisconnect);
  return port;
}

function onHostDisconnect() {
  const message = chrome.runtime.lastError?.message || "本地翻译主机已断开";
  port = null;
  for (const handler of handlers.values()) {
    handler.resolve({ error: { code: "HOST_UNAVAILABLE", message } });
  }
  handlers.clear();
}

function onHostMessage(frame) {
  if (!frame || typeof frame.id !== "string") return;
  const handler = handlers.get(frame.id);
  if (!handler) return;

  if (frame.type === "progress") {
    handler.onProgress?.(frame);
    return;
  }
  if (frame.last === false) {
    handler.accumulator.add(frame);
    return;
  }
  handlers.delete(frame.id);
  handler.resolve(handler.accumulator.resolve(frame));
}

/**
 * @param {object} request
 * @param {(frame: any) => void} [onProgress]
 */
function hostRequest(request, onProgress) {
  return new Promise((resolve) => {
    const id = crypto.randomUUID();
    handlers.set(id, { resolve, onProgress, accumulator: createFrameAccumulator() });
    try {
      ensurePort().postMessage({ id, ...request });
    } catch (error) {
      handlers.delete(id);
      resolve({ error: { code: "HOST_UNAVAILABLE", message: String(error?.message ?? error) } });
    }
  });
}

// --- Content script communication -------------------------------------------

function sendToTab(tabId, message) {
  if (tabId == null) return Promise.resolve();
  return chrome.tabs.sendMessage(tabId, message).catch(() => {});
}

async function activeTabId() {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  return tab?.id;
}

function setBadge(tabId, text) {
  if (tabId == null) return;
  chrome.action.setBadgeText({ tabId, text }).catch(() => {});
}

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message?.type !== MSG.HOST_REQUEST || !message.request) return false;

  const tabId = sender.tab?.id;
  const onProgress = (frame) => {
    setBadge(tabId, `${frame.completed}/${frame.total}`);
    sendToTab(tabId, {
      type: MSG.HOST_PROGRESS,
      id: message.id,
      progress: { phase: frame.phase, completed: frame.completed, total: frame.total },
    });
  };

  hostRequest(message.request, onProgress)
    .then((result) => {
      setBadge(tabId, "");
      sendResponse(result);
    })
    .catch((error) => {
      setBadge(tabId, "");
      sendResponse({ error: { code: "HOST_ERROR", message: String(error?.message ?? error) } });
    });
  return true; // keep the channel open for the async response
});

// --- Toolbar action ----------------------------------------------------------

chrome.action.setBadgeBackgroundColor({ color: "#1d9bf0" }).catch(() => {});

chrome.action.onClicked.addListener((tab) => {
  sendToTab(tab.id, { type: MSG.TOGGLE_PAGE });
});

// --- Context menus -----------------------------------------------------------

const MENUS = [
  { id: "oc-translate-page", title: "翻译此页面", contexts: ["page", "frame"] },
  { id: "oc-translate-selection", title: "翻译选中内容", contexts: ["selection"] },
  { id: "oc-translate-region", title: "翻译此区域", contexts: ["page", "frame"] },
  { id: "oc-revert-page", title: "撤销本页翻译", contexts: ["page", "frame"] },
  { id: "oc-options", title: "设置…", contexts: ["action"] },
];

function createMenus() {
  chrome.contextMenus.removeAll(() => {
    for (const menu of MENUS) chrome.contextMenus.create(menu);
  });
}

chrome.contextMenus.onClicked.addListener((info, tab) => {
  switch (info.menuItemId) {
    case "oc-translate-page":
      sendToTab(tab?.id, { type: MSG.TRANSLATE_PAGE });
      break;
    case "oc-translate-selection":
      sendToTab(tab?.id, { type: MSG.TRANSLATE_SELECTION });
      break;
    case "oc-translate-region":
      sendToTab(tab?.id, { type: MSG.TRANSLATE_REGION });
      break;
    case "oc-revert-page":
      sendToTab(tab?.id, { type: MSG.REVERT_PAGE });
      break;
    case "oc-options":
      chrome.runtime.openOptionsPage();
      break;
    default:
      break;
  }
});

// --- Keyboard commands -------------------------------------------------------

chrome.commands.onCommand.addListener(async (command, tab) => {
  const tabId = tab?.id ?? (await activeTabId());
  if (command === "toggle-translate") sendToTab(tabId, { type: MSG.TOGGLE_PAGE });
  else if (command === "translate-selection") sendToTab(tabId, { type: MSG.TRANSLATE_SELECTION });
});

// --- Dynamic content script registration ------------------------------------

async function syncContentScripts() {
  const settings = await loadSettings();
  const matches = enabledOrigins(settings);
  const existing = await chrome.scripting.getRegisteredContentScripts({ ids: [CONTENT_SCRIPT_ID] }).catch(() => []);

  if (matches.length === 0) {
    if (existing.length > 0) {
      await chrome.scripting.unregisterContentScripts({ ids: [CONTENT_SCRIPT_ID] }).catch(() => {});
    }
    return;
  }

  const script = {
    id: CONTENT_SCRIPT_ID,
    matches,
    js: [CONTENT_FILE],
    runAt: "document_idle",
    persistAcrossSessions: true,
  };
  if (existing.length > 0) await chrome.scripting.updateContentScripts([script]).catch(() => {});
  else await chrome.scripting.registerContentScripts([script]).catch(() => {});
}

chrome.runtime.onInstalled.addListener(() => {
  createMenus();
  syncContentScripts();
});
chrome.runtime.onStartup.addListener(() => {
  createMenus();
  syncContentScripts();
});

chrome.storage.onChanged.addListener((changes, area) => {
  if (area === "sync" && changes.settings) syncContentScripts();
});

// Best effort on cold start so enabled sites work without installing/updating.
syncContentScripts();
